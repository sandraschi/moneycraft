//! PaperBroker rails: limits enforced, dry-run sends nothing,
//! cancel removes, reference prices come from the synthetic series.

use moneycraft_broker::{
    Broker, BrokerLimits, NewOrder, OrderType, PaperBroker, Side, PAPER_VENUE,
};

fn order(symbol: &str, qty: f64, lev: f64, dry: bool) -> NewOrder {
    NewOrder {
        symbol: symbol.to_string(),
        side: Side::Long,
        order_type: OrderType::Perp,
        quantity: qty,
        leverage: lev,
        limit_price: None,
        dry_run: dry,
    }
}

#[test]
fn paper_is_the_only_compiled_venue() {
    assert_eq!(moneycraft_broker::available_brokers(), vec![PAPER_VENUE]);
}

#[test]
fn oversize_and_overleverage_are_rejected() {
    let b = PaperBroker::default();
    let px = 100.0;
    let too_big = order("AAPL", 10_000.0, 1.0, true);
    assert!(b.check_order(&too_big, px).is_err());
    let too_much_lev = order("AAPL", 1.0, 50.0, true);
    assert!(b.check_order(&too_much_lev, px).is_err());
    let bad_qty = order("AAPL", 0.0, 1.0, true);
    assert!(b.check_order(&bad_qty, px).is_err());
}

#[test]
fn dry_run_books_nothing() -> Result<(), String> {
    let b = PaperBroker::default();
    let ack = b
        .place(order("AAPL", 10.0, 2.0, true), 200.0)
        .map_err(|e| format!("dry run must validate: {e}"))?;
    assert_eq!(ack.status, "DRY_RUN");
    assert!(b.positions().is_empty());
    Ok(())
}

#[test]
fn fill_math_matches_legacy_fantasy_formulas() -> Result<(), String> {
    let b = PaperBroker::default();
    let ack = b
        .place(order("AAPL", 10.0, 2.0, false), 200.0)
        .map_err(|e| format!("valid order must fill: {e}"))?;
    assert_eq!(ack.status, "FILLED");
    assert_eq!(ack.fill_price, Some(200.0));
    let pos = b.positions();
    assert_eq!(pos.len(), 1);
    assert!((pos[0].margin_allocated - 1000.0).abs() < 1e-9);
    assert!((pos[0].liquidation_price - 110.0).abs() < 1e-9);
    assert!(b
        .cancel(&pos[0].venue_order_id)
        .map_err(|e| format!("cancel: {e}"))?);
    assert!(b.positions().is_empty());
    assert!(!b
        .cancel("NOPE")
        .map_err(|e| format!("cancel missing: {e}"))?);
    Ok(())
}

#[test]
fn allow_list_is_enforced() {
    let b = PaperBroker::with_limits(BrokerLimits {
        max_notional_per_order: 1_000_000.0,
        max_leverage: 50.0,
        allow_list: vec!["AAPL".to_string()],
    });
    assert!(b.check_order(&order("AAPL", 1.0, 1.0, true), 200.0).is_ok());
    assert!(b
        .check_order(&order("SAP.DE", 1.0, 1.0, true), 200.0)
        .is_err());
}
