# LIVE_TRADING_PLAN — from paper Bloomberg to real Bloomberg desk

Status: **plan only — nothing here is wired yet.** MoneyCraft today is a
paper-trading terminal: simulated fills, delayed or synthetic feeds, zero broker
connections. This document is the scaffold a forker follows to connect real
trading accounts across four rails — **stocks, forex, sovereign bonds, crypto** —
without rewriting the terminal.

> Dark joke, real warning: the point of this plan is a working desk, not a
> working fortune. Real levers, real losses. The safety rails in §5 are not
> optional — a desk without them is how a forker bankrupts himself with day
> trading, efficiently, before lunch.

## 1. What already exists (the seams)

- **One command enum** (`moneycraft-core::Command`): every terminal action is a
  `Command` variant handled by one `dispatch()` fn. Live trading = new
  `Broker*` variants + arms. No UI rewrite.
- **Paper desk that computes like a broker**: `PaperOpenPosition` already does
  margin, leverage clamp (1–50x), and liquidation-price math. A broker adapter
  replaces the math with a venue call and keeps the shape.
- **Honest refusals as precedent**: `PlaceOrder` refuses instead of faking a
  fill; `PaperClosePosition` reports 0.0 instead of a fabricated PnL. Live
  verbs follow the same rule in reverse — never report what the venue didn't
  confirm.
- **Feed abstraction** (`moneycraft-feed::MarketFeed`): live *quotes* already
  flow for crypto (Binance/Hyperliquid), delayed for everything else. Orders
  are the missing half, not data.

## 2. Architecture: one new crate

New crate **`moneycraft-broker`** (sibling of core/feed/mcp/app):

```rust
pub trait Broker {
    fn venue(&self) -> &'static str;               // "oanda", "alpaca", "ibkr", "kraken" ...
    fn account(&self) -> Result<Account, BrokerError>;
    fn positions(&self) -> Result<Vec<Position>, BrokerError>;
    fn place(&self, order: NewOrder) -> Result<OrderAck, BrokerError>;
    fn cancel(&self, venue_order_id: &str) -> Result<(), BrokerError>;
}
```

- One adapter struct per venue, behind **cargo features**
  (`broker-oanda`, `broker-alpaca`, `broker-ibkr`, `broker-kraken`).
  Default build = no adapters = paper-only binary. You cannot accidentally
  compile in live trading.
- `NewOrder` carries a `dry_run: bool`; every adapter must implement dry-run
  (validate + return the would-be order, send nothing).
- `dispatch()` gains `BrokerAccount/BrokerPositions/BrokerPlace/BrokerCancel`
  variants that take a `broker: Option<String>` venue selector and fall back
  to the paper desk when absent — so CLI/MCP/UI keep working with zero flags.
- Secrets: env vars only (`OANDA_API_KEY`, `ALPACA_KEY_ID`, …), same rule as
  the Stripe module — status reports show test/live, never the key.

## 3. Venue matrix (one rail at a time)

| Rail | First venue (scaffold target) | Why this one | Live keys |
|---|---|---|---|
| Forex | **OANDA v20** (practice → live, same API) | Cleanest retail FX REST API; practice server behaves like live; EUR/USD spreads usable | `OANDA_API_KEY`, `OANDA_ACCOUNT_ID`, practice vs live host |
| Stocks (US) | **Alpaca** (paper → live, same API) | Paper/live toggle is one endpoint swap; fractional US equities; free market data good enough to start | `ALPACA_KEY_ID`, `ALPACA_SECRET`, paper vs live base URL |
| Stocks + forex + bonds (EU/universal) | **IBKR Client Portal API** | One account covers US/EU stocks, spot FX, and **sovereign bonds** (Bunds, T-Notes, Gilts via bond desk); runs against local Gateway, no TWS GUI needed | Gateway container + OAuth-less local auth; steepest setup of the four |
| Crypto | **Kraken spot** first, **Hyperliquid perps** second | Kraken: EU-friendly, sane REST, low junk; Hyperliquid: the perps the derivatives desk already mirrors (builder-fee model) | `KRAKEN_KEY/SECRET`, Hyperliquid agent private key |
| Sovereign bonds (retail EU) | **via IBKR bond desk** | No good standalone retail bond API in the EU (Flatex & co. have none public); IBKR is the pragmatic single pipe. Until then: bonds stay read-only (ECB yields + indicative pricing). | Same IBKR account |

Deliberately **not** in v1: options (exercise/assignment complexity), CFDs
(counterparty risk + EU regulatory mess), TradFi real-time data entitlements
(payola matrix stays as-is; live *quotes* for trading come free with the
broker).

## 4. Order of work (each step shippable)

1. **Broker crate + trait + paper adapter.** Move the `Paper*` math behind
   `Broker` as `PaperBroker`. Zero behavior change; proves the seam.
2. **OANDA practice end-to-end.** `finance_broker_place --broker oanda --dry-run`
   → real validation, nothing sent. Then one €1 test trade on practice.
   This step *is* the template every later venue copies.
3. **Alpaca US stocks** (paper endpoint first, same code path to live).
4. **IBKR Gateway in Docker** → stocks + FX + bond *quotes*; orders behind
   the §5 confirm gate.
5. **Crypto live** (Kraken spot; Hyperliquid perps after a full practice round
   on testnet).
6. **Sovereign bonds live** via IBKR; EU retail bond UX stays indicative until
   a forker brings a venue with an API.

Rough sizing per the repo owner's rule of thumb: step 1–2 is the "hour of
Opus or Argon" once this plan is the prompt — one focused agent, one venue,
practice endpoint, dry-run first. Steps 3–6 repeat the template.

## 5. Safety rails (mandatory, not decorative)

- **Paper is the default.** Live requires *all three*: `--broker <venue>`
  flag + `LIVE_TRADING=1` env + typing the venue name as confirmation.
  Anything less routes to the paper desk with a logged warning.
- **Caps file** (`broker.toml`, never committed with keys): max notional per
  order, max daily loss (kill-switch → flatten + lock for the day),
  leverage ceiling below the venue's, allow-listed symbols only.
- **Kill-switch**: one command (`finance_broker_kill`) cancels all open
  orders at every configured venue. Tested against practice endpoints.
- **Audit log**: every order *intent*, venue ack, fill, cancel, and refusal
  appended to a local append-only log (databasecraft store). The ledger the
  tax page reads must reconcile with this log.
- **Fixture-recorded tests**: same honesty pattern as the Stripe module —
  recorded venue responses in `tests/fixtures/broker/<venue>/`, replayed in
  CI. No live keys in CI, ever.
- **Key hygiene**: env-only, `.env.example` documents names, status commands
  print test/live, never key material.

## 6. What changes in the existing code (small)

- `core`: 4 new `Command` variants + arms (~80 lines, same shape as `Paper*`).
- `moneycraft-mcp`: 4 CLI subcommands + 4 MCP tools (`finance_broker_*`),
  each with `--broker`/`--dry-run` flags and paper-only wording until a
  venue is configured.
- `moneycraft-app`: terminal gets a broker selector next to the feed selector
  + a red LIVE badge when any live venue is armed (theme-aware red, both
  themes). Paper UI untouched.
- `README` + in-app Help honesty sections flip from "no broker connection"
  to per-venue status ("OANDA practice armed", "Alpaca live armed"…).

## 7. Explicit non-goals

- No HFT, no colocation, no microwave towers. This is a human-driven desk.
- No options exercise/assignment handling, no margin-call automation beyond
  the kill-switch.
- No custody: crypto stays on-venue; no wallet management in v1.
- No advice: the AI workflows stay illustrative; wiring them to auto-trade
  is a separate plan with its own kill-switches (not this one).
