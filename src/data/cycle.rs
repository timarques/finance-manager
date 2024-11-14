use chrono::{Duration, Months, NaiveDate};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Cycle {
    Daily,
    Weekly,
    Monthly,
    Yearly,
    OneTime
}

impl Cycle {

    const CYCLES: [&'static str; 5] = [
        "Daily",
        "Weekly",
        "Monthly",
        "Yearly",
        "One Time"
    ];

    #[inline]
    pub const fn as_str(&self) -> &'static str {
        Self::CYCLES[*self as usize]
    }

    #[inline]
    pub const fn as_slice() -> [Cycle; 5] {
        [
            Cycle::Daily,
            Cycle::Weekly,
            Cycle::Monthly,
            Cycle::Yearly,
            Cycle::OneTime
        ]
    }

    #[inline]
    pub const fn icon_name(&self) -> &'static str {
        match self {
            Cycle::Daily => "today-alt-symbolic",
            Cycle::Weekly => "work-week-symbolic",
            Cycle::Monthly => "month-symbolic",
            Cycle::Yearly => "year-symbolic",
            Cycle::OneTime => "today-alt-symbolic",
        }
    }

    pub fn next(&self, date: NaiveDate) -> Option<NaiveDate> {
        match self {
            Cycle::Daily => Some(date + Duration::days(1)),
            Cycle::Weekly => Some(date + Duration::days(7)),
            Cycle::Monthly => {
                date.checked_add_months(Months::new(1))
            },
            Cycle::Yearly => {
                date.checked_add_months(Months::new(12))
            },
            Cycle::OneTime => None,
        }
    }

}

impl Default for Cycle {
    fn default() -> Self {
        Self::OneTime
    }
}

impl std::fmt::Display for Cycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.as_str();
        write!(f, "{}", s)
    }
}