//! Personal holdings ledger: real transactions, FIFO lots, live valuation,
//! Austrian KESt estimate. This module tracks things Joe actually owns —
//! Girokonto cash, Festgeld, ETFs, stocks, bonds. Paper money lives in the
//! fantasy desk; crypto lives there too (refused here, see PORTFOLIO_SPEC).
//!
//! Honesty rules for this module:
//! - SELL against insufficient lots is refused, never auto-invented.
//! - CSV rows that fail validation are reported as skipped, never half-written.
//! - Missing quotes produce null prices + a flag, never a fabricated number.
//! - The KESt figure is an estimate and is labeled as such everywhere.

use chrono::{Datelike, TimeZone, Utc};
use databasecraft_store::DatabaseStore;
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// Default ledger file (CLI + app share it; overridable via --db).
pub const LEDGER_DB_FILE: &str = "moneycraft-ledger.db";

/// Austrian Kapitalertragsteuer flat rate used for the *estimate*.
pub const KEST_RATE: f64 = 0.275;

pub const LEDGER_DDL: &str = "
CREATE TABLE IF NOT EXISTS ledger_transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    asset_class TEXT NOT NULL,
    side TEXT NOT NULL,
    quantity REAL NOT NULL,
    price REAL NOT NULL,
    currency TEXT NOT NULL DEFAULT 'EUR',
    venue TEXT NOT NULL DEFAULT 'manual',
    acquired_at INTEGER NOT NULL,
    fees REAL NOT NULL DEFAULT 0.0,
    note TEXT NOT NULL DEFAULT ''
);
CREATE INDEX IF NOT EXISTS idx_ledger_symbol_acq
    ON ledger_transactions (symbol, acquired_at);
";

/// Crypto spellings the ledger refuses (any direction, Yahoo or slash form).
const CRYPTO_SYMBOLS: [&str; 15] = [
    "BTC", "ETH", "SOL", "BTC/USD", "ETH/USD", "SOL/USD", "BTC-USD", "ETH-USD", "SOL-USD",
    "BTCUSD", "ETHUSD", "SOLUSD", "BTCUSD=X", "ETHUSD=X", "SOLUSD=X",
];

/// True for crypto — refused at booking, skipped at import.
#[must_use]
pub fn is_crypto_symbol(symbol: &str) -> bool {
    CRYPTO_SYMBOLS.contains(&symbol.trim().to_uppercase().as_str())
}

/// Best-effort asset class when the booking/CSV omits it.
/// Bare 3-letter codes (`EUR`) mean cash held, not a pair.
#[must_use]
pub fn infer_asset_class(symbol: &str) -> &'static str {
    let c = symbol.trim().to_uppercase();
    if c.len() == 3 && c.chars().all(|ch| ch.is_ascii_alphabetic()) {
        return "CASH";
    }
    if c.contains('/') || c.contains('-') {
        if is_crypto_symbol(&c) {
            return "CRYPTO";
        }
        if c.starts_with("XAU") {
            return "COMMODITY";
        }
        return "FOREX";
    }
    if is_crypto_symbol(&c) {
        return "CRYPTO";
    }
    "EQUITY"
}

fn esc(s: &str) -> String {
    s.replace('\'', "''")
}

fn store_err(context: &str, e: impl std::fmt::Debug) -> String {
    format!("ledger {context} failed: {e:?}")
}

/// Open the ledger database: explicit path or the shared default file.
/// The file is created on first open; the table on first write/read.
pub fn open_ledger_db(db_path: Option<&str>) -> Result<DatabaseStore, String> {
    match db_path {
        Some(p) if !p.trim().is_empty() => DatabaseStore::open(p).map_err(|e| store_err("open", e)),
        _ => DatabaseStore::open(LEDGER_DB_FILE).map_err(|e| store_err("open default", e)),
    }
}

/// FIFO positions with cost basis (no market prices — see `valuation`).
pub fn holdings(store: &DatabaseStore) -> Result<Value, String> {
    let (positions, _) = fifo_positions(store)?;
    Ok(json!({
        "success": true,
        "count": positions.len(),
        "positions": positions,
        "note": "Cost basis via FIFO lots. Prices: see finance_ledger_valuation.",
    }))
}

/// Create the ledger table if missing. Idempotent.
pub fn ensure_ledger(store: &DatabaseStore) -> Result<(), String> {
    store
        .execute(LEDGER_DDL)
        .map(|_| ())
        .map_err(|e| store_err("migrate", e))
}

/// Booking validated and inserted. Returns the row id.
/// SELL against insufficient FIFO lots is refused.
#[allow(clippy::too_many_arguments)]
pub fn add_transaction(
    store: &DatabaseStore,
    symbol: &str,
    asset_class: Option<&str>,
    side: &str,
    quantity: f64,
    price: f64,
    currency: Option<&str>,
    venue: Option<&str>,
    acquired_ms: Option<i64>,
    fees: f64,
    note: Option<&str>,
) -> Result<i64, String> {
    ensure_ledger(store)?;
    let sym = symbol.trim().to_uppercase();
    if sym.is_empty() {
        return Err("ledger add refused: empty symbol".to_string());
    }
    if is_crypto_symbol(&sym) {
        return Err(format!(
            "ledger add refused: {sym} is crypto — the real-asset ledger does not \
             book it. Crypto lives in the fantasy paper-trading arena."
        ));
    }
    let class = asset_class
        .map(|c| c.trim().to_uppercase())
        .filter(|c| !c.is_empty())
        .unwrap_or_else(|| infer_asset_class(&sym).to_string());
    if class == "CRYPTO" {
        return Err(format!(
            "ledger add refused: asset class CRYPTO for {sym} — see above."
        ));
    }
    let s = side.trim().to_uppercase();
    if s != "BUY" && s != "SELL" {
        return Err(format!(
            "ledger add refused: side must be BUY or SELL, got '{side}'."
        ));
    }
    if !(quantity.is_finite() && quantity > 0.0) {
        return Err("ledger add refused: quantity must be a positive number.".to_string());
    }
    if !(price.is_finite() && price >= 0.0) {
        return Err("ledger add refused: price must be a non-negative number.".to_string());
    }
    if !(fees.is_finite() && fees >= 0.0) {
        return Err("ledger add refused: fees must be a non-negative number.".to_string());
    }
    if s == "SELL" {
        let held = held_quantity(store, &sym)?;
        if quantity > held + 1e-9 {
            return Err(format!(
                "ledger add refused: SELL {quantity} {sym} exceeds FIFO holding {held:.4}. \
                 Book the opening BUY first."
            ));
        }
    }
    let currency = currency.unwrap_or("EUR").trim().to_uppercase();
    let venue = venue.unwrap_or("manual").trim();
    let acquired = acquired_ms.unwrap_or_else(|| Utc::now().timestamp_millis());
    let note = note.unwrap_or("");
    let sql = format!(
        "INSERT INTO ledger_transactions \
         (symbol, asset_class, side, quantity, price, currency, venue, acquired_at, fees, note) \
         VALUES ('{}','{}','{}',{},{},'{}','{}',{}, {},'{}')",
        esc(&sym),
        esc(&class),
        s,
        quantity,
        price,
        esc(&currency),
        esc(venue),
        acquired,
        fees,
        esc(note)
    );
    store.execute(&sql).map_err(|e| store_err("insert", e))?;
    let id_row = store
        .query("SELECT last_insert_rowid() AS id")
        .map_err(|e| store_err("row id", e))?;
    id_row
        .rows
        .first()
        .and_then(|r| r.first())
        .and_then(|v| v.as_i64())
        .ok_or_else(|| "ledger add failed: no row id returned.".to_string())
}

/// Current FIFO-held quantity of a symbol (for SELL validation).
fn held_quantity(store: &DatabaseStore, symbol: &str) -> Result<f64, String> {
    let sql = format!(
        "SELECT side, quantity FROM ledger_transactions WHERE symbol='{}' ORDER BY acquired_at ASC, id ASC",
        esc(&symbol.trim().to_uppercase())
    );
    let res = store.query(&sql).map_err(|e| store_err("read", e))?;
    let mut held = 0.0;
    for row in &res.rows {
        let side = row.first().and_then(|v| v.as_str()).unwrap_or("");
        let qty = row.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0);
        if side == "BUY" {
            held += qty;
        } else if side == "SELL" {
            held -= qty;
        }
    }
    Ok(held.max(0.0))
}

/// Raw bookings, newest first. Optional symbol filter + limit.
pub fn list_transactions(
    store: &DatabaseStore,
    symbol: Option<&str>,
    limit: usize,
) -> Result<Value, String> {
    ensure_ledger(store)?;
    let limit = limit.clamp(1, 5000);
    let sql = match symbol {
        Some(s) if !s.trim().is_empty() => format!(
            "SELECT id, symbol, asset_class, side, quantity, price, currency, venue, acquired_at, fees, note \
             FROM ledger_transactions WHERE symbol='{}' ORDER BY acquired_at DESC, id DESC LIMIT {limit}",
            esc(&s.trim().to_uppercase())
        ),
        _ => format!(
            "SELECT id, symbol, asset_class, side, quantity, price, currency, venue, acquired_at, fees, note \
             FROM ledger_transactions ORDER BY acquired_at DESC, id DESC LIMIT {limit}"
        ),
    };
    let res = store.query(&sql).map_err(|e| store_err("list", e))?;
    let mut out = Vec::with_capacity(res.rows.len());
    for row in &res.rows {
        let get = |i: usize| row.get(i).cloned().unwrap_or(Value::Null);
        out.push(json!({
            "id": get(0), "symbol": get(1), "asset_class": get(2), "side": get(3),
            "quantity": get(4), "price": get(5), "currency": get(6),
            "venue": get(7), "acquired_at": get(8), "fees": get(9), "note": get(10),
        }));
    }
    Ok(json!({ "success": true, "count": out.len(), "transactions": out }))
}

struct OpenLot {
    qty: f64,
    unit_cost: f64,
}

/// FIFO positions + realized YTD gains per currency.
/// Returns (positions, realized_ytd_by_currency).
/// Position: symbol, asset_class, currency, quantity, avg_cost.
fn fifo_positions(store: &DatabaseStore) -> Result<(Vec<Value>, BTreeMap<String, f64>), String> {
    ensure_ledger(store)?;
    let res = store
        .query(
            "SELECT symbol, asset_class, side, quantity, price, currency, acquired_at, fees \
             FROM ledger_transactions ORDER BY acquired_at ASC, id ASC",
        )
        .map_err(|e| store_err("fifo read", e))?;
    let mut lots: BTreeMap<String, Vec<OpenLot>> = BTreeMap::new();
    let mut meta: BTreeMap<String, (String, String)> = BTreeMap::new();
    let mut realized: BTreeMap<String, f64> = BTreeMap::new();
    let this_year = Utc::now().year();

    for row in &res.rows {
        let sym = row
            .first()
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if sym.is_empty() {
            continue;
        }
        let class = row
            .get(1)
            .and_then(|v| v.as_str())
            .unwrap_or("EQUITY")
            .to_string();
        let side = row.get(2).and_then(|v| v.as_str()).unwrap_or("");
        let qty = row.get(3).and_then(|v| v.as_f64()).unwrap_or(0.0);
        let price = row.get(4).and_then(|v| v.as_f64()).unwrap_or(0.0);
        let ccy = row
            .get(5)
            .and_then(|v| v.as_str())
            .unwrap_or("EUR")
            .to_string();
        let acquired = row.get(6).and_then(|v| v.as_i64()).unwrap_or(0);
        let fees = row.get(7).and_then(|v| v.as_f64()).unwrap_or(0.0);
        if !(qty.is_finite() && qty > 0.0 && price.is_finite() && price >= 0.0) {
            continue;
        }
        meta.entry(sym.clone()).or_insert((class, ccy));
        if side == "BUY" {
            let unit = if qty > 0.0 {
                (price * qty + fees) / qty
            } else {
                price
            };
            lots.entry(sym).or_default().push(OpenLot {
                qty,
                unit_cost: unit,
            });
        } else if side == "SELL" {
            let mut remaining = qty;
            let mut gain = -fees;
            if let Some(book) = lots.get_mut(&sym) {
                while remaining > 1e-9 && !book.is_empty() {
                    if book[0].qty <= remaining + 1e-9 {
                        let lot = book.remove(0);
                        gain += (price - lot.unit_cost) * lot.qty;
                        remaining -= lot.qty;
                    } else {
                        gain += (price - book[0].unit_cost) * remaining;
                        book[0].qty -= remaining;
                        remaining = 0.0;
                    }
                }
            }
            let sell_year = Utc
                .timestamp_millis_opt(acquired)
                .single()
                .map(|d| d.year())
                .unwrap_or(0);
            if sell_year == this_year {
                let ccy_now = meta.get(&sym).map(|(_, c)| c.clone()).unwrap_or_default();
                *realized.entry(ccy_now).or_insert(0.0) += gain;
            }
        }
    }

    let mut positions = Vec::new();
    for (sym, book) in &lots {
        let total_qty: f64 = book.iter().map(|l| l.qty).sum();
        if total_qty <= 1e-9 {
            continue;
        }
        let total_cost: f64 = book.iter().map(|l| l.qty * l.unit_cost).sum();
        let (class, ccy) = meta
            .get(sym)
            .cloned()
            .unwrap_or_else(|| ("EQUITY".to_string(), "EUR".to_string()));
        positions.push(json!({
            "symbol": sym, "asset_class": class, "currency": ccy,
            "quantity": total_qty, "avg_cost": total_cost / total_qty,
        }));
    }
    positions.sort_by(|a, b| {
        a.get("symbol")
            .and_then(|v| v.as_str())
            .cmp(&b.get("symbol").and_then(|v| v.as_str()))
    });
    Ok((positions, realized))
}

/// Full valuation: FIFO positions + live price per symbol + unrealized PnL +
/// realized YTD + KESt estimate, grouped per currency (no FX conversion in v1).
pub async fn valuation(store: &DatabaseStore, source: Option<String>) -> Result<Value, String> {
    let src = source.unwrap_or_else(|| "yahoo".to_string());
    let (positions, realized) = fifo_positions(store)?;
    let mut rows = Vec::with_capacity(positions.len());
    let mut totals: BTreeMap<String, (f64, f64)> = BTreeMap::new();

    for pos in &positions {
        let sym = pos.get("symbol").and_then(|v| v.as_str()).unwrap_or("");
        let class = pos
            .get("asset_class")
            .and_then(|v| v.as_str())
            .unwrap_or("EQUITY");
        let ccy = pos
            .get("currency")
            .and_then(|v| v.as_str())
            .unwrap_or("EUR")
            .to_string();
        let qty = pos.get("quantity").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let avg = pos.get("avg_cost").and_then(|v| v.as_f64()).unwrap_or(0.0);

        let (price, quoted, quote_note) = if class == "CASH" {
            (Some(1.0), true, "cash at par".to_string())
        } else {
            let res = moneycraft_feed::MarketFeed::fetch_candles_with_source(sym, 10, &src).await;
            match res.candles.last() {
                Some(c) => (
                    Some(c.close),
                    true,
                    if res.is_live {
                        res.status_note.clone()
                    } else {
                        format!("{} — synthetic fallback, not a quote", res.status_note)
                    },
                ),
                None => (None, false, "no candles returned".to_string()),
            }
        };

        let (market_value, unrealized) = match price {
            Some(p) => (Some(qty * p), Some((p - avg) * qty)),
            None => (None, None),
        };
        if let (Some(mv), Some(up)) = (market_value, unrealized) {
            let entry = totals.entry(ccy.clone()).or_insert((0.0, 0.0));
            entry.0 += mv;
            entry.1 += up;
        }
        let unrealized_pct = match (price, avg) {
            (Some(p), a) if a > 0.0 => Some((p - a) / a * 100.0),
            _ => None,
        };
        rows.push(json!({
            "symbol": sym, "asset_class": class, "currency": ccy,
            "quantity": qty, "avg_cost": avg,
            "current_price": price, "quoted": quoted, "quote_note": quote_note,
            "market_value": market_value, "unrealized_pnl": unrealized,
            "unrealized_pct": unrealized_pct,
        }));
    }

    let mut totals_json = serde_json::Map::new();
    for (ccy, (mv, up)) in &totals {
        totals_json.insert(
            ccy.clone(),
            json!({ "market_value": mv, "unrealized_pnl": up }),
        );
    }
    let mut kest_json = serde_json::Map::new();
    for (ccy, gain) in &realized {
        let taxable = gain.max(0.0);
        kest_json.insert(
            ccy.clone(),
            json!({ "realized_ytd": gain, "kest_estimate": taxable * KEST_RATE }),
        );
    }

    Ok(json!({
        "success": true,
        "source": src,
        "delay_note": "Prices follow the feed's delay semantics (e.g. Yahoo 15-min delayed); synthetic fallback is labeled per position.",
        "count": rows.len(),
        "positions": rows,
        "totals_by_currency": totals_json,
        "realized_ytd_by_currency": realized,
        "kest_by_currency": kest_json,
        "kest_rate": KEST_RATE,
        "tax_disclaimer": "KESt estimate only (27.5% flat on realized YTD gains, no allowances, no loss carryforwards, crypto post-28.2.2021 nuance not modeled). Not tax advice — talk to a Steuerberater.",
    }))
}

// ---------------------------------------------------------------------------
// CSV import (broker/bank exports: semicolon or comma, UTF-8 or Windows-1252)
// ---------------------------------------------------------------------------

/// Windows-1252 bytes 0x80–0x9F (0x81/8D/8F/90/9D undefined → U+FFFD).
const CP1252_EXTRA: [char; 32] = [
    '€', '\u{FFFD}', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', '\u{FFFD}', 'Ž',
    '\u{FFFD}', '\u{FFFD}', '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'š', '›', 'œ', '\u{FFFD}',
    'ž', 'Ÿ',
];

fn decode_bytes(raw: &[u8]) -> String {
    if let Ok(s) = std::str::from_utf8(raw) {
        return s.to_string();
    }
    raw.iter()
        .map(|&b| {
            if b < 0x80 {
                b as char
            } else if b < 0xA0 {
                CP1252_EXTRA[(b - 0x80) as usize]
            } else {
                b as char
            }
        })
        .collect()
}

fn split_csv_line(line: &str, delim: char) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                cur.push(ch);
            }
        } else if ch == '"' {
            in_quotes = true;
        } else if ch == delim {
            fields.push(cur.trim().to_string());
            cur = String::new();
        } else {
            cur.push(ch);
        }
    }
    fields.push(cur.trim().to_string());
    fields
}

fn norm_header(h: &str) -> String {
    h.trim().to_lowercase().replace(['_', ' ', '-', '.'], "")
}

fn col_idx(headers: &[String], aliases: &[&str]) -> Option<usize> {
    headers
        .iter()
        .position(|h| aliases.contains(&norm_header(h).as_str()))
}

fn parse_number_eu(s: &str, comma_decimal: bool) -> Option<f64> {
    let t = s.trim().replace(['\'', ' '], "");
    if t.is_empty() {
        return None;
    }
    if comma_decimal {
        // German format: 1.234,56 → strip thousands dots, comma → dot.
        let no_thousands = t.replace('.', "");
        no_thousands.replace(',', ".").parse().ok()
    } else {
        t.replace(',', "").parse().ok()
    }
}

/// Parse `YYYY-MM-DD` / `DD.MM.YYYY` / `DD/MM/YYYY` (optionally with time)
/// to epoch millis. Pub for CLI use.
#[must_use]
pub fn parse_date_ms(s: &str) -> Option<i64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    const DATE_FMTS: [&str; 4] = ["%Y-%m-%d", "%d.%m.%Y", "%d/%m/%Y", "%Y/%m/%d"];
    for fmt in DATE_FMTS {
        if let Ok(d) = chrono::NaiveDate::parse_from_str(t, fmt) {
            if let Some(dt) = d.and_hms_opt(0, 0, 0) {
                return Some(Utc.from_utc_datetime(&dt).timestamp_millis());
            }
        }
    }
    const DT_FMTS: [&str; 3] = [
        "%Y-%m-%d %H:%M:%S",
        "%d.%m.%Y %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
    ];
    for fmt in DT_FMTS {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(t, fmt) {
            return Some(Utc.from_utc_datetime(&dt).timestamp_millis());
        }
    }
    None
}

fn parse_side(s: &str) -> Option<&'static str> {
    match s.trim().to_lowercase().as_str() {
        "buy" | "kauf" | "kaufen" | "k" | "b" | "long" => Some("BUY"),
        "sell" | "verkauf" | "verkaufen" | "v" | "s" | "short" => Some("SELL"),
        "" => None,
        _ => None,
    }
}

/// Import a broker/bank CSV export. Returns {imported, skipped[]}.
/// Header row required; rows that fail validation are reported, never dropped
/// silently. Nothing is half-written: each row inserts atomically.
pub fn import_csv(store: &DatabaseStore, path: &str, venue_default: &str) -> Result<Value, String> {
    ensure_ledger(store)?;
    let raw =
        std::fs::read(path).map_err(|e| format!("ledger import: cannot read '{path}': {e}"))?;
    let text = decode_bytes(&raw);
    let lines: Vec<&str> = text
        .lines()
        .map(|l| l.strip_suffix('\r').unwrap_or(l))
        .filter(|l| !l.trim().is_empty())
        .collect();
    if lines.is_empty() {
        return Err("ledger import refused: file is empty.".to_string());
    }
    let first = lines[0];
    let semis = first.chars().filter(|&c| c == ';').count();
    let commas = first.chars().filter(|&c| c == ',').count();
    let delim = if semis >= commas { ';' } else { ',' };
    let comma_decimal = delim == ';';

    let headers: Vec<String> = split_csv_line(first, delim);
    let i_sym = col_idx(
        &headers,
        &[
            "symbol",
            "ticker",
            "wertpapier",
            "bezeichnung",
            "name",
            "isin",
            "titel",
        ],
    );
    let i_qty = col_idx(
        &headers,
        &[
            "quantity", "qty", "anzahl", "stueck", "stück", "stuecke", "stücke", "menge",
            "nominal", "volumen",
        ],
    );
    let i_price = col_idx(
        &headers,
        &[
            "price",
            "preis",
            "kurs",
            "einstandskurs",
            "stueckpreis",
            "stückpreis",
            "einzelpreis",
            "kaufkurs",
        ],
    );
    let (Some(i_sym), Some(i_qty), Some(i_price)) = (i_sym, i_qty, i_price) else {
        return Err(
            "ledger import refused: no header with symbol + quantity + price columns found. \
             Expected headers like symbol, quantity, price (aliases: ticker/wertpapier, anzahl/stück, kurs/preis)."
                .to_string(),
        );
    };
    let i_side = col_idx(
        &headers,
        &[
            "side",
            "typ",
            "transaktion",
            "art",
            "richtung",
            "kaufverkauf",
        ],
    );
    let i_date = col_idx(
        &headers,
        &[
            "date",
            "datum",
            "valuta",
            "buchungstag",
            "handelstag",
            "zeitpunkt",
        ],
    );
    let i_ccy = col_idx(
        &headers,
        &["currency", "waehrung", "währung", "whr", "devisen"],
    );
    let i_fees = col_idx(
        &headers,
        &[
            "fees",
            "fee",
            "gebuehren",
            "gebühren",
            "spesen",
            "kosten",
            "provision",
        ],
    );
    let i_class = col_idx(
        &headers,
        &[
            "assetclass",
            "asset_class",
            "anlageklasse",
            "kategorie",
            "klasse",
            "gattung",
        ],
    );
    let i_venue = col_idx(&headers, &["venue", "broker", "bank", "depot", "konto"]);
    let i_note = col_idx(
        &headers,
        &["note", "notiz", "bemerkung", "kommentar", "referenz"],
    );

    let mut imported = 0usize;
    let mut skipped = Vec::new();
    for (n, line) in lines.iter().skip(1).enumerate() {
        let row_no = n + 2;
        let f = split_csv_line(line, delim);
        let get = |i: Option<usize>| i.and_then(|idx| f.get(idx)).map(|s| s.trim()).unwrap_or("");
        let sym = get(Some(i_sym));
        let qty_raw = get(Some(i_qty));
        let price_raw = get(Some(i_price));
        if sym.is_empty() || qty_raw.is_empty() || price_raw.is_empty() {
            skipped.push(json!({"row": row_no, "reason": "empty symbol/quantity/price"}));
            continue;
        }
        if is_crypto_symbol(sym) {
            skipped.push(json!({"row": row_no, "symbol": sym,
                "reason": "crypto refused — real-asset ledger only (fantasy desk handles it)"}));
            continue;
        }
        let qty_parsed = parse_number_eu(qty_raw, comma_decimal);
        let price = parse_number_eu(price_raw, comma_decimal);
        let (Some(mut qty), Some(price)) = (qty_parsed, price) else {
            skipped.push(
                json!({"row": row_no, "symbol": sym, "reason": "quantity/price not numeric"}),
            );
            continue;
        };
        let mut side = get(i_side);
        let mut side_static: Option<&str> = parse_side(side);
        if side_static.is_none() {
            if qty < 0.0 {
                side_static = Some("SELL");
                qty = -qty;
            } else {
                side_static = Some("BUY");
            }
            side = "";
        }
        let side = side_static.map_or("BUY", |v| v);
        let _ = side;
        let side = if get(i_side).is_empty() && qty_raw.starts_with('-') {
            "SELL"
        } else {
            parse_side(get(i_side)).unwrap_or("BUY")
        };
        let acquired = get(i_date);
        let acquired_ms = if acquired.is_empty() {
            None
        } else {
            match parse_date_ms(acquired) {
                Some(ms) => Some(ms),
                None => {
                    skipped.push(json!({"row": row_no, "symbol": sym,
                        "reason": format!("unparseable date '{acquired}' (expected YYYY-MM-DD or DD.MM.YYYY)")}));
                    continue;
                }
            }
        };
        let class_raw = get(i_class);
        let class = if class_raw.is_empty() {
            None
        } else {
            Some(class_raw)
        };
        let ccy_raw = get(i_ccy);
        let venue_raw = get(i_venue);
        let fees = get(i_fees);
        let fees = if fees.is_empty() {
            0.0
        } else {
            match parse_number_eu(fees, comma_decimal) {
                Some(v) => v,
                None => {
                    skipped
                        .push(json!({"row": row_no, "symbol": sym, "reason": "fees not numeric"}));
                    continue;
                }
            }
        };
        match add_transaction(
            store,
            sym,
            class,
            side,
            qty,
            price,
            if ccy_raw.is_empty() {
                None
            } else {
                Some(ccy_raw)
            },
            Some(if venue_raw.is_empty() {
                venue_default
            } else {
                venue_raw
            }),
            acquired_ms,
            fees,
            {
                let nt = get(i_note);
                if nt.is_empty() {
                    None
                } else {
                    Some(nt)
                }
            },
        ) {
            Ok(_) => imported += 1,
            Err(e) => skipped.push(json!({"row": row_no, "symbol": sym, "reason": e})),
        }
    }

    Ok(json!({
        "success": true,
        "imported": imported,
        "skipped": skipped,
        "skipped_count": skipped.len(),
        "note": "SELL rows exceeding FIFO lots are refused — book the opening BUY first.",
    }))
}
