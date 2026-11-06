//! Stripe billing mirror tests: fixtures-only (no network, no keys).
//!
//! Acceptance mapping (STRIPE_BILLING_SPEC.md §11):
//! - sync mirrors fixture counts exactly,
//! - reconcile on identical data reports clean (no false positives),
//! - reconcile after planted drift reports exactly the planted diffs,
//! - without fixtures and without a key, commands refuse instead of faking.

use databasecraft_store::DatabaseStore;
use moneycraft_core::{dispatch, stripe, Command};

fn fixtures_dir() -> String {
    format!("{}/tests/fixtures/stripe", env!("CARGO_MANIFEST_DIR"))
}

fn temp_db(name: &str) -> String {
    let mut p = std::env::temp_dir();
    p.push(format!("stripe-test-{}-{}.dbcraft", std::process::id(), name));
    let _ = std::fs::remove_file(&p);
    p.to_string_lossy().to_string()
}

#[tokio::test]
async fn test_stripe_status_is_honest_without_key() -> Result<(), Box<dyn std::error::Error>> {
    // Pure constructor: no env access, deterministic.
    let res = stripe::status_report(false, "none", stripe::STRIPE_API_BASE_DEFAULT);
    assert_eq!(res["success"], true);
    assert_eq!(res["configured"], false);
    let text = serde_json::to_string(&res)?;
    assert!(!text.contains("sk_test") && !text.contains("sk_live"));
    Ok(())
}

#[tokio::test]
async fn test_stripe_status_modes() -> Result<(), Box<dyn std::error::Error>> {
    // Mode classification is a pure function of the key prefix — never the key itself in output.
    let t = stripe::status_report(true, "test", "https://api.stripe.com");
    assert_eq!(t["mode"], "test");
    let l = stripe::status_report(true, "live", "https://api.stripe.com");
    assert_eq!(l["mode"], "live");
    let u = stripe::status_report(true, "unknown", "https://example.invalid");
    assert_eq!(u["base_url"], "https://example.invalid");
    Ok(())
}

#[tokio::test]
async fn test_stripe_sync_fixtures_to_temp_db() -> Result<(), Box<dyn std::error::Error>> {
    let db = temp_db("sync");
    let res = dispatch(
        Command::StripeSync {
            resource: None,
            db_path: Some(db.clone()),
            fixtures_dir: Some(fixtures_dir()),
        },
        &DatabaseStore::open_in_memory()?,
    )
    .await;
    assert_eq!(res["success"], true, "sync failed: {res}");
    assert_eq!(res["total_upserted"], 13);
    assert_eq!(res["persisted"], true);
    let store = DatabaseStore::open(&db)?;
    let tables = ["stripe_customers", "stripe_products", "stripe_prices", "subscriptions", "stripe_invoices"];
    for t in tables {
        let q = store.query(&format!("SELECT COUNT(*) FROM {t}"))?;
        let n = q.rows.first().and_then(|r| r.first()).and_then(|v| v.as_u64()).unwrap_or(0);
        assert!(n > 0, "table {t} empty after sync");
    }
    let subs = store.query("SELECT DISTINCT status FROM subscriptions ORDER BY status")?;
    let statuses: Vec<String> = subs
        .rows
        .iter()
        .filter_map(|r| r.first()?.as_str().map(|s| s.to_string()))
        .collect();
    assert!(statuses.contains(&"active".to_string()));
    assert!(statuses.contains(&"past_due".to_string()));
    assert!(statuses.contains(&"canceled".to_string()));
    let _ = std::fs::remove_file(&db);
    Ok(())
}

#[tokio::test]
async fn test_stripe_subscriptions_rollup_from_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    let res = dispatch(
        Command::StripeSubscriptions {
            db_path: None,
            fixtures_dir: Some(fixtures_dir()),
        },
        &DatabaseStore::open_in_memory()?,
    )
    .await;
    assert_eq!(res["success"], true, "subscriptions failed: {res}");
    assert_eq!(res["count"], 3);
    assert_eq!(res["by_status"]["active"], 1);
    assert_eq!(res["by_status"]["past_due"], 1);
    assert_eq!(res["by_status"]["canceled"], 1);
    Ok(())
}

#[tokio::test]
async fn test_stripe_reconcile_clean_when_nothing_changed() -> Result<(), Box<dyn std::error::Error>> {
    let db = temp_db("clean");
    let sync = dispatch(
        Command::StripeSync {
            resource: None,
            db_path: Some(db.clone()),
            fixtures_dir: Some(fixtures_dir()),
        },
        &DatabaseStore::open_in_memory()?,
    )
    .await;
    assert_eq!(sync["success"], true);
    let res = dispatch(
        Command::StripeReconcile {
            db_path: Some(db.clone()),
            fixtures_dir: Some(fixtures_dir()),
            fixtures_base: None,
        },
        &DatabaseStore::open_in_memory()?,
    )
    .await;
    assert_eq!(res["success"], true, "reconcile failed: {res}");
    assert_eq!(res["clean"], true, "expected clean, got: {res}");
    assert_eq!(res["missing_in_mirror"].as_array().map(|a| a.len()), Some(0));
    assert_eq!(res["missing_in_api"].as_array().map(|a| a.len()), Some(0));
    assert_eq!(res["status_drift"].as_array().map(|a| a.len()), Some(0));
    assert_eq!(res["amount_drift"].as_array().map(|a| a.len()), Some(0));
    let _ = std::fs::remove_file(&db);
    Ok(())
}

#[tokio::test]
async fn test_stripe_reconcile_finds_planted_drift() -> Result<(), Box<dyn std::error::Error>> {
    let db = temp_db("drift");
    let sync = dispatch(
        Command::StripeSync {
            resource: None,
            db_path: Some(db.clone()),
            fixtures_dir: Some(fixtures_dir()),
        },
        &DatabaseStore::open_in_memory()?,
    )
    .await;
    assert_eq!(sync["success"], true);
    // Plant exactly three drifts directly in the mirror.
    let store = DatabaseStore::open(&db)?;
    store.execute("UPDATE subscriptions SET status = 'past_due' WHERE stripe_subscription_id = 'sub_test_aktiv_001'")?;
    store.execute("DELETE FROM stripe_invoices WHERE stripe_invoice_id = 'in_test_003'")?;
    store.execute("UPDATE stripe_invoices SET amount_due = 9999 WHERE stripe_invoice_id = 'in_test_002'")?;
    drop(store);
    let res = dispatch(
        Command::StripeReconcile {
            db_path: Some(db.clone()),
            fixtures_dir: Some(fixtures_dir()),
            fixtures_base: None,
        },
        &DatabaseStore::open_in_memory()?,
    )
    .await;
    assert_eq!(res["success"], true, "reconcile failed: {res}");
    assert_eq!(res["clean"], false);
    let drift: Vec<String> = res["status_drift"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|d| d.get("id")?.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    assert_eq!(drift, vec!["sub_test_aktiv_001".to_string()], "status drift mismatch: {res}");
    let missing: Vec<String> = res["missing_in_mirror"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    assert_eq!(missing, vec!["invoice:in_test_003".to_string()], "missing mismatch: {res}");
    let amounts: Vec<String> = res["amount_drift"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|d| d.get("id")?.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    assert_eq!(amounts, vec!["in_test_002".to_string()], "amount drift mismatch: {res}");
    let _ = std::fs::remove_file(&db);
    Ok(())
}

#[tokio::test]
async fn test_stripe_refuses_without_key_or_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    // Deterministic refusal: point at an empty dir so no fixtures exist.
    let empty = std::env::temp_dir().join(format!("stripe-empty-{}", std::process::id()));
    std::fs::create_dir_all(&empty)?;
    let empty_str = empty.to_string_lossy().to_string();
    // With fixtures present-but-empty-dir, load fails on missing files (no key needed).
    let res = dispatch(
        Command::StripeSync {
            resource: Some("customers".to_string()),
            db_path: None,
            fixtures_dir: Some(empty_str.clone()),
        },
        &DatabaseStore::open_in_memory()?,
    )
    .await;
    assert_eq!(res["success"], false);
    // And without fixtures at all, sync demands a key instead of inventing rows.
    // (KEY branch: only asserted when no key is configured in this environment.)
    if std::env::var("STRIPE_SECRET_KEY").ok().filter(|v| !v.trim().is_empty()).is_none() {
        let res2 = dispatch(
            Command::StripeSync {
                resource: Some("customers".to_string()),
                db_path: None,
                fixtures_dir: None,
            },
            &DatabaseStore::open_in_memory()?,
        )
        .await;
        assert_eq!(res2["success"], false);
        let msg = res2["error"].as_str().unwrap_or("");
        assert!(msg.contains("STRIPE_SECRET_KEY"), "unexpected error: {msg}");
    }
    std::fs::remove_dir_all(&empty).ok();
    Ok(())
}
