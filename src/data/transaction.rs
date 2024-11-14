
use super::*;
use serde::{Serialize, Deserialize, Deserializer};
use chrono::{Local, NaiveDate};
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub id: usize,

    pub name: String,
    pub description: Option<String>,
    pub amount: f64,
    pub cycle: Cycle,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            description: None,
            amount: 0.0,
            start_date: Local::now().naive_local().date(),
            end_date: None,
            cycle: Cycle::OneTime,
        }
    }
}

impl Transaction {

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
        if self.name.trim().is_empty() || self.name.len() > 100 {
            return false;
        }

        if let Some(desc) = &self.description {
            if desc.len() > 500 {
                return false;
            }
        }

        if self.amount == 0.0 {
            return false;
        }

        if let Some(end_date) = self.end_date {
            if self.start_date > end_date {
                return false;
            }
        }

        return true
    }

    pub fn is_different(&self, other: &Transaction) -> bool {
        self.name != other.name
            || self.description != other.description
            || self.amount != other.amount
            || self.start_date != other.start_date
            || self.end_date != other.end_date
            || self.cycle != other.cycle
    }

    fn get_occurrences_in_period(&self, period: &Period) -> Option<Vec<NaiveDate>> {
        let now = Local::now().naive_local().date();
        if self.start_date > now {
            return None;
        }

        let (period_start, period_end) = period.bounds();
        let mut occurrences = Vec::new();
        let mut current_ocurrence = self.start_date;

        loop {
            if current_ocurrence > self.end_date.unwrap_or(now) {
                break;
            } else if current_ocurrence >= period_start && current_ocurrence <= period_end {
                occurrences.push(current_ocurrence);
            } else if current_ocurrence > period_end {
                break;
            }
            
            if let Some(ocurrence) = self.cycle.next(current_ocurrence) {
                current_ocurrence = ocurrence;
            } else {
                break;
            }
        }

        Some(occurrences)
    }

    pub fn count_occurrences_in_period(&self, period: &Period) -> Option<usize> {
        self.get_occurrences_in_period(period)
            .map(|occurrences| occurrences.len())
    }

    pub fn for_period(&self, period: &Period) -> Transaction {
        let occurrences = self.count_occurrences_in_period(period);
        Transaction {
            amount: self.amount * (occurrences.unwrap_or(0) as f64),
            ..self.clone()
        }
    }
}

#[derive(Deserialize, Serialize)]
struct TransactionHelper {
    name: String,
    description: Option<String>,
    amount: f64,
    cycle: Cycle,
    start_date: String,
    end_date: Option<String>,
}

impl<'de> Deserialize<'de> for Transaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let helper = TransactionHelper::deserialize(deserializer)?;
        
        let start_date = NaiveDate::parse_from_str(&helper.start_date, "%Y-%m-%d")
            .map_err(|e| serde::de::Error::custom(format!("Invalid start date: {}", e)))?;
        
        let end_date = helper.end_date.map(|date_str| 
            NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map_err(|e| serde::de::Error::custom(format!("Invalid end date: {}", e)))
        ).transpose()?;
        
        if let Some(ed) = end_date {
            if ed < start_date {
                return Err(serde::de::Error::custom("End date cannot be before start date"));
            }
        }

        Ok(Transaction {
            id: GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            name: helper.name,
            description: helper.description,
            amount: helper.amount,
            cycle: helper.cycle,
            start_date,
            end_date,
        })
    }
}

impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        let helper = TransactionHelper {
            name: self.name.clone(),
            description: self.description.clone(),
            amount: self.amount,
            cycle: self.cycle.clone(),
            start_date: self.start_date.format("%Y-%m-%d").to_string(),
            end_date: self.end_date.map(|date| date.format("%Y-%m-%d").to_string()),
        };
        
        helper.serialize(serializer)
    }
}