//! Stripe billing mirror: fetch from the Stripe API (live, needs a merchant
//! key) or from recorded JSON fixtures (offline, deterministic), mirror into
//! the `billing` blueprint tables, and reconcile mirror vs API.
//!
//! Rules enforced here, not in callers:
//! - never read, log, or return a secret key (only test/live/not-configured);
//! - never invent data: without fixtures and without a key, every command
//!   says so and stops (no silent zeros, no fake greens);
//! - money-moving verbs (checkout create, cancel) are NOT here — v1 is
//!   read + reconcile only. See STRIPE_BILLING_SPEC.md.

use databasecraft_core::blueprints::construct_blueprint;
use databasecraft_store::DatabaseStore;
use serde_json::{json, Value};

pub const STRIPE_API_BASE_DEFAULT: &str = "https://api.stripe.com";
const HTTP_TIMEOUT_SECS: u64 = 10;
const FETCH_LIMIT: u8 = 100;

pub const RESOURCES: [&str; 5] = [
    "customers",
    "products",
    "prices",
    "subscriptions",
    "invoices",
];

pub struct StripeConfig {
    pub secret_key: String,
    pub base_url: String,
}

/// Reads STRIPE_SECRET_KEY (+ optional STRIPE_API_BASE). Returns None when
/// no key is configured — callers must report that state, never fake data.
pub fn config_from_env() -> Option<StripeConfig> {
    let key = std::env::var("STRIPE_SECRET_KEY").ok()?;
    let key = key.trim().to_string();
    if key.is_empty() {
        return None;
    }
    let base = std::env::var("STRIPE_API_BASE")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| STRIPE_API_BASE_DEFAULT.to_string());
    Some(StripeConfig {
        secret_key: key,
        base_url: base,
    })
}

fn mode_for_key(key: &str) -> &'static str {
    if key.starts_with("sk_test_") || key.starts_with("rk_test_") {
        "test"
    } else if key.starts_with("sk_live_") {
        "live"
    } else {
        "unknown"
    }
}

/// Pure status report over key presence/mode — no env access, fully testable.
pub fn status_report(key_present: bool, key_mode: &str, base_url: &str) -> Value {
    json!({
        "success": true,
        "configured": key_present,
        "mode": key_mode,
        "base_url": base_url,
        "write_verbs": "not implemented in v1 (read + reconcile only)",
        "note": if key_present {
            "ready for live sync (token-gated, merchant keys only)"
        } else {
            "no STRIPE_SECRET_KEY in env — live sync disabled; fixtures only"
        },
    })
}

pub fn status_from_env() -> Value {
    match config_from_env() {
        Some(cfg) => {
            let mode = mode_for_key(&cfg.secret_key).to_string();
            status_report(true, &mode, &cfg.base_url)
        }
        None => status_report(false, "none", STRIPE_API_BASE_DEFAULT),
    }
}

// ---------------------------------------------------------------------------
// HTTP layer (live path only; never touched when fixtures are given)
// ---------------------------------------------------------------------------

fn http_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(HTTP_TIMEOUT_SECS))
        .build()
        .map_err(|e| format!("HTTP client build failed: {e}"))
}

async fn fetch_list(
    client: &reqwest::Client,
    cfg: &StripeConfig,
    resource: &str,
) -> Result<Value, String> {
    let url = format!("{}/v1/{}?limit={}", cfg.base_url, resource, FETCH_LIMIT);
    let resp = client
        .get(&url)
        .bearer_auth(&cfg.secret_key)
        .send()
        .await
        .map_err(|e| format!("Stripe request failed ({resource}): {e}"))?;
    if !resp.status().is_success() {
        return Err(format!(
            "Stripe returned HTTP {} for {resource} (check key, mode, permissions)",
            resp.status()
        ));
    }
    resp.json::<Value>()
        .await
        .map_err(|e| format!("Stripe parse failed ({resource}): {e}"))
}

// ---------------------------------------------------------------------------
// Normalized row shapes (shared by live parse + fixtures)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct CustomerRow {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub brand: Option<String>,
    pub last4: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProductRow {
    pub id: String,
    pub name: String,
    pub desc: Option<String>,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PriceRow {
    pub id: String,
    pub product: String,
    pub nickname: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub interval: Option<String>,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubRow {
    pub id: String,
    pub customer: String,
    pub price: String,
    pub status: String,
    pub period_start: Option<i64>,
    pub period_end: Option<i64>,
    pub trial_end: Option<i64>,
    pub cancel_at_end: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InvoiceRow {
    pub id: String,
    pub customer: String,
    pub subscription: Option<String>,
    pub url: Option<String>,
    pub status: String,
    pub amount_due: i64,
    pub amount_paid: i64,
    pub currency: String,
}

fn s(v: &Value) -> String {
    v.as_str().unwrap_or("").to_string()
}

fn opt_s(v: &Value) -> Option<String> {
    match v {
        Value::Null => None,
        Value::String(x) if x.is_empty() => None,
        Value::String(x) => Some(x.clone()),
        _ => None,
    }
}

fn opt_i(v: &Value) -> Option<i64> {
    v.as_i64()
}

fn flag(v: &Value) -> bool {
    v.as_bool().unwrap_or(false)
}

fn data_items(v: &Value) -> Vec<&Value> {
    v.get("data")
        .and_then(|d| d.as_array())
        .map(|a| a.iter().collect())
        .unwrap_or_default()
}

pub fn parse_customers(v: &Value) -> Vec<CustomerRow> {
    data_items(v)
        .iter()
        .filter_map(|o| {
            let id = o.get("id")?.as_str()?.to_string();
            let card = o.get("default_source").filter(|d| d.is_object());
            Some(CustomerRow {
                id,
                email: s(&o["email"]),
                name: opt_s(&o["name"]),
                brand: card
                    .and_then(|c| c.get("card"))
                    .and_then(|c| c.get("brand"))
                    .and_then(|b| b.as_str())
                    .map(|b| b.to_string())
                    .or_else(|| opt_s(&o["default_pm_brand"])),
                last4: card
                    .and_then(|c| c.get("card"))
                    .and_then(|c| c.get("last4"))
                    .and_then(|l| l.as_str())
                    .map(|l| l.to_string())
                    .or_else(|| opt_s(&o["default_pm_last4"])),
            })
        })
        .collect()
}

pub fn parse_products(v: &Value) -> Vec<ProductRow> {
    data_items(v)
        .iter()
        .filter_map(|o| {
            Some(ProductRow {
                id: o.get("id")?.as_str()?.to_string(),
                name: s(&o["name"]),
                desc: opt_s(&o["description"]),
                active: o.get("active").and_then(|a| a.as_bool()).unwrap_or(true),
            })
        })
        .collect()
}

pub fn parse_prices(v: &Value) -> Vec<PriceRow> {
    data_items(v)
        .iter()
        .filter_map(|o| {
            Some(PriceRow {
                id: o.get("id")?.as_str()?.to_string(),
                product: s(&o["product"]),
                nickname: opt_s(&o["nickname"]),
                amount: o.get("unit_amount").and_then(|a| a.as_i64()).unwrap_or(0),
                currency: o
                    .get("currency")
                    .and_then(|c| c.as_str())
                    .unwrap_or("eur")
                    .to_string(),
                interval: o
                    .get("recurring")
                    .and_then(|r| r.get("interval"))
                    .and_then(|i| i.as_str())
                    .map(|i| i.to_string()),
                active: o.get("active").and_then(|a| a.as_bool()).unwrap_or(true),
            })
        })
        .collect()
}

pub fn parse_subscriptions(v: &Value) -> Vec<SubRow> {
    data_items(v)
        .iter()
        .filter_map(|o| {
            let price = o
                .get("items")
                .and_then(|i| i.get("data"))
                .and_then(|d| d.as_array())
                .and_then(|a| a.first())
                .and_then(|it| it.get("price"))
                .and_then(|p| p.as_str())
                .unwrap_or("")
                .to_string();
            Some(SubRow {
                id: o.get("id")?.as_str()?.to_string(),
                customer: s(&o["customer"]),
                price,
                status: o
                    .get("status")
                    .and_then(|s| s.as_str())
                    .unwrap_or("unknown")
                    .to_string(),
                period_start: opt_i(&o["current_period_start"]),
                period_end: opt_i(&o["current_period_end"]),
                trial_end: opt_i(&o["trial_end"]),
                cancel_at_end: flag(&o["cancel_at_period_end"]),
            })
        })
        .collect()
}

pub fn parse_invoices(v: &Value) -> Vec<InvoiceRow> {
    data_items(v)
        .iter()
        .filter_map(|o| {
            Some(InvoiceRow {
                id: o.get("id")?.as_str()?.to_string(),
                customer: o
                    .get("customer")
                    .and_then(|c| match c {
                        Value::String(s) => Some(s.clone()),
                        Value::Object(_) => c.get("id").and_then(|i| i.as_str()).map(|i| i.to_string()),
                        _ => None,
                    })
                    .unwrap_or_default(),
                subscription: opt_s(&o["subscription"]),
                url: opt_s(&o["hosted_invoice_url"]),
                status: o
                    .get("status")
                    .and_then(|s| s.as_str())
                    .unwrap_or("unknown")
                    .to_string(),
                amount_due: o.get("amount_due").and_then(|a| a.as_i64()).unwrap_or(0),
                amount_paid: o.get("amount_paid").and_then(|a| a.as_i64()).unwrap_or(0),
                currency: o
                    .get("currency")
                    .and_then(|c| c.as_str())
                    .unwrap_or("eur")
                    .to_string(),
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// SQL helpers (store.execute takes raw SQL — escape everything from outside)
// ---------------------------------------------------------------------------

fn esc(s: &str) -> String {
    s.replace('\'', "''")
}

fn txt(s: &str) -> String {
    format!("'{}'", esc(s))
}

fn optxt(o: &Option<String>) -> String {
    match o {
        Some(v) => txt(v),
        None => "NULL".to_string(),
    }
}

fn opti(o: &Option<i64>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => "NULL".to_string(),
    }
}

fn flag_sql(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}

fn mirror_customers(store: &DatabaseStore, rows: &[CustomerRow]) -> Result<usize, String> {
    let mut n = 0;
    for r in rows {
        store
            .execute(&format!(
                "INSERT INTO stripe_customers (stripe_customer_id, email, full_name, default_pm_brand, default_pm_last4) VALUES ({},{},{},{},{}) ON CONFLICT(stripe_customer_id) DO UPDATE SET email=excluded.email, full_name=excluded.full_name, default_pm_brand=excluded.default_pm_brand, default_pm_last4=excluded.default_pm_last4",
                txt(&r.id), txt(&r.email), optxt(&r.name), optxt(&r.brand), optxt(&r.last4)
            ))
            .map_err(|e| format!("mirror customers failed: {e}"))?;
        n += 1;
    }
    Ok(n)
}

fn mirror_products(store: &DatabaseStore, rows: &[ProductRow]) -> Result<usize, String> {
    let mut n = 0;
    for r in rows {
        store
            .execute(&format!(
                "INSERT INTO stripe_products (stripe_product_id, name, description, active) VALUES ({},{},{},{}) ON CONFLICT(stripe_product_id) DO UPDATE SET name=excluded.name, description=excluded.description, active=excluded.active",
                txt(&r.id), txt(&r.name), optxt(&r.desc), if r.active { 1 } else { 0 }
            ))
            .map_err(|e| format!("mirror products failed: {e}"))?;
        n += 1;
    }
    Ok(n)
}

fn mirror_prices(store: &DatabaseStore, rows: &[PriceRow]) -> Result<usize, String> {
    let mut n = 0;
    for r in rows {
        store
            .execute(&format!(
                "INSERT INTO stripe_prices (stripe_price_id, stripe_product_id, nickname, unit_amount, currency, recurring_interval, active) VALUES ({},{},{},{},{},{},{}) ON CONFLICT(stripe_price_id) DO UPDATE SET nickname=excluded.nickname, unit_amount=excluded.unit_amount, currency=excluded.currency, recurring_interval=excluded.recurring_interval, active=excluded.active",
                txt(&r.id), txt(&r.product), optxt(&r.nickname), r.amount, txt(&r.currency), optxt(&r.interval), if r.active { 1 } else { 0 }
            ))
            .map_err(|e| format!("mirror prices failed: {e}"))?;
        n += 1;
    }
    Ok(n)
}

fn mirror_subscriptions(store: &DatabaseStore, rows: &[SubRow]) -> Result<usize, String> {
    let mut n = 0;
    for r in rows {
        store
            .execute(&format!(
                "INSERT INTO subscriptions (stripe_subscription_id, stripe_customer_id, stripe_price_id, status, current_period_start, current_period_end, trial_end, cancel_at_period_end) VALUES ({},{},{},{},{},{},{},{}) ON CONFLICT(stripe_subscription_id) DO UPDATE SET stripe_customer_id=excluded.stripe_customer_id, stripe_price_id=excluded.stripe_price_id, status=excluded.status, current_period_start=excluded.current_period_start, current_period_end=excluded.current_period_end, trial_end=excluded.trial_end, cancel_at_period_end=excluded.cancel_at_period_end",
                txt(&r.id), txt(&r.customer), txt(&r.price), txt(&r.status), opti(&r.period_start), opti(&r.period_end), opti(&r.trial_end), flag_sql(r.cancel_at_end)
            ))
            .map_err(|e| format!("mirror subscriptions failed: {e}"))?;
        n += 1;
    }
    Ok(n)
}

fn mirror_invoices(store: &DatabaseStore, rows: &[InvoiceRow]) -> Result<usize, String> {
    let mut n = 0;
    for r in rows {
        store
            .execute(&format!(
                "INSERT INTO stripe_invoices (stripe_invoice_id, stripe_customer_id, stripe_subscription_id, hosted_invoice_url, status, amount_due, amount_paid, currency) VALUES ({},{},{},{},{},{},{},{}) ON CONFLICT(stripe_invoice_id) DO UPDATE SET stripe_customer_id=excluded.stripe_customer_id, stripe_subscription_id=excluded.stripe_subscription_id, hosted_invoice_url=excluded.hosted_invoice_url, status=excluded.status, amount_due=excluded.amount_due, amount_paid=excluded.amount_paid, currency=excluded.currency",
                txt(&r.id), txt(&r.customer), optxt(&r.subscription), optxt(&r.url), txt(&r.status), r.amount_due, r.amount_paid, txt(&r.currency)
            ))
            .map_err(|e| format!("mirror invoices failed: {e}"))?;
        n += 1;
    }
    Ok(n)
}

// ---------------------------------------------------------------------------
// Fixtures (offline path — recorded sanitized shapes, no network)
// ---------------------------------------------------------------------------

fn read_fixture(dir: &str, name: &str) -> Result<Value, String> {
    let text = std::fs::read_to_string(format!("{dir}/{name}.json"))
        .map_err(|e| format!("fixture {name}.json unreadable in {dir}: {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("fixture {name}.json invalid: {e}"))
}

// ---------------------------------------------------------------------------
// Orchestration (shared by CLI, MCP, tests)
// ---------------------------------------------------------------------------

pub struct MirrorCounts {
    pub source: String,
    pub fetched: usize,
    pub upserted: usize,
}

fn open_mirror_store(db_path: Option<&str>) -> Result<DatabaseStore, String> {
    match db_path {
        Some(p) => DatabaseStore::open(p).map_err(|e| format!("cannot open db at {p}: {e}")),
        None => DatabaseStore::open_in_memory().map_err(|e| format!("cannot open memory store: {e}")),
    }
}

fn table_exists(store: &DatabaseStore, name: &str) -> bool {
    match store.query(&format!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='{name}'"
    )) {
        Ok(res) => res.row_count > 0,
        Err(_) => false,
    }
}

fn ensure_billing_schema(store: &DatabaseStore) -> Result<bool, String> {
    if table_exists(store, "stripe_customers") {
        return Ok(false);
    }
    construct_blueprint(store, "billing", false, false)
        .map_err(|e| format!("billing schema setup failed: {e}"))?;
    Ok(true)
}

async fn load_resource(
    client: &reqwest::Client,
    cfg: &Option<StripeConfig>,
    fixtures_dir: &Option<String>,
    resource: &str,
) -> Result<(Value, String), String> {
    if let Some(dir) = fixtures_dir {
        return Ok((read_fixture(dir, resource)?, "fixtures".to_string()));
    }
    match cfg {
        Some(c) => Ok((fetch_list(client, c, resource).await?, "api".to_string())),
        None => Err("no STRIPE_SECRET_KEY in env and no --fixtures given — nothing to sync from (refusing to invent data)".to_string()),
    }
}

/// Sync one resource (or all when `resource` is None) into the billing tables.
/// Without `db_path` the mirror lives in memory and is reported, not persisted.
pub async fn sync_command(
    resource: Option<String>,
    db_path: Option<String>,
    fixtures_dir: Option<String>,
) -> Value {
    let wanted: Vec<String> = match resource {
        Some(r) => {
            if !RESOURCES.contains(&r.as_str()) {
                return json!({"success": false, "error": format!("unknown resource '{r} — expected one of: {}", RESOURCES.join(", "))});
            }
            vec![r]
        }
        None => RESOURCES.iter().map(|s| s.to_string()).collect(),
    };
    let client = match http_client() {
        Ok(c) => c,
        Err(e) => return json!({"success": false, "error": e}),
    };
    let cfg = config_from_env();
    let store = match open_mirror_store(db_path.as_deref()) {
        Ok(s) => s,
        Err(e) => return json!({"success": false, "error": e}),
    };
    if let Err(e) = ensure_billing_schema(&store) {
        return json!({"success": false, "error": e});
    }
    let mut detail = serde_json::Map::new();
    let mut total_upserted = 0;
    for res in &wanted {
        let (raw, source) = match load_resource(&client, &cfg, &fixtures_dir, res).await {
            Ok(v) => v,
            Err(e) => return json!({"success": false, "resource": res, "error": e}),
        };
        let data: Vec<&Value> = raw
            .get("data")
            .and_then(|d| d.as_array())
            .map(|a| a.iter().collect())
            .unwrap_or_default();
        let upserted = match res.as_str() {
            "customers" => mirror_customers(&store, &parse_customers(&raw)),
            "products" => mirror_products(&store, &parse_products(&raw)),
            "prices" => mirror_prices(&store, &parse_prices(&raw)),
            "subscriptions" => mirror_subscriptions(&store, &parse_subscriptions(&raw)),
            "invoices" => mirror_invoices(&store, &parse_invoices(&raw)),
            _ => Ok(0),
        };
        match upserted {
            Ok(n) => {
                total_upserted += n;
                detail.insert(
                    res.clone(),
                    json!({"source": source, "fetched": data.len(), "upserted": n}),
                );
            }
            Err(e) => return json!({"success": false, "resource": res, "error": e}),
        }
    }
    json!({
        "success": true,
        "resources": detail,
        "total_upserted": total_upserted,
        "persisted": db_path.is_some(),
        "persist_note": if db_path.is_some() { "mirrored into billing tables" } else { "in-memory only — pass --db to persist" },
    })
}

/// Subscription list + status rollup. Reads the mirror when a db is given,
/// otherwise parses fixtures/live directly (no persistence).
pub async fn subscriptions_command(
    db_path: Option<String>,
    fixtures_dir: Option<String>,
) -> Value {
    if let Some(db) = db_path {
        let store = match open_mirror_store(Some(db.as_str())) {
            Ok(s) => s,
            Err(e) => return json!({"success": false, "error": e}),
        };
        if !table_exists(&store, "subscriptions") {
            return json!({"success": false, "error": "no billing tables — run sync first"});
        }
        match store.query("SELECT stripe_subscription_id, stripe_customer_id, stripe_price_id, status, current_period_end, cancel_at_period_end FROM subscriptions ORDER BY id") {
            Ok(res) => {
                let rows = res.rows;
                let mut rollup = std::collections::HashMap::new();
                let mut list = Vec::new();
                for r in &rows {
                    let get = |i: usize| r.get(i).and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let status = r.get(3).and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
                    *rollup.entry(status.clone()).or_insert(0) += 1;
                    list.push(json!({
                        "id": r.first().and_then(|v| v.as_str()).unwrap_or(""),
                        "customer": get(1),
                        "price": get(2),
                        "status": status,
                        "period_end": r.get(4).and_then(|v| v.as_i64()),
                        "cancel_at_period_end": get(5) == "1",
                    }));
                }
                return json!({"success": true, "source": "mirror", "count": list.len(), "by_status": rollup, "subscriptions": list});
            }
            Err(e) => return json!({"success": false, "error": format!("mirror read failed: {e}")}),
        }
    }
    let client = match http_client() {
        Ok(c) => c,
        Err(e) => return json!({"success": false, "error": e}),
    };
    let cfg = config_from_env();
    let (raw, source) = match load_resource(&client, &cfg, &fixtures_dir, "subscriptions").await {
        Ok(v) => v,
        Err(e) => return json!({"success": false, "error": e}),
    };
    let subs = parse_subscriptions(&raw);
    let mut rollup = std::collections::HashMap::new();
    let list: Vec<Value> = subs
        .iter()
        .map(|s| {
            *rollup.entry(s.status.clone()).or_insert(0) += 1;
            json!({"id": s.id, "customer": s.customer, "price": s.price, "status": s.status, "period_end": s.period_end, "cancel_at_period_end": s.cancel_at_end})
        })
        .collect();
    json!({"success": true, "source": source, "count": list.len(), "by_status": rollup, "subscriptions": list})
}

/// Reconcile the local mirror against fresh data (fixtures or live API).
/// Reports missing-each-way plus field drift. Never mutates either side.
pub async fn reconcile_command(
    db_path: Option<String>,
    fixtures_dir: Option<String>,
    fixtures_base: Option<String>,
) -> Value {
    let store = match open_mirror_store(db_path.as_deref()) {
        Ok(s) => s,
        Err(e) => return json!({"success": false, "error": e}),
    };
    if let Err(e) = ensure_billing_schema(&store) {
        return json!({"success": false, "error": e});
    }
    // Optional baseline: mirror a fixture set first (pure in-memory flows/tests).
    if let Some(base) = fixtures_base {
        for res in RESOURCES {
            let raw = match read_fixture(&base, res) {
                Ok(v) => v,
                Err(e) => return json!({"success": false, "error": e}),
            };
            let r = match res {
                "customers" => mirror_customers(&store, &parse_customers(&raw)),
                "products" => mirror_products(&store, &parse_products(&raw)),
                "prices" => mirror_prices(&store, &parse_prices(&raw)),
                "subscriptions" => mirror_subscriptions(&store, &parse_subscriptions(&raw)),
                "invoices" => mirror_invoices(&store, &parse_invoices(&raw)),
                _ => Ok(0),
            };
            if let Err(e) = r {
                return json!({"success": false, "error": e});
            }
        }
    }
    let client = match http_client() {
        Ok(c) => c,
        Err(e) => return json!({"success": false, "error": e}),
    };
    let cfg = config_from_env();
    // Fresh side: fixtures dir or live API (needs key).
    if fixtures_dir.is_none() && cfg.is_none() {
        return json!({"success": false, "error": "nothing fresh to compare against — pass --fixtures or set STRIPE_SECRET_KEY"});
    }
    let mut fresh_subs = Vec::new();
    let mut fresh_invs = Vec::new();
    if let Some(dir) = &fixtures_dir {
        match read_fixture(dir, "subscriptions") {
            Ok(v) => fresh_subs = parse_subscriptions(&v),
            Err(e) => return json!({"success": false, "error": e}),
        }
        match read_fixture(dir, "invoices") {
            Ok(v) => fresh_invs = parse_invoices(&v),
            Err(e) => return json!({"success": false, "error": e}),
        }
    } else if let Some(c) = cfg {
        match fetch_list(&client, &c, "subscriptions").await {
            Ok(v) => fresh_subs = parse_subscriptions(&v),
            Err(e) => return json!({"success": false, "error": e}),
        }
        match fetch_list(&client, &c, "invoices").await {
            Ok(v) => fresh_invs = parse_invoices(&v),
            Err(e) => return json!({"success": false, "error": e}),
        }
    }
    compare_mirror_vs_fresh(&store, &fresh_subs, &fresh_invs)
}

fn read_subs(store: &DatabaseStore) -> Result<Vec<SubRow>, String> {
    let res = store
        .query("SELECT stripe_subscription_id, stripe_customer_id, stripe_price_id, status, current_period_start, current_period_end, trial_end, cancel_at_period_end FROM subscriptions")
        .map_err(|e| format!("mirror read failed: {e}"))?;
    let mut out = Vec::new();
    for r in &res.rows {
        let g = |i: usize| r.get(i).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let gi = |i: usize| r.get(i).and_then(|v| v.as_i64());
        let gb = |i: usize| {
            r.get(i)
                .map(|v| v.as_i64() == Some(1) || v.as_str() == Some("1"))
                .unwrap_or(false)
        };
        out.push(SubRow {
            id: g(0),
            customer: g(1),
            price: g(2),
            status: g(3),
            period_start: gi(4),
            period_end: gi(5),
            trial_end: gi(6),
            cancel_at_end: gb(7),
        });
    }
    Ok(out)
}

fn read_invoices(store: &DatabaseStore) -> Result<Vec<InvoiceRow>, String> {
    let res = store
        .query("SELECT stripe_invoice_id, stripe_customer_id, stripe_subscription_id, hosted_invoice_url, status, amount_due, amount_paid, currency FROM stripe_invoices")
        .map_err(|e| format!("mirror read failed: {e}"))?;
    let mut out = Vec::new();
    for r in &res.rows {
        let g = |i: usize| r.get(i).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let go = |i: usize| r.get(i).and_then(|v| v.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let gi = |i: usize| r.get(i).and_then(|v| v.as_i64()).unwrap_or(0);
        out.push(InvoiceRow {
            id: g(0),
            customer: g(1),
            subscription: go(2),
            url: go(3),
            status: g(4),
            amount_due: gi(5),
            amount_paid: gi(6),
            currency: {
                let c = g(7);
                if c.is_empty() {
                    "eur".to_string()
                } else {
                    c
                }
            },
        });
    }
    Ok(out)
}

fn compare_mirror_vs_fresh(store: &DatabaseStore, fresh_subs: &[SubRow], fresh_invs: &[InvoiceRow]) -> Value {
    let mirror_subs = match read_subs(store) {
        Ok(v) => v,
        Err(e) => return json!({"success": false, "error": e}),
    };
    let mirror_invs = match read_invoices(store) {
        Ok(v) => v,
        Err(e) => return json!({"success": false, "error": e}),
    };
    let m_subs: std::collections::HashMap<&str, &SubRow> =
        mirror_subs.iter().map(|s| (s.id.as_str(), s)).collect();
    let f_subs: std::collections::HashMap<&str, &SubRow> =
        fresh_subs.iter().map(|s| (s.id.as_str(), s)).collect();
    let m_inv: std::collections::HashMap<&str, &InvoiceRow> =
        mirror_invs.iter().map(|i| (i.id.as_str(), i)).collect();
    let f_inv: std::collections::HashMap<&str, &InvoiceRow> =
        fresh_invs.iter().map(|i| (i.id.as_str(), i)).collect();

    let mut missing_in_mirror: Vec<String> = f_subs
        .keys()
        .filter(|k| !m_subs.contains_key(**k))
        .map(|k| format!("subscription:{k}"))
        .collect();
    let mut missing_in_api: Vec<String> = m_subs
        .keys()
        .filter(|k| !f_subs.contains_key(**k))
        .map(|k| format!("subscription:{k}"))
        .collect();
    for k in f_inv.keys() {
        if !m_inv.contains_key(*k) {
            missing_in_mirror.push(format!("invoice:{k}"));
        }
    }
    for k in m_inv.keys() {
        if !f_inv.contains_key(*k) {
            missing_in_api.push(format!("invoice:{k}"));
        }
    }
    let mut status_drift = Vec::new();
    for (id, f) in &f_subs {
        if let Some(m) = m_subs.get(*id) {
            if m.status != f.status {
                status_drift.push(json!({"id": id, "mirror_status": m.status, "api_status": f.status}));
            }
            if m.cancel_at_end != f.cancel_at_end {
                status_drift.push(json!({"id": id, "field": "cancel_at_period_end", "mirror": m.cancel_at_end, "api": f.cancel_at_end}));
            }
        }
    }
    let mut amount_drift = Vec::new();
    for (id, f) in &f_inv {
        if let Some(m) = m_inv.get(*id) {
            if m.amount_due != f.amount_due || m.amount_paid != f.amount_paid || m.status != f.status {
                amount_drift.push(json!({
                    "id": id,
                    "mirror": {"status": m.status, "amount_due": m.amount_due, "amount_paid": m.amount_paid},
                    "api": {"status": f.status, "amount_due": f.amount_due, "amount_paid": f.amount_paid},
                }));
            }
        }
    }
    missing_in_mirror.sort();
    missing_in_api.sort();
    json!({
        "success": true,
        "mirror_counts": {"subscriptions": mirror_subs.len(), "invoices": mirror_invs.len()},
        "fresh_counts": {"subscriptions": fresh_subs.len(), "invoices": fresh_invs.len()},
        "missing_in_mirror": missing_in_mirror,
        "missing_in_api": missing_in_api,
        "status_drift": status_drift,
        "amount_drift": amount_drift,
        "clean": missing_in_mirror.is_empty() && missing_in_api.is_empty() && status_drift.is_empty() && amount_drift.is_empty(),
    })
}
