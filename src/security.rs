/// Security definition.
///
/// A security is uniquely identified by an ISIN code.
struct Security {
    /// Unique identifier of the security.
    isin: String,
    /// Security name.
    name: String,
}

/// Repository of securities.
type SecurityRepository = Vec<Security>;