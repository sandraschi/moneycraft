//! MoneyCraft MCP Server & Universal CLI.
//!
//! Follows the MCP 2024-11-05 standard: stdout is 100% JSON-RPC, diagnostics go to stderr.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use clap::{Parser, Subcommand};
use databasecraft_store::DatabaseStore;
use moneycraft_core::{dispatch, Command, APP_VERSION, CONTROL_PORT};
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use tokio::runtime::Runtime;

#[derive(Parser)]
#[command(name = "moneycraft-cli")]
#[command(about = "MoneyCraft: Local-First Financial Intelligence terminal (paper-trading only — no live orders, no real funds)", version = APP_VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Run as MCP 2024-11-05 stdio server
    #[arg(long)]
    mcp: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Check MoneyCraft status and quantitative modules
    Status,
    /// List all switchable Market Data Sources, delays, and Exchange Payola tiers
    Sources,
    /// Fetch candlestick bars and volume for ticker symbol
    Ticker {
        #[arg(default_value = "BTC/USD")]
        symbol: String,
        #[arg(long, default_value = "synthetic")]
        source: String,
    },
    /// Audit smart contract bytecode for honeypots and malicious backdoors
    Audit {
        address: String,
        #[arg(long, default_value = "Ethereum")]
        chain: String,
    },
    /// Run high-speed Monte Carlo strategy backtest
    Backtest {
        strategy: String,
        #[arg(long, default_value = "BTC")]
        symbol: String,
        #[arg(long, default_value_t = 10000.0)]
        capital: f64,
    },
    /// Inspect private local portfolio PnL and active positions
    Portfolio,
    /// List historical financial crises and systemic market panics (1720-2026)
    Panics,
    /// List historical papers and indices (London East India Co, Consols, South Sea)
    HistoricInstruments,
    /// Simulate personal wealth growth and real purchasing power erosion against inflation
    SimulateWealth {
        #[arg(long, default_value_t = 2000.0)]
        income: f64,
        #[arg(long, default_value_t = 1400.0)]
        living_costs: f64,
        #[arg(long, default_value_t = 4000.0)]
        checking: f64,
        #[arg(long, default_value_t = 15000.0)]
        festgeld: f64,
        #[arg(long, default_value_t = 3.0)]
        festgeld_interest: f64,
        #[arg(long, default_value_t = 3.0)]
        festgeld_term: f64,
        #[arg(long, default_value_t = 100.0)]
        bausparer_monthly: f64,
        #[arg(long, default_value_t = 5000.0)]
        bausparer_balance: f64,
        #[arg(long, default_value_t = 1.5)]
        bausparer_interest: f64,
        #[arg(long, default_value_t = 3.8)]
        inflation: f64,
        #[arg(long, default_value_t = 5)]
        years: usize,
    },
    /// Fetch normalized % overlays for several symbols at once (compare workspace)
    Compare {
        /// Symbols, comma-separated (e.g. "DJIA,SPX,KOSPI")
        symbols: String,
        #[arg(long, default_value_t = 60)]
        bars: usize,
        #[arg(long, default_value = "synthetic")]
        source: String,
    },
    /// Fetch crypto derivatives intel: funding rates + liquidation heatmap (fantasy data, no live venue)
    Derivatives {
        #[arg(default_value = "BTC/USD")]
        symbol: String,
    },
    /// Print the breaking-news wire headlines with sentiment tags
    News,
    /// Run an agentic quant workflow: STRATEGY_GEN, STRESS_TEST, SENTIMENT_ALPHA, EXECUTION_ROUTER (illustrative templates, no execution)
    Ai {
        /// Workflow name
        workflow: String,
        #[arg(
            long,
            default_value = "momentum breakout with Bollinger mean-reversion"
        )]
        prompt: String,
        #[arg(long, default_value = "BTC/USD")]
        asset: String,
    },
    /// Show the fantasy paper-trading account (demo positions, no real funds)
    PaperAccount,
    /// Open a fantasy paper position (1x-50x, sandbox only — never a real order)
    PaperOpen {
        #[arg(default_value = "BTC/USD")]
        symbol: String,
        #[arg(long, default_value = "LONG")]
        side: String,
        #[arg(long, default_value = "PERP_FUTURES")]
        order_type: String,
        #[arg(long, default_value_t = 1.0)]
        quantity: f64,
        #[arg(long, default_value_t = 10.0)]
        leverage: f64,
        #[arg(long)]
        price: Option<f64>,
    },
    /// Close a fantasy paper position by id (stateless demo close)
    PaperClose { position_id: String },
    /// Reset the fantasy bankroll (default $100,000, sandbox only)
    PaperReset {
        #[arg(long)]
        balance: Option<f64>,
    },
}

fn main() {
    let cli = Cli::parse();
    let rt = Runtime::new().unwrap_or_else(|e| {
        eprintln!("Failed to create tokio runtime: {e}");
        std::process::exit(1);
    });

    let store = DatabaseStore::open_in_memory().unwrap_or_else(|e| {
        eprintln!("Failed to open in-memory store: {e}");
        std::process::exit(1);
    });

    if cli.mcp {
        run_mcp_stdio(&rt, &store);
        return;
    }

    match cli.command {
        Some(Commands::Status) => {
            let res = rt.block_on(dispatch(Command::Status, &store));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::Sources) => {
            let res = rt.block_on(dispatch(Command::ListDataSources, &store));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::Ticker { symbol, source }) => {
            let res = rt.block_on(dispatch(
                Command::GetMarketFeed {
                    symbol,
                    bar_count: Some(60),
                    source: Some(source),
                },
                &store,
            ));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::Audit { address, chain }) => {
            let res = rt.block_on(dispatch(
                Command::AuditContract {
                    contract_address: address,
                    chain,
                },
                &store,
            ));
            println!(
                "{}",
                serde_json::to_string_pretty(&res["report"]).unwrap_or_default()
            );
        }
        Some(Commands::Backtest {
            strategy,
            symbol,
            capital,
        }) => {
            let res = rt.block_on(dispatch(
                Command::RunBacktest {
                    strategy,
                    symbol,
                    initial_capital: capital,
                },
                &store,
            ));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::Portfolio) => {
            let res = rt.block_on(dispatch(Command::PortfolioSummary, &store));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::Panics) => {
            let res = rt.block_on(dispatch(Command::GetHistoricPanics, &store));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::HistoricInstruments) => {
            let res = rt.block_on(dispatch(Command::GetHistoricInstruments, &store));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::SimulateWealth {
            income,
            living_costs,
            checking,
            festgeld,
            festgeld_interest,
            festgeld_term,
            bausparer_monthly,
            bausparer_balance,
            bausparer_interest,
            inflation,
            years,
        }) => {
            let cfg = moneycraft_core::PersonalWealthConfig {
                monthly_income_eur: income,
                monthly_living_costs_eur: living_costs,
                checking_balance_eur: checking,
                festgeld_principal_eur: festgeld,
                festgeld_interest_rate_pct: festgeld_interest,
                festgeld_term_years: festgeld_term,
                bausparer_monthly_deposit_eur: bausparer_monthly,
                bausparer_balance_eur: bausparer_balance,
                bausparer_interest_rate_pct: bausparer_interest,
                inflation_run_rate_pct: inflation,
                simulation_years: years,
            };
            let res = rt.block_on(dispatch(
                Command::SimulatePersonalWealth { config: cfg },
                &store,
            ));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::Compare {
            symbols,
            bars,
            source,
        }) => {
            let syms: Vec<String> = symbols
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let res = rt.block_on(dispatch(
                Command::GetMultiSymbolFeed {
                    symbols: syms,
                    bar_count: Some(bars),
                    source: Some(source),
                },
                &store,
            ));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::Derivatives { symbol }) => {
            let res = rt.block_on(dispatch(
                Command::GetCryptoDerivativesFeed { symbol },
                &store,
            ));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::News) => {
            let res = rt.block_on(dispatch(Command::GetNewsTicker, &store));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::Ai {
            workflow,
            prompt,
            asset,
        }) => {
            let res = rt.block_on(dispatch(
                Command::RunAiWorkflow {
                    workflow_type: workflow,
                    prompt,
                    target_asset: asset,
                },
                &store,
            ));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::PaperAccount) => {
            let res = rt.block_on(dispatch(Command::PaperGetAccount, &store));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::PaperOpen {
            symbol,
            side,
            order_type,
            quantity,
            leverage,
            price,
        }) => {
            let res = rt.block_on(dispatch(
                Command::PaperOpenPosition {
                    symbol,
                    side,
                    order_type,
                    quantity,
                    leverage,
                    price,
                },
                &store,
            ));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::PaperClose { position_id }) => {
            let res = rt.block_on(dispatch(
                Command::PaperClosePosition { position_id },
                &store,
            ));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::PaperReset { balance }) => {
            let res = rt.block_on(dispatch(
                Command::PaperResetAccount {
                    initial_balance: balance,
                },
                &store,
            ));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        None => {
            println!("MoneyCraft CLI v{APP_VERSION} on Port {CONTROL_PORT}");
            println!("Use --help to view available commands or --mcp to start JSON-RPC server.");
        }
    }
}

fn run_mcp_stdio(rt: &Runtime, store: &DatabaseStore) {
    eprintln!("[moneycraft-mcp] Starting MCP stdio loop on port {CONTROL_PORT}");
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let req: Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let id = req.get("id").cloned().unwrap_or(Value::Null);
        let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");

        let resp = match method {
            "initialize" => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": { "tools": {} },
                    "serverInfo": { "name": "moneycraft-mcp", "version": APP_VERSION }
                }
            }),
            "tools/list" => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "tools": [
                        { "name": "finance_status", "description": "Check MoneyCraft status and quantitative modules", "inputSchema": { "type": "object", "properties": {} } },
                        { "name": "finance_list_data_sources", "description": "List all switchable real and simulated market data sources, latency delays, and Exchange Payola tiers", "inputSchema": { "type": "object", "properties": {} } },
                        {
                            "name": "finance_ticker",
                            "description": "Fetch real-time or delayed candlestick bars and volume for ticker symbol from selected source",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "symbol": { "type": "string" },
                                    "source": { "type": "string", "description": "Data provider ID: synthetic, binance, hyperliquid, yahoo, or ecb" },
                                    "bar_count": { "type": "integer" }
                                },
                                "required": ["symbol"]
                            }
                        },
                        {
                            "name": "finance_audit_contract",
                            "description": "Audit EVM smart contract for honeypots, hidden taxes, and backdoors",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "address": { "type": "string" },
                                    "chain": { "type": "string" }
                                },
                                "required": ["address"]
                            }
                        },
                        {
                            "name": "finance_backtest",
                            "description": "Run Monte Carlo event-driven algorithmic strategy backtest",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "strategy": { "type": "string" },
                                    "symbol": { "type": "string" },
                                    "capital": { "type": "number" }
                                },
                                "required": ["strategy"]
                            }
                        },
                        { "name": "finance_portfolio", "description": "Inspect private local portfolio PnL and active positions", "inputSchema": { "type": "object", "properties": {} } },
                        { "name": "finance_historic_panics", "description": "List all 19 major historical financial crises and market panics from 1720 South Sea Bubble to modern day", "inputSchema": { "type": "object", "properties": {} } },
                        { "name": "finance_historic_instruments", "description": "List historical London Exchange Alley sovereign papers and equities (EIC, Consols, South Sea)", "inputSchema": { "type": "object", "properties": {} } },
                        {
                            "name": "finance_simulate_personal_wealth",
                            "description": "Simulate beginner personal wealth compounding across checking, festgeld, and bausparer vs true inflation erosion",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "income": { "type": "number", "description": "Monthly net income in EUR (e.g. 2000.0)" },
                                    "living_costs": { "type": "number", "description": "Monthly living expenses in EUR (e.g. 1400.0)" },
                                    "checking": { "type": "number", "description": "Current checking account balance (Girokonto) in EUR" },
                                    "festgeld": { "type": "number", "description": "Festgeld principal in EUR" },
                                    "festgeld_interest": { "type": "number", "description": "Festgeld annual interest rate % (e.g. 3.0)" },
                                    "bausparer_monthly": { "type": "number", "description": "Monthly deposit into Bausparvertrag in EUR" },
                                    "bausparer_balance": { "type": "number", "description": "Current Bausparer balance in EUR" },
                                    "inflation": { "type": "number", "description": "Annual inflation run-rate % (e.g. 3.8)" },
                                    "years": { "type": "integer", "description": "Simulation horizon in years (1 to 10)" }
                                }
                            }
                        },
                        {
                            "name": "finance_compare",
                            "description": "Fetch normalized percentage-return overlays for several symbols at once (multi-asset compare workspace)",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "symbols": { "type": "array", "items": { "type": "string" }, "description": "Symbols, e.g. [\"DJIA\",\"SPX\",\"KOSPI\"]" },
                                    "source": { "type": "string" },
                                    "bar_count": { "type": "integer" }
                                },
                                "required": ["symbols"]
                            }
                        },
                        {
                            "name": "finance_derivatives",
                            "description": "Fetch crypto derivatives intel: funding rates and liquidation heatmap (illustrative venue data, not live tradable quotes)",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "symbol": { "type": "string" }
                                },
                                "required": ["symbol"]
                            }
                        },
                        {
                            "name": "finance_news",
                            "description": "Print the breaking-news wire headlines with sentiment tags",
                            "inputSchema": { "type": "object", "properties": {} }
                        },
                        {
                            "name": "finance_ai_workflow",
                            "description": "Run an agentic quant workflow: STRATEGY_GEN, STRESS_TEST, SENTIMENT_ALPHA or EXECUTION_ROUTER. Illustrative templates only — EXECUTION_ROUTER plans, never executes.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "workflow_type": { "type": "string" },
                                    "prompt": { "type": "string" },
                                    "target_asset": { "type": "string" }
                                },
                                "required": ["workflow_type"]
                            }
                        },
                        {
                            "name": "finance_paper_account",
                            "description": "Show the fantasy paper-trading account: demo positions, margin, PnL. No real funds, no real orders.",
                            "inputSchema": { "type": "object", "properties": {} }
                        },
                        {
                            "name": "finance_paper_open",
                            "description": "Open a fantasy paper position, 1x-50x leverage. Sandbox only — never a real order, nothing leaves this machine.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "symbol": { "type": "string" },
                                    "side": { "type": "string", "description": "LONG or SHORT" },
                                    "order_type": { "type": "string", "description": "SPOT or PERP_FUTURES" },
                                    "quantity": { "type": "number" },
                                    "leverage": { "type": "number" },
                                    "price": { "type": "number", "description": "Limit price override; omit for mark price" }
                                },
                                "required": ["symbol"]
                            }
                        },
                        {
                            "name": "finance_paper_close",
                            "description": "Close a fantasy paper position by id. Stateless demo close — no ledger, no real funds moved.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "position_id": { "type": "string" }
                                },
                                "required": ["position_id"]
                            }
                        },
                        {
                            "name": "finance_paper_reset",
                            "description": "Reset the fantasy bankroll (default $100,000). Sandbox only.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "initial_balance": { "type": "number" }
                                }
                            }
                        }
                    ]
                }
            }),
            "tools/call" => {
                let params = req.get("params").cloned().unwrap_or(Value::Null);
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));

                let tool_res = match tool_name {
                    "finance_status" => rt.block_on(dispatch(Command::Status, store)),
                    "finance_list_data_sources" => {
                        rt.block_on(dispatch(Command::ListDataSources, store))
                    }
                    "finance_ticker" => {
                        let sym = args["symbol"].as_str().unwrap_or("BTC/USD").to_string();
                        let bars = args["bar_count"].as_u64().map(|b| b as usize);
                        let src = args["source"].as_str().map(|s| s.to_string());
                        rt.block_on(dispatch(
                            Command::GetMarketFeed {
                                symbol: sym,
                                bar_count: bars,
                                source: src,
                            },
                            store,
                        ))
                    }
                    "finance_audit_contract" => {
                        let addr = args["address"].as_str().unwrap_or("").to_string();
                        let ch = args["chain"].as_str().unwrap_or("Ethereum").to_string();
                        rt.block_on(dispatch(
                            Command::AuditContract {
                                contract_address: addr,
                                chain: ch,
                            },
                            store,
                        ))
                    }
                    "finance_backtest" => {
                        let strat = args["strategy"].as_str().unwrap_or("Momentum").to_string();
                        let sym = args["symbol"].as_str().unwrap_or("BTC").to_string();
                        let cap = args["capital"].as_f64().unwrap_or(10000.0);
                        rt.block_on(dispatch(
                            Command::RunBacktest {
                                strategy: strat,
                                symbol: sym,
                                initial_capital: cap,
                            },
                            store,
                        ))
                    }
                    "finance_portfolio" => rt.block_on(dispatch(Command::PortfolioSummary, store)),
                    "finance_historic_panics" => {
                        rt.block_on(dispatch(Command::GetHistoricPanics, store))
                    }
                    "finance_historic_instruments" => {
                        rt.block_on(dispatch(Command::GetHistoricInstruments, store))
                    }
                    "finance_simulate_personal_wealth" => {
                        let cfg = moneycraft_core::PersonalWealthConfig {
                            monthly_income_eur: args["income"].as_f64().unwrap_or(2000.0),
                            monthly_living_costs_eur: args["living_costs"]
                                .as_f64()
                                .unwrap_or(1400.0),
                            checking_balance_eur: args["checking"].as_f64().unwrap_or(4000.0),
                            festgeld_principal_eur: args["festgeld"].as_f64().unwrap_or(15000.0),
                            festgeld_interest_rate_pct: args["festgeld_interest"]
                                .as_f64()
                                .unwrap_or(3.0),
                            festgeld_term_years: args["festgeld_term"].as_f64().unwrap_or(3.0),
                            bausparer_monthly_deposit_eur: args["bausparer_monthly"]
                                .as_f64()
                                .unwrap_or(100.0),
                            bausparer_balance_eur: args["bausparer_balance"]
                                .as_f64()
                                .unwrap_or(5000.0),
                            bausparer_interest_rate_pct: args["bausparer_interest"]
                                .as_f64()
                                .unwrap_or(1.5),
                            inflation_run_rate_pct: args["inflation"].as_f64().unwrap_or(3.8),
                            simulation_years: args["years"].as_u64().unwrap_or(5) as usize,
                        };
                        rt.block_on(dispatch(
                            Command::SimulatePersonalWealth { config: cfg },
                            store,
                        ))
                    }
                    "finance_compare" => {
                        let syms: Vec<String> = args["symbols"]
                            .as_array()
                            .map(|a| {
                                a.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();
                        let bars = args["bar_count"].as_u64().map(|b| b as usize);
                        let src = args["source"].as_str().map(|s| s.to_string());
                        rt.block_on(dispatch(
                            Command::GetMultiSymbolFeed {
                                symbols: syms,
                                bar_count: bars,
                                source: src,
                            },
                            store,
                        ))
                    }
                    "finance_derivatives" => {
                        let sym = args["symbol"].as_str().unwrap_or("BTC/USD").to_string();
                        rt.block_on(dispatch(
                            Command::GetCryptoDerivativesFeed { symbol: sym },
                            store,
                        ))
                    }
                    "finance_news" => rt.block_on(dispatch(Command::GetNewsTicker, store)),
                    "finance_ai_workflow" => {
                        let wf = args["workflow_type"]
                            .as_str()
                            .unwrap_or("STRATEGY_GEN")
                            .to_string();
                        let p = args["prompt"].as_str().unwrap_or("").to_string();
                        let asset = args["target_asset"]
                            .as_str()
                            .unwrap_or("BTC/USD")
                            .to_string();
                        rt.block_on(dispatch(
                            Command::RunAiWorkflow {
                                workflow_type: wf,
                                prompt: p,
                                target_asset: asset,
                            },
                            store,
                        ))
                    }
                    "finance_paper_account" => {
                        rt.block_on(dispatch(Command::PaperGetAccount, store))
                    }
                    "finance_paper_open" => {
                        let sym = args["symbol"].as_str().unwrap_or("BTC/USD").to_string();
                        let side = args["side"].as_str().unwrap_or("LONG").to_string();
                        let ot = args["order_type"]
                            .as_str()
                            .unwrap_or("PERP_FUTURES")
                            .to_string();
                        let qty = args["quantity"].as_f64().unwrap_or(1.0);
                        let lev = args["leverage"].as_f64().unwrap_or(10.0);
                        let price = args["price"].as_f64();
                        rt.block_on(dispatch(
                            Command::PaperOpenPosition {
                                symbol: sym,
                                side,
                                order_type: ot,
                                quantity: qty,
                                leverage: lev,
                                price,
                            },
                            store,
                        ))
                    }
                    "finance_paper_close" => {
                        let pid = args["position_id"].as_str().unwrap_or("").to_string();
                        rt.block_on(dispatch(
                            Command::PaperClosePosition { position_id: pid },
                            store,
                        ))
                    }
                    "finance_paper_reset" => {
                        let bal = args["initial_balance"].as_f64();
                        rt.block_on(dispatch(
                            Command::PaperResetAccount {
                                initial_balance: bal,
                            },
                            store,
                        ))
                    }
                    unknown => {
                        json!({ "success": false, "error": format!("Unknown tool '{unknown}'") })
                    }
                };

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "content": [{ "type": "text", "text": tool_res.to_string() }] }
                })
            }
            _ => {
                json!({ "jsonrpc": "2.0", "id": id, "error": { "code": -32601, "message": "Method not found" } })
            }
        };

        let _ = writeln!(stdout, "{resp}");
        let _ = stdout.flush();
    }
}
