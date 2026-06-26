use sea_orm::sqlx::types::Decimal;

#[derive(Debug, Clone)]
pub struct AssetRecord {
    pub name: String,
    pub symbol: String,
    pub price: Decimal,
}
