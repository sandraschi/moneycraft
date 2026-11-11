//! Tofu glyph guard ensuring user-facing UI labels, menus, and help dialogs
//! do not contain missing characters (U+FFFD) or unprintable control sequences.
#![forbid(unsafe_code)]

/// Check a string for potential tofu characters (replacement character, unprintable control characters, etc.)
pub fn check_string_for_tofu(s: &str) -> Result<(), String> {
    for (idx, ch) in s.char_indices() {
        if ch == '\u{FFFD}' {
            return Err(format!(
                "Unicode replacement character (U+FFFD) found at byte index {idx} in {s:?}"
            ));
        }
        if ch.is_control() && ch != '\n' && ch != '\r' && ch != '\t' {
            return Err(format!(
                "Unprintable ASCII control character (U+{:04X}) found at byte index {idx} in {s:?}",
                ch as u32
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tofu_guard_on_moneycraft_ui_strings() {
        let labels = [
            "MoneyCraft Sovereign Terminal v0.7.0",
            "100% Safe Rust • 0 Sockets • Zero Tofu",
            "BLOOMBERG // MONEYCRAFT SOVEREIGN TERMINAL",
            // Universal 8-Menu Bar
            "File",
            "Edit",
            "View",
            "Insert",
            "Modify",
            "Commands",
            "Window",
            "Help",
            // Menu Items
            "New Portfolio (Ctrl+N)",
            "Open Ledger... (Ctrl+O)",
            "Save Ledger (Ctrl+S)",
            "Save As... (Ctrl+Shift+S)",
            "Export Tax / CSV (Ctrl+E)",
            "Close Portfolio (Ctrl+W)",
            "Quit (Ctrl+Q)",
            "Undo (Ctrl+Z)",
            "Redo (Ctrl+Y)",
            "Cut (Ctrl+X)",
            "Copy (Ctrl+C)",
            "Paste (Ctrl+V)",
            "Select All (Ctrl+A)",
            "Preferences... (Ctrl+,)",
            "Chart Canvas",
            "Ticker Ribbon",
            "Order Book",
            "News Wire",
            "Backtest Dashboard",
            "Add Asset Watchlist...",
            "Add Transaction...",
            "Add Custom Indicator...",
            "Add Alert Trigger...",
            "Adjust Timeframe",
            "Edit Position...",
            "Recalculate Basis (FIFO/LIFO)",
            "Tune Overlay Parameters...",
            "Run Monte Carlo Backtest",
            "Audit Smart Contract Bytecode",
            "Stress Test Portfolio (PORT)",
            "Fetch Feed Updates",
            "Reset Layout",
            "Chart Viewport",
            "Portfolio Inspector",
            "Trade Journal",
            "News Terminal",
            // Mandatory 5-Dialog Suite
            "Command Palette... (Ctrl+K)",
            "Keyboard Shortcuts... (F1)",
            "Project Files & Docs...",
            "Licenses & Notices...",
            "About MoneyCraft",
        ];

        for label in &labels {
            assert!(
                check_string_for_tofu(label).is_ok(),
                "tofu found in label: {label}"
            );
        }
    }

    #[test]
    fn test_tofu_guard_detects_malformed_chars() {
        assert!(check_string_for_tofu("bad \u{FFFD} glyph").is_err());
        assert!(check_string_for_tofu("bad \x07 bell").is_err());
    }
}
