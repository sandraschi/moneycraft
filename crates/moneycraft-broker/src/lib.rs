//! Broker abstraction: one trait, one adapter per venue, paper by default.
//!
//! Live venues arrive later as cargo features (`broker-oanda`, `broker-alpaca`,
//! `broker-ibkr`, `broker-kraken`). The default build ships only [`PaperBroker`],
//! so no binary compiled from this tree today can send a real order.
//! Every order carries `dry_run`: validate-and-report without sending anything.
//!
//! Safety model (see LIVE_TRADING_PLAN.md §5): paper default, explicit venue,
//! [`BrokerLimits`] enforced inside [`Broker::check_order`] — never in the UI.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// Venue id of the built-in simulator. Also the only venue that exists
/// until a live adapter feature is added.
pub const PAPER_VENUE: &str = "paper";

/// Risk rails enforced on every order, live or paper. Conservative defaults:
/// Joe-scale notional, single-digit leverage. Raise deliberately, per venue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerLimits {
    /// Reject any order above this notional (price × quantity).
    pub max_notional_per_order: f64,
    /// Reject leverage above this (1.0 = spot).
    pub max_leverage: f64,
    /// Empty = every symbol allowed; otherwise only listed symbols.
    pub allow_list: Vec<String>,
}

impl Default for BrokerLimits {
    fn default() -> Self {
        Self {
            max_notional_per_order: 25_000.0,
            max_leverage: 5.0,
            allow_list: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Long,
    Short,
}

impl Side {
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_uppercase().as_str() {
            "LONG" | "BUY" => Some(Self::Long),
            "SHORT" | "SELL" => Some(Self::Short),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Long => "LONG",
            Self::Short => "SHORT",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Spot,
    Perp,
}

impl OrderType {
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_uppercase().replace('-', "_").as_str() {
            "SPOT" => Some(Self::Spot),
            "PERP" | "PERP_FUTURES" | "PERPETUAL" => Some(Self::Perp),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Spot => "SPOT",
            Self::Perp => "PERP_FUTURES",
        }
    }
}

/// One order intent. `dry_run = true` validates against limits and reports
/// the would-be fill without touching any venue (or the paper book).
#[derive(Debug, Clone)]
pub struct NewOrder {
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    pub quantity: f64,
    pub leverage: f64,
    pub limit_price: Option<f64>,
    pub dry_run: bool,
}

/// What the venue (or the simulator) did with the order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAck {
    pub venue_order_id: String,
    pub venue: String,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub quantity: f64,
    pub leverage: f64,
    pub fill_price: Option<f64>,
    pub status: String,
    pub note: String,
}

/// Open position in venue terms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerPosition {
    pub venue_order_id: String,
    pub symbol: String,
    pub side: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub leverage: f64,
    pub liquidation_price: f64,
    pub margin_allocated: f64,
}

pub type BrokerError = String;

/// One adapter per venue. Paper first, live venues later behind features.
pub trait Broker: Send + Sync {
    fn venue(&self) -> &'static str;
    fn limits(&self) -> BrokerLimits;

    /// Validate against limits + symbol allow-list. No side effects.
    fn check_order(&self, order: &NewOrder, reference_price: f64) -> Result<(), BrokerError>;

    /// Fill (or dry-run report) one order. Implementations must call
    /// `check_order` first and refuse when it fails.
    fn place(&self, order: NewOrder, reference_price: f64) -> Result<OrderAck, BrokerError>;

    /// Currently held positions known to this adapter instance.
    fn positions(&self) -> Vec<BrokerPosition>;

    /// Cancel by venue order id. Returns true when something was cancelled.
    fn cancel(&self, venue_order_id: &str) -> Result<bool, BrokerError>;
}

/// Last close of the synthetic series — the simulator's "mark price".
/// Live adapters replace this with the venue's quote; the math stays.
#[must_use]
pub fn reference_price(symbol: &str) -> Option<f64> {
    moneycraft_feed::MarketFeed::mock_candles(symbol)
        .last()
        .map(|c| c.close)
}

/// Shared fill math (margin, liquidation) used by the paper adapter.
/// kept identical to the legacy fantasy-desk formulas so behavior is unchanged.
#[must_use]
pub fn size_position(fill_price: f64, quantity: f64, leverage: f64, long: bool) -> (f64, f64) {
    let notional = fill_price * quantity;
    let margin = if leverage > 0.0 {
        notional / leverage
    } else {
        notional
    };
    let liq = if long {
        fill_price * (1.0 - (0.9 / leverage.max(1.0)))
    } else {
        fill_price * (1.0 + (0.9 / leverage.max(1.0)))
    };
    (margin, liq)
}

/// In-memory simulator. Holds its own book for the process lifetime —
/// a fresh instance starts empty, exactly like the legacy fantasy desk.
pub struct PaperBroker {
    limits: BrokerLimits,
    positions: Mutex<HashMap<String, BrokerPosition>>,
    next_id: Mutex<u64>,
}

impl Default for PaperBroker {
    fn default() -> Self {
        Self {
            limits: BrokerLimits::default(),
            positions: Mutex::new(HashMap::new()),
            next_id: Mutex::new(1),
        }
    }
}

impl PaperBroker {
    #[must_use]
    pub fn with_limits(limits: BrokerLimits) -> Self {
        Self {
            limits,
            positions: Mutex::new(HashMap::new()),
            next_id: Mutex::new(1),
        }
    }
}

impl Broker for PaperBroker {
    fn venue(&self) -> &'static str {
        PAPER_VENUE
    }

    fn limits(&self) -> BrokerLimits {
        self.limits.clone()
    }

    fn check_order(&self, order: &NewOrder, reference_price: f64) -> Result<(), BrokerError> {
        if !(order.quantity.is_finite() && order.quantity > 0.0) {
            return Err("order refused: quantity must be a positive number.".to_string());
        }
        if !(reference_price.is_finite() && reference_price > 0.0) {
            return Err("order refused: no reference price for this symbol.".to_string());
        }
        if order.leverage < 1.0 || !order.leverage.is_finite() {
            return Err("order refused: leverage must be >= 1.".to_string());
        }
        if order.leverage > self.limits.max_leverage {
            return Err(format!(
                "order refused: leverage {:.1}x exceeds venue max {:.1}x.",
                order.leverage, self.limits.max_leverage
            ));
        }
        if let Some(p) = order.limit_price {
            if !(p.is_finite() && p > 0.0) {
                return Err("order refused: limit price must be positive.".to_string());
            }
        }
        let px = order.limit_price.unwrap_or(reference_price);
        if px * order.quantity > self.limits.max_notional_per_order {
            return Err(format!(
                "order refused: notional {:.2} exceeds venue max {:.2}.",
                px * order.quantity,
                self.limits.max_notional_per_order
            ));
        }
        if !self.limits.allow_list.is_empty()
            && !self
                .limits
                .allow_list
                .iter()
                .any(|s| s.eq_ignore_ascii_case(&order.symbol))
        {
            return Err(format!(
                "order refused: {} not on this venue's allow-list.",
                order.symbol
            ));
        }
        Ok(())
    }

    fn place(&self, order: NewOrder, reference_price: f64) -> Result<OrderAck, BrokerError> {
        self.check_order(&order, reference_price)?;
        let fill = order.limit_price.unwrap_or(reference_price);
        if order.dry_run {
            return Ok(OrderAck {
                venue_order_id: "DRY-RUN".to_string(),
                venue: self.venue().to_string(),
                symbol: order.symbol.clone(),
                side: order.side.as_str().to_string(),
                order_type: order.order_type.as_str().to_string(),
                quantity: order.quantity,
                leverage: order.leverage,
                fill_price: Some(fill),
                status: "DRY_RUN".to_string(),
                note: "Validated against venue limits. Nothing was sent or booked.".to_string(),
            });
        }
        let long = order.side == Side::Long;
        let (margin, liq) = size_position(fill, order.quantity, order.leverage, long);
        let mut id_guard = self.next_id.lock().map_err(|e| format!("lock: {e}"))?;
        let id = format!("PAPER-{}", *id_guard);
        *id_guard += 1;
        drop(id_guard);
        let mut book = self.positions.lock().map_err(|e| format!("lock: {e}"))?;
        book.insert(
            id.clone(),
            BrokerPosition {
                venue_order_id: id.clone(),
                symbol: order.symbol.clone(),
                side: order.side.as_str().to_string(),
                quantity: order.quantity,
                entry_price: fill,
                leverage: order.leverage,
                liquidation_price: liq,
                margin_allocated: margin,
            },
        );
        Ok(OrderAck {
            venue_order_id: id,
            venue: self.venue().to_string(),
            symbol: order.symbol,
            side: order.side.as_str().to_string(),
            order_type: order.order_type.as_str().to_string(),
            quantity: order.quantity,
            leverage: order.leverage,
            fill_price: Some(fill),
            status: "FILLED".to_string(),
            note: "Paper fill against the synthetic reference price. No venue, no real funds."
                .to_string(),
        })
    }

    fn positions(&self) -> Vec<BrokerPosition> {
        self.positions
            .lock()
            .map(|b| b.values().cloned().collect())
            .unwrap_or_default()
    }

    fn cancel(&self, venue_order_id: &str) -> Result<bool, BrokerError> {
        let mut book = self.positions.lock().map_err(|e| format!("lock: {e}"))?;
        Ok(book.remove(venue_order_id).is_some())
    }
}

/// Venue registry compiled into this binary. Grows with each adapter feature.
#[must_use]
pub fn available_brokers() -> Vec<&'static str> {
    vec![PAPER_VENUE]
}
