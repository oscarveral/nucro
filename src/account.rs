use std::collections::HashMap;

use crate::asset::Asset;
use crate::currency::Currency;

/// An account that can hold assets.
struct Account {
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
type AccountRepository = Vec<Account>;
