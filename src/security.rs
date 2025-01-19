/// Security definition.
///
/// A security is uniquely identified by an ISIN code.
pub struct Security {
    /// Unique identifier of the security.
    isin: String,
    /// Security name.
    name: String,
    /// Decimal places.
    decimal: u8,
}

/// Default number of decimal places allowed for securities accounting.
const DEFAULT_DECIMAL: u8 = 10;

/// Repository of securities.
pub type SecurityRepository = Vec<Security>;
