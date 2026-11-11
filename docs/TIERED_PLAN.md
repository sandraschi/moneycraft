# MoneyCraft — Tiered Execution Plan

**Parent Standard:** [`RUST_ARTCRAFT_STANDARD.md`](../../mcp-central-docs/standards/RUST_ARTCRAFT_STANDARD.md)  
**Menu Bar Standard:** [`CRAFT_MENU_BAR_STANDARD.md`](../../mcp-central-docs/standards/rust-artcraft/CRAFT_MENU_BAR_STANDARD.md)  
**Architecture Model:** 4-Tier ArtCraft Incremental Delivery  
**Status:** Active  

---

## Architecture Flow

```
Tier 1: Core Engine ──> Tier 2: MCP / CLI ──> Tier 3: Desktop UI ──> Tier 4: Menu Bar, Help & Packaging
(moneycraft-core)       (moneycraft-mcp)       (moneycraft-app)       (Hardened Workstation)
```

---

## Tier Specifications

### Tier 1 — Core Quantitative Engine & Bus (`moneycraft-core`, `moneycraft-feed`)
- [x] L1 `Command::dispatch` universal quant bus (port 14117).
- [x] Multi-asset financial time-series simulation and technical overlays (SMA, EMA, Bollinger, RSI).
- [x] G10 Forex desk and streaming news wire analyzer.
- [x] EVM bytecode honeypot auditor.
- [x] Monte Carlo algorithmic backtester.
- [x] Local FIFO/LIFO tax and portfolio ledger via DatabaseCraft.
- [x] `#![forbid(unsafe_code)]` and zero-panic clippy lints.

### Tier 2 — Headless Driver & FastMCP (`moneycraft-mcp`, `moneycraft-broker`)
- [x] Unified CLI `moneycraft-cli` with `--json` output.
- [x] FastMCP stdio server communicating over JSON-RPC 2.0.
- [x] Paper trading execution broker with fill simulation and safety fences.
- [x] Complete tool catalog mirroring `Command::dispatch` operations.
- [x] Headless `--smoke` verification without window initialization.

### Tier 3 — Desktop Workstation UI (`moneycraft-app`)
- [x] Immediate-mode `egui` 0.37 / `wgpu` workstation.
- [x] Financial time-series canvas (candlestick / mountain / Heikin-Ashi) with snapping tooltips.
- [x] Amber ticker ribbons, news wire stream, and multi-asset watchlist.

### Tier 4 — Craft Menu Bar Standard, Help Suite & Hardened Verification
- [ ] Universal 8-menu bar (`File`, `Edit`, `View`, `Insert`, `Modify`, `Commands`, `Window`, `Help`) with `switch_on_hover`.
- [ ] Mandatory 5-dialog Help suite (`Command Palette...`, `Keyboard Shortcuts...`, `Project Files & Docs...`, `Licenses & Notices...`, `About MoneyCraft`).
- [ ] Fleet UI/UX Pattern 2 adherence: viewport-centric navigation, top button row mapped to View toggles, panel visibility managed via `View` / `Window`.
- [ ] Tofu Guard verification (100% font glyph coverage, zero tofu placeholder boxes).
- [ ] Headless `--smoke` parity validation.
- [ ] Standalone release binary packaging and NSIS installer.
