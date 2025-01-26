use std::collections::HashMap;

use crate::model::asset::Asset;
use crate::model::currency::Currency;

/// An account that can hold assets.
pub struct Account {
    /// Unique identifier of the account.
    id: u128,
    /// Descriptive name.
    name: String,
    /// Holdings of this account.
    positions: HashMap<Asset, u128>,
    /// Currency of reference of this account.
    reference_currency: Currency,
}

/// Repository of multiple accounts.
pub struct AccountRepository(Vec<Account>);

impl AccountRepository {
    /// Create a new empty account repository.
    pub fn new() -> Self {
        AccountRepository(Vec::new())
    }
}
