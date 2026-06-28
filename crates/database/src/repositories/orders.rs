use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set,
    prelude::{Decimal, Uuid},
};
use shared::result::Rs;

use crate::{
    entities::orders,
    enums::{OrderSide, OrderStatus, RejectReason},
};

/// Fields required to persist a new order, grouped to keep `create` cohesive.
pub struct NewOrder<'a> {
    pub user_id: Uuid,
    pub client_order_id: &'a str,
    pub symbol: &'a str,
    pub side: OrderSide,
    pub quantity: Decimal,
    pub status: OrderStatus,
    pub price: Option<Decimal>,
    pub reject_reason: Option<RejectReason>,
}

/// Inserts an order row.
///
/// Idempotency invariant: any path that may insert an order for an existing
/// `client_order_id` must first take the user-row lock
/// ([`users::find_by_id_for_update`](super::users::find_by_id_for_update)) and
/// re-check [`find_by_client_order_id`]. The `(user_id, client_order_id)` unique
/// index is only the backstop; without the lock+recheck a concurrent duplicate
/// would surface as an opaque unique-violation error instead of a clean replay.
pub async fn create(conn: &impl ConnectionTrait, new: NewOrder<'_>) -> Rs<orders::Model> {
    // Timestamps are set explicitly rather than relying on a DB default:
    // `updatedAt` has no `@default` in the schema, so an INSERT must supply it.
    let now = chrono::Utc::now().naive_utc();

    orders::ActiveModel {
        id: Set(Uuid::now_v7()),
        user_id: Set(new.user_id),
        client_order_id: Set(new.client_order_id.to_owned()),
        symbol: Set(new.symbol.to_owned()),
        side: Set(new.side),
        quantity: Set(new.quantity),
        price: Set(new.price),
        status: Set(new.status),
        reject_reason: Set(new.reject_reason),
        created_at: Set(now),
        updated_at: Set(now),
    }
    .insert(conn)
    .await
    .map_err(Into::into)
}

pub async fn find_by_id(conn: &impl ConnectionTrait, id: Uuid) -> Rs<Option<orders::Model>> {
    orders::Entity::find_by_id(id)
        .one(conn)
        .await
        .map_err(Into::into)
}

/// Looks up an order by its owner and client-supplied idempotency key.
///
/// Used to detect a retried order placement so the original result can be
/// replayed instead of executing again.
pub async fn find_by_client_order_id(
    conn: &impl ConnectionTrait,
    user_id: Uuid,
    client_order_id: &str,
) -> Rs<Option<orders::Model>> {
    orders::Entity::find()
        .filter(orders::Column::UserId.eq(user_id))
        .filter(orders::Column::ClientOrderId.eq(client_order_id))
        .one(conn)
        .await
        .map_err(Into::into)
}
