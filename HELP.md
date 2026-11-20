# MoneyCraft — User Guide & Reference Manual

**MoneyCraft Local-First Financial Intelligence & Quantitative Terminal**  
**Version**: `0.7.0` • **Control Port**: `14117` • **Safety**: Pure Safe Rust (`0 unsafe` in workspace) • **Trading Posture**: Paper-only (`live_armed: false`)

---

## 1. What MoneyCraft Is & Does

MoneyCraft is an offline-first, pure-Rust quantitative finance workstation, time-series terminal, and personal portfolio ledger built with `egui 0.37` and backed by `DatabaseCraft` SQLite B-Trees. It combines institutional-style market analysis (global indices, G10 forex crosses, multi-asset normalized comparison, 1720–2026 historical panic archives, Monte Carlo backtesting, and EVM honeypot bytecode auditing) with beginner-friendly personal finance tools (inflation erosion simulator for Girokonto/Festgeld/Bausparer and a local FIFO portfolio ledger with Austrian KESt estimation).

**MoneyCraft cannot move real money.** `PlaceOrder` (`finance_place_order`) always refuses; only the `paper` broker venue is compiled (`finance_brokers`), and all live feeds are read-only.

### Get Started (5-Step First-Run Tour)

1. **Launch & Inspect Personal Wealth**: Run `cargo run -p moneycraft-app` (or `moneycraft`). MoneyCraft opens in **Personal Wealth** with pre-populated Euro cashflow defaults (Girokonto, Festgeld, Bausparer, and 3.8% inflation) so you immediately see nominal vs. real purchasing power.
2. **Switch to the Time-Series Terminal**: Click **Terminal** in the top quick-accelerator bar (or `View -> Chart Canvas`). Pick a global index (`DJIA`, `SPX`, `DAX`, `KOSPI`) or any Yahoo symbol, toggle `Candlestick` / `Heikin-Ashi` / `Mountain Area`, and enable `SMA 20`, `EMA 50`, `BBands`, `Vol`, `RSI`, or `Panics`.
3. **Compare Assets on a Zero Baseline**: Open **Multi-Comparison** to overlay normalized `%` returns (`(P_t - P_0) / P_0 * 100`) across disparate scales (e.g., `DJIA` vs `KOSPI` vs `EUR/USD`).
4. **Book Real Holdings or Run Paper Trades**: Use **Portfolio Ledger** (backed by `moneycraft-ledger.db`) for real stocks, ETFs, bonds, and cash with FIFO cost basis, or **Fantasy Trading** for risk-free `1x–50x` leveraged paper simulations.
5. **Open the AI Chat Dock & Help Center**: Press `Ctrl+J` to toggle the offline **AI Chat Dock** for instant local queries, or `Ctrl+K` / `Ctrl+Shift+P` for the **Command Palette**.

---

## 2. Functions & Workspace Architecture

MoneyCraft is structured as a 6-crate Rust workspace plus a WebAssembly build (`apps/moneycraft-web`):

| Crate | Responsibility |
|---|---|
| `moneycraft-core` | Headless `Command::dispatch` bus, personal wealth projection engine, `ledger` FIFO SQLite module, `history` (19 panics & sovereign instruments), `i18n` (7 locales), and embedded `skills` + `HELP.md`. |
| `moneycraft-feed` | Market data connectors (`yahoo`, `binance`, `hyperliquid`, `ecb`, `synthetic`), technical indicators (`SMA`, `EMA`, `Bollinger`, `RSI`), FX cross math, and sandbox EVM bytecode auditor. |
| `moneycraft-broker` | `Broker` trait (`PaperBroker` only by default), risk/leverage limit validation, and `dry_run` order verification. |
| `moneycraft-mcp` | Dual-mode binary (`moneycraft-cli`): human CLI subcommands and MCP `2024-11-05` `stdio` server (28 tools + `resources/list` / `resources/read` + `prompts/list`). |
| `moneycraft-ui` | Shared `egui 0.37` terminal UI, universal 8-menu bar with `switch_on_hover`, `Ctrl+J` AI Chat Dock, and `tofu_guard`. |
| `moneycraft-app` | Native desktop runner (`moneycraft`) with `--smoke` headless verification. |

### Key Capabilities
- **Personal Wealth & Inflation Simulator**: Models monthly net income, living costs, Girokonto (`0%`), Festgeld (`3.0%` 3-year), and Bausparvertrag (`1.5%`) against macro inflation, computing net real yield and the Rule of 72 purchasing-power halving horizon.
- **Time-Series Terminal & 19 Historic Panics (1720–2022)**: 10 global indices, 15 quick stocks + custom Yahoo tickers, G10 forex (`FROM`/`TO` across 20 majors), and deep-archive series (`EIC`, `UK_CONSOLS`, `SOUTH_SEA`) with interactive crisis annotations.
- **Sovereign Portfolio Ledger (`moneycraft-ledger.db`)**: Local SQLite FIFO lot accounting, multi-currency totals, broker/bank CSV import (German/English headers, `;`/`,` delimiters, UTF-8/`cp1252`), and Austrian KESt (`27.5%`) realized-gain estimates.
- **Monte Carlo Backtester & EVM Honeypot Auditor**: 10,000-path strategy simulation (Sharpe, max drawdown, win rate) and local EVM bytecode inspection for transfer-tax traps, blacklists, and unrenounced mint privileges.
- **Fleet-Standard i18n**: Zero-disk-I/O compile-time localization across `EN`, `DE`, `FR`, `ES`, `JA`, `KO`, and `ZH`.

---

## 3. Usage (GUI, CLI & MCP)

### Desktop GUI (`moneycraft-app`)
- **Launch**: `cargo run -p moneycraft-app`
- **Headless Smoke Gate**: `cargo run -p moneycraft-app -- --smoke` (verifies `Command::Status`, control port `14117`, market feed dispatch, and `tofu_guard` with zero window creation).
- **Menu & Keyboard**: `Ctrl+K` / `Ctrl+Shift+P` (Command Palette), `F1` / `Ctrl+/` (Keyboard Shortcuts), `Ctrl+J` (AI Chat Dock), `Ctrl+,` (Preferences), mouse wheel over chart (pure view zoom without refetching).

### CLI (`moneycraft-cli`)
```powershell
# Runtime status & ticker universe
cargo run -p moneycraft-mcp --bin moneycraft-cli -- status
cargo run -p moneycraft-mcp --bin moneycraft-cli -- universe
cargo run -p moneycraft-mcp --bin moneycraft-cli -- ticker EUR/USD --source yahoo

# Multi-asset comparison & personal wealth simulation
cargo run -p moneycraft-mcp --bin moneycraft-cli -- compare "DJIA,SPX,KOSPI" --bars 60
cargo run -p moneycraft-mcp --bin moneycraft-cli -- simulate-wealth --income 2500 --inflation 3.8 --years 10

# Sovereign ledger (real holdings) & broker dry-run
cargo run -p moneycraft-mcp --bin moneycraft-cli -- ledger-add SAP.DE --side BUY --quantity 10 --price 228 --venue flatex
cargo run -p moneycraft-mcp --bin moneycraft-cli -- ledger-holdings
cargo run -p moneycraft-mcp --bin moneycraft-cli -- ledger-valuation --source yahoo
cargo run -p moneycraft-mcp --bin moneycraft-cli -- broker-place --broker paper AAPL --dry-run
```

### MCP `2024-11-05` Server (`moneycraft-cli --mcp`)
Line-delimited JSON-RPC over `stdio` (diagnostics on `stderr` only):
- **Discovery & Market Tools**: `finance_status`, `finance_universe`, `finance_list_data_sources`, `finance_ticker`, `finance_compare`, `finance_derivatives`, `finance_news`, `finance_historic_panics`, `finance_historic_instruments`, `finance_fx_convert`.
- **Analysis & Sandbox Tools**: `finance_simulate_personal_wealth`, `finance_backtest`, `finance_ai_workflow`, `finance_audit_contract`, `finance_brokers`, `finance_broker_place`, `finance_paper_account`, `finance_paper_open`, `finance_paper_close`, `finance_paper_reset`, `finance_place_order`.
- **Ledger Tools**: `finance_ledger_add`, `finance_ledger_holdings`, `finance_ledger_transactions`, `finance_ledger_import`, `finance_ledger_valuation`, `finance_portfolio`.
- **Bundled Skills**: `moneycraft_skill_read` tool plus `resources/list` and `resources/read` (`skill://moneycraft-workflow/SKILL.md`, `skill://moneycraft-ledger/SKILL.md`).

---

## 4. Tips & Tricks

- **Use `finance_universe` First**: Before querying `finance_ticker` or `finance_compare`, call `finance_universe` to see exact supported indices, equities, 20 major currencies, and historic instruments.
- **Dry-Run Broker Orders**: Pass `--dry-run` (CLI) or `"dry_run": true` (`finance_broker_place`) to validate venue limits, leverage caps, and margin math without booking a position.
- **Airplane Mode & Deterministic Testing**: Switch feed source to `synthetic` for zero-latency offline random-walk bars that work without network access.
- **Pure-View Chart Zoom**: Scroll the mouse wheel over the Terminal canvas to zoom into the most recent N bars. It rescales axes and overlays locally with zero network refetch.
- **Fast Local Queries with `Ctrl+J`**: Open the AI Chat Dock (`Ctrl+J`) and click chips (`Status`, `Portfolio`, `Universe`, `Sources`, `Skills`) or type questions to inspect live state and copy CLI/MCP snippets.

---

## 5. Common Pitfalls & Safety

- **Ledger Reconciliation & Insufficient FIFO Lots**: `moneycraft-core::ledger` enforces strict FIFO lot accounting. A `SELL` whose quantity exceeds held lots is **refused** rather than fabricating negative inventory. Always reconcile partial fills and splits before booking sells.
- **Currency & Floating-Point Rounding**: `ledger` stores prices and quantities as SQLite `REAL` (`f64`) and aggregates `totals_by_currency` separately per currency (`EUR`, `USD`, etc.) without silently mixing currencies. Use `finance_fx_convert` only for indicative math — it is not a tradable FX quote.
- **CSV Import Duplicates & Encoding**: `finance_ledger_import` supports `;`/`,` and UTF-8/`cp1252` with German/English headers, and skips malformed or crypto rows cleanly. However, **re-importing the same CSV file does not deduplicate previously booked rows**. Always check `finance_ledger_transactions` first to avoid double-counting.
- **Backup Before Bulk Edits or Imports**: Before running `finance_ledger_import` on a large broker export or modifying historical lots, copy `moneycraft-ledger.db` to `moneycraft-ledger.db.bak`.
- **Crypto Refused in Real Ledger**: `finance_ledger_add` and `finance_ledger_import` refuse crypto symbols (`BTC`, `ETH`, `SOL`, `BTC/USD`, etc.). Keep crypto simulations in the Fantasy Paper Trading desk (`finance_paper_open`).
- **Feed Delay Semantics**: `yahoo` is 15-minute delayed; `ecb` is a daily fixing (~14:15 CET); `synthetic` is simulated; only `binance` and `hyperliquid` public feeds are real-time crypto. Missing quotes return `null` (`no quote`) rather than invented prices.
- **KESt Tax Disclaimer**: The `27.5%` Austrian KESt calculation on realized YTD gains is an **estimate only**, not tax advice.

---

## 6. Planned Extensions

Per `LIVE_TRADING_PLAN.md`, the `moneycraft-broker` trait seam is in place (`PaperBroker` only). Planned extensions include:
- **Feature-Gated Live Broker Adapters**: `broker-oanda` (OANDA v20 FX practice -> live), `broker-alpaca` (US equities), `broker-ibkr` (IBKR Client Portal Gateway for EU/US stocks, FX, and sovereign bonds), and `broker-kraken` / Hyperliquid perps — all disabled in default builds and gated behind explicit `dry_run` + confirmation rails.
- **Enhanced Ledger Deduplication & Reconciliation**: Optional idempotency keys / import-hash tracking for repeated CSV/OFX statements.
- **Interactive Multi-Panel Layout Persistence**: Saving custom dock/panel visibility across sessions.

---

## 7. Other Crafts

| Craft App | Role & Boundary | Handoff / Integration with MoneyCraft |
|---|---|---|
| `databasecraft` | Local SQLite B-Tree storage & query engine | `moneycraft-core` and `moneycraft-app` depend on `databasecraft-store` to persist `moneycraft-ledger.db` locally with zero cloud telemetry. |
| `democraft` | Automated demo & screenshot harness | Runs deterministic CLI/GUI verification recipes against MoneyCraft's `--smoke` and fixture outputs. |
| `codercraft` | Codebase & workspace engineering studio | Develops and audits Rust crates across the fleet; never modifies upstream Storytold reference repos. |
| `calendarcraft` | Local calendar & scheduling workstation | Tracks tax deadlines, Festgeld maturity dates, and recurring savings reminders independently of MoneyCraft's ledger. |
| `taskcraft` | Local task & workflow tracker | Manages personal finance checklists (e.g., quarterly portfolio rebalancing, CSV export backups). |
