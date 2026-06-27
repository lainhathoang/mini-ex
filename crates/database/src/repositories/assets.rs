use sea_orm::{
    ActiveValue::Set,
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    sea_query::{Expr, OnConflict},
};
use shared::result::Rs;

use crate::{entities::assets, models::assets::AssetRecord};

pub async fn find_all(db: &DatabaseConnection) -> Rs<Vec<assets::Model>> {
    assets::Entity::find().all(db).await.map_err(Into::into)
}

pub async fn find_by_symbol(db: &DatabaseConnection, symbol: &str) -> Rs<Option<assets::Model>> {
    assets::Entity::find()
        .filter(assets::Column::Symbol.eq(symbol))
        .one(db)
        .await
        .map_err(Into::into)
}

pub async fn upsert_many(db: &DatabaseConnection, records: &[AssetRecord]) -> Rs<()> {
    if records.is_empty() {
        return Ok(());
    }

    let active_models = records
        .iter()
        .map(|record| assets::ActiveModel {
            name: Set(record.name.clone()),
            symbol: Set(record.symbol.clone()),
            price: Set(record.price),
            ..Default::default()
        })
        .collect::<Vec<_>>();

    assets::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(assets::Column::Name)
                .update_columns([assets::Column::Symbol, assets::Column::Price])
                .value(assets::Column::UpdatedAt, Expr::current_timestamp())
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
