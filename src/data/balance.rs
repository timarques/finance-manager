use super::*;

#[derive(Copy, Default, Clone)]
pub struct Balance {
    pub income: f64,
    pub expense: f64,
}

impl Balance {

    pub fn format_value(value: f64, prefix_sign: bool) -> String {
        let prefix = match (value >= 0.0, prefix_sign) {
            (true, true) => "+",
            (false, true) => "-",
            _ => ""
        };
        let abs_value = value.abs();
        
        let string = format!("{:.2}", abs_value);
        let parts: Vec<&str> = string.split('.').collect();
        let int_part = parts[0];
        let dec_part = parts[1];
        
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
        
        format!("{}{}.{}", prefix, with_separators, dec_part)
    }

    pub fn from_transactions(transactions: &[&Transaction]) -> Self {
        let (income, expense) = transactions
            .iter()
            .fold((0.0, 0.0), |(inc, exp), transaction| {
                if transaction.amount >= 0.0 {
                    (inc + transaction.amount, exp)
                } else {
                    (inc, exp + transaction.amount.abs())
                }
            });
        Self { income, expense }
    }

    #[inline]
    pub const fn net_balance(&self) -> f64 {
        self.income - self.expense
    }

    #[inline]
    pub const fn join(&self, other: &Balance) -> Balance {
        Balance {
            income: self.income + other.income,
            expense: self.expense + other.expense,
        }
    }

    pub fn formatted_balance(&self) -> String {
        Self::format_value(self.net_balance(), true)
    }

}

impl std::fmt::Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_balance())
    }
}