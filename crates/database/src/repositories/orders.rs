use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, Set,
    prelude::{Decimal, Uuid},
};
use shared::result::Rs;

use crate::{
    entities::orders,
    enums::{OrderSide, OrderStatus, RejectReason},
};

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

pub async fn create(conn: &impl ConnectionTrait, new: NewOrder<'_>) -> Rs<orders::Model> {
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
        created_at: Set(Default::default()),
        updated_at: Set(Default::default()),
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

pub async fn find_by_user(conn: &impl ConnectionTrait, user_id: Uuid) -> Rs<Vec<orders::Model>> {
    orders::Entity::find()
        .filter(orders::Column::UserId.eq(user_id))
        .order_by_desc(orders::Column::CreatedAt)
        .order_by_desc(orders::Column::Id)
        .all(conn)
        .await
        .map_err(Into::into)
}
