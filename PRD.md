# MoneyCraft Product Requirements Document (PRD)

**Title**: MoneyCraft Local-First Financial Intelligence & Algorithmic Terminal  
**Version**: 0.3.0  
**Target Architecture**: Native Rust, egui 0.37, DatabaseCraft SQLite Engine  
**Standard Control Port**: 14117  

---

## 1. Problem Statement
Commercial financial terminals (Bloomberg, Refinitiv) demand prohibitive annual fees (\$24,000–\$32,000/yr), whereas consumer platforms (TradingView, SeekingAlpha) aggressively monetize user telemetry and gate basic indicators behind subscription paywalls. Furthermore, decentralized finance is rife with predatory honeypot tokens and hidden fee traps.

## 2. Product Objectives
- **Zero SaaS Cost**: 100% free, local-first, and self-hosted on local hardware.
- **Zero Privacy Leakage**: All trade journals, portfolio holdings, and tax calculations remain local in DatabaseCraft.
- **Bloomberg-Grade UX**: High-contrast obsidian canvas, amber ticker ribbons, continuous time-series curves, and snapping crosshair tooltips.
- **Cross-Asset Scope**: Unified coverage across G10 Forex (EUR, USD, JPY, GBP, CNY), Crypto, Equities, and Precious Metals.
- **Actionable AI Agents**: Native integration of BQuant strategy synthesis, macro stress testing, sentiment alpha scoring, and smart execution routing.

## 3. Core Functional Requirements
1. **Financial Time-Series Charting**:
   - 60-bar continuous simulation with stochastic volatility and intraday wicks.
   - Dual candlestick / mountain area / Heikin-Ashi visualizations.
   - Technical overlays: SMA 20, EMA 50, Bollinger Bands (2σ), Volume profile, and RSI 14.
2. **Forex Desk**:
   - G10 currency pairs with 2-to-4 decimal pip precision.
3. **Live Breaking News Wire**:
   - Streaming horizontal ticker with semantic category tags and sentiment scoring.
4. **Agentic AI Quantitative Workflows**:
   - BQuant Assistant code generation.
   - PORT Enterprise macro stress-testing.
   - NLP sentiment catalyst scoring.
   - Smart Order Routing (SOR) execution slicing.
5. **Smart Contract Honeypot Auditor**:
   - Bytecode analysis flagging excessive transfer taxes, blacklists, and mint backdoors.
6. **Algorithmic Backtester**:
   - Monte Carlo simulation computing Sharpe, Sortino, and Max Drawdown.
7. **Local Tax & Portfolio Ledger**:
   - Local FIFO/LIFO cost basis accounting.

## 4. UI/UX Architecture & Menu Bar Standard

In accordance with [`CRAFT_MENU_BAR_STANDARD.md`](../mcp-central-docs/standards/rust-artcraft/CRAFT_MENU_BAR_STANDARD.md):
- **Pattern 2 (Viewport-Centric Craft Workstation):** Persistent financial time-series chart viewport and trading terminal with amber ticker ribbons. Top quick button row acts as accelerators into the `View` menu. To add a new asset desk or indicator panel:
  1. Register a visibility toggle in the `View` or `Window` menu.
  2. Add an entry in the sidebar Analytics / Asset toolbox.
  3. Wire the action directly into the universal `Command::dispatch` bus.
- **Top Menu Bar Hierarchy:**
  - `File`: New Portfolio (`Ctrl+N`), Open Ledger (`Ctrl+O`), Open Recent ▸, Save Ledger (`Ctrl+S`), Save As... (`Ctrl+Shift+S`), Export Tax / CSV (`Ctrl+E`), Close Portfolio (`Ctrl+W`), Quit (`Ctrl+Q`).
  - `Edit`: Undo (`Ctrl+Z`), Redo (`Ctrl+Y`), Cut, Copy, Paste, Preferences (`Ctrl+,`).
  - `View`: Chart Canvas, Ticker Ribbon, Order Book, News Wire, Backtest Dashboard, Theme (`Dark`/`Light`).
  - `Insert`: Add Asset Watchlist, Add Transaction, Add Custom Indicator, Add Alert Trigger.
  - `Modify`: Adjust Timeframe, Edit Position, Recalculate Basis (FIFO/LIFO), Tune Overlay Parameters.
  - `Commands`: Run Monte Carlo Backtest, Audit Smart Contract Bytecode, Stress Test Portfolio (PORT), Fetch Feed Updates.
  - `Window`: Reset Layout, Chart Viewport, Portfolio Inspector, Trade Journal, News Terminal.
  - `Help` (Mandatory 5-Dialog Suite): `Command Palette...` (`Ctrl+K`), `Keyboard Shortcuts...` (`F1`), `Project Files & Docs...`, `Licenses & Notices...`, `About MoneyCraft`.
- **Behavioral Mechanics:** Smooth `switch_on_hover` between top-level menus, zero-unsafe immediate-mode rendering, and headless `--smoke` parity.
