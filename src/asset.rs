use crate::currency::Currency;

/// A financial asset of any type.
pub enum Asset {
    CurrencyType(Currency),
    SecurityType(String),
}
