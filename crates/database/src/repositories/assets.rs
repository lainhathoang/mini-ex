use sea_orm::{
    ActiveValue::Set,
    DatabaseConnection, EntityTrait,
    sea_query::{Expr, OnConflict},
};
use shared::result::Rs;

use crate::{entities::assets, models::assets::AssetRecord};

pub async fn upsert_many(db: &DatabaseConnection, records: &[AssetRecord]) -> Rs<()> {
    if records.is_empty() {
        return Ok(());
    }

    let active_models = records
        .iter()
        .map(|record| assets::ActiveModel {
            name: Set(record.name.clone()),
            symbol: Set(record.symbol.clone()),
            price: Set(record.price.clone()),
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
