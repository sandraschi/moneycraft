# MoneyCraft Comprehensive User Guide & Reference Manual

**MoneyCraft Local-First Financial Intelligence Terminal**  
**Version**: 0.7.0  
**Domain**: Quantitative Finance, Fleet-Standard i18n Localization (7 Languages), Historical Deep-Archive (1720–2026), 19 Major Financial Panics Catalog & Chart Annotations, Personal Wealth & Inflation Simulator (Girokonto, Festgeld, Bausparer), Global Indices Desk (Dow Jones, S&P 500, Nasdaq, DAX, Nikkei 225, KOSPI, FTSE, Hang Seng), Historic London Sovereign Papers (East India Co, Consols, South Sea Bubble), Multi-Asset Comparison Graphs, Arbitrary Date-Range Presets (1D to 300+ Years), G10 Forex Desk, Crypto Derivatives (Hyperliquid Arbitrage & Liquidation Heatmaps), Fantasy Futures Paper Trading Sandbox, EVM Honeypot Defense, Local Tax Accounting  
**Control Port**: `14117`

---

## 1. What Is MoneyCraft?

MoneyCraft is a zero-subscription, local-first quantitative finance terminal and algorithmic research studio built in native Rust and immediate-mode GUI (`egui 0.37`). It provides individual quants, independent traders, and family offices with capabilities traditionally gatekept behind \$24,000–\$32,000/year institutional licenses (such as Bloomberg Terminal, Refinitiv Eikon, or FactSet) while strictly guaranteeing zero telemetry, zero data scraping, and local SQLite/DatabaseCraft privacy.

---

## 2. Competitive Matrix & Differentiation

| Feature / Dimension | Bloomberg Terminal | TradingView / SeekingAlpha | Coinglass / Hyperliquid | Claude / General Chatbots | **MoneyCraft v0.6.0** |
|---|---|---|---|---|---|
| **Annual License Cost** | \$24,000 – \$32,000 / year | \$200 – \$720 / year + upsells | Free / Paid VIP Tiers | \$240 / year (No finance kernel) | **100% Free & Open Source** |
| **Data Privacy & Scraping** | Corporate surveillance | Telemetry monetized | Cloud RPC / logs | Cloud conversation logs | **Zero Telemetry (Local DatabaseCraft B-Tree)** |
| **Personal Wealth & Inflation** | None (institutional focus) | None (trading only) | None | Static approximations | **Real Purchasing Power Calculator (Giro, Festgeld, Bausparer vs Inflation)** |
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

### 💶 Workspace 6: Personal Wealth & Inflation Erosion Simulator (`ActiveWorkspace::PersonalWealth`)
*Designed specifically for individuals and beginners navigating savings accounts, fixed deposits, home-loan savings contracts, and the silent wealth tax of inflation.*

- **The Problem It Solves ("The Bank App Illusion")**:
  - Commercial bank apps display a growing nominal number (e.g., checking balance growing with salary deposits, a 3% Festgeld yielding 450 €/year). This creates a dangerous psychological sense of financial safety.
  - In reality, with a 3.8% inflation run-rate, goods and living costs compound upwards faster than savings yield. A portfolio yielding 3.0% against 3.8% inflation has an effective **negative real yield of -0.8%**.
- **Interactive Configuration Parameters**:
  - **Monthly Net Income**: Default 2,000 €/mo.
  - **Monthly Living Expenses**: Rent, food, utilities, subscriptions (default 1,400 €/mo).
  - **Checking Account (Girokonto)**: Default 4,000 € balance (yielding 0.0% interest).
  - **Festgeld (Time Deposit)**: Default 15,000 € principal bound for 3 years at 3.0% APR.
  - **Bausparvertrag (Home-Savings)**: Default 100 €/mo contribution, 5,000 € current balance, 1.5% APR.
  - **Inflation Run-Rate**: Adjustable macro inflation slider (1.0% to 10.0%, default 3.8%).
  - **Simulation Horizon**: 1 to 10 years forward projection.
- **Diagnostic HUD Metrics**:
  - `CURRENT SAVINGS`: Sum of liquid cash, Festgeld principal, and Bausparer balance.
  - `MONTHLY SAVINGS FLOW`: Unallocated cashflow added to Girokonto each month ($\text{Income} - \text{Expenses} - \text{Bausparer}$).
  - `INTEREST EARNED`: Gross annual interest from Festgeld and Bausparer.
  - `INFLATION EROSION`: Annual monetary purchasing power evaporated by inflation.
  - `NET REAL RETURN`: True real interest rate ($\text{Yield} - \text{Inflation}$).
- **The Rule of 72 Purchasing Power Halving Warning**:
  - Automatically calculates when your savings will lose 50% of their real purchasing value:
    $$\text{Years to Halve} = \frac{72}{\text{Inflation Rate}}$$
  - At 3.8% inflation, your money buys half as much in **18.9 years**; at 6% inflation, in **12 years**.
- **Nominal Illusion vs True Purchasing Power Grid**:
  - Side-by-side year-by-year comparison contrasting:
    - **Nominal Total**: What your bank app will show.
    - **Real Value**: What those Euros will actually purchase in terms of groceries, rent, and energy ($P_{\text{real}} = P_{\text{nominal}} / (1 + i)^t$).
    - **Silent Inflation Loss**: Cumulative Euros stolen by inflation.
    - **Purchasing Power Retention**: Percentage of base year value retained.
    - **Real Monthly Income**: Value of your initial 2,000 € monthly wage discounted forward.
- **Actionable Beginner Defense Roadmap**:
  1. **Emergency Cash Buffer (Notfallgroschen)**: Cap Girokonto strictly at 3–6 months living expenses (~4,500 € – 8,000 €).
  2. **Tagesgeld & Money Market ETF (e.g. DBX0AN)**: Sweep excess checking cash into ECB overnight rate trackers yielding ~3.2% with daily liquidity.
  3. **Global Equity ETF (MSCI World / FTSE All-World)**: Direct monthly savings into broad equities historically delivering 7.0%–8.5% nominal return (+3.5% real return after inflation).
  4. **Physical Gold Bullion (XAU)**: Maintain 5%–10% allocation as a counter-fiat debasement hedge.

### 📡 Workspace 7: Market Data Sources & Exchange Payola Matrix (`ActiveWorkspace::DataSources`)
*Instant toggling between air-gapped simulated models and live real-world public feeds, with transparent latency audits and commercial payola disclosures.*

| Provider ID | Provider Name | Delay | Cost | Auth Required | Supported Assets | Why It's Free / Payola Reality |
|---|---|---|---|---|---|---|
| `synthetic` | **Synthetic Stochastic Model** | **0s Instant (0ms)** | **100% Free (\$0)** | None (Air-gapped) | All Global Tickers | Free forever. Runs 100% locally on your machine with zero tracking. |
| `binance` | **Binance Public Market API** | **⚡ Real-Time (<150ms)** | **100% Free (\$0)** | None (Public REST) | BTC/USD, ETH/USD, SOL/USD | Crypto exchanges provide free public feeds to attract order flow. |
| `hyperliquid` | **Hyperliquid L1 DEX** | **⚡ Real-Time (<120ms)** | **100% Free (\$0)** | None (Public L1 Node) | BTC, ETH, SOL, Perp Mark Px, Funding | Pure on-chain transparency. No paywall, no KYC, decentralized node. |
| `yahoo` | **Yahoo Finance Feed** | **⏱️ 15-Min Delayed** | **100% Free (\$0)** | None (Public Web) | DJIA, SPX, DAX, KOSPI, Forex, Gold | Exchange regulations legally exempt 15-minute delayed data from royalties. |
| `ecb` | **ECB Euro Foreign Exchange** | **📅 Daily (14:15 CET)** | **100% Free (\$0)** | None (Official SDMX) | EUR/USD, EUR/GBP, USD/JPY, CNY | Official sovereign benchmark fixing published daily by Eurosystem. |
| `twelve_data` | **TwelveData / Finnhub** | **⚡ Real-Time (Rate-limited)** | Free Tier / \$29-\$79/mo | Free API Key | US Equities, Global Indices | Commercial aggregator; free tier provides 800 calls/day. |
| `interactive_brokers` | **Interactive Brokers Gateway** | **⚡ Real-Time Sub-Second** | \$1.50–\$10/mo pass-through | Local Port `7496` | NYSE, NASDAQ, CME Futures | Bypasses \$24k Bloomberg licenses: IBKR passes through raw exchange fees directly. |
| `polygon_io` | **Polygon.io / Tiingo SIP** | **⚡ Sub-Second WebSocket** | \$29–\$199/mo | Commercial API Key | US Equities, SIP Tape A/B/C | Bundles exchange redistributor licensing into a commercial developer SaaS. |
| `bloomberg_bpipe` | **Bloomberg B-PIPE / Eikon** | **⚡ Sub-Millisecond** | \$2,000–\$2,700/mo | Leased Line Hardware | Institutional All-Asset | Wall Street financial cartel. Costs \$24k-\$32k/year per seat. |

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

### 💱 G10 Forex Desk
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

### 📜 Historic London Sovereign Papers & Deep-Archive (1720–1874)
- `EIC`: **Honourable East India Company Stock (1600–1874)**: The world's first mega-corporation, chartered by Queen Elizabeth I, commanding a 260,000-man private army before dissolution in 1874.
- `UK_CONSOLS`: **Bank of England 3% Consolidated Annuities (1751+)**: The longest uninterrupted sovereign bond in human history, funding the British Empire and Napoleonic Wars without default.
- `SOUTH_SEA`: **South Sea Company (1711–1720)**: The British sovereign debt swap equity vehicle that precipitated the famous 1720 South Sea Bubble and Bubble Act.

---

## 5. Fleet-Standard i18n Localization Architecture

Following the Storytold Craft applications (such as *GameCraft*, *QGISCraft*, and *DesignCraft*), MoneyCraft implements a pure-Rust, zero-runtime-overhead internationalization (i18n) engine:
- **Zero Disk I/O & Zero Gettext Dependencies**: All localized strings are statically compiled into the binary via typed enums (`I18nKey`) and typed locales (`AppLocale`).
- **Supported Locales**:
  - 🇺🇸 `EN` (English - Sovereign Standard)
  - 🇩🇪 `DE` (Deutsch - Girokonto, Festgeld & Bausparer)
  - 🇫🇷 `FR` (Français)
  - 🇪🇸 `ES` (Español)
  - 🇯🇵 `JA` (日本語)
  - 🇰🇷 `KO` (한국어 - KOSPI & FX)
  - 🇨🇳 `ZH` (中文)
- **Instant Hot-Switching**: Selectable directly from the top header bar dropdown (`[🌐 🇺🇸 EN ▼]`), dynamically repainting the entire Bloomberg GUI in real time without restarting.

---

## 6. The 19 Historic Financial Panics (1720 to 2026)

MoneyCraft includes interactive chart annotations, vertical timeline flags, catalyst autopsies, and modern quant risk takeaways for the 19 greatest financial market crises:
1. **1720: The South Sea Bubble & Mississippi Collapse** (-84.0%, Exchange Alley & Paris) — Newton's folly and John Law's fiat credit expansion.
2. **1772: London Credit Crisis** (-38.0%, Lombard Street) — Alexander Fordyce EIC short squeeze dominoes across Ayr Bank and triggers the Boston Tea Party.
3. **1825: Latin American Sovereign Bubble & Poyais Fraud** (-45.0%, London Stock Exchange) — Fictitious Poyais bonds and Bank of England near-exhaustion.
4. **1837: Panic of 1837 & Great Deflation** (-42.0%, New York & New Orleans) — Jackson's Specie Circular and collapse of 600 banks.
5. **1847: Railway Mania Collapse** (-67.0%, Great George Street) — Margin calls on duplicate railway lines; Bank Charter Act of 1844 suspended.
6. **1857: Panic of 1857 (First World Telegraph Panic)** (-35.0%, Wall Street & London) — Sinking of SS Central America with 30,000 lbs gold; instant electric wire panic.
7. **1866: Overend Gurney Collapse & Black Friday** (-42.0%, Lombard Street) — "The Bankers' Bank" implosion; Bagehot's classical lender-of-last-resort doctrine established.
8. **1873: Panic of 1873 & Gründerkrach** (-48.0%, Vienna Börse & NYSE) — Jay Cooke failure, silver demonetization, and EIC 1874 dissolution.
9. **1890: Baring Crisis** (-32.0%, London & Buenos Aires) — Barings underwriting of Argentine sovereign debt averted by Bank of England consortium.
10. **1893: Panic of 1893 (Silver Repeal)** (-37.0%, Wall Street) — Philadelphia and Reading Railroad bankruptcy and US Treasury gold drain.
11. **1907: Panic of 1907 (Knickerbocker Run)** (-49.0%, Wall Street) — F. Augustus Heinze copper corner; J.P. Morgan's private rescue leads to the 1913 Federal Reserve Act.
12. **1929: The Great Crash & Great Depression** (-89.2%, Wall Street & Worldwide) — Black Tuesday, margin liquidations, Smoot-Hawley tariffs, and 4-year deflationary collapse.
13. **1973: First Oil Shock & Stagflation Crash** (-48.2%, Global) — OPEC embargo, end of Bretton Woods gold convertibility, and stagflation.
14. **1987: Black Monday & Portfolio Insurance Cascade** (-33.5%, NYSE & Global) — Algorithmic constant-mix portfolio insurance stop-loss execution cascade (-22.6% in 1 day).
15. **1997: Asian Financial Crisis (Tom Yam Kung)** (-65.0%, Bangkok, Seoul, Jakarta, HK) — Break of currency pegs and Korean KOSPI crash.
16. **1998: Russian Ruble Default & LTCM Liquidation** (-22.0%, Moscow & Greenwich CT) — Gaussian VaR underestimating fat tails; 100:1 leverage blowout.
17. **2000: Dot-Com Tech Bubble Crash** (-78.4%, NASDAQ) — Speculative frenzy in non-profitable internet IPOs with extreme price-to-eyeball multiples.
18. **2008: Global Financial Crisis (Lehman Bankruptcy)** (-56.8%, Wall Street & Global) — Subprime MBS/CDO securitization run and birth of ZIRP / Quantitative Easing.
19. **2020: Covid-19 Liquidity Shock & Negative Oil Flash** (-34.0%, Global) — Fastest 30% drop in market history; WTI crude flash to -$37.63/bbl.
20. **2022: Global Inflation & Rate Hiking Shock (SVB Run)** (-25.4%, US Treasuries & Tech) — Fastest 500 bps Fed hikes in 40 years; social-media bank run on Silicon Valley Bank.
