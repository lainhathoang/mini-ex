use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set,
    prelude::{Decimal, Uuid},
};
use shared::result::Rs;

use crate::entities::asset_balances;

pub async fn find_by_user(
    conn: &impl ConnectionTrait,
    user_id: Uuid,
) -> Rs<Vec<asset_balances::Model>> {
    asset_balances::Entity::find()
        .filter(asset_balances::Column::UserId.eq(user_id))
        .all(conn)
        .await
        .map_err(Into::into)
}

pub async fn find_by_user_and_symbol(
    conn: &impl ConnectionTrait,
    user_id: Uuid,
    symbol: &str,
) -> Rs<Option<asset_balances::Model>> {
    asset_balances::Entity::find_by_id((user_id, symbol.to_owned()))
        .one(conn)
        .await
        .map_err(Into::into)
}

pub async fn add_quantity(
    conn: &impl ConnectionTrait,
    user_id: Uuid,
    symbol: &str,
    delta: Decimal,
) -> Rs<()> {
    match find_by_user_and_symbol(conn, user_id, symbol).await? {
        Some(existing) => {
            let new_qty = existing.quantity + delta;
            let mut active: asset_balances::ActiveModel = existing.into();
            active.quantity = Set(new_qty);
            active.updated_at = Set(chrono::Utc::now().naive_utc());
            active.update(conn).await?;
        }
        None => {
            // `updatedAt` has no DB default, so set both timestamps explicitly.
            let now = chrono::Utc::now().naive_utc();
            asset_balances::ActiveModel {
                user_id: Set(user_id),
                symbol: Set(symbol.to_owned()),
                quantity: Set(delta),
                created_at: Set(now),
                updated_at: Set(now),
            }
            .insert(conn)
            .await?;
        }
    }
    Ok(())
}

pub async fn sub_quantity(
    conn: &impl ConnectionTrait,
    user_id: Uuid,
    symbol: &str,
    delta: Decimal,
) -> Rs<()> {
    if let Some(existing) = find_by_user_and_symbol(conn, user_id, symbol).await? {
        let new_qty = existing.quantity - delta;
        let mut active: asset_balances::ActiveModel = existing.into();
        active.quantity = Set(new_qty);
        active.updated_at = Set(chrono::Utc::now().naive_utc());
        active.update(conn).await?;
    }
    Ok(())
}
