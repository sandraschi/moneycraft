---
name: moneycraft-ledger
description: Local-first personal portfolio ledger, FIFO lot accounting, broker/bank CSV import, multi-currency valuation, and Austrian KESt (27.5%) tax estimation in MoneyCraft.
---

# MoneyCraft Sovereign Portfolio Ledger Workflow

Use this skill when recording real-world holdings (stocks, ETFs, bonds, cash, commodities), importing broker/bank CSVs, inspecting FIFO cost-basis lots, or running portfolio valuation and KESt estimates using `moneycraft-mcp` or `moneycraft-cli`.

## 1. Ledger Honesty & Safety Gates

- **Default database**: `moneycraft-ledger.db` (DatabaseCraft/SQLite) in the working directory, overridable via `db` / `--db`.
- **Empty ledger = empty portfolio**: `finance_portfolio` and `finance_ledger_valuation` never fabricate demo holdings.
- **Crypto is refused**: Real ledger bookings and CSV imports refuse crypto symbols (`BTC`, `ETH`, `SOL`, `BTC/USD`, etc.) — keep crypto experiments in the fantasy paper desk (`finance_paper_open`).
- **Insufficient FIFO lots**: `SELL` orders exceeding held quantity are refused, never auto-invented.
- **Backup before bulk edits or imports**: Copy `moneycraft-ledger.db` (e.g. `moneycraft-ledger.db.bak`) before importing large CSVs or re-booking historical lots.
- **KESt estimate disclaimer**: Austrian Kapitalertragsteuer (`27.5%` flat on realized YTD gains) is an **estimate only**, not tax advice.

## 2. Recording & Inspecting Holdings

1. Book a transaction (`BUY` or `SELL`):
   - MCP: `finance_ledger_add` (`symbol`, `side`, `quantity`, `price`, optional `asset_class`, `currency`, `venue`, `date`, `fees`, `note`, `db`)
   - CLI: `moneycraft-cli ledger-add SAP.DE --side BUY --quantity 10 --price 228 --venue flatex`
   - Note: Bare 3-letter codes like `EUR` auto-detect as `CASH`; pairs with `/` or `-` auto-detect as `FOREX` (or `COMMODITY` for `XAU/...`).
2. Inspect FIFO cost-basis lots without network calls:
   - MCP: `finance_ledger_holdings` (optional `db`)
   - CLI: `moneycraft-cli ledger-holdings`
3. Audit raw bookings (newest first, optional symbol filter):
   - MCP: `finance_ledger_transactions` (optional `symbol`, `limit`, `db`)
   - CLI: `moneycraft-cli ledger-transactions --symbol SAP.DE --limit 50`

## 3. Broker & Bank CSV Import

- `finance_ledger_import` (CLI: `moneycraft-cli ledger-import <path> --venue flatex`) supports semicolon or comma delimiters, UTF-8 or Windows-1252 (`cp1252`), and German or English headers.
- Malformed or crypto rows are reported as `skipped` rather than half-written.
- **Duplicate import guard**: CSV import does not deduplicate identical rows across repeated runs on the same file. Always check `finance_ledger_transactions` before re-importing an overlapping date range, and back up `moneycraft-ledger.db` first.

## 4. Valuation, FX & Tax Estimation

- Full valuation with market prices (`yahoo` 15-min delayed default, `ecb`, or `synthetic`), unrealized PnL, realized YTD, and KESt estimate:
  - MCP: `finance_ledger_valuation` (optional `source`, `db`) or `finance_portfolio` (default DB + `yahoo`)
  - CLI: `moneycraft-cli ledger-valuation --source yahoo`
- Missing quotes produce `null` prices and a `valuation_error` / `no quote` flag rather than a fabricated price.
- For indicative offline currency conversion between the 20 majors, use `finance_fx_convert` (`amount`, `from`, `to`).
