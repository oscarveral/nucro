use crate::model::asset::Asset;

/// Transactions that can take place.
#[repr(u8)]
pub enum Transaction {
    /// A balance neutral transfer of assets into an account.
    ///
    /// (Account ID, Asset, Quantity)
    TransferIn(u128, Asset, u128) = 0,
    /// A balance neutral transfer of assets out of an account.
    ///
    /// (Account ID, Asset, Quantity)
    TransferOut(u128, Asset, u128) = 1,
}

/// Repository of transactions.
pub struct TransactionRepository(Vec<Transaction>);

impl TransactionRepository {
    /// Create a new empty transaction repository.
    pub fn new() -> Self {
        TransactionRepository(Vec::new())
    }
}
