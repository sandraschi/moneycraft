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
#[command(about = "MoneyCraft: Sovereign Financial Intelligence & Algorithmic Trading CLI", version = APP_VERSION)]
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
    /// Fetch candlestick bars and volume for ticker symbol
    Ticker {
        #[arg(default_value = "BTC/USD")]
        symbol: String,
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
    /// Inspect private sovereign portfolio PnL and active positions
    Portfolio,
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
        Some(Commands::Ticker { symbol }) => {
            let res = rt.block_on(dispatch(Command::GetMarketFeed { symbol }, &store));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::Audit { address, chain }) => {
            let res = rt.block_on(dispatch(Command::AuditContract { contract_address: address, chain }, &store));
            println!("{}", serde_json::to_string_pretty(&res["report"]).unwrap_or_default());
        }
        Some(Commands::Backtest { strategy, symbol, capital }) => {
            let res = rt.block_on(dispatch(Command::RunBacktest { strategy, symbol, initial_capital: capital }, &store));
            println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
        }
        Some(Commands::Portfolio) => {
            let res = rt.block_on(dispatch(Command::PortfolioSummary, &store));
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
                        {
                            "name": "finance_ticker",
                            "description": "Fetch real-time candlestick bars and volume for ticker symbol",
                            "inputSchema": {
                                "type": "object",
                                "properties": { "symbol": { "type": "string" } },
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
                        { "name": "finance_portfolio", "description": "Inspect private sovereign portfolio PnL and active positions", "inputSchema": { "type": "object", "properties": {} } }
                    ]
                }
            }),
            "tools/call" => {
                let params = req.get("params").cloned().unwrap_or(Value::Null);
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));

                let tool_res = match tool_name {
                    "finance_status" => rt.block_on(dispatch(Command::Status, store)),
                    "finance_ticker" => {
                        let sym = args["symbol"].as_str().unwrap_or("BTC/USD").to_string();
                        rt.block_on(dispatch(Command::GetMarketFeed { symbol: sym }, store))
                    }
                    "finance_audit_contract" => {
                        let addr = args["address"].as_str().unwrap_or("").to_string();
                        let ch = args["chain"].as_str().unwrap_or("Ethereum").to_string();
                        rt.block_on(dispatch(Command::AuditContract { contract_address: addr, chain: ch }, store))
                    }
                    "finance_backtest" => {
                        let strat = args["strategy"].as_str().unwrap_or("Momentum").to_string();
                        let sym = args["symbol"].as_str().unwrap_or("BTC").to_string();
                        let cap = args["capital"].as_f64().unwrap_or(10000.0);
                        rt.block_on(dispatch(Command::RunBacktest { strategy: strat, symbol: sym, initial_capital: cap }, store))
                    }
                    "finance_portfolio" => rt.block_on(dispatch(Command::PortfolioSummary, store)),
                    unknown => json!({ "success": false, "error": format!("Unknown tool '{unknown}'") })
                };

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "content": [{ "type": "text", "text": tool_res.to_string() }] }
                })
            }
            _ => json!({ "jsonrpc": "2.0", "id": id, "error": { "code": -32601, "message": "Method not found" } })
        };

        let _ = writeln!(stdout, "{}", resp.to_string());
        let _ = stdout.flush();
    }
}
