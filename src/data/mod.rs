mod wallet;
mod transaction;
mod currency;
mod balance;
mod period;
mod cycle;

pub use wallet::Wallet;
pub use transaction::Transaction;
pub use currency::Currency;
pub use balance::Balance;
pub use period::Period;
pub use cycle::Cycle;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Data {
    pub wallets: Vec<Wallet>,
    pub currency: Currency,
    pub period: Period,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            wallets: Vec::new(),
            currency: Currency::USD,
            period: Period::Month,
        }
    }
}

impl Data {

    pub fn is_valid(&self) -> bool {
        self.wallets.len() > 0 && 
        self.wallets.iter().all(|w| w.is_valid())
    }

    pub fn is_empty(&self) -> bool {    
        self.wallets.len() == 0
    }

    pub fn find_wallet_by_id(&self, id: usize) -> Option<&Wallet> {
        self.wallets
            .iter()
            .find(|w| w.id == id)
    }

    pub fn remove_wallet_by_id(&mut self, id: usize) {
        self.wallets.retain(|w| w.id != id);
    }

    pub fn add_or_update_wallet(&mut self, wallet: Wallet) {
        if let Some(index) = self.wallets.iter().position(|w| w.id == wallet.id) {
            self.wallets[index] = wallet;
        } else {
            self.wallets.push(wallet);
        }
    }

    pub fn wallets_for_period(&self) -> Vec<Wallet> {
        self.wallets
            .iter()
            .map(|w| w.for_period(self.period))
            .collect()
    }

    pub fn total_balance_for_period(&self) -> Balance {
        self.wallets_for_period()
            .iter()
            .fold(Balance::default(), |acc, wallet| {
                acc.join(&wallet.convert_to_currency(self.currency).balance())
            })
    }

}