# MoneyCraft PRD (docs pointer)

**Canonical spec:** [`../PRD.md`](../PRD.md) (v0.3.0)
**Companions:** [`../LIVE_TRADING_PLAN.md`](../LIVE_TRADING_PLAN.md),
[`../PORTFOLIO_SPEC.md`](../PORTFOLIO_SPEC.md), [`../SPEC.md`](../SPEC.md),
[`../HELP.md`](../HELP.md)
**Control port:** 14117 (`moneycraft --control 14117`)
**Crates:** `moneycraft-core` (L1 quant bus, port 14117) /
`moneycraft-feed` (market feeds, indicators, sandbox EVM) /
`moneycraft-broker` (paper execution) / `moneycraft-mcp` (L2 stdio + CLI) /
`moneycraft-app` (L3 egui terminal)

## Summary (do not drift; edit ../PRD.md instead)

MoneyCraft is the offline-first quantitative terminal: personal wealth and
inflation simulator, global indices desk, normalized multi-asset overlays,
candlestick/mountain/Heikin-Ashi canvas with SMA/EMA/Bollinger/Volume/RSI,
G10 forex, news wire with NLP sentiment, quant agent workflows (BQuant,
PORT stress, SOR), EVM honeypot auditor, 10k-path Monte Carlo backtester,
and a local FIFO/LIFO tax ledger on DatabaseCraft B-Trees.

## Tier mapping
See [`TIERED_PLAN.md`](TIERED_PLAN.md) for full execution breakdown:
- Tier 1: Core quant bus, indicators, sandbox EVM, DatabaseCraft B-Tree storage.
- Tier 2: Headless stdio MCP server (`moneycraft-cli --mcp`) + CLI with paper execution.
- Tier 3: Immediate-mode `egui` terminal (`moneycraft-app`) with continuous charts and news wire.
- Tier 4: Craft Menu Bar Standard (`File`, `Edit`, `View`, `Insert`, `Modify`, `Commands`, `Window`, `Help`) with `switch_on_hover`, 5-dialog Help suite, View toggles, Tofu Guard scans, and headless `--smoke` parity.

## Safety (binding)

MoneyCraft cannot move real money. `PaperOpen`/`PaperClose` simulate fills;
`PlaceOrder` refuses instead of faking a fill; every live-data pipe is
read-only (Yahoo 15-min delayed, public klines, ECB daily fixing). Going
live is a defined small job per `LIVE_TRADING_PLAN.md`, never a silent
flag flip. No leverage above paper sandbox, no broker keys in the repo.

## Non-goals

Not financial advice; no live-money trading by default, no exceptions; no
telemetry exfiltration of holdings or journals.

## Gates

`cargo clippy --workspace --all-targets --all-features -- -D warnings`,
`cargo test --workspace` (fixture series with known numbers, no lookahead),
`cargo fmt --all --check`, byte-exact MCP stdio probe, `Cargo.lock`
committed, no `target/` / `*.bak` / `*.exe` in status.
