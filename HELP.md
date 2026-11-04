# MoneyCraft Comprehensive User Guide & Reference Manual

**MoneyCraft Sovereign Financial Intelligence Terminal**  
**Version**: 0.5.0  
**Domain**: Quantitative Finance, Global Indices Desk (Dow Jones, S&P 500, Nasdaq, DAX, Nikkei 225, KOSPI, FTSE, Hang Seng), Multi-Asset Comparison Graphs, Date-Range Presets (1D, 1W, 1M, 3M, YTD, 1Y), G10 Forex Desk, Crypto Derivatives (Hyperliquid Arbitrage & Liquidation Heatmaps), Fantasy Futures Paper Trading Sandbox, EVM Honeypot Defense, Sovereign Tax Accounting  
**Control Port**: `14117`

---

## 1. What Is MoneyCraft?

MoneyCraft is a sovereign, zero-subscription, local-first quantitative finance terminal and algorithmic research studio built in native Rust and immediate-mode GUI (`egui 0.37`). It provides individual quants, independent traders, and family offices with capabilities traditionally gatekept behind \$24,000–\$32,000/year institutional licenses (such as Bloomberg Terminal, Refinitiv Eikon, or FactSet) while strictly guaranteeing zero telemetry, zero data scraping, and local SQLite/DatabaseCraft privacy.

---

## 2. Competitive Matrix & Differentiation

| Feature / Dimension | Bloomberg Terminal | TradingView / SeekingAlpha | Coinglass / Hyperliquid | Claude / General Chatbots | **MoneyCraft v0.5.0** |
|---|---|---|---|---|---|
| **Annual License Cost** | \$24,000 – \$32,000 / year | \$200 – \$720 / year + upsells | Free / Paid VIP Tiers | \$240 / year (No finance kernel) | **100% Free & Sovereign Open Source** |
| **Data Privacy & Scraping** | Corporate surveillance | Telemetry monetized | Cloud RPC / logs | Cloud conversation logs | **Zero Telemetry (Local DatabaseCraft B-Tree)** |
| **High-Density Charting** | Classic Amber/Obsidian | WebGL / Browser Canvas | WebGL | None (Text / Static ASCII) | **Native High-FPS Immediate-Mode egui (stochastic drift)** |
| **Multi-Asset Comparisons** | Normalized `%` return overlays | Compare tool (gated alerts) | Crypto only | None | **Dedicated Multi-Asset Normalized Return Graph (`0.0%` Baseline)** |
| **Date Range Filters** | Standard intervals | Limited on free tier | Timeframe only | None | **Instant Range Switching (`1D`, `1W`, `1M`, `3M`, `YTD`, `1Y`)** |
| **Global Indices Desk** | All world markets | Stocks & ETFs | None | None | **DJIA, SPX, NDX, DAX, Nikkei (N225), KOSPI, FTSE, Hang Seng (HSI)** |
| **Crypto Perp Derivatives** | Basic CEX tickers | Third-party feeds | L1 feeds (No macro desk) | None | **Hyperliquid Funding Arbitrage + Liquidation Heatmaps** |
| **Fantasy Paper Trading** | Paper accounts (TradFi only) | Delayed paper trading | Demo accounts (often KYC) | None | **Risk-Free 1x–50x Leveraged Futures & Spot Sandbox** |
| **Multi-Asset Scope** | Multi-asset institutional | Stocks, FX, Crypto | Crypto only | Text advice only | **Global Indices, G10 Forex, Crypto, Equities, Gold** |
| **Live News Wire Ticker** | Breaking News Terminal Wire | Paid newsfeed banner | Crypto news only | None | **Real-Time Scrolling News Wire with Sentiment Polarity** |
| **Institutional AI Agents** | ASKB + BQuant Assistant | Basic AI PineScript indicators | None | General code generation | **4 Built-In Grounded Agentic AI Workflows (Strategy, PORT, Alpha, SOR)** |

---

## 3. Workspaces & Key Capabilities

### 📊 Workspace 1: Time-Series Terminal (`ActiveWorkspace::Terminal`)
- **Date Range Filters**: Instant one-click presets directly on the canvas toolbar:
  - `1D`: Intraday 24 hourly bars.
  - `1W`: 35-bar weekly trend view.
  - `1M`: 60-bar standard monthly perspective.
  - `3M`: 90-bar intermediate swing cycle.
  - `YTD`: 120-bar Year-to-Date cycle.
  - `1Y`: 180-bar annual macroeconomic trend.
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

### 📈 Workspace 2: Multi-Asset Normalized Comparison (`ActiveWorkspace::MultiComparison`)
- **How Multiple Stocks Are Shown in the Same Graph**:
  - When comparing assets with wildly different nominal prices (e.g., Dow Jones at 42,200 vs KOSPI at 2,580 vs EUR/USD at 1.0845), standard price charts fail.
  - MoneyCraft normalizes every selected asset's starting price at the beginning of the selected date range to **0.0%**.
  - All subsequent points reflect pure percentage capital returns:
    $$\text{Return}_t = \left(\frac{P_t - P_0}{P_0}\right) \times 100\%$$
  - Every asset is drawn with a distinct high-contrast color trace (Cyan, Gold, Jade, Crimson, Purple, Orange).
  - The horizontal `0.0% (BASE)` dashed guideline serves as the waterline separating positive outperformance from negative drawdowns.
  - Active benchmark tags display the latest percentage return right at the line termination (e.g. `KOSPI (+6.0%)`, `DJIA (+1.8%)`).

### 🔥 Workspace 3: Crypto Derivatives & Liquidation Heatmap (`ActiveWorkspace::CryptoDerivatives`)
- **Cross-Exchange Funding Rate Arbitrage**:
  - Real-time comparison across **Hyperliquid (L1 Appchain DEX)**, **Binance Futures**, **Bybit Perps**, and **OKX Swaps**.
  - Calculates annualized APR yield spread for delta-neutral cash-and-carry trades.
- **Forward-Looking Liquidation Heatmap**:
  - Identifies massive open interest clusters at key price thresholds.
  - Flags `🔥 SHORT SQUEEZE` zones and `🩸 LONG FLUSH` cascade flushes with notional volume in \$ millions.

### 🎮 Workspace 4: Fantasy Paper Trading Arena (`ActiveWorkspace::FantasyTrading`)
- **Risk-Free Simulated Sandbox**:
  - Default \$100,000 initial bankroll with 1-click account reset.
  - Order Entry Ticket with Long/Short, Spot/Perp, and **1x to 50x Leverage Slider**.
  - Real-time maintenance margin, floating PnL, win/loss stats, and 1-click position closure.

### 🧠 Workspace 5: Agentic AI Quant Workflows (`ActiveWorkspace::AiWorkflows`)
1. **BQuant Strategy Generator**: Compiles vectorized Python/Rust alpha models.
2. **PORT Macro Stress-Testing**: Central bank rate shock simulations and portfolio $\text{VaR}_{99}$.
3. **NLP Sentiment Alpha Scorer**: News wire catalyst analysis.
4. **Smart Order Routing (SOR)**: Slices blocks across dark pools and ECNs using adaptive VWAP.

---

## 4. Supported Global Market Tickers

### 🌐 Major Global Indices
- `DJIA`: Dow Jones Industrial Average (US)
- `SPX`: S&P 500 Large-Cap Index (US)
- `NDX`: Nasdaq-100 Tech Index (US)
- `DAX`: DAX 40 (Germany)
- `N225`: Nikkei 225 (Japan)
- `KOSPI`: KOSPI Composite (South Korea)
- `FTSE`: FTSE 100 (United Kingdom)
- `HSI`: Hang Seng Index (Hong Kong)

### 💱 G10 Sovereign Forex Desk
- `EUR/USD`: Euro / US Dollar
- `USD/JPY`: US Dollar / Japanese Yen
- `GBP/USD`: British Pound / US Dollar
- `USD/CNY`: US Dollar / Chinese Renminbi
- `EUR/GBP`: Euro / British Pound

### 🪙 Crypto, Equities & Commodities
- `BTC/USD`: Bitcoin / US Dollar
- `ETH/USD`: Ethereum / US Dollar
- `SOL/USD`: Solana / US Dollar
- `NVDA`: Nvidia Corporation
- `SPY`: S&P 500 ETF Trust
- `XAU/USD`: Gold Bullion / US Dollar
