# PORTFOLIO_SPEC — a real personal holdings ledger for Joe Schmoe

Status: **spec + build in progress.** Goal: turn the portfolio workspace from a
display case with mannequins (hardcoded $486k blob) into a ledger of things Joe
actually owns: his Girokonto balance, his Festgeld ladder, his ETF savings plan,
a few stocks, maybe some BTC — valued live, taxed honestly under Austrian
rules, importable from the CSVs his broker/bank already gives him.

Non-audience: trust-fund cockpit software at €1000/month. This is the
un-glamorous middle: positive net worth, no private banker, files his own
Arbeitnehmerveranlagung.

## 1. Data model (one table, FIFO lots)

`ledger_transactions` in the moneycraft store (SQLite via `DatabaseStore`):

| column | type | meaning |
|---|---|---|
| id | INTEGER PK | auto |
| symbol | TEXT | `AAPL`, `EUR/USD`… same universe as the terminal — **except crypto** (see §7) |
| asset_class | TEXT | `EQUITY`, `ETF`, `BOND`, `FOREX`, `CRYPTO`, `CASH`, `COMMODITY` |
| side | TEXT | `BUY` or `SELL` |
| quantity | REAL | units / coins / ounces |
| price | REAL | per-unit price in `currency` |
| currency | TEXT | `EUR`, `USD`… (no FX conversion in v1 — see §6) |
| venue | TEXT | free text: `flatex`, `George`, `Bitpanda`, `manual`… |
| acquired_at | INTEGER | epoch ms |
| fees | REAL | explicit costs, default 0 |
| note | TEXT | free text |

**No holdings table.** Holdings are derived: FIFO match sells against buys per
symbol. Remaining lots = position (quantity, cost basis). This makes cost basis
auditable and makes the tax number computable instead of decorative. Cash
(`CASH` class, e.g. `EUR` on the Girokonto) is a holding like any other: buys
are deposits, sells are withdrawals, price always 1.0.

## 2. Operations (dispatch commands)

- `LedgerAdd { symbol, asset_class, side, quantity, price, ... }` — one manual
  booking. Validates side/quantity/price; SELL against insufficient lots is
  refused (never invent negative inventory).
- `LedgerImportCsv { path, venue }` — parses a broker/bank CSV export
  (semicolon **or** comma, UTF-8 **or** Windows-1252, headers auto-detected:
  symbol/quantity/price/date). Every parsed row validated; the import reports
  `imported` + `skipped` with reasons. Nothing half-written: rows that fail
  validation are listed, never silently dropped.
- `LedgerHoldings { source }` — FIFO positions with cost basis per symbol.
- `LedgerTransactions { symbol?, limit }` — raw bookings, newest first.
- `LedgerValuation { source }` — holdings + live price per symbol from
  `MarketFeed` (same delayed/labeled semantics as the terminal) + unrealized
  PnL per position and total + realized PnL YTD + KESt estimate. One call
  feeds the whole portfolio workspace.

All five surface on CLI (`ledger-add`, `ledger-holdings`, …) and MCP
(`finance_ledger_*`). Descriptions carry the paper/delayed caveats.

## 3. Valuation & tax (Austrian rules, estimated)

- Current prices come from the same `MarketFeed` the terminal uses
  (default Yahoo 15-min delayed; source selectable). Prices are labeled with
  their delay — a portfolio valued on delayed quotes says so.
- **Realized PnL**: FIFO lots, sells matched oldest-first, fees added to cost
  basis. Year-to-date realized gains feed the tax line.
- **KESt estimate**: 27.5% flat on realized YTD gains (Austrian
  Kapitalertragsteuer on stocks/bonds/funds; crypto gains post-28.2.2021
  likewise 27.5% — nuance noted in the UI, not modeled per-asset in v1).
  Rendered as **ESTIMATE, not tax advice**, with the disclaimer inline.
- Unrealized PnL is informational only — never taxed, never added to realized.

## 4. App UI (portfolio workspace keeps its shape)

`fetch_portfolio` switches from `PortfolioSummary` (hardcoded blob, retired but
kept as offline fallback) to `LedgerValuation`. The existing grid
(symbol/class/qty/entry/current/PnL) renders real rows; empty state tells Joe
how to add his first position (manual entry docs + CSV import command) instead
of showing mannequins. Tax line shows the computed estimate + disclaimer.

## 5. Persistence

File-backed store, not in-memory: the app and CLI default to the same ledger
file (`moneycraft-ledger.db` next to the working directory, overridable via
`--db` / `MONEYCRAFT_LEDGER_DB`). In-memory stays available for tests and
demos. First run creates the table (`CREATE TABLE IF NOT EXISTS`) — no
migrations needed, no separate setup step for Joe.

## 6. The crypto exception

The ledger refuses `CRYPTO` holdings: `BTC`, `ETH`, `SOL` (in any spelling)
and asset class `CRYPTO` are rejected at booking time and skipped at import
with an explicit reason. Rationale: this ledger tracks rent money —
Girokonto, Festgeld, ETFs, stocks, bonds. The casino lives next door in the
fantasy paper-trading arena, where crypto belongs and where no real funds can
be harmed. Read-only crypto *quotes* stay in the feed (data is data); crypto
*ownership* has no place in a serious personal ledger.

## 7. Explicit non-goals (v1)

- No multi-currency conversion: books in mixed EUR/USD; valuation converts
  nothing, totals shown per currency. ECB-rate conversion is the defined
  follow-up (the ECB feed already exists).
- No sells-short, no leverage in the ledger (fantasy desk keeps those).
- No automatic bank sync (George/PSD2: no public API; CSV export is the
  import path — matches the earlier George-export thread).
- No tax filing output; estimate only, with disclaimer.
