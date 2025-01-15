use std::{collections::HashMap, hash::Hash};

pub enum Asset {
    Currency(Currency),
    Security(String),
}

/// Currency representation.
/// 
/// Enum that describes all posible currencies with their numeric codes.
#[derive(PartialEq, Eq)]
pub enum Currency {
    EUR = 978,
    USD = 840,
}

/// Security definition.
/// 
/// A security is uniquely identified by an ISIN code.
pub struct Security {
    isin: String,
}

impl PartialEq for Security {
    fn eq(&self, other: &Self) -> bool {
        self.isin == other.isin
    }
}

impl Eq for Security {}

impl Hash for Security {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.isin.hash(state);
    }
}

/// Asset Collection.
/// 
/// Repository where all the financial assets are stored.
pub struct AssetCollection {
    securities: HashMap<String, Security>
}