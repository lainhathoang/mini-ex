use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, prelude::Uuid,
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

pub async fn create_user(
    db: &DatabaseConnection,
    username: &str,
    password_hash: &str,
) -> Rs<users::Model> {
    users::ActiveModel {
        username: Set(username.to_owned()),
        password_hash: Set(password_hash.to_owned()),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(Into::into)
}
