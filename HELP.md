# MoneyCraft Comprehensive User Guide & Reference Manual

**MoneyCraft Sovereign Financial Intelligence Terminal**  
**Version**: 0.4.0  
**Domain**: Quantitative Finance, Forex Desk, Time-Series Analysis, Institutional Agentic AI Workflows, Crypto Perpetual Futures, Hyperliquid Funding Arbitrage, Liquidation Heatmaps, Fantasy Paper Trading Sandbox, EVM Honeypot Defense, Sovereign Tax Accounting  
**Control Port**: `14117`

---

## 1. What Is MoneyCraft?

MoneyCraft is a sovereign, zero-subscription, local-first quantitative finance terminal and algorithmic research studio built in native Rust and immediate-mode GUI (`egui 0.37`). It provides individual quants, independent traders, and family offices with capabilities traditionally gatekept behind \$24,000–\$32,000/year institutional licenses (such as Bloomberg Terminal, Refinitiv Eikon, or FactSet) while strictly guaranteeing zero telemetry, zero data scraping, and local SQLite/DatabaseCraft privacy.

---

## 2. Competitive Matrix & Differentiation

| Feature / Dimension | Bloomberg Terminal | TradingView / SeekingAlpha | Coinglass / Hyperliquid | Claude / General Chatbots | **MoneyCraft v0.4.0** |
|---|---|---|---|---|---|
| **Annual License Cost** | \$24,000 – \$32,000 / year | \$200 – \$720 / year + upsells | Free / Paid VIP Tiers | \$240 / year (No finance kernel) | **100% Free & Sovereign Open Source** |
| **Data Privacy & Scraping** | Corporate surveillance | Telemetry monetized | Cloud RPC / logs | Cloud conversation logs | **Zero Telemetry (Local DatabaseCraft B-Tree)** |
| **High-Density Charting** | Classic Amber/Obsidian | WebGL / Browser Canvas | WebGL | None (Text / Static ASCII) | **Native High-FPS Immediate-Mode egui (60-bar stochastic)** |
| **Technical Indicators** | ASKB / BQL Overlays | Gated behind subscription | Basic overlays | None | **SMA 20, EMA 50, Bollinger Bands, RSI 14, CVD, Volume** |
| **Crypto Perp Derivatives** | Basic CEX tickers | Third-party feeds | L1 feeds (No macro desk) | None | **Hyperliquid Funding Arbitrage + Liquidation Heatmaps** |
| **Fantasy Paper Trading** | Paper accounts (TradFi only) | Delayed paper trading | Demo accounts (often KYC) | None | **Risk-Free 1x–50x Leveraged Futures & Spot Sandbox** |
| **Multi-Asset Scope** | Multi-asset institutional | Stocks, FX, Crypto | Crypto only | Text advice only | **Crypto, G10 Forex (EUR, USD, JPY, GBP, CNY), Equities, Gold** |
| **Live News Wire Ticker** | Breaking News Terminal Wire | Paid newsfeed banner | Crypto news only | None | **Real-Time Scrolling News Wire with Sentiment Polarity** |
| **Institutional AI Agents** | ASKB + BQuant Assistant | Basic AI PineScript indicators | None | General code generation | **4 Built-In Grounded Agentic AI Workflows (Strategy, PORT, Alpha, SOR)** |
| **Smart Contract Scam Defense** | None (TradFi focused) | None | None | Naive code review | **Local Sandbox EVM Honeypot & Drainer Auditor** |
| **Backtesting Engine** | BQuant Python / BQL | PineScript cloud server | Historical funding stats | Conceptual code | **10,000-Path Monte Carlo Simulation (Sharpe, Drawdown, VaR)** |
| **Private Tax Accounting** | Requires third-party tools | None | None | General formulas | **Autonomous FIFO/LIFO Sovereign Tax Ledger** |

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

### 🔥 Workspace 2: Crypto Derivatives & Liquidation Heatmap (`ActiveWorkspace::CryptoDerivatives`)
- **Cross-Exchange Funding Rate Arbitrage**:
  - Real-time comparison across **Hyperliquid (L1 Appchain DEX)**, **Binance Futures**, **Bybit Perps**, and **OKX Swaps**.
  - Calculates annualized APR yield spread for delta-neutral cash-and-carry trades (e.g. Long low-rate CEX / Short high-rate DEX).
- **Forward-Looking Liquidation Heatmap**:
  - Identifies massive open interest clusters at key price thresholds.
  - Flags `🔥 SHORT SQUEEZE` zones (forced liquidations pushing price higher) and `🩸 LONG FLUSH` zones (cascading stop flushes pushing price lower).
  - Displays cascade notional volume (\$ millions) and distance from current spot.

### 🎮 Workspace 3: Fantasy Paper Trading Arena (`ActiveWorkspace::FantasyTrading`)
- **Risk-Free Simulated Sandbox**:
  - Default \$100,000 initial bankroll with 1-click account reset.
  - Real-time account telemetry: Total Equity, Available Balance, Margin Allocated, Unrealized Floating PnL, Win Count, and Loss Count.
- **Order Entry Ticket**:
  - Long (Buy) or Short (Sell).
  - Spot vs Perpetual Futures.
  - Interactive **Leverage Slider (1x to 50x)**.
  - Projected maintenance margin and automatic liquidation price calculation.
- **Active Positions Management**:
  - Interactive table displaying Position ID, Symbol, Side, Leverage, Entry Price, Mark Price, Liquidation Price, Margin, Floating PnL ($ and %), and 1-Click `❌ Close` execution.

### 🧠 Workspace 4: Agentic AI Quant Workflows (`ActiveWorkspace::AiWorkflows`)
Grounded directly in sovereign market data, matching the Bloomberg ASKB / BQuant / PORT Enterprise standard:
1. **BQuant Strategy Generator**: Translates natural language alpha concepts into vectorized Python/Rust backtests, Sharpe ratios, win rates, and drawdown limits.
2. **Macro Shock Stress-Testing (PORT)**: Simulates central bank rate shocks (e.g., ECB cuts, BOJ hikes) and computes Value-at-Risk ($\text{VaR}_{99}$), factor betas, and delta-neutral hedging strangles.
3. **NLP Sentiment Alpha Scorer**: Evaluates breaking headlines, regulatory filings, and central bank speeches for hawkish/dovish semantic drift and directional confidence.
4. **Smart Order Routing (SOR)**: Slices institutional blocks into adaptive VWAP sub-orders across ECNs, dark pools, and dealer desks to minimize slippage.

### 🛡️ Workspace 5: EVM Honeypot Auditor (`ActiveWorkspace::HoneypotAuditor`)
- Simulates on-chain ERC-20 / EVM token contracts in an isolated local sandbox.
- Identifies malicious traps:
  - Sell tax $> 5\%$ or dynamic $99\%$ honeypot transfer fees.
  - Blacklist functions allowing creator to freeze buyer balances.
  - Unrenounced mint backdoors allowing unlimited token inflation.
  - Unlocked developer liquidity.
- Outputs risk scores from `0` (Clean) to `100` (Critical Honeypot).

### ⚙️ Workspace 6: Quantitative Backtester (`ActiveWorkspace::Backtester`)
- High-speed 10,000-path Monte Carlo permutation engine.
- Computes annual ROI, Sharpe ratio, Sortino ratio, win rate percentage, and maximum peak-to-trough drawdown.

### 💼 Workspace 7: Sovereign Portfolio Ledger (`ActiveWorkspace::PortfolioLedger`)
- Local SQLite / DatabaseCraft immutable ledger.
- Real-time mark-to-market valuations across multi-asset holdings (Crypto, Forex, Equities, Commodities).
- Automated capital gains tax liability calculation with zero data leakage.

---

## 4. Supported Multi-Asset & Forex Tickers

### Crypto Spot & Perpetual Futures
- `BTC/USD`: Bitcoin / US Dollar
- `ETH/USD`: Ethereum / US Dollar
- `SOL/USD`: Solana / US Dollar

### G10 Forex Desk
- `EUR/USD`: Euro / US Dollar (Pip precision: 4 decimals, e.g. `1.0845`)
- `USD/JPY`: US Dollar / Japanese Yen (Pip precision: 2 decimals, e.g. `152.40`)
- `GBP/USD`: British Pound / US Dollar (Pip precision: 4 decimals, e.g. `1.2980`)
- `USD/CNY`: US Dollar / Chinese Renminbi (Pip precision: 4 decimals, e.g. `7.1250`)
- `EUR/GBP`: Euro / British Pound Cross Rate (Pip precision: 4 decimals, e.g. `0.8355`)

### Equities & Commodities
- `NVDA`: Nvidia Corporation
- `SPY`: S&P 500 ETF Trust
- `XAU/USD`: Gold Bullion / US Dollar
