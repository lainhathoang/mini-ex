use sea_orm::sqlx::types::Decimal;

use crate::result::{AppErr, Rs};

pub fn checked_mul(price: Decimal, quantity: Decimal) -> Rs<Decimal> {
    price
        .checked_mul(quantity)
        .ok_or_else(|| AppErr::custom("value is too large"))
}
