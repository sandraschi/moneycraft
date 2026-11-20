---
name: moneycraft-workflow
description: End-to-end MoneyCraft market discovery, charting, multi-asset comparison, personal wealth simulation, backtesting, and paper-trading workflow using moneycraft-mcp tools.
---

# MoneyCraft Quantitative & Terminal Workflow

Use this skill when exploring markets, comparing assets, running personal wealth or Monte Carlo simulations, auditing smart contracts, or testing strategies in the paper-trading sandbox via `moneycraft-mcp` (`moneycraft-cli --mcp`) or CLI subcommands.

## 1. Safety & Data-Delay Rules (Read First)

- **No live money**: `finance_place_order` always refuses. Only the `paper` broker venue is compiled (`finance_brokers` confirms `live_armed: false`).
- **Feed delay semantics**:
  - `yahoo` (default): 15-minute delayed exchange data for indices, stocks, forex, and gold.
  - `binance` / `hyperliquid`: live public crypto klines/mark prices.
  - `ecb`: daily Eurosystem FX fixing (~14:15 CET).
  - `synthetic`: offline deterministic random-walk (0 ms, air-gapped).
- Never present delayed, synthetic, or fantasy paper numbers as live tradable quotes.

## 2. Discovery Sequence

1. Check runtime health and port (`14117`):
   - MCP: `finance_status`
   - CLI: `moneycraft-cli status`
2. Discover valid tickers and 20 major currencies:
   - MCP: `finance_universe`
   - CLI: `moneycraft-cli universe`
3. Inspect available feeds and payola/delay tiers:
   - MCP: `finance_list_data_sources`
   - CLI: `moneycraft-cli sources`

## 3. Market Data, Comparison & Macro Context

- Fetch OHLCV bars for any universe ticker or Yahoo symbol:
  - MCP: `finance_ticker` (`symbol`, optional `source`, optional `bar_count`)
  - CLI: `moneycraft-cli ticker EUR/USD --source yahoo`
- Compare normalized percentage returns from a `0.0%` baseline:
  - MCP: `finance_compare` (`symbols: ["DJIA", "SPX", "KOSPI", "DAX"]`, optional `source`, `bar_count`)
  - CLI: `moneycraft-cli compare "DJIA,SPX,KOSPI" --bars 60`
- Check breaking news sentiment and crypto derivatives funding/liquidation levels:
  - MCP: `finance_news`, `finance_derivatives` (`symbol`)
- Explore historical deep-archive instruments and 19 systemic panics (1720–2022):
  - MCP: `finance_historic_instruments`, `finance_historic_panics`

## 4. Personal Wealth, Backtesting & Auditing

- **Personal wealth & inflation erosion** (Girokonto, Festgeld, Bausparer vs inflation):
  - MCP: `finance_simulate_personal_wealth` (`income`, `living_costs`, `checking`, `festgeld`, `festgeld_interest`, `bausparer_monthly`, `bausparer_balance`, `inflation`, `years`)
- **Monte Carlo backtest**:
  - MCP: `finance_backtest` (`strategy`, `symbol`, `capital`)
- **Agentic quant templates** (planning/templates only, no execution):
  - MCP: `finance_ai_workflow` (`workflow_type`: `STRATEGY_GEN` | `STRESS_TEST` | `SENTIMENT_ALPHA` | `EXECUTION_ROUTER`, `prompt`, `target_asset`)
- **EVM honeypot bytecode audit** (local sandbox):
  - MCP: `finance_audit_contract` (`address`, `chain`)

## 5. Paper-Trading Sandbox & Broker Dry-Run

- Inspect compiled broker venues and risk limits: `finance_brokers`
- Validate or paper-fill via broker trait (`dry_run: true` validates without booking):
  - MCP: `finance_broker_place` (`broker: "paper"`, `symbol`, `side`, `order_type`, `quantity`, `leverage`, `dry_run: true`)
- Stateless fantasy derivatives desk (1x–50x):
  - `finance_paper_account`, `finance_paper_open`, `finance_paper_close`, `finance_paper_reset`
- Read any bundled skill on demand via `moneycraft_skill_read` or `resources/read`.
