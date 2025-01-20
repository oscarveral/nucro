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

impl Security {
    /// Create a new security.
    fn new(isin: String, name: String, decimal: u8) -> Self {
        Security {
            isin: isin,
            name: name,
            decimal: decimal,
        }
    }
}

/// Repository of securities.
pub struct SecurityRepository(Vec<Security>);

impl SecurityRepository {
    /// Create a new empty security repository.
    pub fn new() -> Self {
        SecurityRepository(Vec::new())
    }
}
