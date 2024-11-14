use serde::{Serialize, Deserialize};
use chrono::{Datelike, Duration, Local, Months, NaiveDate};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Period {
    Day,
    Week,
    Month,
    Year,
    All,
}

impl Default for Period {
    fn default() -> Self {
        Period::All
    }
}

impl Period {

    const PERIODS: [&'static str; 5] = [
        "Day",
        "Week",
        "Month",
        "Year",
        "All"
    ];

    #[inline]
    pub const fn as_str(&self) -> &'static str {
        Self::PERIODS[*self as usize]
    }

    #[inline]
    pub const fn as_slice() -> [Self; 5] {
        [
            Self::Day,
            Self::Week,
            Self::Month,
            Self::Year,
            Self::All
        ]
    }

    pub fn bounds(&self) -> (NaiveDate, NaiveDate) {
        let now = Local::now().naive_local().date();
        let (start, end) = match self {
            Self::Day => {
                (now, now)
            },
            Self::Week => {
                let week_start = now - Duration::days(now.weekday().num_days_from_monday() as i64);
                let week_end = week_start + Duration::days(6);
                (week_start, week_end.min(now))
            },
            Self::Month => {
                let month_start = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap_or(now);
                let month_end = if let Some(next_month) = month_start.checked_add_months(Months::new(1)) {
                    next_month - Duration::days(1)
                } else {
                    now
                };
                (month_start, month_end.min(now))
            },
            Self::Year => {
                let year_start = NaiveDate::from_ymd_opt(now.year(), 1, 1).unwrap_or(now);
                let year_end = NaiveDate::from_ymd_opt(now.year(), 12, 31).unwrap_or(now);
                (year_start, year_end.min(now))
            },
            Self::All => {
                let all_start = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
                (all_start, now)
            },
        };
    
        (start, end)
    }

}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.as_str();
        write!(f, "{}", s)
    }
}