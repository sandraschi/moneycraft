# MoneyCraft

MoneyCraft is an offline-first Rust financial intelligence studio, time-series terminal, and local portfolio ledger built for personal wealth analysis, market research, and paper trading.

## What You Can Do

- **Model personal wealth and inflation drag**: Compare Girokonto, Festgeld, and Bausparvertrag returns against inflation with Rule-of-72 alerts and year-by-year purchasing power tables.
- **Analyze global indices and G10 forex**: Track `DJIA`, `SPX`, `NDX`, `DAX`, `N225`, `KOSPI`, `FTSE`, `HSI`, and 20 major FX pairs with `1D`–`1Y` presets and zero-baseline multi-asset comparison.
- **Chart with institutional indicators**: Switch between Candlestick, Heikin-Ashi, and Mountain Area views with `SMA 20`, `EMA 50`, Bollinger Bands, Volume, `RSI 14`, and 1720–2026 historical panic overlays.
- **Track holdings in a local FIFO ledger**: Record stocks, ETFs, bonds, and cash in `moneycraft-ledger.db` (SQLite) with FIFO cost-basis and Austrian KESt estimation.
- **Practice in a paper-only trading sandbox**: Test `1x–50x` simulated spot and futures positions (`PaperOpen`/`PaperClose`) — `PlaceOrder` always refuses, and MoneyCraft never connects to a real broker or moves real money.
- **Audit EVM bytecode and run Monte Carlo backtests**: Check token contracts locally for transfer taxes or honeypot patterns, and run 10,000-path strategy simulations.
- **Use `Ctrl+J` AI Chat, Help menu, and Bundled Skills**: Press `Ctrl+J` for the offline **AI Chat Dock**, `Ctrl+K` for the Command Palette, or open the **10-item Help menu** and **Bundled Skills** (`skill://moneycraft-workflow/SKILL.md`, `skill://moneycraft-ledger/SKILL.md`).
- **Automate via CLI and 28 MCP tools**: Drive queries, backtests, and ledger operations from Claude Desktop or any MCP client via `moneycraft-cli`.

## Install

1. Download **`moneycraft-0.7.0-setup.exe`** from the [latest release](https://github.com/sandraschi/moneycraft/releases/latest).
2. Run it. Windows may show a SmartScreen warning because the installer is unsigned: choose **More info > Run anyway**.
3. Open **MoneyCraft** from the Start menu.

**Only want the tools inside Claude?** The `.mcpb` bundle registers the MCP server in Claude Desktop but does **not** install the desktop terminal. Paste this to Claude:

> Install the moneycraft MCP extension: download `moneycraft-0.7.0.mcpb` from https://github.com/sandraschi/moneycraft/releases/latest and open it with Claude Desktop.

**From a local checkout (Windows):**

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\install-craft.ps1
```

Installs the Windows app, `moneycraft-cli`, and the MCP bundle.

## Example Prompts

> "Simulate 2,000 EUR/month income with 3.8% inflation across Girokonto vs Festgeld over 10 years."

> "Fetch `EUR/USD` and `DAX` time series and compare their normalized percentage return over 3M."

> "Record a buy of 10 shares in the local FIFO ledger and show my current cost basis."

> "Run a 10,000-path Monte Carlo backtest on the current strategy and report Sharpe and max drawdown."

## Three Ways to Use It

| Surface | Start With | Best For |
|---|---|---|
| **Desktop Terminal** | `moneycraft` | Personal wealth simulator, time-series charts, portfolio ledger, and `Ctrl+J` Chat |
| **Command Line** | `moneycraft-cli --help` | Headless ticker queries, backtests, and scripted ledger checks |
| **MCP Server** | `moneycraft-cli` (stdio) | Calling 28 finance tools and bundled skills from Claude Desktop or your IDE |

## Keyboard Shortcuts (Desktop Terminal)

| Shortcut | Action |
|---|---|
| `Ctrl+J` | Toggle the offline **AI Chat Dock** |
| `Ctrl+K` / `Ctrl+Shift+P` | Open the Command Palette |
| `F1` | Open the Help Center |

## Safety Posture (Paper-Only)

MoneyCraft cannot execute live trades. Only the `paper` broker venue is compiled (`live_armed: false`), `finance_place_order` refuses live orders, and all external market feeds (Yahoo delayed, Binance/Hyperliquid public klines, ECB daily fixing) are read-only.

## Documentation

| Document | Contents |
|---|---|
| [HELP.md](HELP.md) | Complete user guide, 6-crate architecture, shortcuts (`Ctrl+J`, `Ctrl+K`), and ledger reference |
| [docs/PRD.md](docs/PRD.md) | Product requirements, capabilities, and safety rules |
| [docs/TIERED_PLAN.md](docs/TIERED_PLAN.md) | Release tiers and verification roadmap |
| [docs/UI_PARITY.md](docs/UI_PARITY.md) | Desktop UI and WebAssembly parity matrix |
| [LIVE_TRADING_PLAN.md](LIVE_TRADING_PLAN.md) | Architectural notes on broker-adapter seams (not enabled) |

## Requirements

- **OS**: Windows 10/11 (x64) for the installer; Linux/macOS supported from source
- **Network**: Optional; core terminal, personal wealth simulator, and SQLite ledger work offline

## License

MIT or Apache-2.0, at your option.
