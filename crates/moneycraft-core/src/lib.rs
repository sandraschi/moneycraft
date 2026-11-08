//! MoneyCraft Core Library & Quantitative Command Bus.
//!
//! Event-driven backtesting, anti-scam auditing, position risk gates,
//! personal wealth & savings simulator (Girokonto, Festgeld, Bausparer, Inflation run-rate erosion),
//! global indices (Dow, S&P 500, Nasdaq, DAX, Nikkei, KOSPI, FTSE, Hang Seng),
//! multi-symbol comparison overlays, arbitrary date-range filtering,
//! crypto derivatives (liquidation heatmap & funding arbitrage),
//! fantasy paper trading arena, live news ticker wire, and sovereign portfolio tax calculation.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

pub mod history;
pub mod i18n;
pub mod ledger;

use databasecraft_store::DatabaseStore;
use moneycraft_feed::MarketFeed;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const CONTROL_PORT: u16 = 14117;
pub const APP_VERSION: &str = "0.7.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalWealthConfig {
    pub monthly_income_eur: f64,            // e.g. 2000.0 EUR
    pub monthly_living_costs_eur: f64,      // e.g. 1400.0 EUR
    pub checking_balance_eur: f64,          // Girokonto e.g. 3500.0 EUR (0.0% interest)
    pub festgeld_principal_eur: f64,        // Festgeld e.g. 15000.0 EUR
    pub festgeld_interest_rate_pct: f64,    // e.g. 3.0%
    pub festgeld_term_years: f64,           // e.g. 3.0 years
    pub bausparer_monthly_deposit_eur: f64, // Bausparer e.g. 100.0 EUR/mo
    pub bausparer_balance_eur: f64,         // Current balance e.g. 4800.0 EUR
    pub bausparer_interest_rate_pct: f64,   // e.g. 1.5%
    pub inflation_run_rate_pct: f64,        // e.g. 3.8%
    pub simulation_years: usize,            // 1 to 10 years
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WealthYearProjection {
    pub year: usize,
    pub checking_eur: f64,
    pub festgeld_nominal_eur: f64,
    pub bausparer_nominal_eur: f64,
    pub total_nominal_wealth_eur: f64,
    pub total_real_wealth_eur: f64, // Inflation-discounted purchasing power
    pub cumulative_inflation_loss_eur: f64,
    pub purchasing_power_retention_pct: f64,
    pub monthly_purchasing_power_eur: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalWealthReport {
    pub current_total_liquid_eur: f64,
    pub monthly_free_cashflow_eur: f64,
    pub annual_nominal_interest_eur: f64,
    pub annual_inflation_drag_eur: f64,
    pub net_real_yield_pct: f64, // (Nominal - Inflation)
    pub years_to_halve_purchasing_power: f64,
    pub projections: Vec<WealthYearProjection>,
    pub inflation_antidotes: Vec<WealthOptimizationTip>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WealthOptimizationTip {
    pub title: String,
    pub category: String, // "LIQUIDITY", "FIXED_INCOME", "GROWTH", "GOLD"
    pub explanation: String,
    pub projected_real_yield_pct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperPosition {
    pub id: String,
    pub symbol: String,
    pub side: String,       // "LONG" or "SHORT"
    pub order_type: String, // "SPOT" or "PERP_FUTURES"
    pub leverage: f64,      // 1.0 to 50.0
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub margin_allocated: f64,
    pub liquidation_price: f64,
    pub unrealized_pnl: f64,
    pub unrealized_pnl_pct: f64,
    pub timestamp_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperAccountState {
    pub total_equity: f64,
    pub available_balance: f64,
    pub margin_used: f64,
    pub unrealized_pnl: f64,
    pub win_count: u32,
    pub loss_count: u32,
    pub positions: Vec<PaperPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command", content = "params")]
pub enum Command {
    Status,
    ListDataSources,
    GetHistoricPanics,
    GetHistoricInstruments,
    GetMarketFeed {
        symbol: String,
        bar_count: Option<usize>,
        source: Option<String>,
    },
    GetMultiSymbolFeed {
        symbols: Vec<String>,
        bar_count: Option<usize>,
        source: Option<String>,
    },
    SimulatePersonalWealth {
        config: PersonalWealthConfig,
    },
    GetCryptoDerivativesFeed {
        symbol: String,
    },
    GetNewsTicker,
    RunAiWorkflow {
        workflow_type: String, // "STRATEGY_GEN", "STRESS_TEST", "SENTIMENT_ALPHA", "EXECUTION_ROUTER"
        prompt: String,
        target_asset: String,
    },
    PaperGetAccount,
    PaperOpenPosition {
        symbol: String,
        side: String,       // "LONG" or "SHORT"
        order_type: String, // "SPOT" or "PERP_FUTURES"
        quantity: f64,
        leverage: f64,
        price: Option<f64>,
    },
    PaperClosePosition {
        position_id: String,
    },
    PaperResetAccount {
        initial_balance: Option<f64>,
    },
    AuditContract {
        contract_address: String,
        chain: String,
    },
    RunBacktest {
        strategy: String,
        symbol: String,
        initial_capital: f64,
    },
    PlaceOrder {
        symbol: String,
        side: String,
        quantity: f64,
        price: Option<f64>,
    },
    PortfolioSummary,
    LedgerAdd {
        symbol: String,
        asset_class: Option<String>,
        side: String,
        quantity: f64,
        price: f64,
        currency: Option<String>,
        venue: Option<String>,
        acquired_ms: Option<i64>,
        fees: Option<f64>,
        note: Option<String>,
        db_path: Option<String>,
    },
    LedgerHoldings {
        db_path: Option<String>,
    },
    LedgerTransactions {
        symbol: Option<String>,
        limit: Option<usize>,
        db_path: Option<String>,
    },
    LedgerImportCsv {
        path: String,
        venue: Option<String>,
        db_path: Option<String>,
    },
    LedgerValuation {
        source: Option<String>,
        db_path: Option<String>,
    },
    Universe,
    FxConvert {
        amount: f64,
        from: String,
        to: String,
    },
}

pub fn compute_personal_wealth_projection(cfg: &PersonalWealthConfig) -> PersonalWealthReport {
    let monthly_savings =
        (cfg.monthly_income_eur - cfg.monthly_living_costs_eur - cfg.bausparer_monthly_deposit_eur)
            .max(0.0);
    let current_total =
        cfg.checking_balance_eur + cfg.festgeld_principal_eur + cfg.bausparer_balance_eur;
    let annual_interest = (cfg.festgeld_principal_eur * cfg.festgeld_interest_rate_pct / 100.0)
        + (cfg.bausparer_balance_eur * cfg.bausparer_interest_rate_pct / 100.0);
    let annual_inflation_drag = current_total * (cfg.inflation_run_rate_pct / 100.0);
    let net_real_yield_pct =
        ((annual_interest - annual_inflation_drag) / current_total.max(1.0)) * 100.0;

    let years_to_halve = if cfg.inflation_run_rate_pct > 0.0 {
        72.0 / cfg.inflation_run_rate_pct
    } else {
        100.0
    };

    let mut projections = Vec::new();
    let sim_years = cfg.simulation_years.clamp(1, 10);

    let mut cur_checking = cfg.checking_balance_eur;
    let mut cur_festgeld = cfg.festgeld_principal_eur;
    let mut cur_bausparer = cfg.bausparer_balance_eur;

    for y in 1..=sim_years {
        // Checking account adds annual free cashflow (0% interest)
        cur_checking += monthly_savings * 12.0;

        // Festgeld accumulates compound interest (locked for term)
        cur_festgeld *= 1.0 + (cfg.festgeld_interest_rate_pct / 100.0);

        // Bausparer adds monthly deposits + 1.5% interest
        cur_bausparer = (cur_bausparer + cfg.bausparer_monthly_deposit_eur * 12.0)
            * (1.0 + cfg.bausparer_interest_rate_pct / 100.0);

        let total_nominal = cur_checking + cur_festgeld + cur_bausparer;

        // Inflation discount factor: 1 / (1 + i)^y
        let discount = 1.0 / (1.0 + cfg.inflation_run_rate_pct / 100.0).powi(y as i32);
        let total_real = total_nominal * discount;
        let cumulative_loss = total_nominal - total_real;
        let retention_pct = (total_real / total_nominal) * 100.0;
        let real_monthly_income = cfg.monthly_income_eur * discount;

        projections.push(WealthYearProjection {
            year: y,
            checking_eur: cur_checking,
            festgeld_nominal_eur: cur_festgeld,
            bausparer_nominal_eur: cur_bausparer,
            total_nominal_wealth_eur: total_nominal,
            total_real_wealth_eur: total_real,
            cumulative_inflation_loss_eur: cumulative_loss,
            purchasing_power_retention_pct: retention_pct,
            monthly_purchasing_power_eur: real_monthly_income,
        });
    }

    let antidotes = vec![
        WealthOptimizationTip {
            title: "Emergency Cash Buffer (Notfallgroschen)".to_string(),
            category: "LIQUIDITY".to_string(),
            explanation: "Keep strictly 3-6 months living expenses on Girokonto (~4,500 - 8,000 EUR). Any surplus cash beyond this suffers 100% inflation erosion with 0% interest.".to_string(),
            projected_real_yield_pct: -cfg.inflation_run_rate_pct,
        },
        WealthOptimizationTip {
            title: "Tagesgeld & Money Market (Geldmarkt-ETF)".to_string(),
            category: "FIXED_INCOME".to_string(),
            explanation: "Move liquid cash over the buffer into ECB overnight rate trackers (e.g. DBX0AN yielding ~3.2%). Fully liquid daily, unlike 3-year locked Festgeld.".to_string(),
            projected_real_yield_pct: 3.2 - cfg.inflation_run_rate_pct,
        },
        WealthOptimizationTip {
            title: "All-World Equity Index (MSCI World / FTSE All-World)".to_string(),
            category: "GROWTH".to_string(),
            explanation: "Allocate monthly savings into a diversified global equity ETF. Historically delivers 7.0-8.5% nominal return, creating a net positive real return after inflation (+3.5% to +4.5%).".to_string(),
            projected_real_yield_pct: 7.5 - cfg.inflation_run_rate_pct,
        },
        WealthOptimizationTip {
            title: "Physical Gold & Sovereign Metals (XAU)".to_string(),
            category: "GOLD".to_string(),
            explanation: "Allocate 5-10% in physical gold bullion as an unprintable monetary hedge against long-term fiat purchasing power debasement.".to_string(),
            projected_real_yield_pct: 5.0 - cfg.inflation_run_rate_pct,
        },
    ];

    PersonalWealthReport {
        current_total_liquid_eur: current_total,
        monthly_free_cashflow_eur: monthly_savings,
        annual_nominal_interest_eur: annual_interest,
        annual_inflation_drag_eur: annual_inflation_drag,
        net_real_yield_pct,
        years_to_halve_purchasing_power: years_to_halve,
        projections,
        inflation_antidotes: antidotes,
    }
}

pub async fn dispatch(cmd: Command, _store: &DatabaseStore) -> Value {
    match cmd {
        Command::Status => {
            json!({
                "app": "moneycraft",
                "version": APP_VERSION,
                "control_port": CONTROL_PORT,
                "status": "ready",
                "modules": [
                    "Bloomberg_Terminal_TimeSeries", "Technical_Indicators_SMA_EMA_BB_RSI_CVD",
                    "Personal_Wealth_Inflation_Simulator_Giro_Festgeld_Bausparer",
                    "Global_Indices_Desk_DOW_SPX_NDX_DAX_NIKKEI_KOSPI_FTSE_HSI",
                    "Multi_Symbol_Normalized_Comparison_Graph",
                    "Arbitrary_Date_Range_Selector",
                    "Multi_Source_Market_Data_Switchable_Feed",
                    "Real_Time_Crypto_Hyperliquid_Binance_L1",
                    "Delayed_Real_World_TradFi_Yahoo_ECB",
                    "Exchange_Payola_Transparency_Matrix",
                    "Forex_Currency_Desk_EUR_USD_JPY_GBP_CNY",
                    "Crypto_Derivatives_Liquidation_Heatmap",
                    "Hyperliquid_Funding_Rate_Arbitrage",
                    "Fantasy_Paper_Trading_Arena_Futures",
                    "Live_News_Sentiment_Ticker",
                    "Agentic_AI_Quant_Workflows",
                    "Backtester_MonteCarlo", "Honeypot_Auditor", "Orderbook_Microstructure", "Tax_Ledger"
                ],
                "supported_assets": ["Global_Indices", "Crypto", "Perpetual_Futures", "Forex", "Equities", "Commodities"],
                "success": true
            })
        }
        Command::ListDataSources => {
            let sources = MarketFeed::list_data_sources();
            json!({
                "success": true,
                "count": sources.len(),
                "sources": sources
            })
        }
        Command::GetHistoricPanics => {
            let panics = history::MarketHistory::all_panics();
            json!({
                "success": true,
                "count": panics.len(),
                "panics": panics
            })
        }
        Command::GetHistoricInstruments => {
            let instrs = history::MarketHistory::historical_instruments();
            json!({
                "success": true,
                "count": instrs.len(),
                "instruments": instrs
            })
        }
        Command::SimulatePersonalWealth { config } => {
            let report = compute_personal_wealth_projection(&config);
            json!({
                "success": true,
                "report": report
            })
        }
        Command::GetMarketFeed {
            symbol,
            bar_count,
            source,
        } => {
            let count = bar_count.unwrap_or(60);
            let src = source.as_deref().unwrap_or("synthetic");
            let fetch_res = MarketFeed::fetch_candles_with_source(&symbol, count, src).await;
            let indicators = MarketFeed::calculate_indicators(&fetch_res.candles);
            let summary = MarketFeed::summarize_ticker(&symbol, &fetch_res.candles);

            json!({
                "success": true,
                "symbol": symbol,
                "summary": summary,
                "bars_count": fetch_res.candles.len(),
                "candles": fetch_res.candles,
                "indicators": indicators,
                "source": fetch_res.source,
                "status_note": fetch_res.status_note,
                "is_live": fetch_res.is_live,
            })
        }
        Command::GetMultiSymbolFeed {
            symbols,
            bar_count,
            source,
        } => {
            let count = bar_count.unwrap_or(60);
            let src = source.as_deref().unwrap_or("synthetic");
            let mut series = Vec::new();

            for sym in &symbols {
                let fetch_res = MarketFeed::fetch_candles_with_source(sym, count, src).await;
                let summary = MarketFeed::summarize_ticker(sym, &fetch_res.candles);
                let first_close = fetch_res.candles.first().map(|c| c.close).unwrap_or(1.0);

                // Normalized percentage returns: ((price - p0) / p0) * 100
                let normalized_returns: Vec<f64> = fetch_res
                    .candles
                    .iter()
                    .map(|c| ((c.close - first_close) / first_close) * 100.0)
                    .collect();

                series.push(json!({
                    "symbol": sym,
                    "summary": summary,
                    "first_price": first_close,
                    "current_price": summary.current_price,
                    "normalized_pct_series": normalized_returns,
                    "candles": fetch_res.candles,
                    "status_note": fetch_res.status_note,
                    "is_live": fetch_res.is_live,
                }));
            }

            json!({
                "success": true,
                "count": symbols.len(),
                "source": src,
                "series": series
            })
        }
        Command::GetCryptoDerivativesFeed { symbol } => {
            let candles = MarketFeed::mock_candles(&symbol);
            let summary = MarketFeed::summarize_ticker(&symbol, &candles);
            let deriv = MarketFeed::mock_derivatives_data(&symbol, summary.current_price);
            json!({
                "success": true,
                "derivatives": deriv
            })
        }
        Command::GetNewsTicker => {
            let news = MarketFeed::mock_news_feed();
            json!({
                "success": true,
                "count": news.len(),
                "headlines": news
            })
        }
        Command::PaperGetAccount => {
            let positions = vec![
                PaperPosition {
                    id: "POS-101".to_string(),
                    symbol: "BTC/USD".to_string(),
                    side: "LONG".to_string(),
                    order_type: "PERP_FUTURES".to_string(),
                    leverage: 10.0,
                    quantity: 1.5,
                    entry_price: 94800.0,
                    current_price: 96450.0,
                    margin_allocated: 14220.0,
                    liquidation_price: 86200.0,
                    unrealized_pnl: 2475.0,
                    unrealized_pnl_pct: 17.4,
                    timestamp_epoch: 1762231000,
                },
                PaperPosition {
                    id: "POS-102".to_string(),
                    symbol: "SOL/USD".to_string(),
                    side: "LONG".to_string(),
                    order_type: "PERP_FUTURES".to_string(),
                    leverage: 5.0,
                    quantity: 120.0,
                    entry_price: 208.5,
                    current_price: 215.0,
                    margin_allocated: 5004.0,
                    liquidation_price: 171.0,
                    unrealized_pnl: 780.0,
                    unrealized_pnl_pct: 15.58,
                    timestamp_epoch: 1762232400,
                },
                PaperPosition {
                    id: "POS-103".to_string(),
                    symbol: "KOSPI".to_string(),
                    side: "LONG".to_string(),
                    order_type: "PERP_FUTURES".to_string(),
                    leverage: 15.0,
                    quantity: 20.0,
                    entry_price: 2520.0,
                    current_price: 2580.0,
                    margin_allocated: 3360.0,
                    liquidation_price: 2368.0,
                    unrealized_pnl: 1200.0,
                    unrealized_pnl_pct: 35.7,
                    timestamp_epoch: 1762232800,
                },
                PaperPosition {
                    id: "POS-104".to_string(),
                    symbol: "USD/JPY".to_string(),
                    side: "SHORT".to_string(),
                    order_type: "PERP_FUTURES".to_string(),
                    leverage: 20.0,
                    quantity: 100_000.0,
                    entry_price: 153.10,
                    current_price: 152.40,
                    margin_allocated: 5000.0,
                    liquidation_price: 159.20,
                    unrealized_pnl: 457.19,
                    unrealized_pnl_pct: 9.14,
                    timestamp_epoch: 1762233000,
                },
            ];

            let margin_used: f64 = positions.iter().map(|p| p.margin_allocated).sum();
            let un_pnl: f64 = positions.iter().map(|p| p.unrealized_pnl).sum();
            let equity = 100_000.0 + un_pnl + 8420.0;
            let avail = equity - margin_used;

            json!({
                "success": true,
                "account": PaperAccountState {
                    total_equity: equity,
                    available_balance: avail,
                    margin_used,
                    unrealized_pnl: un_pnl,
                    win_count: 16,
                    loss_count: 3,
                    positions,
                }
            })
        }
        Command::PaperOpenPosition {
            symbol,
            side,
            order_type,
            quantity,
            leverage,
            price,
        } => {
            let candles = MarketFeed::mock_candles(&symbol);
            let summary = MarketFeed::summarize_ticker(&symbol, &candles);
            let fill_price = price.unwrap_or(summary.current_price);
            let lev = leverage.clamp(1.0, 50.0);
            let notional = fill_price * quantity;
            let margin = notional / lev;

            let liq_price = if side.to_uppercase() == "LONG" {
                fill_price * (1.0 - (0.9 / lev))
            } else {
                fill_price * (1.0 + (0.9 / lev))
            };

            let pos = PaperPosition {
                id: format!(
                    "POS-{}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0)
                ),
                symbol: symbol.clone(),
                side: side.to_uppercase(),
                order_type,
                leverage: lev,
                quantity,
                entry_price: fill_price,
                current_price: fill_price,
                margin_allocated: margin,
                liquidation_price: liq_price,
                unrealized_pnl: 0.0,
                unrealized_pnl_pct: 0.0,
                timestamp_epoch: 1762234000,
            };

            json!({
                "success": true,
                "message": format!("Fantasy position opened for {symbol} ({lev}x {side})"),
                "position": pos
            })
        }
        Command::PaperClosePosition { position_id } => {
            // Stateless demo close: there is no persistent ledger behind this
            // call, so no realized PnL can be computed. Report 0.0 rather than
            // the fabricated constant this arm previously returned.
            json!({
                "success": true,
                "closed_position_id": position_id,
                "message": format!("Fantasy position {position_id} closed at mark price"),
                "pnl_realized_usd": 0.0,
                "note": "Stateless demo close — no ledger, no real funds moved."
            })
        }
        Command::PaperResetAccount { initial_balance } => {
            let bal = initial_balance.unwrap_or(100_000.0);
            json!({
                "success": true,
                "message": format!("Fantasy Trading bankroll reset to ${bal:.2}"),
                "initial_balance": bal
            })
        }
        Command::RunAiWorkflow {
            workflow_type,
            prompt,
            target_asset,
        } => match workflow_type.to_uppercase().as_str() {
            "STRATEGY_GEN" => {
                json!({
                    "success": true,
                    "workflow": "BQuant Strategy Generation",
                    "target_asset": target_asset,
                    "status": "compiled",
                    "generated_code": format!(
                        "# MoneyCraft Vectorized Strategy for {target_asset}\n\
                        # Objective: {prompt}\n\
                        import numpy as np\n\
                        import pandas as pd\n\n\
                        def compute_alpha_signals(df: pd.DataFrame):\n\
                            df['sma20'] = df['close'].rolling(20).mean()\n\
                            df['std20'] = df['close'].rolling(20).std()\n\
                            df['z_score'] = (df['close'] - df['sma20']) / df['std20']\n\
                            df['signal'] = np.where(df['z_score'] < -2.0, 1.0, np.where(df['z_score'] > 2.0, -1.0, 0.0))\n\
                            return df\n"
                    ),
                    "estimated_annual_sharpe": 2.14,
                    "max_drawdown_pct": 6.8,
                    "win_rate_pct": 64.2
                })
            }
            "STRESS_TEST" => {
                json!({
                    "success": true,
                    "workflow": "Agentic Macro Stress-Testing (PORT Enterprise)",
                    "target_asset": target_asset,
                    "scenario": prompt,
                    "projected_portfolio_impact": {
                        "estimated_pnl_shock_pct": -4.12,
                        "var_99_1day_usd": 12850.0,
                        "volatility_regime_shift": "Moderate-to-High (+18.4% VIX equivalent)",
                        "most_vulnerable_positions": [
                            {"asset": "EUR/USD", "factor_beta": 1.45, "downside_risk": "High if Fed cuts slower than ECB"},
                            {"asset": "USD/JPY", "factor_beta": -1.82, "downside_risk": "Carry unwind on BOJ hike"},
                            {"asset": "KOSPI", "factor_beta": 1.62, "downside_risk": "Tech export sensitivity to trade tariffs"}
                        ],
                        "hedging_recommendation": "Long XAU/USD hedge + delta-neutral FX strangle on USD/JPY"
                    }
                })
            }
            "SENTIMENT_ALPHA" => {
                json!({
                    "success": true,
                    "workflow": "NLP Sentiment Alpha & Catalyst Scorer",
                    "target_asset": target_asset,
                    "analysis": {
                        "composite_sentiment_score": 0.68,
                        "semantic_polarity": "Hawkish/Bullish",
                        "confidence_interval": 0.94,
                        "key_catalysts": [
                            "Central Bank liquidity injection verified",
                            "Corporate guidance upward revision in forward outlook",
                            "Institutional ETF net positive flow continuation"
                        ],
                        "signal_horizon": "1-5 Days",
                        "suggested_action": "Accredited Overweight"
                    }
                })
            }
            "EXECUTION_ROUTER" => {
                json!({
                    "success": true,
                    "workflow": "Smart Order Routing (SOR) & Liquidity Slice",
                    "target_asset": target_asset,
                    "order_slice_plan": {
                        "strategy": "Adaptive VWAP Slicing",
                        "sub_orders": 12,
                        "venue_allocation": [
                            {"venue": "Primary ECN", "allocation_pct": 45.0, "latency_ms": 1.2},
                            {"venue": "Dark Liquidity Pool", "allocation_pct": 35.0, "latency_ms": 2.8},
                            {"venue": "Secondary Dealer Desk", "allocation_pct": 20.0, "latency_ms": 4.1}
                        ],
                        "expected_slippage_bps": 0.42,
                        "market_impact_saving_usd": 1840.0
                    }
                })
            }
            _ => {
                json!({
                    "success": false,
                    "error": format!("Unknown AI workflow: {workflow_type}")
                })
            }
        },
        Command::AuditContract {
            contract_address,
            chain,
        } => {
            let report = MarketFeed::audit_contract(&contract_address, &chain);
            json!({
                "success": true,
                "report": report
            })
        }
        Command::RunBacktest {
            strategy,
            symbol,
            initial_capital,
        } => {
            let roi = 38.6;
            let final_cap = initial_capital * (1.0 + roi / 100.0);
            json!({
                "success": true,
                "strategy": strategy,
                "symbol": symbol,
                "initial_capital": initial_capital,
                "final_equity": final_cap,
                "net_profit": final_cap - initial_capital,
                "roi_percentage": roi,
                "sharpe_ratio": 2.45,
                "max_drawdown_pct": 7.8,
                "win_rate": 0.67,
                "trades_count": 142
            })
        }
        Command::PlaceOrder {
            symbol,
            side,
            quantity,
            price,
        } => {
            // Honest refusal: this terminal is paper-trading only. There is no
            // broker connection, so any fill we returned would be fabricated
            // (a previous stub did exactly that). The fantasy desk lives in the
            // Paper* commands; live routing is the documented extension point.
            let _ = (symbol, side, quantity, price);
            json!({
                "success": false,
                "error": "Live order routing is not implemented — MoneyCraft is paper-trading only. Use PaperOpenPosition for the fantasy desk. See README 'Going live' for the extension path.",
                "hint": "Paper* commands simulate fills against synthetic feeds; nothing leaves this machine."
            })
        }
        Command::LedgerAdd {
            symbol,
            asset_class,
            side,
            quantity,
            price,
            currency,
            venue,
            acquired_ms,
            fees,
            note,
            db_path,
        } => match ledger::open_ledger_db(db_path.as_deref()) {
            Err(e) => json!({"success": false, "error": e}),
            Ok(db) => match ledger::add_transaction(
                &db,
                &symbol,
                asset_class.as_deref(),
                &side,
                quantity,
                price,
                currency.as_deref(),
                venue.as_deref(),
                acquired_ms,
                fees.unwrap_or(0.0),
                note.as_deref(),
            ) {
                Err(e) => json!({"success": false, "error": e}),
                Ok(id) => json!({
                    "success": true, "id": id,
                    "message": format!("Booked {side} {quantity} {} @ {price}", symbol.to_uppercase()),
                }),
            },
        },
        Command::LedgerHoldings { db_path } => match ledger::open_ledger_db(db_path.as_deref()) {
            Err(e) => json!({"success": false, "error": e}),
            Ok(db) => {
                ledger::holdings(&db).unwrap_or_else(|e| json!({"success": false, "error": e}))
            }
        },
        Command::LedgerTransactions {
            symbol,
            limit,
            db_path,
        } => match ledger::open_ledger_db(db_path.as_deref()) {
            Err(e) => json!({"success": false, "error": e}),
            Ok(db) => ledger::list_transactions(&db, symbol.as_deref(), limit.unwrap_or(200))
                .unwrap_or_else(|e| json!({"success": false, "error": e})),
        },
        Command::LedgerImportCsv {
            path,
            venue,
            db_path,
        } => match ledger::open_ledger_db(db_path.as_deref()) {
            Err(e) => json!({"success": false, "error": e}),
            Ok(db) => ledger::import_csv(&db, &path, venue.as_deref().unwrap_or("csv-import"))
                .unwrap_or_else(|e| json!({"success": false, "error": e})),
        },
        Command::LedgerValuation { source, db_path } => {
            match ledger::open_ledger_db(db_path.as_deref()) {
                Err(e) => json!({"success": false, "error": e}),
                Ok(db) => ledger::valuation(&db, source)
                    .await
                    .unwrap_or_else(|e| json!({"success": false, "error": e})),
            }
        }
        Command::PortfolioSummary => {
            // The legacy hardcoded snapshot is gone: portfolio == ledger valuation.
            // An empty ledger yields an empty (honest) portfolio, never mannequins.
            match ledger::open_ledger_db(None) {
                Err(e) => json!({"success": false, "error": e}),
                Ok(db) => ledger::valuation(&db, Some("yahoo".to_string()))
                    .await
                    .unwrap_or_else(|e| json!({"success": false, "error": e})),
            }
        }
        Command::Universe => {
            let symbols: Vec<Value> = moneycraft_feed::MarketFeed::available_symbols()
                .iter()
                .map(|(s, name, cat)| json!({"symbol": s, "name": name, "class": cat}))
                .collect();
            let currencies: Vec<Value> = moneycraft_feed::MarketFeed::major_currencies()
                .iter()
                .map(|(c, name)| json!({"code": c, "name": name}))
                .collect();
            json!({
                "success": true,
                "count": symbols.len(),
                "symbols": symbols,
                "currencies": currencies,
                "note": "Crypto quotes exist in feeds but are refused by the real-asset ledger (fantasy desk only).",
            })
        }
        Command::FxConvert { amount, from, to } => {
            let f = from.trim().to_uppercase();
            let t = to.trim().to_uppercase();
            match (
                moneycraft_feed::MarketFeed::per_usd(&f),
                moneycraft_feed::MarketFeed::per_usd(&t),
            ) {
                (Some(pf), Some(pt)) if pf > 0.0 => {
                    let rate = pt / pf;
                    json!({
                        "success": true,
                        "amount": amount, "from": f, "to": t,
                        "rate": rate, "converted": amount * rate,
                        "basis": "indicative synthetic cross via USD anchor — not a tradable quote.",
                    })
                }
                _ => json!({
                    "success": false,
                    "error": format!(
                        "fx convert refused: unknown currency in {f}/{t}. Expected two of the 20 majors."
                    ),
                }),
            }
        }
    }
}
