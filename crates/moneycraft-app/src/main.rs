//! MoneyCraft Native Desktop Studio.
//!
//! Visual financial terminal with Bloomberg Terminal grade time-series charting,
//! Personal Wealth & Savings simulator for beginners (Girokonto, Festgeld, Bausparer, Inflation run-rate erosion),
//! Global Indices desk (Dow Jones, S&P 500, Nasdaq, DAX, Nikkei, KOSPI, FTSE, Hang Seng),
//! multi-symbol normalized comparison overlays, arbitrary date-range filtering,
//! crypto derivatives intelligence (Hyperliquid funding arbitrage & liquidation heatmaps),
//! fantasy paper trading arena (leveraged futures & spot sandbox),
//! real-time Bloomberg news ticker wire, institutional agentic AI workflows,
//! sandbox EVM honeypot auditor, algorithmic backtester, and sovereign portfolio ledger.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use databasecraft_store::DatabaseStore;
use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};
use moneycraft_core::{compute_personal_wealth_projection, dispatch, Command, PersonalWealthConfig, CONTROL_PORT};
use serde_json::Value;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveWorkspace {
    Terminal,
    DataSources,
    PersonalWealth,
    MultiComparison,
    CryptoDerivatives,
    FantasyTrading,
    AiWorkflows,
    HoneypotAuditor,
    Backtester,
    PortfolioLedger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartStyle {
    Candlestick,
    MountainArea,
    HeikinAshi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateRangePreset {
    Day1,
    Week1,
    Month1,
    Month3,
    YearToDate,
    Year1,
    Year5,
    Max1990,      // Since 1990 (35+ years)
    Historic1850, // Back to 1850 (London Victorian Era, East India Co., 1857 Panic, 1866 Overend Gurney)
    AllTime1720,  // Back to 1720 (South Sea Bubble, Exchange Alley)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiWorkflowMode {
    StrategyGen,
    MacroStressTest,
    SentimentAlpha,
    ExecutionRouter,
}

pub struct MoneyCraftGui {
    pub store: Arc<DatabaseStore>,
    pub runtime: Arc<Runtime>,
    pub active_workspace: ActiveWorkspace,
    pub selected_symbol: String,
    pub command_input: String,

    // Personal Wealth Simulator Inputs (Beginner Financial Reality)
    pub wealth_config: PersonalWealthConfig,
    pub wealth_report: Option<Value>,

    // Switchable Market Data Feed & Exchange Payola Options
    pub active_data_source: String,

    // Bloomberg Charting Controls & Date Range
    pub chart_style: ChartStyle,
    pub date_preset: DateRangePreset,
    pub custom_bar_count: usize,
    pub show_sma20: bool,
    pub show_ema50: bool,
    pub show_bollinger: bool,
    pub show_volume: bool,
    pub show_rsi: bool,
    pub show_historic_panics: bool,

    // Fleet-Standard i18n
    pub locale: moneycraft_core::i18n::AppLocale,

    // Multi-Symbol Comparison Engine
    pub comparison_symbols: Vec<String>,
    pub comparison_data: Option<Value>,

    // News Ticker
    pub news_index: usize,
    pub news_data: Option<Value>,

    // Derivatives Data
    pub derivatives_data: Option<Value>,

    // Fantasy Paper Trading Desk
    pub paper_account: Option<Value>,
    pub paper_order_side: String, // "LONG" or "SHORT"
    pub paper_order_type: String, // "SPOT" or "PERP_FUTURES"
    pub paper_leverage: f64,
    pub paper_quantity: f64,

    // AI Workflows
    pub ai_mode: AiWorkflowMode,
    pub ai_prompt: String,
    pub ai_result: Option<Value>,

    // Module States
    pub audit_address: String,
    pub audit_chain: String,
    pub strategy_name: String,
    pub initial_capital: f64,
    pub market_data: Option<Value>,
    pub audit_report: Option<Value>,
    pub backtest_result: Option<Value>,
    pub portfolio_data: Option<Value>,
    pub status_message: String,

    // UI state
    pub is_dark_theme: bool,
    pub show_about_dialog: bool,
    pub show_auth_modal: bool,
    pub is_authenticated: bool,
    pub auth_username: String,
    pub auth_input_user: String,
    pub auth_input_pass: String,
}

impl MoneyCraftGui {
    pub fn new(store: Arc<DatabaseStore>, runtime: Arc<Runtime>) -> Self {
        let default_wealth_cfg = PersonalWealthConfig {
            monthly_income_eur: 2000.0,
            monthly_living_costs_eur: 1400.0,
            checking_balance_eur: 4200.0,
            festgeld_principal_eur: 15000.0,
            festgeld_interest_rate_pct: 3.0,
            festgeld_term_years: 3.0,
            bausparer_monthly_deposit_eur: 100.0,
            bausparer_balance_eur: 5400.0,
            bausparer_interest_rate_pct: 1.5,
            inflation_run_rate_pct: 3.8,
            simulation_years: 5,
        };

        let mut app = Self {
            store,
            runtime,
            active_workspace: ActiveWorkspace::PersonalWealth,
            selected_symbol: "DJIA".to_string(),
            command_input: String::new(),
            wealth_config: default_wealth_cfg,
            wealth_report: None,
            active_data_source: "synthetic".to_string(),
            chart_style: ChartStyle::Candlestick,
            date_preset: DateRangePreset::Month1,
            custom_bar_count: 60,
            show_sma20: true,
            show_ema50: true,
            show_bollinger: true,
            show_volume: true,
            show_rsi: true,
            show_historic_panics: true,
            locale: moneycraft_core::i18n::AppLocale::En,
            comparison_symbols: vec!["DJIA".to_string(), "SPX".to_string(), "KOSPI".to_string(), "DAX".to_string()],
            comparison_data: None,
            news_index: 0,
            news_data: None,
            derivatives_data: None,
            paper_account: None,
            paper_order_side: "LONG".to_string(),
            paper_order_type: "PERP_FUTURES".to_string(),
            paper_leverage: 10.0,
            paper_quantity: 0.5,
            ai_mode: AiWorkflowMode::StrategyGen,
            ai_prompt: "Develop a momentum breakout strategy with mean-reversion Bollinger bands on KOSPI vs DJIA".to_string(),
            ai_result: None,
            audit_address: "0x123_scam_token_trap".to_string(),
            audit_chain: "Ethereum".to_string(),
            strategy_name: "BollingerBreakout_v2".to_string(),
            initial_capital: 100_000.0,
            market_data: None,
            audit_report: None,
            backtest_result: None,
            portfolio_data: None,
            status_message: "Bloomberg Sovereign Terminal Ready".to_string(),
            is_dark_theme: true,
            show_about_dialog: false,
            show_auth_modal: false,
            is_authenticated: false,
            auth_username: "Sandra_Desk_01".to_string(),
            auth_input_user: "sandra".to_string(),
            auth_input_pass: String::new(),
        };

        app.run_wealth_simulation();
        app.fetch_market_feed();
        app.fetch_multi_comparison();
        app.fetch_derivatives();
        app.fetch_news_ticker();
        app.fetch_paper_account();
        app.fetch_portfolio();
        app
    }

    pub fn run_wealth_simulation(&mut self) {
        let rep = compute_personal_wealth_projection(&self.wealth_config);
        self.wealth_report = Some(serde_json::to_value(&rep).unwrap_or(Value::Null));
        self.status_message = "Personal wealth & inflation erosion simulation recalculated".to_string();
    }

    pub fn fetch_market_feed(&mut self) {
        let sym = self.selected_symbol.clone();
        let bars = self.custom_bar_count;
        let src = self.active_data_source.clone();
        let res = self.runtime.block_on(dispatch(Command::GetMarketFeed { symbol: sym.clone(), bar_count: Some(bars), source: Some(src.clone()) }, &self.store));
        let note = res["status_note"].as_str().unwrap_or("Feed updated").to_string();
        self.market_data = Some(res);
        self.status_message = format!("[{src}] {note} for {sym} ({bars} bars)");
    }

    pub fn fetch_multi_comparison(&mut self) {
        let syms = self.comparison_symbols.clone();
        let bars = self.custom_bar_count;
        let src = self.active_data_source.clone();
        let res = self.runtime.block_on(dispatch(Command::GetMultiSymbolFeed { symbols: syms, bar_count: Some(bars), source: Some(src) }, &self.store));
        self.comparison_data = Some(res);
    }

    pub fn set_date_preset(&mut self, preset: DateRangePreset) {
        self.date_preset = preset;
        self.custom_bar_count = match preset {
            DateRangePreset::Day1 => 24,
            DateRangePreset::Week1 => 35,
            DateRangePreset::Month1 => 60,
            DateRangePreset::Month3 => 90,
            DateRangePreset::YearToDate => 120,
            DateRangePreset::Year1 => 180,
            DateRangePreset::Year5 => 240,
            DateRangePreset::Max1990 => 300,
            DateRangePreset::Historic1850 => 350,
            DateRangePreset::AllTime1720 => 400,
        };
        self.fetch_market_feed();
        self.fetch_multi_comparison();
    }

    pub fn fetch_derivatives(&mut self) {
        let sym = self.selected_symbol.clone();
        let res = self.runtime.block_on(dispatch(Command::GetCryptoDerivativesFeed { symbol: sym.clone() }, &self.store));
        self.derivatives_data = Some(res);
    }

    pub fn fetch_news_ticker(&mut self) {
        let res = self.runtime.block_on(dispatch(Command::GetNewsTicker, &self.store));
        self.news_data = Some(res);
    }

    pub fn fetch_paper_account(&mut self) {
        let res = self.runtime.block_on(dispatch(Command::PaperGetAccount, &self.store));
        self.paper_account = Some(res);
    }

    pub fn open_paper_order(&mut self) {
        let sym = self.selected_symbol.clone();
        let side = self.paper_order_side.clone();
        let ot = self.paper_order_type.clone();
        let qty = self.paper_quantity;
        let lev = self.paper_leverage;
        let res = self.runtime.block_on(dispatch(
            Command::PaperOpenPosition {
                symbol: sym,
                side,
                order_type: ot,
                quantity: qty,
                leverage: lev,
                price: None,
            },
            &self.store,
        ));
        self.status_message = res["message"].as_str().unwrap_or("Fantasy position placed").to_string();
        self.fetch_paper_account();
    }

    pub fn close_paper_position(&mut self, pos_id: &str) {
        let res = self.runtime.block_on(dispatch(Command::PaperClosePosition { position_id: pos_id.to_string() }, &self.store));
        self.status_message = res["message"].as_str().unwrap_or("Position closed").to_string();
        self.fetch_paper_account();
    }

    pub fn reset_paper_account(&mut self) {
        let res = self.runtime.block_on(dispatch(Command::PaperResetAccount { initial_balance: Some(100_000.0) }, &self.store));
        self.status_message = res["message"].as_str().unwrap_or("Account reset").to_string();
        self.fetch_paper_account();
    }

    pub fn run_ai_workflow(&mut self) {
        let wf_str = match self.ai_mode {
            AiWorkflowMode::StrategyGen => "STRATEGY_GEN",
            AiWorkflowMode::MacroStressTest => "STRESS_TEST",
            AiWorkflowMode::SentimentAlpha => "SENTIMENT_ALPHA",
            AiWorkflowMode::ExecutionRouter => "EXECUTION_ROUTER",
        };
        let p = self.ai_prompt.clone();
        let sym = self.selected_symbol.clone();
        let res = self.runtime.block_on(dispatch(
            Command::RunAiWorkflow {
                workflow_type: wf_str.to_string(),
                prompt: p,
                target_asset: sym,
            },
            &self.store,
        ));
        self.ai_result = Some(res);
        self.status_message = format!("AI Workflow [{wf_str}] completed successfully");
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
            let mut visuals = egui::Visuals::dark();
            visuals.panel_fill = Color32::from_rgb(10, 13, 20); // Bloomberg Obsidian
            visuals.window_fill = Color32::from_rgb(8, 10, 16);
            ui.ctx().set_visuals(visuals);
        } else {
            ui.ctx().set_visuals(egui::Visuals::light());
        }

        let amber = Color32::from_rgb(255, 179, 0); // Classic Bloomberg Orange
        let green = Color32::from_rgb(0, 230, 118);  // Jade Bull Green
        let red = Color32::from_rgb(255, 23, 68);    // Crimson Bear Red

        // 0. Desktop Menu Bar
        egui::Panel::top("desktop_menu_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Sync Sovereign Ledger").clicked() {
                        self.fetch_portfolio();
                        ui.close();
                    }
                    if ui.button("Refresh Market Feed").clicked() {
                        self.fetch_market_feed();
                        self.fetch_multi_comparison();
                        self.fetch_derivatives();
                        ui.close();
                    }
                    if ui.button("Recalculate Wealth Simulation").clicked() {
                        self.run_wealth_simulation();
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Personal Wealth", |ui| {
                    if ui.button("💶 Beginner Wealth & Inflation Simulator").clicked() {
                        self.active_workspace = ActiveWorkspace::PersonalWealth;
                        ui.close();
                    }
                    if ui.button("💼 Sovereign Portfolio Ledger").clicked() {
                        self.active_workspace = ActiveWorkspace::PortfolioLedger;
                        ui.close();
                    }
                });

                ui.menu_button("Global Indices", |ui| {
                    for sym in &["DJIA", "SPX", "NDX", "DAX", "N225", "KOSPI", "FTSE", "HSI"] {
                        if ui.button(*sym).clicked() {
                            self.selected_symbol = sym.to_string();
                            self.fetch_market_feed();
                            self.fetch_derivatives();
                            ui.close();
                        }
                    }
                });

                ui.menu_button("Forex Desk", |ui| {
                    for sym in &["EUR/USD", "USD/JPY", "GBP/USD", "USD/CNY", "EUR/GBP"] {
                        if ui.button(*sym).clicked() {
                            self.selected_symbol = sym.to_string();
                            self.fetch_market_feed();
                            self.fetch_derivatives();
                            ui.close();
                        }
                    }
                });

                ui.menu_button("Crypto & Perps", |ui| {
                    for sym in &["BTC/USD", "ETH/USD", "SOL/USD"] {
                        if ui.button(*sym).clicked() {
                            self.selected_symbol = sym.to_string();
                            self.fetch_market_feed();
                            self.fetch_derivatives();
                            ui.close();
                        }
                    }
                });

                ui.menu_button("AI Workflows", |ui| {
                    if ui.button("BQuant Strategy Generator").clicked() {
                        self.active_workspace = ActiveWorkspace::AiWorkflows;
                        self.ai_mode = AiWorkflowMode::StrategyGen;
                        ui.close();
                    }
                    if ui.button("Macro Shock Stress-Testing").clicked() {
                        self.active_workspace = ActiveWorkspace::AiWorkflows;
                        self.ai_mode = AiWorkflowMode::MacroStressTest;
                        ui.close();
                    }
                    if ui.button("NLP Sentiment Alpha Scorer").clicked() {
                        self.active_workspace = ActiveWorkspace::AiWorkflows;
                        self.ai_mode = AiWorkflowMode::SentimentAlpha;
                        ui.close();
                    }
                    if ui.button("Smart Order Router (SOR)").clicked() {
                        self.active_workspace = ActiveWorkspace::AiWorkflows;
                        self.ai_mode = AiWorkflowMode::ExecutionRouter;
                        ui.close();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About MoneyCraft Bloomberg Terminal").clicked() {
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
                        ui.colored_label(amber, format!("💰 {}", self.auth_username));
                    } else if ui.small_button("🔑 Login").clicked() {
                        self.show_auth_modal = true;
                    }
                });
            });
        });

        // 1. Bloomberg Top Ticker Bar
        egui::Panel::top("finance_top_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.strong(egui::RichText::new("BLOOMBERG // MONEYCRAFT SOVEREIGN TERMINAL").color(amber).monospace().size(15.0));
                ui.separator();

                // Live Summary Ticker
                if let Some(ref mkt) = self.market_data {
                    if let Some(summary) = mkt.get("summary") {
                        let cur_price = summary["current_price"].as_f64().unwrap_or(0.0);
                        let change_pct = summary["change_24h_pct"].as_f64().unwrap_or(0.0);
                        let change_usd = summary["change_24h_usd"].as_f64().unwrap_or(0.0);
                        let is_forex = summary["is_forex"].as_bool().unwrap_or(false);
                        let is_index = summary["is_index"].as_bool().unwrap_or(false);
                        let pip_precision = summary["pip_precision"].as_u64().unwrap_or(2) as usize;

                        ui.label(egui::RichText::new(&self.selected_symbol).strong().color(Color32::WHITE));

                        let fmt_price = if is_forex {
                            format!("{cur_price:.prec$}", prec = pip_precision)
                        } else if is_index {
                            format!("{cur_price:.2} PTS")
                        } else {
                            format!("${cur_price:.2}")
                        };

                        ui.label(egui::RichText::new(fmt_price).monospace().size(15.0).color(if change_pct >= 0.0 { green } else { red }));

                        let sign = if change_pct >= 0.0 { "+" } else { "" };
                        let change_txt = format!("{sign}{change_usd:+.2} ({sign}{change_pct:.2}%)");
                        ui.colored_label(if change_pct >= 0.0 { green } else { red }, change_txt);
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(format!("PORT: {CONTROL_PORT}")).monospace().color(amber));
                    ui.separator();

                    // Fleet Standard i18n Language Switcher
                    let cur_loc = self.locale;
                    let label = format!("{} {}", cur_loc.flag(), cur_loc.code().to_uppercase());
                    egui::ComboBox::from_id_salt("topbar_locale_selector")
                        .selected_text(label)
                        .show_ui(ui, |ui| {
                            for loc in moneycraft_core::i18n::AppLocale::all() {
                                let text = format!("{} {}", loc.flag(), loc.display_name());
                                let _ = ui.selectable_value(&mut self.locale, *loc, text);
                            }
                        });
                });
            });
        });

        // 2. Real-Time Bloomberg Breaking News Ticker Bar
        egui::Panel::top("breaking_news_ticker").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("⚡ NEWS WIRE:").color(amber).monospace().strong());
                if let Some(ref nd) = self.news_data {
                    if let Some(headlines) = nd["headlines"].as_array() {
                        if !headlines.is_empty() {
                            let item = &headlines[self.news_index % headlines.len()];
                            let cat = item["category"].as_str().unwrap_or("MACRO");
                            let time = item["timestamp_str"].as_str().unwrap_or("12:00:00");
                            let txt = item["headline"].as_str().unwrap_or("");
                            let sentiment = item["sentiment"].as_str().unwrap_or("NEUTRAL");

                            let s_color = match sentiment {
                                "BULLISH" => green,
                                "BEARISH" => red,
                                _ => amber,
                            };

                            ui.label(egui::RichText::new(format!("[{time}]")).weak().monospace());
                            ui.label(egui::RichText::new(format!("[{cat}]")).color(amber).monospace());
                            ui.label(egui::RichText::new(txt).color(Color32::WHITE).strong());
                            ui.label(egui::RichText::new(format!("• {sentiment}")).color(s_color).monospace().strong());

                            if ui.button("▶ Next Wire").clicked() {
                                self.news_index = (self.news_index + 1) % headlines.len();
                            }
                        }
                    }
                } else {
                    ui.label(egui::RichText::new("Connecting to sovereign news wire...").weak());
                }
            });
        });

        // 3. Bloomberg Bottom Command & Status Bar
        egui::Panel::bottom("finance_bottom_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&self.status_message).weak().monospace());
                ui.separator();
                ui.label(egui::RichText::new("QUANT KERNEL: ACTIVE").color(green).monospace());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("GO <ENTER>").clicked() {
                        let cmd_str = self.command_input.trim().to_uppercase();
                        if !cmd_str.is_empty() {
                            self.selected_symbol = cmd_str.clone();
                            self.fetch_market_feed();
                            self.fetch_derivatives();
                            self.command_input.clear();
                        }
                    }
                    ui.text_edit_singleline(&mut self.command_input);
                    ui.label(egui::RichText::new("TERMINAL CMD:").color(amber).monospace());
                });
            });
        });

        // 4. Navigation Rail
        egui::Panel::left("finance_nav_rail").min_size(220.0).show(ui, |ui| {
            ui.add_space(8.0);
            ui.strong(egui::RichText::new("TERMINAL FUNCTIONS").color(amber));
            ui.add_space(4.0);

            let nav_btn = |ui: &mut egui::Ui, active: bool, text: &str| -> bool {
                let rich = egui::RichText::new(text).color(if active { amber } else { Color32::WHITE });
                ui.selectable_label(active, rich).clicked()
            };

            let loc = self.locale;
            use moneycraft_core::i18n::{t, I18nKey};

            if nav_btn(ui, self.active_workspace == ActiveWorkspace::PersonalWealth, t(I18nKey::NavPersonalWealth, loc)) {
                self.active_workspace = ActiveWorkspace::PersonalWealth;
                self.run_wealth_simulation();
            }
            if nav_btn(ui, self.active_workspace == ActiveWorkspace::Terminal, t(I18nKey::NavTerminal, loc)) {
                self.active_workspace = ActiveWorkspace::Terminal;
                self.fetch_market_feed();
            }
            if nav_btn(ui, self.active_workspace == ActiveWorkspace::DataSources, t(I18nKey::NavDataSources, loc)) {
                self.active_workspace = ActiveWorkspace::DataSources;
            }
            if nav_btn(ui, self.active_workspace == ActiveWorkspace::MultiComparison, t(I18nKey::NavMultiCompare, loc)) {
                self.active_workspace = ActiveWorkspace::MultiComparison;
                self.fetch_multi_comparison();
            }
            if nav_btn(ui, self.active_workspace == ActiveWorkspace::CryptoDerivatives, t(I18nKey::NavCryptoDerivatives, loc)) {
                self.active_workspace = ActiveWorkspace::CryptoDerivatives;
                self.fetch_derivatives();
            }
            if nav_btn(ui, self.active_workspace == ActiveWorkspace::FantasyTrading, t(I18nKey::NavFantasyTrading, loc)) {
                self.active_workspace = ActiveWorkspace::FantasyTrading;
                self.fetch_paper_account();
            }
            if nav_btn(ui, self.active_workspace == ActiveWorkspace::AiWorkflows, t(I18nKey::NavAiWorkflows, loc)) {
                self.active_workspace = ActiveWorkspace::AiWorkflows;
            }
            if nav_btn(ui, self.active_workspace == ActiveWorkspace::HoneypotAuditor, t(I18nKey::NavHoneypotAuditor, loc)) {
                self.active_workspace = ActiveWorkspace::HoneypotAuditor;
            }
            if nav_btn(ui, self.active_workspace == ActiveWorkspace::Backtester, t(I18nKey::NavBacktester, loc)) {
                self.active_workspace = ActiveWorkspace::Backtester;
            }
            if nav_btn(ui, self.active_workspace == ActiveWorkspace::PortfolioLedger, t(I18nKey::NavPortfolioLedger, loc)) {
                self.active_workspace = ActiveWorkspace::PortfolioLedger;
                self.fetch_portfolio();
            }

            ui.add_space(8.0);
            ui.separator();
            ui.strong(egui::RichText::new("GLOBAL INDICES").color(amber));
            for sym in &["DJIA", "SPX", "NDX", "DAX", "N225", "KOSPI", "FTSE", "HSI"] {
                let is_sel = self.selected_symbol == *sym;
                if ui.selectable_label(is_sel, *sym).clicked() {
                    self.selected_symbol = sym.to_string();
                    self.fetch_market_feed();
                    self.fetch_derivatives();
                }
            }

            ui.add_space(8.0);
            ui.separator();
            ui.strong(egui::RichText::new("FOREX DESK").color(amber));
            for sym in &["EUR/USD", "USD/JPY", "GBP/USD", "USD/CNY", "EUR/GBP"] {
                let is_sel = self.selected_symbol == *sym;
                if ui.selectable_label(is_sel, *sym).clicked() {
                    self.selected_symbol = sym.to_string();
                    self.fetch_market_feed();
                    self.fetch_derivatives();
                }
            }

            ui.add_space(8.0);
            ui.separator();
            ui.strong(egui::RichText::new("📜 HISTORIC PAPERS (LONDON)").color(amber));
            for (sym, desc) in &[
                ("EIC", "Honourable East India Co. (1600-1874)"),
                ("UK_CONSOLS", "Bank of England 3% Consols (1751+)"),
                ("SOUTH_SEA", "South Sea Company (1711-1720)"),
            ] {
                let is_sel = self.selected_symbol == *sym;
                if ui.selectable_label(is_sel, format!("{sym} - {desc}")).clicked() {
                    self.selected_symbol = sym.to_string();
                    self.date_preset = DateRangePreset::Historic1850;
                    self.set_date_preset(DateRangePreset::Historic1850);
                    self.show_historic_panics = true;
                }
            }

            ui.add_space(8.0);
            ui.separator();
            ui.strong(egui::RichText::new("🚩 HISTORIC PANICS").color(red));
            for (year, name) in &[
                (1720, "1720: South Sea Bubble"),
                (1825, "1825: Latin American / Poyais"),
                (1847, "1847: Railway Mania"),
                (1857, "1857: Telegraph Panic"),
                (1866, "1866: Overend Gurney"),
                (1873, "1873: Gründerkrach"),
                (1929, "1929: Wall Street Crash"),
                (1987, "1987: Black Monday"),
                (2008, "2008: Global Financial Crisis"),
                (2020, "2020: Covid Liquidity Shock"),
            ] {
                if ui.button(format!("{name}")).clicked() {
                    self.show_historic_panics = true;
                    if *year < 1850 {
                        self.set_date_preset(DateRangePreset::AllTime1720);
                    } else if *year < 1990 {
                        self.set_date_preset(DateRangePreset::Historic1850);
                    } else {
                        self.set_date_preset(DateRangePreset::Max1990);
                    }
                    self.status_message = format!("Inspecting Crisis Year {year}: {name}");
                }
            }
        });

        // 5. Central Panel
        egui::CentralPanel::default().show(ui, |ui| match self.active_workspace {
            ActiveWorkspace::PersonalWealth => self.render_personal_wealth(ui),
            ActiveWorkspace::Terminal => self.render_bloomberg_terminal(ui),
            ActiveWorkspace::DataSources => self.render_data_sources_and_payola(ui),
            ActiveWorkspace::MultiComparison => self.render_multi_comparison(ui),
            ActiveWorkspace::CryptoDerivatives => self.render_crypto_derivatives(ui),
            ActiveWorkspace::FantasyTrading => self.render_fantasy_trading(ui),
            ActiveWorkspace::AiWorkflows => self.render_ai_workflows(ui),
            ActiveWorkspace::HoneypotAuditor => self.render_honeypot_auditor(ui),
            ActiveWorkspace::Backtester => self.render_backtester(ui),
            ActiveWorkspace::PortfolioLedger => self.render_portfolio_ledger(ui),
        });

        // Modals
        if self.show_auth_modal {
            egui::Window::new("🔑 Sovereign Trader Authentication")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label("Enter credentials for institutional execution rights:");
                    ui.horizontal(|ui| {
                        ui.label("Username:");
                        ui.text_edit_singleline(&mut self.auth_input_user);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Password:");
                        ui.text_edit_singleline(&mut self.auth_input_pass);
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Authenticate").clicked() {
                            self.is_authenticated = true;
                            self.auth_username = self.auth_input_user.clone();
                            self.show_auth_modal = false;
                            self.status_message = format!("Signed in as {}", self.auth_username);
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_auth_modal = false;
                        }
                    });
                });
        }

        if self.show_about_dialog {
            egui::Window::new("ℹ About MoneyCraft")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.heading("MoneyCraft Sovereign Terminal v0.6.0");
                    ui.label("Personal wealth & savings simulator (Girokonto, Festgeld 3%, Bausparer, Inflation run-rate erosion),");
                    ui.label("Bloomberg-grade financial analysis, high-density time-series charting,");
                    ui.label("Global Indices desk (Dow Jones, S&P 500, Nasdaq, DAX, Nikkei 225, KOSPI, FTSE, Hang Seng),");
                    ui.label("Multi-asset normalized comparison graph & arbitrary date-range presets (1D, 1W, 1M, 3M, YTD, 1Y),");
                    ui.label("Hyperliquid funding arbitrage & forward-looking liquidation heatmap,");
                    ui.label("Fantasy paper trading futures desk (1x-50x leverage sandbox),");
                    ui.label("G10 Forex desk (EUR, USD, JPY, GBP, CNY), breaking news ticker,");
                    ui.label("agentic AI quantitative workflows, EVM honeypot auditor, and sovereign tax ledger.");
                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.show_about_dialog = false;
                    }
                });
        }
    }
}

impl MoneyCraftGui {
    fn render_personal_wealth(&mut self, ui: &mut egui::Ui) {
        let amber = Color32::from_rgb(255, 179, 0);
        let green = Color32::from_rgb(0, 230, 118);
        let red = Color32::from_rgb(255, 23, 68);
        let cyan = Color32::from_rgb(0, 229, 255);

        ui.heading(egui::RichText::new("💶 PERSONAL WEALTH, SAVINGS & INFLATION EROSION SIMULATOR").color(amber));
        ui.label("Money for beginners: Visualize how inflation silently eats cash across checking accounts, fixed deposits, and Bausparer:");

        ui.add_space(8.0);
        let mut recompute = false;

        // 1. Interactive Inputs Panel
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.strong(egui::RichText::new("YOUR PERSONAL ACCOUNTS & CASHFLOW").color(amber));
            ui.columns(3, |cols| {
                // Column 1: Monthly Cashflow
                cols[0].label(egui::RichText::new("1. Monthly Cashflow").strong());
                cols[0].horizontal(|ui| {
                    ui.label("Monthly Income (In):");
                    if ui.add(egui::DragValue::new(&mut self.wealth_config.monthly_income_eur).range(500.0..=20000.0).speed(50.0).suffix(" €")).changed() {
                        recompute = true;
                    }
                });
                cols[0].horizontal(|ui| {
                    ui.label("Living Expenses (Out):");
                    if ui.add(egui::DragValue::new(&mut self.wealth_config.monthly_living_costs_eur).range(200.0..=15000.0).speed(50.0).suffix(" €")).changed() {
                        recompute = true;
                    }
                });
                cols[0].horizontal(|ui| {
                    ui.label("Checking (Girokonto):");
                    if ui.add(egui::DragValue::new(&mut self.wealth_config.checking_balance_eur).range(0.0..=50000.0).speed(100.0).suffix(" €")).changed() {
                        recompute = true;
                    }
                });

                // Column 2: Fixed Income & Bausparer
                cols[1].label(egui::RichText::new("2. Savings Products").strong());
                cols[1].horizontal(|ui| {
                    ui.label("Festgeld (Bound):");
                    if ui.add(egui::DragValue::new(&mut self.wealth_config.festgeld_principal_eur).range(0.0..=200000.0).speed(500.0).suffix(" €")).changed() {
                        recompute = true;
                    }
                });
                cols[1].horizontal(|ui| {
                    ui.label("Festgeld Rate / Term:");
                    if ui.add(egui::DragValue::new(&mut self.wealth_config.festgeld_interest_rate_pct).range(0.1..=8.0).speed(0.1).suffix(" %")).changed() {
                        recompute = true;
                    }
                    ui.label("for");
                    if ui.add(egui::DragValue::new(&mut self.wealth_config.festgeld_term_years).range(1.0..=10.0).speed(1.0).suffix(" yrs")).changed() {
                        recompute = true;
                    }
                });
                cols[1].horizontal(|ui| {
                    ui.label("Bausparer Deposit/mo:");
                    if ui.add(egui::DragValue::new(&mut self.wealth_config.bausparer_monthly_deposit_eur).range(0.0..=500.0).speed(10.0).suffix(" €")).changed() {
                        recompute = true;
                    }
                });
                cols[1].horizontal(|ui| {
                    ui.label("Bausparer Balance:");
                    if ui.add(egui::DragValue::new(&mut self.wealth_config.bausparer_balance_eur).range(0.0..=100000.0).speed(250.0).suffix(" €")).changed() {
                        recompute = true;
                    }
                });

                // Column 3: The Inflation Monster
                cols[2].label(egui::RichText::new("3. Macro Inflation Drag").strong());
                cols[2].horizontal(|ui| {
                    ui.label("Inflation Run Rate:");
                    if ui.add(egui::DragValue::new(&mut self.wealth_config.inflation_run_rate_pct).range(0.5..=15.0).speed(0.1).suffix(" %")).changed() {
                        recompute = true;
                    }
                });
                cols[2].horizontal(|ui| {
                    ui.label("Simulate Forward:");
                    if ui.add(egui::Slider::new(&mut self.wealth_config.simulation_years, 1..=10).text("Years")).changed() {
                        recompute = true;
                    }
                });
            });
        });

        if recompute {
            self.run_wealth_simulation();
        }

        ui.add_space(8.0);
        if let Some(ref rep_val) = self.wealth_report {
            let total_liq = rep_val["current_total_liquid_eur"].as_f64().unwrap_or(0.0);
            let free_cf = rep_val["monthly_free_cashflow_eur"].as_f64().unwrap_or(0.0);
            let annual_interest = rep_val["annual_nominal_interest_eur"].as_f64().unwrap_or(0.0);
            let annual_inflation_drag = rep_val["annual_inflation_drag_eur"].as_f64().unwrap_or(0.0);
            let net_real_yield = rep_val["net_real_yield_pct"].as_f64().unwrap_or(0.0);
            let halve_years = rep_val["years_to_halve_purchasing_power"].as_f64().unwrap_or(0.0);

            // 2. Big Numbers Diagnostic HUD
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("CURRENT SAVINGS: {total_liq:.0} €")).size(15.0).strong());
                    ui.separator();
                    ui.colored_label(green, format!("MONTHLY SAVINGS FLOW: +{free_cf:.0} €/mo"));
                    ui.separator();
                    ui.label(format!("INTEREST EARNED: +{annual_interest:.0} €/yr"));
                    ui.separator();
                    ui.colored_label(red, format!("INFLATION EROSION: -{annual_inflation_drag:.0} €/yr"));
                    ui.separator();

                    let net_color = if net_real_yield >= 0.0 { green } else { red };
                    ui.colored_label(net_color, format!("NET REAL RETURN: {net_real_yield:+.2}%"));
                });
            });

            // 3. The Shocking Truth Callout Box
            if net_real_yield < 0.0 {
                ui.add_space(4.0);
                egui::Frame::group(ui.style()).fill(Color32::from_rgb(32, 12, 16)).show(ui, |ui| {
                    ui.colored_label(red, egui::RichText::new(format!("⚠️ INFLATION WARNING: Your capital is losing purchasing power at {net_real_yield:.2}% per year.")).strong());
                    ui.label(format!("At this run-rate, your money's real purchasing power will be cut in half in {halve_years:.1} years (Rule of 72)."));
                    ui.label("Your 3% Festgeld sounds safe, but inflation (3.8%) means you are guaranteed to end up poorer in real terms.");
                });
            }

            ui.add_space(10.0);
            // 4. Nominal vs Real Wealth Projections Table
            ui.strong(egui::RichText::new("NOMINAL ILLUSION VS. REAL PURCHASING POWER (YEAR-BY-YEAR)").color(amber));
            if let Some(proj) = rep_val["projections"].as_array() {
                egui::Grid::new("wealth_proj_grid").striped(true).min_col_width(95.0).show(ui, |ui| {
                    ui.strong("Year");
                    ui.strong("Checking");
                    ui.strong("Festgeld");
                    ui.strong("Bausparer");
                    ui.strong("Nominal Total (Bank App)");
                    ui.strong("Real Value (True Goods)");
                    ui.strong("Silent Inflation Loss");
                    ui.strong("Value Kept");
                    ui.strong("Real Monthly Income");
                    ui.end_row();

                    for p in proj {
                        let y = p["year"].as_u64().unwrap_or(0);
                        let chk = p["checking_eur"].as_f64().unwrap_or(0.0);
                        let fest = p["festgeld_nominal_eur"].as_f64().unwrap_or(0.0);
                        let bau = p["bausparer_nominal_eur"].as_f64().unwrap_or(0.0);
                        let nom = p["total_nominal_wealth_eur"].as_f64().unwrap_or(0.0);
                        let real = p["total_real_wealth_eur"].as_f64().unwrap_or(0.0);
                        let loss = p["cumulative_inflation_loss_eur"].as_f64().unwrap_or(0.0);
                        let ret = p["purchasing_power_retention_pct"].as_f64().unwrap_or(0.0);
                        let m_real = p["monthly_purchasing_power_eur"].as_f64().unwrap_or(0.0);

                        ui.label(format!("Year {y}"));
                        ui.label(format!("{chk:.0} €"));
                        ui.label(format!("{fest:.0} €"));
                        ui.label(format!("{bau:.0} €"));
                        ui.colored_label(cyan, format!("{nom:.0} €"));
                        ui.colored_label(if real >= nom * 0.9 { green } else { amber }, format!("{real:.0} €"));
                        ui.colored_label(red, format!("-{loss:.0} €"));
                        ui.label(format!("{ret:.1}%"));
                        ui.label(format!("{m_real:.0} €/mo"));
                        ui.end_row();
                    }
                });
            }

            ui.add_space(12.0);
            // 5. Educational Antidote Cards (Money for Beginners)
            ui.strong(egui::RichText::new("SOLUTIONS: HOW TO DEFEND YOUR CAPITAL (ACTIONABLE ROADMAP)").color(green));
            if let Some(tips) = rep_val["inflation_antidotes"].as_array() {
                ui.horizontal(|ui| {
                    for t in tips {
                        let title = t["title"].as_str().unwrap_or("");
                        let cat = t["category"].as_str().unwrap_or("");
                        let exp = t["explanation"].as_str().unwrap_or("");
                        let net_y = t["projected_real_yield_pct"].as_f64().unwrap_or(0.0);

                        egui::Frame::group(ui.style()).show(ui, |ui| {
                            ui.set_max_width(260.0);
                            ui.label(egui::RichText::new(format!("[{cat}]")).color(amber).monospace());
                            ui.strong(title);
                            ui.label(exp);
                            ui.separator();
                            let col = if net_y >= 0.0 { green } else { red };
                            ui.colored_label(col, format!("Estimated Real Yield: {net_y:+.1}%"));
                        });
                    }
                });
            }
        }
    }

    fn render_data_sources_and_payola(&mut self, ui: &mut egui::Ui) {
        let amber = Color32::from_rgb(255, 179, 0);
        let green = Color32::from_rgb(0, 230, 118);
        let cyan = Color32::from_rgb(0, 229, 255);
        let purple = Color32::from_rgb(179, 136, 255);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading(egui::RichText::new("📡 MARKET DATA FEEDS, DELAY AUDITING & EXCHANGE PAYOLA MATRIX").color(amber));
            ui.label("Sovereign financial transparency: switch between simulated models and real-world public feeds, audit latency delays, and bypass Wall Street exchange redistributor paywalls.");
            ui.add_space(8.0);

            // 1. Diagnostics HUD Bar
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("ACTIVE PROVIDER: {}", self.active_data_source.to_uppercase())).strong().size(15.0));
                    ui.separator();
                    ui.colored_label(green, "YOUR DATA FEE: 0.00 € / month");
                    ui.separator();
                    ui.colored_label(cyan, "BLOOMBERG TERMINAL AVOIDED: +24,000 $ – 32,000 $ / year SAVED");
                    ui.separator();
                    if self.active_data_source == "synthetic" {
                        ui.colored_label(cyan, "MODE: Air-Gapped Local Stochastic (0ms, 100% Offline)");
                    } else if self.active_data_source == "yahoo" {
                        ui.colored_label(amber, "MODE: Public Delayed TradFi (15-Min Lag, Real World)");
                    } else {
                        ui.colored_label(green, "MODE: Real-Time Live Feed Connected (<150ms)");
                    }
                });
            });

            ui.add_space(10.0);
            ui.strong(egui::RichText::new("AVAILABLE DATA PROVIDERS & EXCHANGE TIERS (CLICK TO SWITCH)").color(amber));
            ui.add_space(4.0);

            let sources = moneycraft_feed::MarketFeed::list_data_sources();
            egui::Grid::new("data_sources_grid").num_columns(3).spacing([12.0, 12.0]).show(ui, |ui| {
                let mut col_count = 0;
                for s in sources {
                    let is_active = self.active_data_source.eq_ignore_ascii_case(&s.id);
                    let border_color = if is_active { green } else { Color32::from_rgb(45, 55, 72) };

                    egui::Frame::group(ui.style())
                        .stroke(Stroke::new(if is_active { 2.0 } else { 1.0 }, border_color))
                        .fill(if is_active { Color32::from_rgb(10, 24, 18) } else { Color32::from_rgb(14, 18, 26) })
                        .show(ui, |ui| {
                            ui.set_width(280.0);
                            ui.set_height(260.0);

                            ui.horizontal(|ui| {
                                ui.strong(egui::RichText::new(&s.name).size(14.0));
                                if is_active {
                                    ui.colored_label(green, "[ACTIVE]");
                                }
                            });

                            ui.horizontal(|ui| {
                                let delay_color = if s.delay_seconds == 0 { green } else if s.delay_seconds <= 900 { amber } else { cyan };
                                ui.colored_label(delay_color, &s.delay_label);
                                ui.separator();
                                let cost_color = if !s.is_payola { green } else { purple };
                                ui.colored_label(cost_color, &s.cost_label);
                            });

                            ui.label(egui::RichText::new(format!("Auth: {}", s.auth_type)).small().monospace());
                            ui.label(&s.description);
                            ui.separator();
                            ui.label(egui::RichText::new(&s.payola_breakdown).small().color(Color32::from_rgb(160, 174, 192)));

                            ui.add_space(4.0);
                            if is_active {
                                ui.colored_label(green, "✓ Currently Feeding Terminal & Compare");
                            } else {
                                let btn_label = if s.is_payola {
                                    format!("⚡ Test / Select {}", s.name)
                                } else {
                                    format!("⚡ Switch to {}", s.name)
                                };
                                if ui.button(btn_label).clicked() {
                                    self.active_data_source = s.id.clone();
                                    self.fetch_market_feed();
                                    self.fetch_multi_comparison();
                                }
                            }
                        });

                    col_count += 1;
                    if col_count % 3 == 0 {
                        ui.end_row();
                    }
                }
            });

            ui.add_space(16.0);
            // 3. Deep Dive Educational Guide on Exchange Payola
            egui::Frame::group(ui.style()).fill(Color32::from_rgb(12, 16, 24)).show(ui, |ui| {
                ui.strong(egui::RichText::new("📖 THE EXCHANGE DATA MONOPOLY EXPLAINED (WHY MARKET DATA COSTS MONEY)").color(cyan).size(15.0));
                ui.add_space(6.0);

                ui.label(egui::RichText::new("1. The TradFi Exchange Monopoly (NYSE, NASDAQ, CME Group, Deutsche Börse):").strong());
                ui.label("Under US and European law, traditional exchanges own proprietary copyright over the bid/ask prices generated by market participants. They bundle this data into private feeds (e.g., NASDAQ TotalView, NYSE OpenBook) and charge monthly redistribution fees to every software vendor, broker, and trader.");
                ui.add_space(4.0);

                ui.label(egui::RichText::new("2. The 15-Minute Exemption:").strong());
                ui.label("By regulatory mandate, quotes and trades delayed by 15 minutes are considered public historical information and are exempt from direct redistributor royalties. This is how Yahoo Finance, Google Finance, and MoneyCraft provide 100% free delayed global equity indices (Dow Jones, S&P 500, DAX, KOSPI).");
                ui.add_space(4.0);

                ui.label(egui::RichText::new("3. Why Crypto Feeds Are 100% Free:").strong());
                ui.label("In decentralized crypto and derivative markets (Binance, Hyperliquid, Bybit), revenue is generated purely on trading execution fees. To maximize liquidity and algorithmic arbitrage, they broadcast ultra-low-latency real-time WebSockets and REST APIs for free with zero authentication.");
                ui.add_space(4.0);

                ui.label(egui::RichText::new("4. How Sovereign Quants Beat the $30,000 Bloomberg Cartel:").strong());
                ui.label("Professional retail quants combine: (a) Hyperliquid/Binance for real-time crypto perps ($0), (b) Yahoo Finance & ECB for macro reference ($0), and (c) Interactive Brokers Gateway for real-time TradFi — paying only the raw exchange pass-through fee ($1.50/month for OPRA US Equity Options, $1.25/month for Network A/B) via local port 7496, saving 99% compared to Wall Street institutional licenses.");
            });
        });
    }

    fn render_bloomberg_terminal(&mut self, ui: &mut egui::Ui) {
        let amber = Color32::from_rgb(255, 179, 0);
        let green = Color32::from_rgb(0, 230, 118);
        let red = Color32::from_rgb(255, 23, 68);
        let cyan = Color32::from_rgb(0, 229, 255);
        let gold = Color32::from_rgb(255, 214, 0);
        let purple = Color32::from_rgb(179, 136, 255);

        // Top Toolbar: Data Feed Source Selector & Real/Delayed Status
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("📡 FEED SOURCE:").color(amber).monospace().strong());
            let current_source_name = match self.active_data_source.as_str() {
                "binance" => "⚡ Binance (Live 0s, $0)",
                "hyperliquid" => "⚡ Hyperliquid L1 (Live 0s, $0)",
                "yahoo" => "⏱️ Yahoo Finance (15m Delay, $0)",
                "ecb" => "📅 ECB Reference (Daily, $0)",
                "twelve_data" => "⚡ TwelveData (Pro API Key)",
                "interactive_brokers" => "⚡ IBKR Gateway (Local:7496)",
                "polygon_io" => "⚡ Polygon.io SIP ($29/mo)",
                "bloomberg_bpipe" => "🔒 Bloomberg B-PIPE ($2.5k/mo)",
                _ => "⚡ Synthetic Offline (0ms, $0)",
            };

            egui::ComboBox::from_id_salt("feed_source_combo")
                .selected_text(current_source_name)
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.active_data_source, "synthetic".to_string(), "⚡ Synthetic Offline (0ms, $0)").clicked() {
                        self.fetch_market_feed();
                    }
                    if ui.selectable_value(&mut self.active_data_source, "binance".to_string(), "⚡ Binance Public (Real-Time 0s, $0)").clicked() {
                        self.fetch_market_feed();
                    }
                    if ui.selectable_value(&mut self.active_data_source, "hyperliquid".to_string(), "⚡ Hyperliquid L1 DEX (Real-Time 0s, $0)").clicked() {
                        self.fetch_market_feed();
                    }
                    if ui.selectable_value(&mut self.active_data_source, "yahoo".to_string(), "⏱️ Yahoo Finance (15m Delay, $0)").clicked() {
                        self.fetch_market_feed();
                    }
                    if ui.selectable_value(&mut self.active_data_source, "ecb".to_string(), "📅 ECB Euro Reference (Daily, $0)").clicked() {
                        self.fetch_market_feed();
                    }
                    ui.separator();
                    ui.label(egui::RichText::new("🔒 EXCHANGE PAYOLA TIERS").color(amber).small());
                    if ui.selectable_value(&mut self.active_data_source, "twelve_data".to_string(), "TwelveData Pro ($29/mo)").clicked() {
                        self.fetch_market_feed();
                    }
                    if ui.selectable_value(&mut self.active_data_source, "interactive_brokers".to_string(), "IBKR Gateway ($1.50/mo exchange fee)").clicked() {
                        self.fetch_market_feed();
                    }
                    if ui.selectable_value(&mut self.active_data_source, "polygon_io".to_string(), "Polygon.io SIP ($29-$199/mo)").clicked() {
                        self.fetch_market_feed();
                    }
                    if ui.selectable_value(&mut self.active_data_source, "bloomberg_bpipe".to_string(), "Bloomberg B-PIPE ($2,500/mo)").clicked() {
                        self.fetch_market_feed();
                    }
                });

            if let Some(ref mkt) = self.market_data {
                if let Some(note) = mkt["status_note"].as_str() {
                    let is_live = mkt["is_live"].as_bool().unwrap_or(false);
                    let badge_col = if is_live { green } else { gold };
                    ui.colored_label(badge_col, note);
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("📡 View Payola Matrix").clicked() {
                    self.active_workspace = ActiveWorkspace::DataSources;
                }
            });
        });
        ui.separator();

        // Sub-toolbar: Date Ranges, Timeframes, Chart Styles & Overlays
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("RANGE:").color(amber).monospace());
            if ui.selectable_label(self.date_preset == DateRangePreset::Day1, "1D").clicked() {
                self.set_date_preset(DateRangePreset::Day1);
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::Week1, "1W").clicked() {
                self.set_date_preset(DateRangePreset::Week1);
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::Month1, "1M").clicked() {
                self.set_date_preset(DateRangePreset::Month1);
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::Month3, "3M").clicked() {
                self.set_date_preset(DateRangePreset::Month3);
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::YearToDate, "YTD").clicked() {
                self.set_date_preset(DateRangePreset::YearToDate);
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::Year1, "1Y").clicked() {
                self.set_date_preset(DateRangePreset::Year1);
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::Year5, "5Y").clicked() {
                self.set_date_preset(DateRangePreset::Year5);
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::Max1990, "MAX (1990)").clicked() {
                self.set_date_preset(DateRangePreset::Max1990);
                self.show_historic_panics = true;
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::Historic1850, "HISTORIC (1850)").clicked() {
                self.set_date_preset(DateRangePreset::Historic1850);
                self.show_historic_panics = true;
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::AllTime1720, "ALL-TIME (1720)").clicked() {
                self.set_date_preset(DateRangePreset::AllTime1720);
                self.show_historic_panics = true;
            }

            ui.separator();
            ui.label(egui::RichText::new("STYLE:").color(amber).monospace());
            if ui.selectable_label(self.chart_style == ChartStyle::Candlestick, "🕯️ Candlestick").clicked() {
                self.chart_style = ChartStyle::Candlestick;
            }
            if ui.selectable_label(self.chart_style == ChartStyle::MountainArea, "📈 Mountain Area").clicked() {
                self.chart_style = ChartStyle::MountainArea;
            }
            if ui.selectable_label(self.chart_style == ChartStyle::HeikinAshi, "📊 Heikin-Ashi").clicked() {
                self.chart_style = ChartStyle::HeikinAshi;
            }

            ui.separator();
            ui.label(egui::RichText::new("OVERLAYS:").color(amber).monospace());
            ui.toggle_value(&mut self.show_sma20, egui::RichText::new("SMA20").color(cyan));
            ui.toggle_value(&mut self.show_ema50, egui::RichText::new("EMA50").color(gold));
            ui.toggle_value(&mut self.show_bollinger, egui::RichText::new("BBands").color(purple));
            ui.toggle_value(&mut self.show_volume, "Vol");
            ui.toggle_value(&mut self.show_rsi, "RSI");
            ui.toggle_value(&mut self.show_historic_panics, egui::RichText::new("🚩 Panics").color(red));
        });
        ui.separator();

        // High-Density Bloomberg Time-Series Graph Area
        let available_size = ui.available_size();
        let chart_height = (available_size.y - 40.0).max(360.0);
        let (response, painter) = ui.allocate_painter(Vec2::new(available_size.x, chart_height), egui::Sense::hover());
        let rect = response.rect;

        // Black Obsidian Canvas Background
        painter.rect_filled(rect, 4.0, Color32::from_rgb(6, 8, 12));
        painter.rect_stroke(rect, 4.0, Stroke::new(1.0, Color32::from_rgb(33, 41, 54)), egui::StrokeKind::Inside);

        let Some(ref mkt) = self.market_data else {
            painter.text(rect.center(), egui::Align2::CENTER_CENTER, "No market data loaded", egui::FontId::proportional(16.0), Color32::WHITE);
            return;
        };

        let Some(candles) = mkt["candles"].as_array() else {
            return;
        };

        let n = candles.len();
        if n == 0 {
            return;
        }

        // Sub-panel boundaries
        let margin_left = 60.0;
        let margin_right = 70.0;
        let top_padding = 24.0;
        let bottom_padding = 20.0;

        let has_vol = self.show_volume;
        let has_rsi = self.show_rsi;
        let rsi_height = if has_rsi { 75.0 } else { 0.0 };
        let vol_height = if has_vol { 60.0 } else { 0.0 };

        let total_w = rect.width() - margin_left - margin_right;
        let main_chart_h = rect.height() - top_padding - bottom_padding - rsi_height - vol_height - 16.0;

        let main_top = rect.top() + top_padding;
        let main_bottom = main_top + main_chart_h;

        let vol_top = main_bottom + 8.0;
        let vol_bottom = vol_top + vol_height;

        let rsi_top = if has_vol { vol_bottom + 8.0 } else { main_bottom + 8.0 };
        let rsi_bottom = rsi_top + rsi_height;

        // Calculate min/max price for auto-scaling
        let mut min_price = f64::MAX;
        let mut max_price = f64::MIN;
        let mut max_vol = 0.0;

        for c in candles {
            let h = c["high"].as_f64().unwrap_or(1.0);
            let l = c["low"].as_f64().unwrap_or(1.0);
            let v = c["volume"].as_f64().unwrap_or(0.0);
            if h > max_price { max_price = h; }
            if l < min_price { min_price = l; }
            if v > max_vol { max_vol = v; }
        }

        // Add 5% padding
        let p_range = (max_price - min_price).max(0.0001);
        let scaled_min = min_price - p_range * 0.05;
        let scaled_max = max_price + p_range * 0.05;
        let scaled_range = scaled_max - scaled_min;

        let to_y = |val: f64| -> f32 {
            let norm = ((val - scaled_min) / scaled_range) as f32;
            main_bottom - norm * main_chart_h
        };

        let step_x = total_w / (n as f32);
        let to_x = |idx: usize| -> f32 {
            rect.left() + margin_left + (idx as f32 + 0.5) * step_x
        };

        // Horizontal Grid Lines & Price Labels
        for i in 0..=5 {
            let f = i as f64 / 5.0;
            let val = scaled_min + f * scaled_range;
            let y = to_y(val);

            painter.line_segment(
                [Pos2::new(rect.left() + margin_left, y), Pos2::new(rect.right() - margin_right, y)],
                Stroke::new(0.8, Color32::from_rgb(20, 26, 36)),
            );

            let is_fx = self.selected_symbol.contains("EUR") || self.selected_symbol.contains("JPY") || self.selected_symbol.contains("GBP") || self.selected_symbol.contains("CNY");
            let label = if is_fx {
                if self.selected_symbol.contains("JPY") { format!("{val:.2}") } else { format!("{val:.4}") }
            } else {
                format!("{val:.2}")
            };

            painter.text(
                Pos2::new(rect.right() - margin_right + 6.0, y),
                egui::Align2::LEFT_CENTER,
                label,
                egui::FontId::monospace(10.0),
                Color32::from_rgb(140, 150, 165),
            );
        }

        // Bollinger Bands Background Ribbon
        if self.show_bollinger {
            if let Some(inds) = mkt.get("indicators") {
                if let (Some(u_arr), Some(l_arr)) = (inds["bb_upper"].as_array(), inds["bb_lower"].as_array()) {
                    let mut points_upper = Vec::new();
                    let mut points_lower = Vec::new();
                    for idx in 0..n {
                        if let (Some(u), Some(l)) = (u_arr.get(idx).and_then(|v| v.as_f64()), l_arr.get(idx).and_then(|v| v.as_f64())) {
                            let x = to_x(idx);
                            points_upper.push(Pos2::new(x, to_y(u)));
                            points_lower.push(Pos2::new(x, to_y(l)));
                        }
                    }
                    if points_upper.len() >= 2 {
                        for i in 0..(points_upper.len() - 1) {
                            let p1 = points_upper[i];
                            let p2 = points_upper[i + 1];
                            let p3 = points_lower[i + 1];
                            let p4 = points_lower[i];
                            painter.add(egui::Shape::convex_polygon(
                                vec![p1, p2, p3, p4],
                                Color32::from_rgba_unmultiplied(179, 136, 255, 20),
                                Stroke::NONE,
                            ));
                            painter.line_segment([p1, p2], Stroke::new(1.0, Color32::from_rgba_unmultiplied(179, 136, 255, 90)));
                            painter.line_segment([p4, p3], Stroke::new(1.0, Color32::from_rgba_unmultiplied(179, 136, 255, 90)));
                        }
                    }
                }
            }
        }

        // Render Candles / Mountain Chart
        let bar_w = (step_x * 0.7).clamp(2.0, 16.0);

        if self.chart_style == ChartStyle::MountainArea {
            let mut line_points = Vec::with_capacity(n);
            for (idx, c) in candles.iter().enumerate() {
                let close = c["close"].as_f64().unwrap_or(0.0);
                line_points.push(Pos2::new(to_x(idx), to_y(close)));
            }
            if line_points.len() >= 2 {
                for i in 0..(line_points.len() - 1) {
                    let p1 = line_points[i];
                    let p2 = line_points[i + 1];
                    let base1 = Pos2::new(p1.x, main_bottom);
                    let base2 = Pos2::new(p2.x, main_bottom);
                    painter.add(egui::Shape::convex_polygon(
                        vec![p1, p2, base2, base1],
                        Color32::from_rgba_unmultiplied(0, 230, 118, 30),
                        Stroke::NONE,
                    ));
                    painter.line_segment([p1, p2], Stroke::new(2.0, green));
                }
            }
        } else {
            // Candlesticks (Standard or Heikin-Ashi)
            for (idx, c) in candles.iter().enumerate() {
                let x = to_x(idx);
                let open = c["open"].as_f64().unwrap_or(0.0);
                let high = c["high"].as_f64().unwrap_or(0.0);
                let low = c["low"].as_f64().unwrap_or(0.0);
                let close = c["close"].as_f64().unwrap_or(0.0);

                let is_bull = close >= open;
                let candle_color = if is_bull { green } else { red };

                let y_high = to_y(high);
                let y_low = to_y(low);
                let y_open = to_y(open);
                let y_close = to_y(close);

                // Wick
                painter.line_segment([Pos2::new(x, y_high), Pos2::new(x, y_low)], Stroke::new(1.0, candle_color));

                // Body
                let (top, bot) = if y_open > y_close { (y_close, y_open) } else { (y_open, y_close) };
                let body_h = (bot - top).max(1.5);
                let candle_rect = Rect::from_center_size(Pos2::new(x, top + body_h * 0.5), Vec2::new(bar_w, body_h));
                painter.rect_filled(candle_rect, 1.0, candle_color);
            }
        }

        // Render SMA 20 Overlay
        if self.show_sma20 {
            if let Some(inds) = mkt.get("indicators") {
                if let Some(arr) = inds["sma_20"].as_array() {
                    let mut prev = None;
                    for (idx, v) in arr.iter().enumerate() {
                        if let Some(val) = v.as_f64() {
                            let curr = Pos2::new(to_x(idx), to_y(val));
                            if let Some(p) = prev {
                                painter.line_segment([p, curr], Stroke::new(1.6, cyan));
                            }
                            prev = Some(curr);
                        }
                    }
                }
            }
        }

        // Render EMA 50 Overlay
        if self.show_ema50 {
            if let Some(inds) = mkt.get("indicators") {
                if let Some(arr) = inds["ema_50"].as_array() {
                    let mut prev = None;
                    for (idx, v) in arr.iter().enumerate() {
                        if let Some(val) = v.as_f64() {
                            let curr = Pos2::new(to_x(idx), to_y(val));
                            if let Some(p) = prev {
                                painter.line_segment([p, curr], Stroke::new(1.6, gold));
                            }
                            prev = Some(curr);
                        }
                    }
                }
            }
        }

        // Volume Profile Sub-Panel
        if has_vol {
            painter.line_segment([Pos2::new(rect.left(), vol_top), Pos2::new(rect.right(), vol_top)], Stroke::new(1.0, Color32::from_rgb(25, 32, 44)));
            painter.text(Pos2::new(rect.left() + 6.0, vol_top + 4.0), egui::Align2::LEFT_TOP, "VOL", egui::FontId::monospace(10.0), Color32::from_rgb(140, 150, 165));

            let safe_max_vol = max_vol.max(1.0);
            for (idx, c) in candles.iter().enumerate() {
                let x = to_x(idx);
                let v = c["volume"].as_f64().unwrap_or(0.0);
                let open = c["open"].as_f64().unwrap_or(0.0);
                let close = c["close"].as_f64().unwrap_or(0.0);
                let is_bull = close >= open;
                let c_color = if is_bull {
                    Color32::from_rgba_unmultiplied(0, 230, 118, 120)
                } else {
                    Color32::from_rgba_unmultiplied(255, 23, 68, 120)
                };

                let h = ((v / safe_max_vol) as f32 * (vol_height - 6.0)).max(1.0);
                let v_rect = Rect::from_min_max(Pos2::new(x - bar_w * 0.45, vol_bottom - h), Pos2::new(x + bar_w * 0.45, vol_bottom));
                painter.rect_filled(v_rect, 0.5, c_color);
            }
        }

        // Historic Panics Vertical Marker Overlays
        if self.show_historic_panics {
            let panics = moneycraft_core::history::MarketHistory::all_panics();
            // Determine time span based on current date preset
            let (start_year, end_year) = match self.date_preset {
                DateRangePreset::AllTime1720 => (1720.0, 2026.0),
                DateRangePreset::Historic1850 => (1850.0, 2026.0),
                DateRangePreset::Max1990 => (1990.0, 2026.0),
                DateRangePreset::Year5 => (2021.0, 2026.0),
                DateRangePreset::Year1 => (2025.0, 2026.0),
                _ => (2025.0, 2026.0),
            };

            for p in &panics {
                let py = p.year as f64;
                if py >= start_year && py <= end_year {
                    let norm = (py - start_year) / (end_year - start_year);
                    let px = rect.left() + margin_left + (norm as f32) * total_w;

                    // Crimson vertical line with marker flag at top
                    painter.line_segment(
                        [Pos2::new(px, main_top), Pos2::new(px, main_bottom)],
                        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 23, 68, 160)),
                    );

                    // Panic Badge Banner
                    let badge_rect = Rect::from_min_size(Pos2::new(px - 16.0, main_top + 4.0), Vec2::new(32.0, 14.0));
                    painter.rect_filled(badge_rect, 2.0, Color32::from_rgb(180, 15, 45));
                    painter.text(
                        badge_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        format!("{}", p.year),
                        egui::FontId::monospace(9.0),
                        Color32::WHITE,
                    );

                    // Hover tooltip trigger if mouse is near vertical marker line
                    if let Some(mouse_pos) = response.hover_pos() {
                        if (mouse_pos.x - px).abs() < 12.0 && mouse_pos.y >= main_top && mouse_pos.y <= main_bottom {
                            let tip_w = 340.0;
                            let tip_h = 130.0;
                            let tip_x = (px + 10.0).min(rect.right() - tip_w - 10.0);
                            let tip_rect = Rect::from_min_size(Pos2::new(tip_x, mouse_pos.y.min(main_bottom - tip_h)), Vec2::new(tip_w, tip_h));

                            painter.rect_filled(tip_rect, 4.0, Color32::from_rgba_unmultiplied(15, 18, 26, 245));
                            painter.rect_stroke(tip_rect, 4.0, Stroke::new(1.0, Color32::from_rgb(255, 23, 68)), egui::StrokeKind::Inside);

                            let tip_txt = format!(
                                "🚩 CRISIS: {}\nDATE: {} | DROP: {:.1}%\nEPICENTER: {}\nCATALYST: {}\nLESSON: {}",
                                p.name, p.date_str, p.peak_to_trough_drop_pct, p.epicenter, p.primary_catalyst, p.lessons_for_modern_quants
                            );
                            painter.text(
                                Pos2::new(tip_rect.left() + 8.0, tip_rect.top() + 8.0),
                                egui::Align2::LEFT_TOP,
                                tip_txt,
                                egui::FontId::proportional(11.0),
                                Color32::WHITE,
                            );
                        }
                    }
                }
            }
        }

        // RSI 14 Sub-Panel
        if has_rsi {
            painter.line_segment([Pos2::new(rect.left(), rsi_top), Pos2::new(rect.right(), rsi_top)], Stroke::new(1.0, Color32::from_rgb(25, 32, 44)));
            painter.text(Pos2::new(rect.left() + 6.0, rsi_top + 4.0), egui::Align2::LEFT_TOP, "RSI 14", egui::FontId::monospace(10.0), Color32::from_rgb(140, 150, 165));

            // Threshold bands: 70 and 30
            let rsi_to_y = |val: f64| -> f32 {
                let norm = (val as f32) / 100.0;
                rsi_bottom - norm * (rsi_height - 10.0) - 5.0
            };

            let y70 = rsi_to_y(70.0);
            let y30 = rsi_to_y(30.0);
            painter.line_segment([Pos2::new(rect.left() + margin_left, y70), Pos2::new(rect.right() - margin_right, y70)], Stroke::new(0.8, Color32::from_rgb(45, 55, 75)));
            painter.line_segment([Pos2::new(rect.left() + margin_left, y30), Pos2::new(rect.right() - margin_right, y30)], Stroke::new(0.8, Color32::from_rgb(45, 55, 75)));

            if let Some(inds) = mkt.get("indicators") {
                if let Some(arr) = inds["rsi_14"].as_array() {
                    let mut prev = None;
                    for (idx, v) in arr.iter().enumerate() {
                        if let Some(val) = v.as_f64() {
                            let curr = Pos2::new(to_x(idx), rsi_to_y(val));
                            if let Some(p) = prev {
                                painter.line_segment([p, curr], Stroke::new(1.5, purple));
                            }
                            prev = Some(curr);
                        }
                    }
                }
            }
        }

        // Interactive Crosshair & Floating HUD
        if let Some(mouse_pos) = response.hover_pos() {
            if mouse_pos.x >= rect.left() + margin_left && mouse_pos.x <= rect.right() - margin_right {
                let hover_x = mouse_pos.x;
                let rel_x = hover_x - (rect.left() + margin_left);
                let bar_idx = ((rel_x / step_x).floor() as usize).min(n.saturating_sub(1));

                // Vertical Crosshair line
                let cross_x = to_x(bar_idx);
                painter.line_segment([Pos2::new(cross_x, rect.top()), Pos2::new(cross_x, rect.bottom())], Stroke::new(1.0, Color32::from_rgb(80, 95, 120)));

                // Horizontal Crosshair line in main chart
                if mouse_pos.y <= main_bottom {
                    painter.line_segment([Pos2::new(rect.left() + margin_left, mouse_pos.y), Pos2::new(rect.right() - margin_right, mouse_pos.y)], Stroke::new(1.0, Color32::from_rgb(80, 95, 120)));
                }

                // Floating HUD Tooltip Card
                if let Some(c) = candles.get(bar_idx) {
                    let o = c["open"].as_f64().unwrap_or(0.0);
                    let h = c["high"].as_f64().unwrap_or(0.0);
                    let l = c["low"].as_f64().unwrap_or(0.0);
                    let cl = c["close"].as_f64().unwrap_or(0.0);
                    let v = c["volume"].as_f64().unwrap_or(0.0);

                    let hud_rect = Rect::from_min_size(Pos2::new(rect.left() + margin_left + 8.0, main_top + 8.0), Vec2::new(260.0, 95.0));
                    painter.rect_filled(hud_rect, 4.0, Color32::from_rgba_unmultiplied(12, 16, 24, 230));
                    painter.rect_stroke(hud_rect, 4.0, Stroke::new(1.0, Color32::from_rgb(45, 55, 75)), egui::StrokeKind::Inside);

                    let hud_txt = format!(
                        "BAR #{bar_idx} | O: {o:.4} H: {h:.4}\nL: {l:.4} C: {cl:.4}\nVOL: {v:.1}"
                    );
                    painter.text(Pos2::new(hud_rect.left() + 8.0, hud_rect.top() + 8.0), egui::Align2::LEFT_TOP, hud_txt, egui::FontId::monospace(11.0), Color32::WHITE);
                }
            }
        }
    }

    fn render_multi_comparison(&mut self, ui: &mut egui::Ui) {
        let amber = Color32::from_rgb(255, 179, 0);
        let colors = [
            Color32::from_rgb(0, 229, 255),   // Cyan
            Color32::from_rgb(255, 214, 0),   // Gold
            Color32::from_rgb(0, 230, 118),   // Jade
            Color32::from_rgb(255, 23, 68),    // Crimson
            Color32::from_rgb(179, 136, 255), // Purple
            Color32::from_rgb(255, 128, 0),   // Orange
        ];

        ui.heading(egui::RichText::new("📈 MULTI-ASSET NORMALIZED COMPARISON OVERLAYS").color(amber));
        ui.label("Direct correlation and relative percentage return comparison against global benchmarks (Baseline = 0.0%):");

        // Sub-toolbar: Date Ranges & Active Ticker Tags
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("DATE RANGE:").color(amber).monospace());
            if ui.selectable_label(self.date_preset == DateRangePreset::Day1, "1D").clicked() {
                self.set_date_preset(DateRangePreset::Day1);
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::Week1, "1W").clicked() {
                self.set_date_preset(DateRangePreset::Week1);
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::Month1, "1M").clicked() {
                self.set_date_preset(DateRangePreset::Month1);
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::Month3, "3M").clicked() {
                self.set_date_preset(DateRangePreset::Month3);
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::YearToDate, "YTD").clicked() {
                self.set_date_preset(DateRangePreset::YearToDate);
            }
            if ui.selectable_label(self.date_preset == DateRangePreset::Year1, "1Y").clicked() {
                self.set_date_preset(DateRangePreset::Year1);
            }

            ui.separator();
            ui.label("COMPARE ASSETS:");
            for (idx, sym) in self.comparison_symbols.iter().enumerate() {
                let c = colors[idx % colors.len()];
                ui.colored_label(c, format!("● {sym}"));
            }
        });
        ui.separator();

        let available_size = ui.available_size();
        let chart_height = (available_size.y - 40.0).max(380.0);
        let (response, painter) = ui.allocate_painter(Vec2::new(available_size.x, chart_height), egui::Sense::hover());
        let rect = response.rect;

        painter.rect_filled(rect, 4.0, Color32::from_rgb(6, 8, 12));
        painter.rect_stroke(rect, 4.0, Stroke::new(1.0, Color32::from_rgb(33, 41, 54)), egui::StrokeKind::Inside);

        let Some(ref comp) = self.comparison_data else {
            painter.text(rect.center(), egui::Align2::CENTER_CENTER, "Loading multi-symbol comparison feeds...", egui::FontId::proportional(16.0), Color32::WHITE);
            return;
        };

        let Some(series_list) = comp["series"].as_array() else {
            return;
        };

        if series_list.is_empty() {
            return;
        }

        let margin_left = 60.0;
        let margin_right = 70.0;
        let top_padding = 24.0;
        let bottom_padding = 24.0;

        let total_w = rect.width() - margin_left - margin_right;
        let main_h = rect.height() - top_padding - bottom_padding;
        let main_top = rect.top() + top_padding;
        let main_bottom = main_top + main_h;

        // Auto-scale normalized percentage returns (e.g. -15% to +25%)
        let mut min_pct = 0.0_f64;
        let mut max_pct = 0.0_f64;

        for s in series_list {
            if let Some(ret_arr) = s["normalized_pct_series"].as_array() {
                for r in ret_arr {
                    if let Some(val) = r.as_f64() {
                        if val > max_pct { max_pct = val; }
                        if val < min_pct { min_pct = val; }
                    }
                }
            }
        }

        let pct_range = (max_pct - min_pct).max(2.0);
        let scaled_min = min_pct - pct_range * 0.1;
        let scaled_max = max_pct + pct_range * 0.1;
        let scaled_range = scaled_max - scaled_min;

        let to_y = |val: f64| -> f32 {
            let norm = ((val - scaled_min) / scaled_range) as f32;
            main_bottom - norm * main_h
        };

        // Baseline (0.0% return dashed line)
        let y_zero = to_y(0.0);
        painter.line_segment(
            [Pos2::new(rect.left() + margin_left, y_zero), Pos2::new(rect.right() - margin_right, y_zero)],
            Stroke::new(1.2, Color32::from_rgb(100, 115, 140)),
        );
        painter.text(
            Pos2::new(rect.right() - margin_right + 6.0, y_zero),
            egui::Align2::LEFT_CENTER,
            "0.0% (BASE)",
            egui::FontId::monospace(10.0),
            Color32::from_rgb(180, 190, 210),
        );

        // Horizontal Grid Lines & Percent Labels
        for i in 0..=4 {
            let f = i as f64 / 4.0;
            let val = scaled_min + f * scaled_range;
            let y = to_y(val);

            painter.line_segment(
                [Pos2::new(rect.left() + margin_left, y), Pos2::new(rect.right() - margin_right, y)],
                Stroke::new(0.8, Color32::from_rgb(20, 26, 36)),
            );

            painter.text(
                Pos2::new(rect.right() - margin_right + 6.0, y),
                egui::Align2::LEFT_CENTER,
                format!("{val:+.1}%"),
                egui::FontId::monospace(10.0),
                Color32::from_rgb(140, 150, 165),
            );
        }

        // Draw each stock's normalized percentage line
        for (idx, s) in series_list.iter().enumerate() {
            let sym = s["symbol"].as_str().unwrap_or("");
            let stroke_color = colors[idx % colors.len()];

            if let Some(ret_arr) = s["normalized_pct_series"].as_array() {
                let n = ret_arr.len();
                if n < 2 { continue; }
                let step_x = total_w / ((n - 1) as f32);

                let mut prev = None;
                for (b_idx, r) in ret_arr.iter().enumerate() {
                    if let Some(val) = r.as_f64() {
                        let curr = Pos2::new(rect.left() + margin_left + (b_idx as f32) * step_x, to_y(val));
                        if let Some(p) = prev {
                            painter.line_segment([p, curr], Stroke::new(2.0, stroke_color));
                        }
                        prev = Some(curr);
                    }
                }

                // Tag current final return at line end
                if let Some(last_pos) = prev {
                    let last_val = ret_arr.last().and_then(|v| v.as_f64()).unwrap_or(0.0);
                    painter.text(
                        Pos2::new(last_pos.x + 4.0, last_pos.y),
                        egui::Align2::LEFT_CENTER,
                        format!("{sym} ({last_val:+.1}%)"),
                        egui::FontId::monospace(11.0),
                        stroke_color,
                    );
                }
            }
        }
    }

    fn render_crypto_derivatives(&mut self, ui: &mut egui::Ui) {
        let amber = Color32::from_rgb(255, 179, 0);
        let green = Color32::from_rgb(0, 230, 118);
        let red = Color32::from_rgb(255, 23, 68);
        let cyan = Color32::from_rgb(0, 229, 255);

        ui.heading(egui::RichText::new("🔥 CRYPTO PERPETUAL FUTURES & LIQUIDATION HEATMAP").color(amber));
        ui.label("Institutional order flow, forward-looking liquidation cascades, and Hyperliquid funding arbitrage:");

        ui.add_space(8.0);
        if let Some(ref d_json) = self.derivatives_data {
            if let Some(d) = d_json.get("derivatives") {
                let mark = d["mark_price"].as_f64().unwrap_or(0.0);
                let arb_apr = d["arbitrage_spread_apr_pct"].as_f64().unwrap_or(0.0);
                let arb_long = d["best_arb_long_venue"].as_str().unwrap_or("");
                let arb_short = d["best_arb_short_venue"].as_str().unwrap_or("");

                // 1. Funding Arbitrage Banner
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("⚡ DELTA-NEUTRAL FUNDING ARBITRAGE SPREAD:").strong().color(amber));
                        ui.colored_label(green, format!("+{arb_apr:.2}% APR Yield"));
                        ui.separator();
                        ui.label(format!("Strategy: Long {arb_long} / Short {arb_short}"));
                    });
                });

                ui.add_space(8.0);
                ui.columns(2, |columns| {
                    // Left Column: Multi-Exchange Funding Rates
                    columns[0].strong(egui::RichText::new("PERP VENUE FUNDING RATES (8H / ANNUALIZED)").color(amber));
                    columns[0].add_space(4.0);
                    if let Some(rates) = d["funding_rates"].as_array() {
                        egui::Grid::new("funding_rates_grid").striped(true).show(&mut columns[0], |ui| {
                            ui.strong("Venue");
                            ui.strong("8h Rate");
                            ui.strong("Annual APR");
                            ui.strong("Open Interest");
                            ui.end_row();

                            for r in rates {
                                let v = r["venue"].as_str().unwrap_or("");
                                let r8 = r["rate_8h_pct"].as_f64().unwrap_or(0.0);
                                let apr = r["annualized_apr_pct"].as_f64().unwrap_or(0.0);
                                let oi = r["open_interest_mil"].as_f64().unwrap_or(0.0);

                                ui.label(v);
                                ui.colored_label(if r8 >= 0.02 { green } else { cyan }, format!("{r8:.4}%"));
                                ui.colored_label(if apr >= 20.0 { green } else { Color32::WHITE }, format!("{apr:.2}%"));
                                ui.label(format!("${oi:.1}M"));
                                ui.end_row();
                            }
                        });
                    }

                    // Right Column: Liquidation Heatmap Clusters
                    columns[1].strong(egui::RichText::new("FORWARD-LOOKING LIQUIDATION HEATMAP").color(amber));
                    columns[1].add_space(4.0);
                    if let Some(bands) = d["liquidation_heatmap"].as_array() {
                        egui::Grid::new("liq_heatmap_grid").striped(true).show(&mut columns[1], |ui| {
                            ui.strong("Price Level");
                            ui.strong("Cluster Type");
                            ui.strong("Cascade Notional");
                            ui.strong("Distance");
                            ui.end_row();

                            for b in bands {
                                let p = b["price_level"].as_f64().unwrap_or(0.0);
                                let side = b["side"].as_str().unwrap_or("");
                                let vol = b["volume_notional_mil"].as_f64().unwrap_or(0.0);
                                let dist = b["distance_pct"].as_f64().unwrap_or(0.0);

                                let (label, col) = if side == "SHORT_LIQ" {
                                    ("🔥 SHORT SQUEEZE", red)
                                } else {
                                    ("🩸 LONG FLUSH", green)
                                };

                                ui.label(format!("${p:.2}"));
                                ui.colored_label(col, label);
                                ui.label(format!("${vol:.1}M"));
                                ui.colored_label(if dist >= 0.0 { green } else { red }, format!("{dist:+.1}%"));
                                ui.end_row();
                            }
                        });
                    }
                });

                ui.add_space(12.0);
                ui.label(egui::RichText::new(format!("CURRENT MARK PRICE: ${mark:.2}")).size(14.0).strong().color(amber));
            }
        }
    }

    fn render_fantasy_trading(&mut self, ui: &mut egui::Ui) {
        let amber = Color32::from_rgb(255, 179, 0);
        let green = Color32::from_rgb(0, 230, 118);
        let red = Color32::from_rgb(255, 23, 68);
        let cyan = Color32::from_rgb(0, 229, 255);

        ui.heading(egui::RichText::new("🎮 FANTASY PAPER TRADING ARENA (SPOT & 1x-50x LEVERAGED PERPS)").color(amber));
        ui.label("Risk-free sovereign paper trading sandbox with realistic margin calls and simulated execution:");

        ui.add_space(8.0);

        let mut do_reset = false;
        let mut do_open = false;
        let mut close_id = None;

        let pa_json = self.paper_account.clone();
        if let Some(ref pa) = pa_json {
            if let Some(acc) = pa.get("account") {
                let equity = acc["total_equity"].as_f64().unwrap_or(0.0);
                let avail = acc["available_balance"].as_f64().unwrap_or(0.0);
                let margin = acc["margin_used"].as_f64().unwrap_or(0.0);
                let un_pnl = acc["unrealized_pnl"].as_f64().unwrap_or(0.0);
                let wins = acc["win_count"].as_u64().unwrap_or(0);
                let losses = acc["loss_count"].as_u64().unwrap_or(0);

                // Account Telemetry Header
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(format!("EQUITY: ${equity:.2}")).size(15.0).strong());
                        ui.colored_label(if un_pnl >= 0.0 { green } else { red }, format!("(PnL: {un_pnl:+.2})"));
                        ui.separator();
                        ui.label(format!("AVAILABLE: ${avail:.2}"));
                        ui.label(format!("MARGIN USED: ${margin:.2}"));
                        ui.separator();
                        ui.colored_label(green, format!("WINS: {wins}"));
                        ui.colored_label(red, format!("LOSSES: {losses}"));

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("🔄 Reset Account ($100k)").clicked() {
                                do_reset = true;
                            }
                        });
                    });
                });

                ui.add_space(10.0);
                // Order Placement Ticket
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.strong(egui::RichText::new("NEW ORDER ENTRY TICKET").color(amber));
                    ui.horizontal(|ui| {
                        ui.label("Target Asset:");
                        ui.colored_label(cyan, &self.selected_symbol);
                        ui.separator();

                        ui.label("Side:");
                        if ui.selectable_label(self.paper_order_side == "LONG", egui::RichText::new("🟢 LONG / BUY").color(green)).clicked() {
                            self.paper_order_side = "LONG".to_string();
                        }
                        if ui.selectable_label(self.paper_order_side == "SHORT", egui::RichText::new("🔴 SHORT / SELL").color(red)).clicked() {
                            self.paper_order_side = "SHORT".to_string();
                        }

                        ui.separator();
                        ui.label("Type:");
                        if ui.selectable_label(self.paper_order_type == "PERP_FUTURES", "Perp Futures").clicked() {
                            self.paper_order_type = "PERP_FUTURES".to_string();
                        }
                        if ui.selectable_label(self.paper_order_type == "SPOT", "Spot").clicked() {
                            self.paper_order_type = "SPOT".to_string();
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Order Size:");
                        ui.add(egui::DragValue::new(&mut self.paper_quantity).range(0.01..=1_000_000.0).speed(0.1));

                        if self.paper_order_type == "PERP_FUTURES" {
                            ui.separator();
                            ui.label("Leverage Slider:");
                            ui.add(egui::Slider::new(&mut self.paper_leverage, 1.0..=50.0).text("x Leverage"));
                        }

                        ui.separator();
                        if ui.button(egui::RichText::new(format!("⚡ EXECUTE FANTASY {} ORDER", self.paper_order_side)).color(amber).strong()).clicked() {
                            do_open = true;
                        }
                    });
                });

                ui.add_space(10.0);
                // Active Positions Table
                ui.strong(egui::RichText::new("ACTIVE FANTASY POSITIONS").color(amber));
                if let Some(positions) = acc["positions"].as_array() {
                    egui::Grid::new("paper_positions_grid").striped(true).show(ui, |ui| {
                        ui.strong("ID");
                        ui.strong("Symbol");
                        ui.strong("Side");
                        ui.strong("Leverage");
                        ui.strong("Size");
                        ui.strong("Entry");
                        ui.strong("Mark");
                        ui.strong("Liq Price");
                        ui.strong("Margin");
                        ui.strong("PnL (USD / %)");
                        ui.strong("Action");
                        ui.end_row();

                        for pos in positions {
                            let pid = pos["id"].as_str().unwrap_or("");
                            let sym = pos["symbol"].as_str().unwrap_or("");
                            let side = pos["side"].as_str().unwrap_or("");
                            let lev = pos["leverage"].as_f64().unwrap_or(1.0);
                            let qty = pos["quantity"].as_f64().unwrap_or(0.0);
                            let entry = pos["entry_price"].as_f64().unwrap_or(0.0);
                            let cur = pos["current_price"].as_f64().unwrap_or(0.0);
                            let liq = pos["liquidation_price"].as_f64().unwrap_or(0.0);
                            let marg = pos["margin_allocated"].as_f64().unwrap_or(0.0);
                            let pnl = pos["unrealized_pnl"].as_f64().unwrap_or(0.0);
                            let pnl_pct = pos["unrealized_pnl_pct"].as_f64().unwrap_or(0.0);

                            ui.label(pid);
                            ui.label(sym);
                            ui.colored_label(if side == "LONG" { green } else { red }, side);
                            ui.label(format!("{lev:.0}x"));
                            ui.label(format!("{qty:.2}"));
                            ui.label(format!("{entry:.2}"));
                            ui.label(format!("{cur:.2}"));
                            ui.colored_label(red, format!("{liq:.2}"));
                            ui.label(format!("${marg:.1}"));
                            ui.colored_label(if pnl >= 0.0 { green } else { red }, format!("{pnl:+.2} ({pnl_pct:+.1}%)"));

                            if ui.small_button("❌ Close").clicked() {
                                close_id = Some(pid.to_string());
                            }
                            ui.end_row();
                        }
                    });
                }
            }
        }

        if do_reset {
            self.reset_paper_account();
        }
        if do_open {
            self.open_paper_order();
        }
        if let Some(id) = close_id {
            self.close_paper_position(&id);
        }
    }

    fn render_ai_workflows(&mut self, ui: &mut egui::Ui) {
        let amber = Color32::from_rgb(255, 179, 0);
        let green = Color32::from_rgb(0, 230, 118);
        let cyan = Color32::from_rgb(0, 229, 255);

        ui.heading(egui::RichText::new("🧠 AGENTIC AI QUANT WORKFLOWS (BLOOMBERG ASKB // BQUANT)").color(amber));
        ui.label("Execute institutional generative workflows grounded in sovereign real-time market data:");

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.selectable_label(self.ai_mode == AiWorkflowMode::StrategyGen, "1. Code Strategy Gen (BQuant)").clicked() {
                self.ai_mode = AiWorkflowMode::StrategyGen;
                self.ai_prompt = "Build a mean-reverting statistical arbitrage model on KOSPI vs DJIA".to_string();
            }
            if ui.selectable_label(self.ai_mode == AiWorkflowMode::MacroStressTest, "2. Macro Stress-Test (PORT)").clicked() {
                self.ai_mode = AiWorkflowMode::MacroStressTest;
                self.ai_prompt = "Simulate 50bps ECB rate cut coupled with hawkish BOJ 25bps hike".to_string();
            }
            if ui.selectable_label(self.ai_mode == AiWorkflowMode::SentimentAlpha, "3. NLP Sentiment Alpha").clicked() {
                self.ai_mode = AiWorkflowMode::SentimentAlpha;
                self.ai_prompt = "Analyze breaking central bank speeches and quantify hawkish semantic drift".to_string();
            }
            if ui.selectable_label(self.ai_mode == AiWorkflowMode::ExecutionRouter, "4. Smart Order Routing (SOR)").clicked() {
                self.ai_mode = AiWorkflowMode::ExecutionRouter;
                self.ai_prompt = "Slice 5,000,000 EUR block order across dark pools and ECNs to minimize slippage".to_string();
            }
        });

        ui.add_space(10.0);
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Target Symbol:").strong());
                ui.colored_label(cyan, &self.selected_symbol);
            });
            ui.add_space(4.0);
            ui.label(egui::RichText::new("Agent Prompt / Scenario Input:").monospace().color(amber));
            ui.add(egui::TextEdit::multiline(&mut self.ai_prompt).desired_rows(3).desired_width(f32::INFINITY));

            ui.add_space(6.0);
            if ui.button(egui::RichText::new("⚡ Execute AI Quant Workflow").color(amber).strong()).clicked() {
                self.run_ai_workflow();
            }
        });

        ui.add_space(12.0);
        if let Some(ref res) = self.ai_result {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.strong(egui::RichText::new("WORKFLOW TELEMETRY & RESULTS").color(green));
                ui.separator();
                let pretty_json = serde_json::to_string_pretty(res).unwrap_or_default();
                ui.add(
                    egui::TextEdit::multiline(&mut pretty_json.as_str())
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(15)
                        .desired_width(f32::INFINITY),
                );
            });
        }
    }

    fn render_honeypot_auditor(&mut self, ui: &mut egui::Ui) {
        let amber = Color32::from_rgb(255, 179, 0);
        let green = Color32::from_rgb(0, 230, 118);
        let red = Color32::from_rgb(255, 23, 68);

        ui.heading(egui::RichText::new("🛡️ EVM SMART CONTRACT HONEYPOT AUDITOR").color(amber));
        ui.label("Detect hidden transfer taxes, blacklist privileges, and malicious proxy traps:");

        ui.add_space(8.0);
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Contract Address:");
                ui.text_edit_singleline(&mut self.audit_address);
            });
            ui.horizontal(|ui| {
                ui.label("Blockchain:");
                ui.text_edit_singleline(&mut self.audit_chain);
            });
            ui.add_space(6.0);
            if ui.button(egui::RichText::new("🔍 Audit Contract In Sandbox").color(amber).strong()).clicked() {
                self.run_audit();
            }
        });

        if let Some(ref rep) = self.audit_report {
            if let Some(report) = rep.get("report") {
                ui.add_space(12.0);
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    let is_hp = report["is_honeypot"].as_bool().unwrap_or(false);
                    let score = report["risk_score_0_to_100"].as_u64().unwrap_or(0);
                    let verdict = report["audit_verdict"].as_str().unwrap_or("");

                    let (status_text, score_color) = if is_hp {
                        ("🚨 HONEYPOT / HIGH-RISK SCAM DETECTED", red)
                    } else {
                        ("✅ VERIFIED CLEAN / SAFE CONTRACT", green)
                    };

                    ui.colored_label(score_color, egui::RichText::new(status_text).strong().size(15.0));
                    ui.label(format!("Risk Score: {score}/100"));
                    ui.label(format!("Verdict: {verdict}"));
                    ui.separator();

                    ui.label(format!("Buy Tax: {}%", report["buy_tax_pct"]));
                    ui.label(format!("Sell Tax: {}%", report["sell_tax_pct"]));
                    ui.label(format!("Can Mint Tokens: {}", report["can_mint_tokens"]));
                    ui.label(format!("Can Blacklist Wallets: {}", report["can_blacklist_wallets"]));
                    ui.label(format!("Liquidity Locked: {}", report["liquidity_locked"]));
                });
            }
        }
    }

    fn render_backtester(&mut self, ui: &mut egui::Ui) {
        let amber = Color32::from_rgb(255, 179, 0);
        let green = Color32::from_rgb(0, 230, 118);

        ui.heading(egui::RichText::new("⚙️ HIGH-SPEED MONTE CARLO BACKTESTER").color(amber));
        ui.label("Simulate quantitative strategies over multi-asset historical tick distributions:");

        ui.add_space(8.0);
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Strategy ID:");
                ui.text_edit_singleline(&mut self.strategy_name);
            });
            ui.horizontal(|ui| {
                ui.label("Initial Capital ($):");
                ui.add(egui::DragValue::new(&mut self.initial_capital).range(1000.0..=10_000_000.0).speed(5000.0));
            });
            ui.add_space(6.0);
            if ui.button(egui::RichText::new("▶ Run 10,000-Path Simulation").color(amber).strong()).clicked() {
                self.run_backtest();
            }
        });

        if let Some(ref res) = self.backtest_result {
            ui.add_space(12.0);
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.strong(egui::RichText::new("SIMULATION SUMMARY").color(green));
                ui.separator();
                ui.label(format!("Strategy: {}", res["strategy"].as_str().unwrap_or("")));
                ui.label(format!("Final Capital: ${:.2}", res["final_equity"].as_f64().unwrap_or(0.0)));
                ui.label(format!("Net Profit: ${:.2} (+{:.1}%)", res["net_profit"].as_f64().unwrap_or(0.0), res["roi_percentage"].as_f64().unwrap_or(0.0)));
                ui.label(format!("Sharpe Ratio: {:.2}", res["sharpe_ratio"].as_f64().unwrap_or(0.0)));
                ui.label(format!("Max Drawdown: {:.1}%", res["max_drawdown_pct"].as_f64().unwrap_or(0.0)));
                ui.label(format!("Win Rate: {:.1}%", res["win_rate"].as_f64().unwrap_or(0.0) * 100.0));
                ui.label(format!("Total Simulated Trades: {}", res["trades_count"].as_u64().unwrap_or(0)));
            });
        }
    }

    fn render_portfolio_ledger(&mut self, ui: &mut egui::Ui) {
        let amber = Color32::from_rgb(255, 179, 0);
        let green = Color32::from_rgb(0, 230, 118);

        ui.heading(egui::RichText::new("💼 SOVEREIGN PORTFOLIO LEDGER").color(amber));
        ui.label("Zero-leakage local balance sheet with real-time unrealized PnL & tax estimates:");

        ui.add_space(8.0);
        if let Some(ref p) = self.portfolio_data {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                let total = p["total_equity_usd"].as_f64().unwrap_or(0.0);
                let un_pnl = p["unrealized_pnl_usd"].as_f64().unwrap_or(0.0);
                let tax = p["estimated_tax_liability"].as_f64().unwrap_or(0.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("TOTAL EQUITY: ${total:.2}")).size(16.0).strong());
                    ui.colored_label(green, format!("(UNREALIZED PnL: +${un_pnl:.2})"));
                    ui.label(egui::RichText::new(format!("EST. TAX: ${tax:.2}")).weak().monospace());
                });
            });

            ui.add_space(8.0);
            if let Some(positions) = p["positions"].as_array() {
                egui::Grid::new("positions_grid").striped(true).min_col_width(90.0).show(ui, |ui| {
                    ui.strong("Symbol");
                    ui.strong("Class");
                    ui.strong("Quantity");
                    ui.strong("Entry");
                    ui.strong("Current");
                    ui.strong("PnL (USD)");
                    ui.end_row();

                    for pos in positions {
                        let sym = pos["symbol"].as_str().unwrap_or("");
                        let cls = pos["asset_class"].as_str().unwrap_or("");
                        let qty = pos["qty"].as_f64().unwrap_or(0.0);
                        let entry = pos["entry_price"].as_f64().unwrap_or(0.0);
                        let cur = pos["current_price"].as_f64().unwrap_or(0.0);
                        let pnl = pos["pnl_usd"].as_f64().unwrap_or(0.0);

                        ui.label(sym);
                        ui.label(cls);
                        ui.label(format!("{qty:.2}"));
                        ui.label(format!("{entry:.4}"));
                        ui.label(format!("{cur:.4}"));
                        ui.colored_label(if pnl >= 0.0 { green } else { Color32::RED }, format!("{pnl:+.2}"));
                        ui.end_row();
                    }
                });
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let store = DatabaseStore::open_in_memory().expect("Failed to initialize sovereign SQLite store");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 840.0])
            .with_min_inner_size([960.0, 600.0])
            .with_title("MoneyCraft Bloomberg Sovereign Terminal"),
        ..Default::default()
    };

    eframe::run_native(
        "MoneyCraft Bloomberg Sovereign Terminal",
        options,
        Box::new(|_cc| Ok(Box::new(MoneyCraftGui::new(Arc::new(store), Arc::new(rt))))),
    )
}
