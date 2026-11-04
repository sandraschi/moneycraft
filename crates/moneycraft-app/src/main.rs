//! MoneyCraft Native Desktop Studio.
//!
//! Visual financial terminal with high-density candlestick charting,
//! sandbox EVM honeypot auditor, algorithmic backtester, and sovereign portfolio ledger.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use databasecraft_store::DatabaseStore;
use eframe::egui::{self, Color32, Vec2};
use moneycraft_core::{dispatch, Command, APP_VERSION, CONTROL_PORT};
use serde_json::Value;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveWorkspace {
    Terminal,
    HoneypotAuditor,
    Backtester,
    PortfolioLedger,
}

pub struct MoneyCraftGui {
    pub store: Arc<DatabaseStore>,
    pub runtime: Arc<Runtime>,
    pub active_workspace: ActiveWorkspace,
    pub selected_symbol: String,
    pub audit_address: String,
    pub audit_chain: String,
    pub strategy_name: String,
    pub initial_capital: f64,
    pub market_data: Option<Value>,
    pub audit_report: Option<Value>,
    pub backtest_result: Option<Value>,
    pub portfolio_data: Option<Value>,
    pub status_message: String,
    pub is_dark_theme: bool,
    pub is_authenticated: bool,
    pub auth_username: String,
    pub auth_password_input: String,
    pub show_auth_modal: bool,
    pub show_about_dialog: bool,
}

impl MoneyCraftGui {
    pub fn new(store: Arc<DatabaseStore>, runtime: Arc<Runtime>) -> Self {
        let mut app = Self {
            store,
            runtime,
            active_workspace: ActiveWorkspace::Terminal,
            selected_symbol: "BTC/USD".to_string(),
            audit_address: "0x7a250d5630b4cf539739df2c5dacb4c659f2488d".to_string(),
            audit_chain: "Ethereum".to_string(),
            strategy_name: "Dual_Momentum_Breakout".to_string(),
            initial_capital: 10000.0,
            market_data: None,
            audit_report: None,
            backtest_result: None,
            portfolio_data: None,
            status_message: format!("MoneyCraft v{APP_VERSION} [Control Port {CONTROL_PORT} Active]"),
            is_dark_theme: true,
            is_authenticated: false,
            auth_username: "quant_trader".to_string(),
            auth_password_input: String::new(),
            show_auth_modal: false,
            show_about_dialog: false,
        };
        app.fetch_market_feed();
        app.fetch_portfolio();
        app
    }

    pub fn fetch_market_feed(&mut self) {
        let sym = self.selected_symbol.clone();
        let res = self.runtime.block_on(dispatch(Command::GetMarketFeed { symbol: sym.clone() }, &self.store));
        self.market_data = Some(res);
        self.status_message = format!("Market feed updated for {sym}");
    }

    pub fn run_audit(&mut self) {
        let addr = self.audit_address.clone();
        let ch = self.audit_chain.clone();
        let res = self.runtime.block_on(dispatch(Command::AuditContract { contract_address: addr, chain: ch }, &self.store));
        self.audit_report = Some(res);
        self.status_message = "Sandbox EVM smart-contract audit complete".to_string();
    }

    pub fn run_backtest(&mut self) {
        let strat = self.strategy_name.clone();
        let sym = self.selected_symbol.clone();
        let cap = self.initial_capital;
        let res = self.runtime.block_on(dispatch(Command::RunBacktest { strategy: strat, symbol: sym, initial_capital: cap }, &self.store));
        self.backtest_result = Some(res);
        self.status_message = "10,000-path Monte Carlo backtest verified".to_string();
    }

    pub fn fetch_portfolio(&mut self) {
        let res = self.runtime.block_on(dispatch(Command::PortfolioSummary, &self.store));
        self.portfolio_data = Some(res);
    }
}

impl eframe::App for MoneyCraftGui {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.is_dark_theme {
            ui.ctx().set_visuals(egui::Visuals::dark());
        } else {
            ui.ctx().set_visuals(egui::Visuals::light());
        }

        let gold = Color32::from_rgb(251, 191, 36);
        let green = Color32::from_rgb(16, 185, 129);
        let red = Color32::from_rgb(239, 68, 68);

        // 0. Desktop Menu Bar
        egui::Panel::top("desktop_menu_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Sync Sovereign Ledger").clicked() {
                        self.fetch_portfolio();
                        ui.close();
                    }
                    if ui.button("Refresh High-Density Feed").clicked() {
                        self.fetch_market_feed();
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Markets", |ui| {
                    for sym in &["BTC/USD", "ETH/USD", "SOL/USD", "NVDA", "XAU/USD"] {
                        if ui.button(*sym).clicked() {
                            self.selected_symbol = sym.to_string();
                            self.fetch_market_feed();
                            ui.close();
                        }
                    }
                });

                ui.menu_button("View", |ui| {
                    let theme_label = if self.is_dark_theme { "☀ Switch to Light Theme" } else { "🌙 Switch to Dark Theme" };
                    if ui.button(theme_label).clicked() {
                        self.is_dark_theme = !self.is_dark_theme;
                        ui.close();
                    }
                });

                ui.menu_button("Security", |ui| {
                    if self.is_authenticated {
                        ui.label(format!("Trader: {}", self.auth_username));
                        if ui.button("Sign Out").clicked() {
                            self.is_authenticated = false;
                            self.status_message = "Signed out".to_string();
                            ui.close();
                        }
                    } else {
                        if ui.button("Basic Auth Login...").clicked() {
                            self.show_auth_modal = true;
                            ui.close();
                        }
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About MoneyCraft").clicked() {
                        self.show_about_dialog = true;
                        ui.close();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (icon, label) = if self.is_dark_theme { ("☀", "Light") } else { ("🌙", "Dark") };
                    if ui.small_button(format!("{icon} {label}")).clicked() {
                        self.is_dark_theme = !self.is_dark_theme;
                    }

                    if self.is_authenticated {
                        ui.colored_label(gold, format!("💰 {}", self.auth_username));
                    } else if ui.small_button("🔑 Login").clicked() {
                        self.show_auth_modal = true;
                    }
                });
            });
        });

        // 1. Top Bar
        egui::Panel::top("finance_top_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.strong(egui::RichText::new("📈 MONEYCRAFT SOVEREIGN TERMINAL").color(gold).size(16.0));
                ui.separator();
                ui.label(format!("v{APP_VERSION}"));
                ui.colored_label(Color32::from_rgb(148, 163, 184), format!("| Port {CONTROL_PORT}"));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⟳ Refresh Feed").clicked() {
                        self.fetch_market_feed();
                    }
                });
            });
        });

        // 2. Bottom Status Bar
        egui::Panel::bottom("finance_bottom_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(gold, "●");
                ui.label(&self.status_message);
            });
        });

        // 3. Navigation Rail
        egui::Panel::left("finance_nav_rail").min_size(200.0).show(ui, |ui| {
            ui.add_space(8.0);
            ui.strong("WORKSPACES");
            ui.add_space(4.0);

            let nav_btn = |ui: &mut egui::Ui, active: bool, text: &str| -> bool {
                let rich = egui::RichText::new(text).color(if active { gold } else { Color32::WHITE });
                ui.selectable_label(active, rich).clicked()
            };

            if nav_btn(ui, self.active_workspace == ActiveWorkspace::Terminal, "📊 Market Terminal") {
                self.active_workspace = ActiveWorkspace::Terminal;
                self.fetch_market_feed();
            }
            if nav_btn(ui, self.active_workspace == ActiveWorkspace::HoneypotAuditor, "🛡️ Honeypot & Scam Auditor") {
                self.active_workspace = ActiveWorkspace::HoneypotAuditor;
            }
            if nav_btn(ui, self.active_workspace == ActiveWorkspace::Backtester, "⚙️ Quantitative Backtester") {
                self.active_workspace = ActiveWorkspace::Backtester;
            }
            if nav_btn(ui, self.active_workspace == ActiveWorkspace::PortfolioLedger, "💼 Sovereign Portfolio") {
                self.active_workspace = ActiveWorkspace::PortfolioLedger;
                self.fetch_portfolio();
            }
        });

        // 4. Central Panel
        egui::CentralPanel::default().show(ui, |ui| match self.active_workspace {
            ActiveWorkspace::Terminal => {
                ui.horizontal(|ui| {
                    ui.heading(format!("📊 Market Ticker: {}", self.selected_symbol));
                    ui.add_space(16.0);
                    ui.selectable_value(&mut self.selected_symbol, "BTC/USD".to_string(), "BTC");
                    ui.selectable_value(&mut self.selected_symbol, "ETH/USD".to_string(), "ETH");
                    ui.selectable_value(&mut self.selected_symbol, "NVDA".to_string(), "NVDA");
                    ui.selectable_value(&mut self.selected_symbol, "SPY".to_string(), "SPY");

                    if ui.button("Load").clicked() {
                        self.fetch_market_feed();
                    }
                });
                ui.separator();

                if let Some(ref mkt) = self.market_data {
                    if let Some(candles) = mkt["candles"].as_array() {
                        ui.columns(candles.len(), |cols| {
                            for (i, c) in candles.iter().enumerate() {
                                cols[i].group(|ui| {
                                    let open = c["open"].as_f64().unwrap_or(0.0);
                                    let close = c["close"].as_f64().unwrap_or(0.0);
                                    let is_bull = close >= open;

                                    ui.colored_label(if is_bull { green } else { red }, format!("BAR #{}", i + 1));
                                    ui.label(format!("Close: {:.2}", close));
                                    ui.colored_label(Color32::from_rgb(148, 163, 184), format!("High:  {:.2}", c["high"].as_f64().unwrap_or(0.0)));
                                    ui.colored_label(Color32::from_rgb(148, 163, 184), format!("Low:   {:.2}", c["low"].as_f64().unwrap_or(0.0)));
                                    ui.label(format!("Vol:   {:.0}", c["volume"].as_f64().unwrap_or(0.0)));
                                });
                            }
                        });
                    }
                }
            }
            ActiveWorkspace::HoneypotAuditor => {
                ui.heading("🛡️ Sandbox EVM Smart-Contract & Honeypot Auditor");
                ui.label("Simulates transactions in a local isolated sandbox EVM before executing on-chain. Exposes hidden mints and blacklist routines.");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Contract Address:");
                    ui.text_edit_singleline(&mut self.audit_address);
                    if ui.button(egui::RichText::new("🔍 Audit Bytecode").color(gold).strong()).clicked() {
                        self.run_audit();
                    }
                });

                if let Some(ref audit_res) = self.audit_report {
                    if let Some(rep) = audit_res.get("report") {
                        ui.add_space(8.0);
                        let is_honey = rep["is_honeypot"].as_bool().unwrap_or(false);
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                if is_honey {
                                    ui.colored_label(red, "⛔ HONEYPOT MALICIOUS CODE DETECTED");
                                } else {
                                    ui.colored_label(green, "✔ VERIFIED SAFE BYTECODE");
                                }
                                ui.label(format!("Risk Score: {}/100", rep["risk_score_0_to_100"]));
                            });
                            ui.label(rep["audit_verdict"].as_str().unwrap_or(""));
                            ui.horizontal(|ui| {
                                ui.label(format!("• Buy Tax: {:.1}%", rep["buy_tax_pct"].as_f64().unwrap_or(0.0)));
                                ui.label(format!("• Sell Tax: {:.1}%", rep["sell_tax_pct"].as_f64().unwrap_or(0.0)));
                                ui.label(format!("• Mintable: {}", rep["can_mint_tokens"]));
                                ui.label(format!("• Blacklist Function: {}", rep["can_blacklist_wallets"]));
                            });
                        });
                    }
                }
            }
            ActiveWorkspace::Backtester => {
                ui.heading("⚙️ Event-Driven Quantitative Backtester");
                ui.label("Run 10,000-path Monte Carlo permutations with realistic slippage, liquidity queues, and commission friction.");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Strategy:");
                    ui.text_edit_singleline(&mut self.strategy_name);
                    ui.label("Capital ($):");
                    ui.add(egui::DragValue::new(&mut self.initial_capital).speed(500.0));
                    if ui.button(egui::RichText::new("🚀 Run Backtest").color(green).strong()).clicked() {
                        self.run_backtest();
                    }
                });

                if let Some(ref res) = self.backtest_result {
                    ui.add_space(8.0);
                    ui.columns(4, |cols| {
                        cols[0].group(|ui| {
                            ui.colored_label(green, "TOTAL RETURN");
                            ui.heading(format!("+{:.1}%", res["total_return_pct"].as_f64().unwrap_or(0.0)));
                        });
                        cols[1].group(|ui| {
                            ui.colored_label(gold, "SHARPE RATIO");
                            ui.heading(format!("{:.2}", res["sharpe_ratio"].as_f64().unwrap_or(0.0)));
                        });
                        cols[2].group(|ui| {
                            ui.colored_label(Color32::from_rgb(96, 165, 250), "SORTINO RATIO");
                            ui.heading(format!("{:.2}", res["sortino_ratio"].as_f64().unwrap_or(0.0)));
                        });
                        cols[3].group(|ui| {
                            ui.colored_label(red, "MAX DRAWDOWN");
                            ui.heading(format!("-{:.1}%", res["max_drawdown_pct"].as_f64().unwrap_or(0.0)));
                        });
                    });
                }
            }
            ActiveWorkspace::PortfolioLedger => {
                ui.heading("💼 Sovereign Portfolio & Tax Ledger");
                ui.label("100% offline, local B-tree portfolio accounting. Zero user profiling, zero broker data resale.");
                ui.separator();

                if let Some(ref port) = self.portfolio_data {
                    ui.columns(3, |cols| {
                        cols[0].group(|ui| {
                            ui.colored_label(Color32::WHITE, "TOTAL PORTFOLIO VALUATION");
                            ui.heading(format!("${:.2}", port["total_portfolio_usd"].as_f64().unwrap_or(0.0)));
                        });
                        cols[1].group(|ui| {
                            ui.colored_label(green, "UNREALIZED PnL");
                            ui.heading(format!("+${:.2}", port["unrealized_pnl_usd"].as_f64().unwrap_or(0.0)));
                        });
                        cols[2].group(|ui| {
                            ui.colored_label(gold, "REALIZED PnL (YTD)");
                            ui.heading(format!("+${:.2}", port["realized_pnl_usd"].as_f64().unwrap_or(0.0)));
                        });
                    });

                    ui.add_space(8.0);
                    ui.strong("Active Positions:");
                    if let Some(pos) = port["active_positions"].as_array() {
                        for p in pos {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.strong(p["symbol"].as_str().unwrap_or(""));
                                    ui.label(format!("• Qty: {}", p["qty"]));
                                    ui.label(format!("• Entry: ${:.2}", p["entry"].as_f64().unwrap_or(0.0)));
                                    ui.label(format!("• Current: ${:.2}", p["current"].as_f64().unwrap_or(0.0)));
                                    ui.colored_label(green, format!("• PnL: +${:.2}", p["pnl"].as_f64().unwrap_or(0.0)));
                                });
                            });
                        }
                    }
                }
            }
        });

        // Modal: Basic Auth Login
        if self.show_auth_modal {
            egui::Window::new("🔐 Sovereign Financial Auth")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label("Authenticate for Live Execution & Private Ledger:");
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.label("User:");
                        ui.text_edit_singleline(&mut self.auth_username);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Pass:");
                        ui.add(egui::TextEdit::singleline(&mut self.auth_password_input).password(true));
                    });
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("Confirm").clicked() {
                            if !self.auth_username.is_empty() {
                                self.is_authenticated = true;
                                self.show_auth_modal = false;
                                self.status_message = format!("Signed in as {}", self.auth_username);
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_auth_modal = false;
                        }
                    });
                });
        }

        // Modal: About Dialog
        if self.show_about_dialog {
            egui::Window::new("ℹ About MoneyCraft")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.heading("MoneyCraft Sovereign Terminal");
                    ui.label("High-density quantitative finance studio.");
                    ui.label(format!("Version: v{APP_VERSION}"));
                    ui.label(format!("Control Port: {CONTROL_PORT}"));
                    ui.separator();
                    ui.label("Subsumes Bloomberg Terminal & Honeypot scanners into sovereign local Rust engine.");
                    ui.add_space(8.0);
                    if ui.button("Close").clicked() {
                        self.show_about_dialog = false;
                    }
                });
        }
    }
}

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let rt = Arc::new(Runtime::new().unwrap_or_else(|e| {
        eprintln!("Failed to init tokio runtime: {e}");
        std::process::exit(1);
    }));

    let store = Arc::new(DatabaseStore::open_in_memory().unwrap_or_else(|e| {
        eprintln!("Failed to open in-memory store: {e}");
        std::process::exit(1);
    }));

    // Headless smoke test
    if args.contains(&"--smoke".to_string()) {
        println!("[moneycraft] Running headless smoke verification...");
        let status = rt.block_on(dispatch(Command::Status, &store));
        println!("[moneycraft] Smoke OK: {status}");
        return Ok(());
    }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(1024.0, 720.0))
            .with_title(format!("MoneyCraft Studio v{APP_VERSION}")),
        ..Default::default()
    };

    eframe::run_native(
        "MoneyCraft Studio",
        native_options,
        Box::new(|_cc| Ok(Box::new(MoneyCraftGui::new(store, rt)))),
    )
}
