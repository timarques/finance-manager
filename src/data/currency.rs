use serde::{Serialize, Deserialize};

use std::str::FromStr;
use std::sync::LazyLock;
use serde_json::Value;

static EXCHANGE_RATES: LazyLock<[f64; Currency::count()]> = LazyLock::new(|| fetch_exchange_rates().expect("Failed to fetch exchange rates"));

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Currency {
    USD,
    EUR,
    CAD,
    GBP,
    JPY
}

impl Currency {

    const CURRENCIES: [(&'static str, &'static str, &'static str); 5] = [
        ("$", "USD", "US Dollar"),
        ("€", "EUR", "Euro"),
        ("$", "CAD", "Canadian Dollar"),
        ("£", "GBP", "Pound Sterling"),
        ("¥", "JPY", "Japanese Yen")
    ];

    #[inline]
    pub const fn count() -> usize {
        Self::CURRENCIES.len()
    }

    #[inline]
    pub const fn as_slice() -> [Currency; Currency::count()] {
        [
            Currency::USD,
            Currency::EUR,
            Currency::CAD,
            Currency::GBP,
            Currency::JPY
        ]   
    }

    pub fn is_default(&self) -> bool {
        *self == Currency::default()
    }

    #[inline]
    pub const fn as_symbol(&self) -> &'static str {
        Self::CURRENCIES[*self as usize].0
    }

    #[inline]
    pub const fn as_short_str(&self) -> &'static str {
        Self::CURRENCIES[*self as usize].1
    }

    #[inline]
    pub const fn as_long_str(&self) -> &'static str {
        Self::CURRENCIES[*self as usize].2
    }

    pub fn format_amount(&self, value: f64) -> String {
        if !value.is_finite() {
            return String::from("N/A");
        }
        
        let prefix = if value < 0.0 { "-" } else { "" };
        let abs_value = value.abs();
        
        let (int_part, dec_part) = if *self == Currency::JPY {
            let rounded = abs_value.round();
            (rounded.to_string(), None)
        } else {
            let formatted = format!("{:.2}", abs_value);
            let parts: Vec<&str> = formatted.split('.').collect();
            (parts[0].to_string(), Some(parts[1].to_string()))
        };

        let with_separators: String = int_part
            .chars()
            .rev()
            .enumerate()
            .flat_map(|(i, c)| {
                if i > 0 && i % 3 == 0 {
                    vec![' ', c]
                } else {
                    vec![c]
                }
            })
            .collect::<String>()
            .chars()
            .rev()
            .collect();

        match dec_part {
            Some(dec) => format!("{}{}{}.{}", prefix, self.as_symbol(), with_separators, dec),
            None => format!("{}{}", self.as_symbol(), with_separators)
        }
    }

    pub fn normalize_amount(&self, amount: f64) -> f64 {
        if !amount.is_finite() {
            return 0.0;
        }
        match self {
            Currency::JPY => amount.round(),
            _ => (amount * 100.0).round() / 100.0
        }
    }

    fn get_exchange_rate(&self) -> f64 {
        if *self == Self::EUR {
            1.0
        } else {
            EXCHANGE_RATES[*self as usize]
        }
    }

    pub fn convert_amount(&self, amount: f64, to: Currency) -> f64 {
        if *self == to || !amount.is_finite() {
            return amount;
        }
        
        let origin_rate = self.get_exchange_rate();
        let target_rate = to.get_exchange_rate();
        
        self.normalize_amount(amount * (target_rate / origin_rate))
    }
}

impl Default for Currency {
    fn default() -> Self {
        Currency::USD
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_short_str())
    }
}

impl std::str::FromStr for Currency {
    type Err = ();
    
    fn from_str(input: &str) -> Result<Currency, Self::Err> {
        match input.trim().to_uppercase().as_str() {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "CAD" => Ok(Currency::CAD),
            "GBP" => Ok(Currency::GBP),
            "JPY" => Ok(Currency::JPY),
            _ => Err(())
        }
    }
}

fn fetch_exchange_rates() -> Result<[f64; Currency::count()], Box<dyn std::error::Error>> {
    let response = ureq::get("https://api.frankfurter.app/latest")
        .call()?
        .into_string()?;
    
    let json: Value = serde_json::from_str(&response)?;
    
    let mut rates = [0.0; Currency::count()];
    if let Some(rates_obj) = json.get("rates").and_then(|v| v.as_object()) {
        for (currency, rate) in rates_obj {
            if let (Some(rate_num), Ok(currency)) = (rate.as_f64(), Currency::from_str(currency)) {
                rates[currency as usize] = rate_num;
            }
        }
    }

    Ok(rates)
}