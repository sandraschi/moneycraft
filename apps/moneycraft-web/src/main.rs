//! MoneyCraft in the browser.
//!
//! Runs the same [`moneycraft_ui::MoneyCraftGui`] as the desktop app through eframe's web
//! runner. Build with `trunk build --release` from this folder (or `just web-build` from the
//! repository root); the site lands in `dist/web`.
//!
//! Differences from the desktop app:
//! - commands run on the page's event loop, so data shows a frame or more after a click;
//! - the SQLite store is sqlite-wasm in memory: nothing persists across reloads yet;
//! - market feeds call the providers straight from the browser, so a provider that sends no
//!   CORS headers fails and the app shows its offline fallback for that feed;
//! - no `--control` port and no MCP server (browsers cannot listen on sockets).

#[cfg(target_arch = "wasm32")]
fn main() {
    use moneycraft_ui::{Bridge, MoneyCraftGui};
    use std::sync::Arc;
    use wasm_bindgen::JsCast as _;

    const CANVAS_ID: &str = "moneycraft_canvas";
    const LOADING_ID: &str = "moneycraft_loading";

    eframe::WebLogger::init(log::LevelFilter::Info).ok();

    wasm_bindgen_futures::spawn_local(async {
        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            log::error!("MoneyCraft: no document");
            return;
        };
        let show_error = |msg: &str| {
            if let Some(el) = document.get_element_by_id(LOADING_ID) {
                el.set_inner_html(&format!("<p>MoneyCraft failed to start: {msg}</p>"));
            }
        };
        let Some(canvas) = document
            .get_element_by_id(CANVAS_ID)
            .and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok())
        else {
            show_error("the page has no #moneycraft_canvas");
            return;
        };
        let store = match databasecraft_store::DatabaseStore::open_in_memory() {
            Ok(s) => Arc::new(s),
            Err(e) => {
                show_error(&format!("the in-browser SQLite store did not open ({e})"));
                return;
            }
        };

        let result = eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(move |cc| {
                    Ok(Box::new(MoneyCraftGui::new(Bridge::web(
                        store,
                        cc.egui_ctx.clone(),
                    ))))
                }),
            )
            .await;

        match result {
            Ok(()) => {
                if let Some(el) = document.get_element_by_id(LOADING_ID) {
                    el.remove();
                }
            }
            Err(e) => show_error(&format!(
                "{e:?}. A browser with WebGL2 or WebGPU is required."
            )),
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!(
        "moneycraft-web only runs in the browser: `trunk serve` in apps/moneycraft-web, or `just web`"
    );
}
