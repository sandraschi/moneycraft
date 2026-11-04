//! Integration and dispatch verification tests for MoneyCraft.
use databasecraft_store::DatabaseStore;
use moneycraft_core::{dispatch, Command};

#[tokio::test]
async fn test_moneycraft_status_and_control_port() -> Result<(), Box<dyn std::error::Error>> {
    let store = DatabaseStore::open_in_memory()?;
    let res = dispatch(Command::Status, &store).await;
    assert_eq!(res["success"], true);
    assert_eq!(res["app"], "moneycraft");
    assert_eq!(res["control_port"], 14117);
    Ok(())
}

#[tokio::test]
async fn test_moneycraft_market_feed_and_honeypot_auditor() -> Result<(), Box<dyn std::error::Error>> {
    let store = DatabaseStore::open_in_memory()?;

    // 1. Market feed query
    let feed_res = dispatch(Command::GetMarketFeed { symbol: "BTC/USD".to_string() }, &store).await;
    assert_eq!(feed_res["success"], true);
    let candles = feed_res["candles"].as_array().ok_or("missing candles")?;
    assert!(candles.len() >= 4);

    // 2. Anti-scam honeypot audit on malicious contract
    let scam_audit = dispatch(
        Command::AuditContract {
            contract_address: "0x123_scam_token_trap".to_string(),
            chain: "Ethereum".to_string(),
        },
        &store,
    )
    .await;
    assert_eq!(scam_audit["success"], true);
    assert_eq!(scam_audit["report"]["is_honeypot"], true);
    assert!(scam_audit["report"]["risk_score_0_to_100"].as_u64().unwrap_or(0) > 90);

    // 3. Anti-scam audit on clean contract
    let safe_audit = dispatch(
        Command::AuditContract {
            contract_address: "0x456_legit_token".to_string(),
            chain: "Ethereum".to_string(),
        },
        &store,
    )
    .await;
    assert_eq!(safe_audit["success"], true);
    assert_eq!(safe_audit["report"]["is_honeypot"], false);
    assert_eq!(safe_audit["report"]["allows_trading"], true);

    // 4. Backtest simulation
    let bt_res = dispatch(
        Command::RunBacktest {
            strategy: "Dual_Momentum_Breakout".to_string(),
            symbol: "BTC".to_string(),
            initial_capital: 10000.0,
        },
        &store,
    )
    .await;
    assert_eq!(bt_res["success"], true);
    assert!(bt_res["sharpe_ratio"].as_f64().unwrap_or(0.0) > 1.5);
    assert!(bt_res["max_drawdown_pct"].as_f64().unwrap_or(100.0) < 15.0);

    Ok(())
}
