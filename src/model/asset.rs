use crate::model::currency::Currency;

/// A financial asset of any type.
pub enum Asset {
	/// Any currency is also a financial asset.
	CurrencyType(Currency),
	/// Securities idenfied by their ISIN.
	SecurityType(String),
}
