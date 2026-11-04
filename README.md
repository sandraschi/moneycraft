# MoneyCraft: Sovereign Financial Intelligence & Quantitative Terminal

**MoneyCraft** is an offline-first, pure-Rust quantitative finance studio, Bloomberg-grade time-series terminal, and sovereign algorithmic workbench for the sandraschi fleet.

```
┌────────────────────────────────────────────────────────────────────────┐
│               MONEYCRAFT SOVEREIGN TERMINAL v0.3.0                     │
│                  Control Port: 14117 // Rust egui 0.37                 │
├────────────────────────────────────────────────────────────────────────┤
│ 📊 Time-Series Terminal     | 🕯️ Candlesticks, Mountain Area, Heikin-Ashi│
│ 📈 Technical Indicators     | SMA 20, EMA 50, Bollinger Bands, Vol, RSI│
│ 💱 Sovereign Forex Desk     | EUR/USD, USD/JPY, GBP/USD, USD/CNY, EUR/GBP│
│ ⚡ Breaking News Wire       | Real-time Macro/FX wire with NLP Sentiment│
│ 🧠 Agentic AI Quant Workflows| Strategy Gen, PORT Stress-Test, Alpha, SOR│
│ 🛡️ EVM Honeypot Auditor    | Local Sandbox bytecode scam analyzer      │
│ ⚙️ Monte Carlo Backtester  | 10,000-path simulation (Sharpe, Drawdown) │
│ 💼 Sovereign Tax Ledger     | Offline FIFO/LIFO private balance sheet   │
└────────────────────────────────────────────────────────────────────────┘
```

## Key Modules & Capabilities

1. **Bloomberg-Grade Time-Series Charting**:
   - High-density 60-bar stochastic time-series canvas with true black obsidian background (`#06080c`).
   - Dynamic switching between standard candlesticks, filled mountain area curves, and smoothed Heikin-Ashi.
   - Built-in technical indicators: SMA 20, EMA 50, Bollinger Bands ribbon (2σ), Volume profile, and RSI 14 oscillator.
   - Snapping crosshair HUD projecting time, open, high, low, close, and volume.

2. **G10 Sovereign Forex Desk**:
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

7. **Zero-Leakage Sovereign Portfolio Ledger**:
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
