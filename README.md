# MoneyCraft: Local-First Financial Intelligence & Quantitative Terminal

![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)
![Rust 1.90+](https://img.shields.io/badge/rust-1.90%2B-orange?logo=rust)
![MCP stdio](https://img.shields.io/badge/MCP-stdio-purple)
![ArtCraft harness](https://img.shields.io/badge/ArtCraft-harness-green)

**MoneyCraft** is an offline-first, pure-Rust quantitative finance studio, Bloomberg-grade time-series terminal, and local-first algorithmic workbench for the sandraschi fleet.

```
┌────────────────────────────────────────────────────────────────────────┐
│               MONEYCRAFT SOVEREIGN TERMINAL v0.7.0                     │
│                  Control Port: 14117 // Rust egui 0.37                 │
├────────────────────────────────────────────────────────────────────────┤
│ 💶 Personal Wealth & Inflation| Girokonto, Festgeld, Bausparer vs Drag  │
│ 🌐 Global Indices Desk      | DJIA, SPX, NDX, DAX, N225, KOSPI, FTSE…+3 │
│ 📈 Multi-Asset Comparison   | Normalized % Return Overlays (0.0% Base) │
│ ⏱️ Date Range Presets       | 1D, 1W, 1M, 3M, YTD, 1Y Presets          │
│ 📊 Time-Series Terminal     | 🕯️ Candlesticks, Mountain Area, Heikin-Ashi│
│ 📈 Technical Indicators     | SMA 20, EMA 50, Bollinger Bands, Vol, RSI│
│ 💱 Forex Desk     | Any FROM/TO cross of 20 majors (Yahoo 15m)  │
│ 🔥 Crypto Derivatives Desk  | Hyperliquid Funding Arbitrage & Liq Map  │
│ 🎮 Fantasy Paper Trading    | 1x–50x Leverage Futures & Spot Sandbox   │
│ ⚡ Breaking News Wire       | Real-time Macro/FX wire with NLP Sentiment│
│ 🧠 Agentic AI Quant Workflows| Strategy Gen, PORT Stress-Test, Alpha, SOR│
│ 🛡️ EVM Honeypot Auditor    | Local Sandbox bytecode scam analyzer      │
│ ⚙️ Monte Carlo Backtester  | 10,000-path simulation (Sharpe, Drawdown) │
│ 💼 Local Tax Ledger     | Offline FIFO/LIFO private balance sheet   │
└────────────────────────────────────────────────────────────────────────┘
```

## Is this real trading? No. (Read this first)

If you installed this from the repo hoping to trade: **MoneyCraft cannot move real money.** There is no broker connection, no exchange account, no order routing. The trading desk is a **fantasy/paper sandbox** — it simulates fills against delayed or synthetic feeds so you can learn, test ideas, and bankrupt yourself with day trading only *in your imagination* (dark joke, real warning: the habits transfer, the profits don't).

Concretely: `PaperOpen`/`PaperClose` never touch a venue, `PlaceOrder` refuses instead of faking a fill, and every live-data pipe in the building is read-only (Yahoo 15-min delayed, Binance/Hyperledger public klines, ECB daily fixing).

**Going live is a defined, small job, not a rewrite.** The seams already exist: one `Command` enum, one `MarketFeed` abstraction, one set of `Paper*` arms that a broker adapter can replace 1:1. With an hour of Opus or Argon, a forker can wire a real venue behind each asset class and own a genuine little Bloomberg desk. See **[LIVE_TRADING_PLAN.md](LIVE_TRADING_PLAN.md)** for the venue-by-venue scaffold (stocks, forex, sovereign bonds, crypto), the safety rails it demands, and what to wire first.

## Key Modules & Capabilities

1. **Beginner Personal Wealth & Inflation Erosion Simulator**:
   - Compounding engine modeling monthly income (e.g. 2,000 €/mo), living expenses, Girokonto (0% interest), Festgeld (3.0% 3-year term), and Bausparvertrag (1.5% interest + monthly contributions).
   - Real-time diagnostic HUD exposing interest earned vs inflation drag and net real yield.
   - Rule of 72 halving alert showing years until real purchasing power is halved.
   - Year-by-year table comparing nominal bank app balance against real purchasing power with cumulative silent inflation losses.
   - Actionable beginner portfolio playbook (Emergency cash buffer, Geldmarkt-ETF DBX0AN, MSCI World ETF, and physical gold).

2. **Global Market Indices Desk & Date Ranges**:
   - Major world benchmarks: Dow Jones (`DJIA`), S&P 500 (`SPX`), Nasdaq-100 (`NDX`), DAX 40 (`DAX`), Nikkei 225 (`N225`), KOSPI (`KOSPI`), FTSE 100 (`FTSE`), and Hang Seng (`HSI`).
   - One-click date range presets: `1D`, `1W`, `1M`, `3M`, `YTD`, and `1Y`.

3. **Multi-Asset Normalized Return Comparison Graph**:
   - Zero-baseline normalized percentage return curves allowing fair side-by-side comparison across assets of vastly different nominal scales (e.g. DJIA at 42,200 vs KOSPI at 2,580 vs EUR/USD at 1.0845).

4. **Bloomberg-Grade Time-Series Charting**:
   - High-density 60-bar stochastic time-series canvas with true black obsidian background (`#06080c`).
   - Dynamic switching between standard candlesticks, filled mountain area curves, and smoothed Heikin-Ashi.
   - Built-in technical indicators: SMA 20, EMA 50, Bollinger Bands ribbon (2σ), Volume profile, and RSI 14 oscillator.
   - Snapping crosshair HUD projecting time, open, high, low, close, and volume.

2. **G10 Forex Desk**:
   - Complete support for Euro (`EUR`), US Dollar (`USD`), Japanese Yen (`JPY`), British Pound (`GBP`), and Chinese Renminbi (`CNY/RMB`).
   - 4-decimal and 2-decimal pip scaling across chart axes and tooltips.

3. **Breaking News Wire & NLP Sentiment Ticker**:
   - Continuous horizontal news ribbon below main navigation bar.
   - Real-time sentiment tagging: `🟢 BULLISH`, `🔴 BEARISH`, and `🟡 NEUTRAL` with semantic polarity scores.

4. **Agentic AI Quantitative Workflows**:
   - **BQuant Strategy Generator**: Natural language prompt to vectorized Python/Rust strategy code, Sharpe ratio, and drawdown metrics.
   - **PORT Macro Shock Stress-Testing**: Central bank interest rate simulations, factor betas, and portfolio $\text{VaR}_{99}$.
   - **NLP Sentiment Alpha**: Real-time transcript and news catalyst analysis.
   - **Smart Order Routing (SOR)**: Adaptive VWAP block slicing across venues to minimize market impact.

5. **Anti-Scam EVM Honeypot Auditor**:
   - Local sandbox EVM simulation detecting transfer taxes (>5%), blacklist backdoors, and unrenounced mint privileges.

6. **Microsecond Monte Carlo Backtester**:
   - 10,000-path algorithmic testing with win rates, maximum drawdowns, and Sharpe ratios.

7. **Zero-Leakage Local Portfolio Ledger**:
   - Local private SQLite database (backed by **DatabaseCraft** B-Trees).
   - Real-time FIFO/LIFO capital gains tax accounting.

## Architecture

- `crates/moneycraft-feed`: Market feeds, stochastic drift generator, technical indicators, and sandbox EVM.
- `crates/moneycraft-core`: Quantitative command bus (`Command::dispatch`), port `14117`, and AI workflow engines.
- `crates/moneycraft-mcp`: Line-delimited MCP stdio server + CLI (`moneycraft-cli`).
- `crates/moneycraft-app`: Native desktop graphical terminal (`moneycraft`).

## Quick Start

```powershell
# Run desktop terminal
cargo run -p moneycraft-app

# Run CLI ticker
cargo run -p moneycraft-mcp --bin moneycraft-cli -- ticker EUR/USD
cargo run -p moneycraft-mcp --bin moneycraft-cli -- ticker USD/JPY

# Run test suite
cargo test --workspace
```

## Standard Port
Assigned port: **14117** (registered in `mcp-central-docs/operations/WEBAPP_PORTS.md`).

## Status — does today / next week / non-goals

**Does today:** 4-crate Rust workspace (core/feed/mcp/app) + root
PRD/SPEC/HELP; feed ingest + backtest verbs declared; historical
deep-archive to 1720/1850 with panic annotations; 2 test fns.

**Next week:** paper-only safety rule enforced in code + credential
redaction; backtest fixture tests (known series → known numbers, no
lookahead); first democraft recipe on fixtures.

**Non-goals:** not financial advice; no live-money trading — paper-only by
default, no exceptions.

## Lineage — the Storytold craft-app pattern

Storytold ArtCraft ([storytold/artcraft](https://github.com/storytold/artcraft) — "the IDE for artists") was the starting point. The 16 craft apps in the Storytold stable each ship one binary that is both the human CLI and the agent MCP server (stdio), headless-first, with `run.json` + report artifacts. That modus operandi and harness is what this repo implements; the code here is independent, the pattern is shared.
