use portfolio::Portfolio;

mod account;
mod asset;
mod defaults;
mod currency;
mod portfolio;
mod security;
mod transaction;

fn main() {
    let portfolio: Portfolio = Portfolio::new();
}
