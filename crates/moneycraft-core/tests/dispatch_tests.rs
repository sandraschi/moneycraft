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
async fn test_moneycraft_market_feed_and_honeypot_auditor() -> Result<(), Box<dyn std::error::Error>>
{
    let store = DatabaseStore::open_in_memory()?;

    // 1. Market feed query (Synthetic & Real sources)
    let feed_res = dispatch(
        Command::GetMarketFeed {
            symbol: "BTC/USD".to_string(),
            bar_count: Some(60),
            source: None,
        },
        &store,
    )
    .await;
    assert_eq!(feed_res["success"], true);
    let candles = feed_res["candles"].as_array().ok_or("missing candles")?;
    assert!(candles.len() >= 4);

    // 1b. Registry of Data Sources & Payola Tiers
    let sources_res = dispatch(Command::ListDataSources, &store).await;
    assert_eq!(sources_res["success"], true);
    let sources_list = sources_res["sources"].as_array().ok_or("missing sources")?;
    assert!(sources_list.len() >= 9);

    // 1c. Binance public live feed test
    let binance_res = dispatch(
        Command::GetMarketFeed {
            symbol: "BTC/USD".to_string(),
            bar_count: Some(30),
            source: Some("binance".to_string()),
        },
        &store,
    )
    .await;
    assert_eq!(binance_res["success"], true);
    assert!(
        binance_res["candles"]
            .as_array()
            .ok_or("missing binance candles")?
            .len()
            >= 10
    );

    // 1d. Yahoo Finance delayed index feed test
    let yahoo_res = dispatch(
        Command::GetMarketFeed {
            symbol: "DJIA".to_string(),
            bar_count: Some(30),
            source: Some("yahoo".to_string()),
        },
        &store,
    )
    .await;
    assert_eq!(yahoo_res["success"], true);
    assert!(
        yahoo_res["candles"]
            .as_array()
            .ok_or("missing yahoo candles")?
            .len()
            >= 5
    );

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
    assert!(
        scam_audit["report"]["risk_score_0_to_100"]
            .as_u64()
            .unwrap_or(0)
            > 90
    );

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

    // 5. Personal wealth and inflation erosion simulation (Checking, Festgeld, Bausparer)
    let wealth_config = moneycraft_core::PersonalWealthConfig {
        monthly_income_eur: 2000.0,
        monthly_living_costs_eur: 1400.0,
        checking_balance_eur: 4000.0,
        festgeld_principal_eur: 15000.0,
        festgeld_interest_rate_pct: 3.0,
        festgeld_term_years: 3.0,
        bausparer_monthly_deposit_eur: 100.0,
        bausparer_balance_eur: 5000.0,
        bausparer_interest_rate_pct: 1.5,
        inflation_run_rate_pct: 3.8,
        simulation_years: 5,
    };
    let sim_res = dispatch(
        Command::SimulatePersonalWealth {
            config: wealth_config,
        },
        &store,
    )
    .await;
    assert_eq!(sim_res["success"], true);
    let report = &sim_res["report"];
    assert_eq!(report["current_total_liquid_eur"], 24000.0);
    assert_eq!(report["monthly_free_cashflow_eur"], 500.0); // 2000 - 1400 - 100 bausparer
    assert!(report["net_real_yield_pct"].as_f64().unwrap_or(0.0) < 0.0); // negative real return!
    let projections = report["projections"]
        .as_array()
        .ok_or("missing projections")?;
    assert_eq!(projections.len(), 5); // Year 1 to 5
    assert!(
        projections[4]["cumulative_inflation_loss_eur"]
            .as_f64()
            .unwrap_or(0.0)
            > 0.0
    );

    Ok(())
}
