use crate::result::Rs;

/// Trait for safe percentage calculations with overflow checking
pub trait CheckedPercent: Sized {
    fn checked_percent(&self, percent: u8) -> Rs<Self>;
    fn checked_percent_f32(&self, percent: f32) -> Rs<Self>;
}

/// Trait for unchecked percentage calculations (may overflow)
pub trait Percent {
    type Output;

    /// Calculate percentage (0-100)
    fn percent(&self, percent: u8) -> Self::Output;

    /// Calculate percentage with decimal precision
    fn percent_f32(&self, percent: f32) -> Self::Output;
}

impl Percent for f64 {
    type Output = f64;

    fn percent(&self, percent: u8) -> f64 {
        self / 100_f64 * (percent as f64)
    }

    fn percent_f32(&self, percent: f32) -> f64 {
        (*self) / 100f64 * (percent as f64)
    }
}

impl Percent for u64 {
    type Output = u64;

    fn percent(&self, percent: u8) -> u64 {
        self / 100u64 * (percent as u64)
    }

    fn percent_f32(&self, percent: f32) -> u64 {
        ((*self as f64) / 100f64 * (percent as f64)) as u64
    }
}

impl Percent for u128 {
    type Output = u128;

    fn percent(&self, percent: u8) -> u128 {
        self / 100u128 * (percent as u128)
    }

    fn percent_f32(&self, percent: f32) -> u128 {
        ((*self as f64) / 100f64 * (percent as f64)) as u128
    }
}
