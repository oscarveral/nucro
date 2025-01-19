use crate::{
    account::AccountRepository, security::SecurityRepository, transaction::TransactionRepository,
};

/// An instance of a portfolio.
struct Portfolio {
    securities: SecurityRepository,
    accounts: AccountRepository,
    transactions: TransactionRepository,
}
