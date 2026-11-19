//! Command bridge between the egui terminal and the async `moneycraft_core::dispatch`.
//!
//! Native: a tokio runtime runs each command to completion (`block_on`), so the reply is
//! ready the moment [`Bridge::submit`] returns, exactly as before the UI crate split.
//!
//! Browser (wasm32): nothing may block the page, so each command runs on the page's event
//! loop (`spawn_local`) and its reply lands in an inbox the GUI drains at the start of every
//! frame. A finished command asks egui for a repaint, so the reply shows without input.

use databasecraft_store::DatabaseStore;
use moneycraft_core::{dispatch, Command};
use serde_json::Value;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

/// What the GUI does with a command's reply once it arrives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reply {
    /// Carries the request's parameters so a late reply for an old selection is dropped.
    MarketFeed {
        symbol: String,
        bars: usize,
        source: String,
    },
    Comparison,
    Derivatives,
    News,
    PaperAccount,
    /// A paper-trading action: show its message (or `fallback`), then refresh the account.
    PaperAction {
        fallback: &'static str,
    },
    AiWorkflow {
        workflow: &'static str,
    },
    Audit,
    Backtest,
    Portfolio,
}

type Inbox = Rc<RefCell<Vec<(Reply, Value)>>>;

pub struct Bridge {
    store: Arc<DatabaseStore>,
    inbox: Inbox,
    #[cfg(not(target_arch = "wasm32"))]
    runtime: Arc<tokio::runtime::Runtime>,
    #[cfg(target_arch = "wasm32")]
    pending: Rc<std::cell::Cell<usize>>,
    #[cfg(target_arch = "wasm32")]
    ctx: eframe::egui::Context,
}

impl Bridge {
    /// Desktop: commands run on `runtime` and finish before `submit` returns.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn native(store: Arc<DatabaseStore>, runtime: Arc<tokio::runtime::Runtime>) -> Self {
        Self {
            store,
            inbox: Inbox::default(),
            runtime,
        }
    }

    /// Browser: commands run on the page's event loop; `ctx` repaints when one finishes.
    #[cfg(target_arch = "wasm32")]
    pub fn web(store: Arc<DatabaseStore>, ctx: eframe::egui::Context) -> Self {
        Self {
            store,
            inbox: Inbox::default(),
            pending: Rc::default(),
            ctx,
        }
    }

    pub fn store(&self) -> &Arc<DatabaseStore> {
        &self.store
    }

    /// Start `cmd`. Its reply is handed out by [`Bridge::take_ready`] with `reply` attached.
    pub fn submit(&self, cmd: Command, reply: Reply) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let value = self.runtime.block_on(dispatch(cmd, &self.store));
            self.inbox.borrow_mut().push((reply, value));
        }
        #[cfg(target_arch = "wasm32")]
        {
            let store = Arc::clone(&self.store);
            let inbox = Rc::clone(&self.inbox);
            let pending = Rc::clone(&self.pending);
            let ctx = self.ctx.clone();
            pending.set(pending.get() + 1);
            wasm_bindgen_futures::spawn_local(async move {
                let value = dispatch(cmd, &store).await;
                inbox.borrow_mut().push((reply, value));
                pending.set(pending.get().saturating_sub(1));
                ctx.request_repaint();
            });
        }
    }

    /// Every reply that has arrived since the last call, oldest first.
    pub fn take_ready(&self) -> Vec<(Reply, Value)> {
        std::mem::take(&mut *self.inbox.borrow_mut())
    }

    /// Commands still running (always 0 on native, where commands finish inside `submit`).
    pub fn in_flight(&self) -> usize {
        #[cfg(not(target_arch = "wasm32"))]
        {
            0
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.pending.get()
        }
    }
}
