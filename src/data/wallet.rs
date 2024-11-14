use super::*;
use serde::{Serialize, Deserialize, Deserializer};
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Wallet {
    #[serde(skip_serializing)]
    pub id: usize,

    pub name: String,
    pub description: Option<String>,
    pub currency: Currency,
    pub transactions: Vec<Transaction>,
}

impl Default for Wallet {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            description: None,
            currency: Currency::USD,
            transactions: Vec::new(),
        }
    }
}

impl Wallet {

    pub fn assign_global_id(self) -> Self {
        if self.is_created() {
            self
        } else {
            Self {
                id: GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
                ..self
            }
        }
    }

    #[inline]
    pub const fn is_created(&self) -> bool {
        self.id != 0
    }

    pub fn is_valid(&self) -> bool {
        let name_valid = !self.name.trim().is_empty() && self.name.len() <= 100;
        let description_valid = self.description.as_ref().map_or(true, |desc| desc.len() <= 500);
        let transactions_valid = self.transactions.iter().all(|t| t.is_valid());

        name_valid && description_valid && transactions_valid
    }

    pub fn is_different(&self, other: &Wallet) -> bool {
        self.name != other.name
            || self.description != other.description
            || self.currency != other.currency
            || self.transactions
                .iter()
                .zip(other.transactions.iter())
                .any(|(t1, t2)| t1.is_different(t2))
    }

    pub fn find_transaction_by_id(&self, transaction_id: usize) -> Option<&Transaction> {
        self.transactions
            .iter()
            .find(|t| t.id == transaction_id)
    }

    pub fn remove_transaction_by_id(&mut self, transaction_id: usize) {
        self.transactions.retain(|t| t.id != transaction_id);
    }

    pub fn add_or_update_transaction(&mut self, transaction: Transaction) {
        if let Some(index) = self.transactions.iter().position(|t| t.id == transaction.id) {
            self.transactions[index] = transaction;
        } else {
            self.transactions.push(transaction);
        }
    }

    pub fn balance(&self) -> Balance {
        let transactions: Vec<&Transaction> = self.transactions.iter().collect();
        Balance::from_transactions(&transactions)
    }

    pub fn for_period(&self, period: Period) -> Wallet {
        let filtered_transactions: Vec<Transaction> = self.transactions
            .iter()
            .map(|t| t.for_period(&period))
            .collect();
    
        Wallet {
            transactions: filtered_transactions,
            ..self.clone()
        }
    }

    pub fn convert_to_currency(&self, target_currency: Currency) -> Self {
        let transactions = self.transactions
            .iter()
            .map(|t| {
                let amount = self.currency.convert_amount(t.amount, target_currency);
                Transaction {
                    amount,
                    ..t.clone()
                }
            })
            .collect();
    
        Self {
            currency: target_currency,
            transactions,
            ..self.clone()
        }
    }

}

impl std::fmt::Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Deserialize)]
struct WalletHelper {
    name: String,
    description: Option<String>,
    currency: Currency,
    transactions: Vec<Transaction>,
}

impl<'de> Deserialize<'de> for Wallet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let helper = WalletHelper::deserialize(deserializer)?;
        
        Ok(Wallet {
            id: GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            name: helper.name,
            description: helper.description,
            currency: helper.currency,
            transactions: helper.transactions,
        })
    }
}