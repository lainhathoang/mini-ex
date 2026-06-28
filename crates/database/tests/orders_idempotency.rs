//! Integration tests for order idempotency (client_order_id) building blocks.
//!
//! These run against the database in `PORTFOLIO_DATABASE_URL` (loaded from `.env`). Each
//! test runs inside a transaction that is rolled back, so it leaves no rows
//! behind. If no database is reachable the test skips rather than failing, so
//! `cargo test` still works in environments without Postgres.

use std::str::FromStr;

use database::{
    entities::{orders, users},
    enums::{OrderSide, OrderStatus},
    repositories::{self, orders::NewOrder},
    sea_orm::{
        ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait,
        prelude::Decimal, prelude::Uuid,
    },
};

async fn connect() -> Option<DatabaseConnection> {
    dotenv::dotenv().ok();
    let url = std::env::var("PORTFOLIO_DATABASE_URL").ok()?;
    database::establish_connection(&url).await.ok()
}

fn executed_order<'a>(user_id: Uuid, client_order_id: &'a str) -> NewOrder<'a> {
    NewOrder {
        user_id,
        client_order_id,
        symbol: "btc",
        side: OrderSide::Buy,
        quantity: Decimal::from_str("0.01").unwrap(),
        status: OrderStatus::Executed,
        price: Some(Decimal::from_str("100").unwrap()),
        reject_reason: None,
    }
}

#[tokio::test]
async fn create_persists_key_and_find_by_client_order_id_returns_it() {
    let Some(db) = connect().await else {
        eprintln!("skipping: PORTFOLIO_DATABASE_URL unset or database unreachable");
        return;
    };
    let txn = db.begin().await.unwrap();

    let user_id = Uuid::now_v7();
    let key = format!("test-{}", Uuid::now_v7());

    let created = repositories::orders::create(&txn, executed_order(user_id, &key))
        .await
        .expect("create should succeed");
    assert_eq!(created.client_order_id, key);

    // Same owner + key resolves back to the created order.
    let found = repositories::orders::find_by_client_order_id(&txn, user_id, &key)
        .await
        .unwrap();
    assert_eq!(found.expect("order should be found").id, created.id);

    // A different key is a distinct order.
    let other_key = format!("test-{}", Uuid::now_v7());
    let other = repositories::orders::create(&txn, executed_order(user_id, &other_key))
        .await
        .expect("different key should create a new order");
    assert_ne!(other.id, created.id);

    // An unknown key resolves to nothing.
    let missing = repositories::orders::find_by_client_order_id(&txn, user_id, "no-such-key")
        .await
        .unwrap();
    assert!(missing.is_none());

    // Rollback (no commit) — leaves the database untouched.
    txn.rollback().await.unwrap();
}

#[tokio::test]
async fn duplicate_client_order_id_for_same_user_is_rejected_by_unique_constraint() {
    let Some(db) = connect().await else {
        eprintln!("skipping: PORTFOLIO_DATABASE_URL unset or database unreachable");
        return;
    };
    let txn = db.begin().await.unwrap();

    let user_id = Uuid::now_v7();
    let key = format!("test-{}", Uuid::now_v7());

    repositories::orders::create(&txn, executed_order(user_id, &key))
        .await
        .expect("first insert should succeed");

    // Second insert with the same (user_id, client_order_id) must violate the
    // unique index — this is the safety net behind idempotent placement.
    let duplicate = repositories::orders::create(&txn, executed_order(user_id, &key)).await;
    assert!(
        duplicate.is_err(),
        "duplicate client_order_id must be rejected"
    );

    // The transaction is now aborted; just roll it back.
    txn.rollback().await.ok();
}

#[tokio::test]
async fn same_key_for_different_users_is_independent() {
    let Some(db) = connect().await else {
        eprintln!("skipping: PORTFOLIO_DATABASE_URL unset or database unreachable");
        return;
    };
    let txn = db.begin().await.unwrap();

    // Same key string, two different users → the unique index is per-user, so
    // both succeed as distinct orders.
    let key = format!("test-{}", Uuid::now_v7());
    let a = repositories::orders::create(&txn, executed_order(Uuid::now_v7(), &key))
        .await
        .unwrap();
    let b = repositories::orders::create(&txn, executed_order(Uuid::now_v7(), &key))
        .await
        .expect("same key under a different user must be allowed");
    assert_ne!(a.id, b.id);

    txn.rollback().await.unwrap();
}

/// Mirrors the handler's idempotent placement core (lock user row → re-check by
/// key → insert if absent → commit) so the concurrent double-submit guarantee
/// is proven against real Postgres, not just by analysis.
async fn place_once(db: DatabaseConnection, user_id: Uuid, key: String) -> Uuid {
    let txn = db.begin().await.unwrap();
    repositories::users::find_by_id_for_update(&txn, user_id)
        .await
        .unwrap();
    let id = match repositories::orders::find_by_client_order_id(&txn, user_id, &key)
        .await
        .unwrap()
    {
        Some(existing) => existing.id,
        None => {
            repositories::orders::create(&txn, executed_order(user_id, &key))
                .await
                .unwrap()
                .id
        }
    };
    txn.commit().await.unwrap();
    id
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_same_key_places_exactly_one_order() {
    let Some(db) = connect().await else {
        eprintln!("skipping: PORTFOLIO_DATABASE_URL unset or database unreachable");
        return;
    };

    // A committed user row is required so both transactions contend on the same
    // `SELECT ... FOR UPDATE` lock.
    let username = format!("idem-test-{}", Uuid::now_v7());
    let user = repositories::users::create_user(&db, &username, "hash")
        .await
        .unwrap();
    let user_id = user.id;
    let key = format!("test-{}", Uuid::now_v7());

    let (a, b) = tokio::join!(
        place_once(db.clone(), user_id, key.clone()),
        place_once(db.clone(), user_id, key.clone()),
    );

    // Both racing requests resolve to the same single order...
    assert_eq!(a, b, "concurrent same-key requests must resolve to one order");
    // ...and exactly one row exists for (user, key).
    let count = orders::Entity::find()
        .filter(orders::Column::UserId.eq(user_id))
        .filter(orders::Column::ClientOrderId.eq(key.as_str()))
        .all(&db)
        .await
        .unwrap()
        .len();
    assert_eq!(count, 1, "exactly one order must be persisted");

    // Cleanup (committed rows can't be rolled back).
    orders::Entity::delete_many()
        .filter(orders::Column::UserId.eq(user_id))
        .exec(&db)
        .await
        .ok();
    users::Entity::delete_by_id(user_id).exec(&db).await.ok();
}
