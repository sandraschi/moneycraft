//! Local-first Ctrl+J AI Chat Dock for MoneyCraft.
//!
//! Operates 100% offline with zero external LLM calls: inspects live terminal state,
//! searches embedded `HELP.md` and `skills/*/SKILL.md`, dispatches read-only
//! `moneycraft_core::Command` queries via `Bridge`, and generates CLI/MCP snippets.
#![forbid(unsafe_code)]

use crate::bridge::{Bridge, Reply};
use eframe::egui::{self, Color32, Frame, Margin, RichText, ScrollArea, Stroke, Ui};
use moneycraft_core::skills::{skill_read, HELP_MD, SKILLS};
use moneycraft_core::Command;

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub role: &'static str,
    pub content: String,
    pub snippet: Option<String>,
}

pub struct ChatContext<'a> {
    pub workspace: &'a str,
    pub symbol: &'a str,
    pub data_source: &'a str,
    pub locale: &'a str,
}

pub struct ChatDockState {
    pub open: bool,
    pub maximized: bool,
    pub input: String,
    pub messages: Vec<ChatMessage>,
}

impl Default for ChatDockState {
    fn default() -> Self {
        Self {
            open: false,
            maximized: false,
            input: String::new(),
            messages: vec![ChatMessage {
                role: "assistant",
                content: "MoneyCraft Local Quant Assistant (100% offline, 0 external AI calls).\nAsk about terminal status, ticker universe, data sources, FIFO ledger, KESt tax, or bundled skills. Try:\n• \"status\"\n• \"universe\"\n• \"sources\"\n• \"ledger\"\n• \"skills\"\n• \"pitfalls\"".into(),
                snippet: None,
            }],
        }
    }
}

impl ChatDockState {
    pub fn push_assistant_reply(&mut self, content: String, snippet: Option<String>) {
        self.messages.push(ChatMessage {
            role: "assistant",
            content,
            snippet,
        });
    }

    pub fn ui(&mut self, ui: &mut Ui, bridge: &Bridge, ctx_info: &ChatContext<'_>) {
        let is_dark = ui.visuals().dark_mode;
        let accent = if is_dark {
            Color32::from_rgb(255, 179, 0)
        } else {
            Color32::from_rgb(155, 88, 0)
        };
        let muted = ui.visuals().weak_text_color();

        ui.horizontal(|ui| {
            ui.heading(RichText::new("MoneyCraft AI Chat (Ctrl+J)").color(accent));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .small_button("X")
                    .on_hover_text("Close Chat Dock")
                    .clicked()
                {
                    self.open = false;
                }
                let max_label = if self.maximized { "Restore" } else { "Expand" };
                if ui.small_button(max_label).clicked() {
                    self.maximized = !self.maximized;
                }
                if ui.small_button("Clear").clicked() {
                    self.messages.truncate(1);
                }
            });
        });

        ui.label(
            RichText::new(format!(
                "Local-First • {} • {} ({}) • {}",
                ctx_info.workspace, ctx_info.symbol, ctx_info.data_source, ctx_info.locale,
            ))
            .small()
            .color(muted),
        );
        ui.separator();

        // Quick local inspection chips
        ui.horizontal_wrapped(|ui| {
            if ui.small_button("Status").clicked() {
                self.run_local_query("status", bridge, ctx_info);
            }
            if ui.small_button("Universe").clicked() {
                self.run_local_query("universe", bridge, ctx_info);
            }
            if ui.small_button("Sources").clicked() {
                self.run_local_query("sources", bridge, ctx_info);
            }
            if ui.small_button("Ledger").clicked() {
                self.run_local_query("ledger", bridge, ctx_info);
            }
            if ui.small_button("Bundled Skills").clicked() {
                self.run_local_query("skills", bridge, ctx_info);
            }
        });
        ui.separator();

        // Message history
        let avail_h = (ui.available_height() - 78.0).max(120.0);
        ScrollArea::vertical()
            .id_salt("moneycraft_chat_scroll")
            .max_height(avail_h)
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for msg in &self.messages {
                    let is_user = msg.role == "user";
                    let (bg, border, title_color) = if is_dark {
                        if is_user {
                            (
                                Color32::from_rgb(30, 42, 58),
                                Color32::from_rgb(56, 72, 92),
                                Color32::from_rgb(0, 229, 255),
                            )
                        } else {
                            (
                                Color32::from_rgb(20, 24, 34),
                                Color32::from_rgb(48, 54, 68),
                                accent,
                            )
                        }
                    } else if is_user {
                        (
                            Color32::from_rgb(235, 244, 255),
                            Color32::from_rgb(180, 205, 235),
                            Color32::from_rgb(0, 110, 140),
                        )
                    } else {
                        (
                            Color32::from_rgb(245, 246, 250),
                            Color32::from_rgb(210, 214, 224),
                            accent,
                        )
                    };
                    let title = if is_user { "You" } else { "MoneyCraft Local" };

                    Frame::new()
                        .fill(bg)
                        .stroke(Stroke::new(1.0, border))
                        .inner_margin(Margin::same(8))
                        .corner_radius(6.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new(title).strong().small().color(title_color));
                            ui.label(&msg.content);
                            if let Some(snippet) = &msg.snippet {
                                ui.add_space(4.0);
                                ui.horizontal(|ui| {
                                    ui.code(snippet);
                                    if ui.small_button("Copy").clicked() {
                                        ui.ctx().copy_text(snippet.clone());
                                    }
                                });
                            }
                        });
                    ui.add_space(6.0);
                }
            });

        ui.separator();
        ui.horizontal(|ui| {
            let resp = ui.add(
                egui::TextEdit::singleline(&mut self.input)
                    .hint_text(
                        "Ask locally (status, universe, sources, ledger, skills, pitfalls...)",
                    )
                    .desired_width((ui.available_width() - 62.0).max(120.0)),
            );
            let enter = resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if (ui.button("Send").clicked() || enter) && !self.input.trim().is_empty() {
                let q = self.input.trim().to_string();
                self.input.clear();
                self.run_local_query(&q, bridge, ctx_info);
                resp.request_focus();
            }
        });
    }

    pub fn run_local_query(&mut self, query: &str, bridge: &Bridge, ctx_info: &ChatContext<'_>) {
        self.messages.push(ChatMessage {
            role: "user",
            content: query.to_string(),
            snippet: None,
        });

        let q = query.to_lowercase();
        if q == "status" || q.contains("engine") || q.contains("health") || q.contains("port") {
            bridge.submit(
                Command::Status,
                Reply::ChatQuery {
                    title: "MoneyCraft Runtime Status",
                    snippet: Some("moneycraft-cli status"),
                },
            );
        } else if q.contains("universe") || q.contains("ticker") || q.contains("symbol") {
            bridge.submit(
                Command::Universe,
                Reply::ChatQuery {
                    title: "Queryable Symbol Universe",
                    snippet: Some("moneycraft-cli universe"),
                },
            );
        } else if q.contains("source")
            || q.contains("feed")
            || q.contains("payola")
            || q.contains("delay")
        {
            bridge.submit(
                Command::ListDataSources,
                Reply::ChatQuery {
                    title: "Market Data Sources & Delay Matrix",
                    snippet: Some("moneycraft-cli sources"),
                },
            );
        } else if q.contains("ledger")
            || q.contains("fifo")
            || q.contains("kest")
            || q.contains("holding")
            || q.contains("portfolio")
        {
            bridge.submit(
                Command::LedgerHoldings { db_path: None },
                Reply::ChatQuery {
                    title: "FIFO Ledger Holdings (moneycraft-ledger.db)",
                    snippet: Some("moneycraft-cli ledger-holdings"),
                },
            );
        } else if q.contains("panic")
            || q.contains("crisis")
            || q.contains("1720")
            || q.contains("historic")
        {
            bridge.submit(
                Command::GetHistoricPanics,
                Reply::ChatQuery {
                    title: "Historic Financial Panics (1720-2022)",
                    snippet: Some("moneycraft-cli panics"),
                },
            );
        } else if q.contains("skill") {
            let list: Vec<String> = SKILLS
                .iter()
                .map(|s| format!("• {} ({}) — {}", s.name, s.uri, s.description))
                .collect();
            let preview = skill_read("moneycraft-workflow")
                .ok()
                .and_then(|v| v.get("uri").and_then(|u| u.as_str()).map(str::to_string))
                .unwrap_or_else(|| "skill://moneycraft-workflow/SKILL.md".into());
            self.push_assistant_reply(
                format!(
                    "Bundled Skills (Help -> Bundled Skills... / MCP resources/list):\n{}\nExample URI: {preview}",
                    list.join("\n")
                ),
                Some("moneycraft_skill_read {\"name\": \"moneycraft-workflow\"}".to_string()),
            );
        } else if q.contains("pitfall")
            || q.contains("safety")
            || q.contains("csv")
            || q.contains("backup")
        {
            self.push_assistant_reply(
                "Key Safety Gates & Pitfalls:\n• Paper-only: finance_place_order always refuses; only the 'paper' broker venue is compiled.\n• Ledger FIFO: SELL exceeding held lots is refused; crypto is refused in the real ledger.\n• CSV Imports: Back up moneycraft-ledger.db before bulk imports; re-importing the same CSV does not deduplicate rows.\n• KESt 27.5%: Estimate only, not tax advice.".into(),
                Some("moneycraft-cli ledger-transactions --limit 20".to_string()),
            );
        } else if q.contains("help") || q.contains("manual") || q.contains("doc") {
            let lines = HELP_MD.lines().count();
            self.push_assistant_reply(
                format!("HELP.md is embedded ({lines} lines). Open Help -> Getting Started... or Help -> Project Files & Docs... to browse all 7 sections."),
                None,
            );
        } else {
            self.push_assistant_reply(
                format!(
                    "Active context: {} | Symbol: {} | Feed: {} | Locale: {}.\nTry asking about: status, universe, sources, ledger, panics, skills, or pitfalls — or open Help -> Project Files & Docs... for the full manual.",
                    ctx_info.workspace, ctx_info.symbol, ctx_info.data_source, ctx_info.locale,
                ),
                None,
            );
        }
    }
}
