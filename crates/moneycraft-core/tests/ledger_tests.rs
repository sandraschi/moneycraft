//! Personal ledger tests: FIFO lots, crypto refusal, CSV import, valuation.
//! Uses an in-memory store; synthetic feed for offline valuation.

use databasecraft_store::DatabaseStore;
use moneycraft_core::ledger;

fn mem_store() -> Result<DatabaseStore, Box<dyn std::error::Error>> {
    Ok(DatabaseStore::open_in_memory()?)
}

#[test]
fn test_fifo_holdings_and_avg_cost() -> Result<(), Box<dyn std::error::Error>> {
    let db = mem_store()?;
    ledger::ensure_ledger(&db).map_err(|e| format!("migrate: {e}"))?;
    ledger::add_transaction(
        &db,
        "SAP.DE",
        None,
        "BUY",
        10.0,
        100.0,
        Some("EUR"),
        Some("t"),
        None,
        0.0,
        None,
    )
    .map_err(|e| format!("buy1: {e}"))?;
    ledger::add_transaction(
        &db,
        "SAP.DE",
        None,
        "BUY",
        10.0,
        110.0,
        Some("EUR"),
        Some("t"),
        None,
        0.0,
        None,
    )
    .map_err(|e| format!("buy2: {e}"))?;
    ledger::add_transaction(
        &db,
        "SAP.DE",
        None,
        "SELL",
        5.0,
        120.0,
        Some("EUR"),
        Some("t"),
        None,
        0.0,
        None,
    )
    .map_err(|e| format!("sell: {e}"))?;
    let h = ledger::holdings(&db).map_err(|e| format!("holdings: {e}"))?;
    let pos = h["positions"]
        .as_array()
        .and_then(|a| a.first().cloned())
        .ok_or("no positions")?;
    let qty = pos["quantity"].as_f64().ok_or("no qty")?;
    let avg = pos["avg_cost"].as_f64().ok_or("no avg")?;
    assert!((qty - 15.0).abs() < 1e-9, "qty {qty}");
    assert!((avg - 106.666_666_7).abs() < 1e-4, "avg {avg}");
    Ok(())
}

#[test]
fn test_crypto_and_oversell_refused() -> Result<(), Box<dyn std::error::Error>> {
    let db = mem_store()?;
    ledger::ensure_ledger(&db).map_err(|e| format!("migrate: {e}"))?;
    let btc = ledger::add_transaction(
        &db,
        "BTC",
        None,
        "BUY",
        1.0,
        90000.0,
        Some("EUR"),
        Some("t"),
        None,
        0.0,
        None,
    );
    assert!(btc.is_err(), "BTC must be refused");
    let err = btc.err().unwrap_or_default();
    assert!(err.contains("crypto"), "unexpected error: {err}");
    ledger::add_transaction(
        &db,
        "AAPL",
        None,
        "BUY",
        5.0,
        200.0,
        Some("USD"),
        Some("t"),
        None,
        0.0,
        None,
    )
    .map_err(|e| format!("buy: {e}"))?;
    let over = ledger::add_transaction(
        &db,
        "AAPL",
        None,
        "SELL",
        6.0,
        210.0,
        Some("USD"),
        Some("t"),
        None,
        0.0,
        None,
    );
    assert!(over.is_err(), "oversell must be refused");
    Ok(())
}

#[test]
fn test_csv_import_german_bank_style() -> Result<(), Box<dyn std::error::Error>> {
    let db = mem_store()?;
    let path = std::env::temp_dir().join(format!("ledger-csv-{}.csv", std::process::id()));
    let csv = "Symbol;Anzahl;Kurs;Datum;Währung;Typ\nSAP.DE;5;230,50;15.04.2024;EUR;Kauf\nBTC;1;90000;01.05.2024;EUR;Kauf\nNESN;20;xx;01.06.2024;CHF;Kauf\n";
    std::fs::write(&path, csv)?;
    let res = ledger::import_csv(&db, path.to_str().ok_or("bad temp path")?, "flatex")
        .map_err(|e| format!("import: {e}"))?;
    assert_eq!(res["imported"], 1);
    assert_eq!(res["skipped_count"], 2);
    let _ = std::fs::remove_file(&path);
    Ok(())
}

#[tokio::test]
async fn test_valuation_synthetic_offline() -> Result<(), Box<dyn std::error::Error>> {
    let db = mem_store()?;
    ledger::add_transaction(
        &db,
        "EUR",
        None,
        "BUY",
        3500.0,
        1.0,
        Some("EUR"),
        Some("george"),
        None,
        0.0,
        None,
    )
    .map_err(|e| format!("cash: {e}"))?;
    ledger::add_transaction(
        &db,
        "SAP.DE",
        None,
        "BUY",
        10.0,
        228.0,
        Some("EUR"),
        Some("flatex"),
        None,
        0.0,
        None,
    )
    .map_err(|e| format!("buy: {e}"))?;
    ledger::add_transaction(
        &db,
        "SAP.DE",
        None,
        "SELL",
        4.0,
        245.0,
        Some("EUR"),
        Some("flatex"),
        None,
        0.0,
        None,
    )
    .map_err(|e| format!("sell: {e}"))?;
    let v = ledger::valuation(&db, Some("synthetic".to_string()))
        .await
        .map_err(|e| format!("valuation: {e}"))?;
    assert_eq!(v["success"], true);
    assert_eq!(v["count"], 2);
    let realized = v["realized_ytd_by_currency"]["EUR"]
        .as_f64()
        .ok_or("no realized")?;
    assert!((realized - 68.0).abs() < 1e-9, "realized {realized}");
    let kest = v["kest_by_currency"]["EUR"]["kest_estimate"]
        .as_f64()
        .ok_or("no kest")?;
    assert!((kest - 68.0 * 0.275).abs() < 1e-9, "kest {kest}");
    Ok(())
}
