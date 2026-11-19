//! MoneyCraft desktop app: the terminal from `moneycraft-ui` in a native window.
//! The browser build is `apps/moneycraft-web`; both run the same `MoneyCraftGui`.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use databasecraft_store::DatabaseStore;
use eframe::egui;
use moneycraft_ui::{Bridge, MoneyCraftGui};
use std::sync::Arc;

fn main() -> Result<(), eframe::Error> {
    let rt = tokio::runtime::Runtime::new().unwrap_or_else(|e| {
        eprintln!("Failed to create Tokio runtime: {e}");
        std::process::exit(1);
    });

    if std::env::args().any(|a| a == "--smoke") {
        let code = MoneyCraftGui::smoke(&rt);
        std::process::exit(code);
    }

    let store = DatabaseStore::open_in_memory().unwrap_or_else(|e| {
        eprintln!("Failed to initialize local SQLite store: {e}");
        std::process::exit(1);
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 840.0])
            .with_min_inner_size([960.0, 600.0])
            .with_title("MoneyCraft Bloomberg Sovereign Terminal"),
        ..Default::default()
    };

    let bridge = Bridge::native(Arc::new(store), Arc::new(rt));
    eframe::run_native(
        "MoneyCraft Bloomberg Sovereign Terminal",
        options,
        Box::new(move |_cc| Ok(Box::new(MoneyCraftGui::new(bridge)))),
    )
}
