use crate::{
    account::AccountRepository, security::SecurityRepository, transaction::TransactionRepository,
};

/// An instance of a portfolio.
pub struct Portfolio {
    /// Collection of all securities registered.
    securities: SecurityRepository,
    /// Collection of all accounts on this portfolio.
    accounts: AccountRepository,
    /// Collection of all transactions of this portfolio.
    transactions: TransactionRepository,
}

impl Portfolio {
    /// Create a new empty portfolio.
    pub fn new() -> Self {
        Portfolio {
            securities: SecurityRepository::new(),
            accounts: AccountRepository::new(),
            transactions: TransactionRepository::new()
        }
    }
}