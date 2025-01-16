use crate::currency::Currency;

/// A financial asset of any type.
pub enum Asset {
    Currency(Currency),
    Security(String),
}