use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QuerySelect, Set,
    prelude::{Decimal, Uuid},
};
use shared::result::Rs;

use crate::entities::users;

pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Rs<Option<users::Model>> {
    users::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(Into::into)
}

pub async fn find_by_username(db: &DatabaseConnection, username: &str) -> Rs<Option<users::Model>> {
    users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(Into::into)
}

/// Loads a user while taking a `SELECT ... FOR UPDATE` row lock.
///
/// Must run inside a transaction. Locking the user row serializes all of that
/// user's concurrent balance mutations (cash and asset balances), preventing
/// lost updates that a bare read-modify-write under READ COMMITTED would allow.
pub async fn find_by_id_for_update(
    conn: &impl ConnectionTrait,
    id: Uuid,
) -> Rs<Option<users::Model>> {
    users::Entity::find_by_id(id)
        .lock_exclusive()
        .one(conn)
        .await
        .map_err(Into::into)
}

pub async fn update_cash_balance(
    conn: &impl ConnectionTrait,
    id: Uuid,
    new_balance: Decimal,
) -> Rs<()> {
    users::ActiveModel {
        id: Set(id),
        cash_balance: Set(new_balance),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .update(conn)
    .await?;
    Ok(())
}

pub async fn create_user(
    db: &DatabaseConnection,
    username: &str,
    password_hash: &str,
) -> Rs<users::Model> {
    users::ActiveModel {
        id: Set(Uuid::now_v7()),
        username: Set(username.to_owned()),
        password_hash: Set(password_hash.to_owned()),
        cash_balance: Default::default(),
        created_at: Default::default(),
        updated_at: Default::default(),
    }
    .insert(db)
    .await
    .map_err(Into::into)
}
