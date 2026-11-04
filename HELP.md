# MoneyCraft Comprehensive User Guide & Reference Manual

**MoneyCraft Sovereign Financial Intelligence Terminal**  
**Version**: 0.3.0  
**Domain**: Quantitative Finance, Forex Desk, Time-Series Analysis, Institutional Agentic AI Workflows, EVM Honeypot Defense, Sovereign Tax Accounting  
**Control Port**: `14117`

---

## 1. What Is MoneyCraft?

MoneyCraft is a sovereign, zero-subscription, local-first quantitative finance terminal and algorithmic research studio built in native Rust and immediate-mode GUI (`egui 0.37`). It provides individual quants, independent traders, and family offices with capabilities traditionally gatekept behind \$24,000–\$32,000/year institutional licenses (such as Bloomberg Terminal, Refinitiv Eikon, or FactSet) while strictly guaranteeing zero telemetry, zero data scraping, and local SQLite/DatabaseCraft privacy.

---

## 2. Competitive Matrix & Differentiation

| Feature / Dimension | Bloomberg Terminal | TradingView / SeekingAlpha | Claude / General Chatbots | OpenCode / Standard IDEs | **MoneyCraft v0.3.0** |
|---|---|---|---|---|---|
| **Annual License Cost** | \$24,000 – \$32,000 / year | \$200 – \$720 / year + upsells | \$240 / year (No financial kernel) | Free (Generic code only) | **100% Free & Sovereign Open Source** |
| **Data Privacy & Scraping** | Proprietary corporate surveillance | Telemetry tracked & monetized | Cloud conversation logs | Local / Cloud telemetry | **Zero Telemetry (Local DatabaseCraft B-Tree)** |
| **High-Density Charting** | Classic Amber/Obsidian Terminal | WebGL / Canvas Browser | None (Text / Static ASCII) | None | **Native High-FPS Immediate-Mode egui (60-bar stochastic)** |
| **Technical Indicators** | ASKB / BQL Overlays | Gated behind subscription tiers | None | None | **SMA 20, EMA 50, Bollinger Bands (2σ), RSI 14, Volume Profile** |
| **Multi-Asset Coverage** | Multi-asset institutional | Stocks, FX, Crypto (Rate limited) | Text advice only | None | **Crypto, G10 Forex (EUR, USD, JPY, GBP, CNY), Equities, Gold** |
| **Live News Wire Ticker** | Breaking News Terminal Wire | Paid newsfeed banner | None | None | **Real-Time Scrolling News Wire with Sentiment Polarity** |
| **Institutional AI Workflows** | ASKB + BQuant Assistant + PORT | Basic AI PineScript indicators | General coding (no financial API) | General code generation | **4 Built-In Grounded Agentic AI Workflows (Strategy, PORT, Sentiment, SOR)** |
| **Smart Contract Scam Defense** | None (TradFi focused) | None | Naive code review | Static linters | **Local Sandbox EVM Honeypot & Drainer Auditor** |
| **Backtesting Engine** | BQuant Python / BQL | PineScript cloud server | Generates conceptual code | Local interpreter | **10,000-Path Monte Carlo Simulation (Sharpe, Drawdown, VaR)** |
| **Private Tax Accounting** | Requires third-party enterprise tools | None | General formulas | None | **Autonomous FIFO/LIFO Sovereign Tax Ledger** |

---

## 3. Workspaces & Key Capabilities

### 📊 Workspace 1: Time-Series Terminal (`ActiveWorkspace::Terminal`)
- **Obsidian Dark Canvas**: High-contrast black `#06080c` background matching the classic institutional terminal.
- **Header Telemetry**: Amber (`#ffb300`) monospace ribbon displaying real-time Price, 24h Delta (USD & %), VWAP, 24h High/Low, and Control Port (`14117`).
- **Rendering Modes**:
  - `🕯️ Candlestick`: Precision wicks and bodies in Jade Bull Green (`#00e676`) and Crimson Bear Red (`#ff1744`).
  - `📈 Mountain Area`: Continuous price boundary curve with vertical gradient alpha fill.
  - `📊 Heikin-Ashi`: Smoothed synthetic trend candles.
- **Overlay Indicators**:
  - `SMA 20` (Cyan `#00e5ff`)
  - `EMA 50` (Gold `#ffd600`)
  - `Bollinger Bands` (Purple `#b388ff`, 20-period $\pm 2\sigma$ channel ribbon)
  - `Volume Profile` (Base panel with bull/bear volume bars)
  - `RSI 14` (Oscillator sub-panel with 70/30 overbought/oversold boundaries)
- **Interactive Crosshair HUD**: Moving cursor across canvas projects coordinate crosshairs and renders floating tooltip with `TIME`, `OPEN`, `HIGH`, `LOW`, `CLOSE`, and `VOLUME`.

### 🧠 Workspace 2: Agentic AI Quant Workflows (`ActiveWorkspace::AiWorkflows`)
Grounded directly in sovereign market data, matching the Bloomberg ASKB / BQuant / PORT Enterprise standard:
1. **BQuant Strategy Generator**: Translates natural language alpha concepts into vectorized Python/Rust backtests, Sharpe ratios, win rates, and drawdown limits.
2. **Macro Shock Stress-Testing (PORT)**: Simulates central bank rate shocks (e.g., ECB cuts, BOJ hikes) and computes Value-at-Risk ($\text{VaR}_{99}$), factor betas, and delta-neutral hedging strangles.
3. **NLP Sentiment Alpha Scorer**: Evaluates breaking headlines, regulatory filings, and central bank speeches for hawkish/dovish semantic drift and directional confidence.
4. **Smart Order Routing (SOR)**: Slices institutional blocks into adaptive VWAP sub-orders across ECNs, dark pools, and dealer desks to minimize slippage.

### 🛡️ Workspace 3: EVM Honeypot Auditor (`ActiveWorkspace::HoneypotAuditor`)
- Simulates on-chain ERC-20 / EVM token contracts in an isolated local sandbox.
- Identifies malicious traps:
  - Sell tax $> 5\%$ or dynamic $99\%$ honeypot transfer fees.
  - Blacklist functions allowing creator to freeze buyer balances.
  - Unrenounced mint backdoors allowing unlimited token inflation.
  - Unlocked developer liquidity.
- Outputs risk scores from `0` (Clean) to `100` (Critical Honeypot).

### ⚙️ Workspace 4: Quantitative Backtester (`ActiveWorkspace::Backtester`)
- High-speed 10,000-path Monte Carlo permutation engine.
- Computes annual ROI, Sharpe ratio, Sortino ratio, win rate percentage, and maximum peak-to-trough drawdown.

### 💼 Workspace 5: Sovereign Portfolio Ledger (`ActiveWorkspace::PortfolioLedger`)
- Local SQLite / DatabaseCraft immutable ledger.
- Real-time mark-to-market valuations across multi-asset holdings (Crypto, Forex, Equities, Commodities).
- Automated capital gains tax liability calculation with zero data leakage.

---

## 4. Supported Multi-Asset & Forex Tickers

### G10 Forex Desk
- `EUR/USD`: Euro / US Dollar (Pip precision: 4 decimals, e.g. `1.0845`)
- `USD/JPY`: US Dollar / Japanese Yen (Pip precision: 2 decimals, e.g. `152.40`)
- `GBP/USD`: British Pound / US Dollar (Pip precision: 4 decimals, e.g. `1.2980`)
- `USD/CNY`: US Dollar / Chinese Renminbi (Pip precision: 4 decimals, e.g. `7.1250`)
- `EUR/GBP`: Euro / British Pound Cross Rate (Pip precision: 4 decimals, e.g. `0.8355`)

### Crypto, Equities & Commodities
- `BTC/USD`: Bitcoin / US Dollar
- `ETH/USD`: Ethereum / US Dollar
- `SOL/USD`: Solana / US Dollar
- `NVDA`: Nvidia Corporation
- `SPY`: S&P 500 ETF Trust
- `XAU/USD`: Gold Bullion / US Dollar

---

## 5. CLI & MCP Tool Reference

MoneyCraft runs seamlessly as a standalone desktop application, an institutional CLI tool (`moneycraft-cli`), and an MCP tool server (`moneycraft-mcp`):

### CLI Commands
```powershell
# Check status and loaded quantitative modules
.\target\debug\moneycraft-cli.exe status

# Fetch 60-bar high-density candlestick feed with technical indicators
.\target\debug\moneycraft-cli.exe ticker EUR/USD
.\target\debug\moneycraft-cli.exe ticker USD/JPY
.\target\debug\moneycraft-cli.exe ticker BTC/USD

# Audit EVM contract address in local sandbox
.\target\debug\moneycraft-cli.exe audit 0x123_scam_token_trap --chain Ethereum

# Execute 10,000-path backtest
.\target\debug\moneycraft-cli.exe backtest BollingerBreakout_v2 --symbol EUR/USD --capital 100000

# Inspect sovereign portfolio balance and tax liability
.\target\debug\moneycraft-cli.exe portfolio
```

### FastMCP Tool Bindings
- `moneycraft_status`: Health check, control port, and module manifest.
- `moneycraft_market_feed`: Returns candlestick time-series, SMA20, EMA50, Bollinger Bands, RSI14, and summary.
- `moneycraft_news_ticker`: Streams live Bloomberg wire headlines with sentiment polarity.
- `moneycraft_ai_workflow`: Triggers Strategy Gen, Macro Stress-Test, Sentiment Alpha, or SOR.
- `moneycraft_audit_contract`: Runs EVM bytecode analysis for honeypot traps.
- `moneycraft_run_backtest`: Computes Sharpe ratio, drawdown, and ROI.
- `moneycraft_portfolio_summary`: Retrieves mark-to-market balances and tax liabilities.
