use std::{collections::HashMap, time::Duration};

use crate::asset::{Asset, Currency};

struct Account {
    positions: HashMap<Asset, u128>,
    transactions: Vec<Transaction>,
    reference_currency: Currency
}

enum Transaction {
    TX(Asset, u128, Asset, u128, Option<u128>, Duration),
    IN(Asset, u128, Duration),
    OUT(Asset, u128, Duration),
}
