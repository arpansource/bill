use chrono::{Datelike, Local, NaiveDate};

pub struct Budget {
    pub need_monthly: f64,
    pub want_monthly: f64,
    pub need_daily: f64,
    pub want_daily: f64,
    pub days_in_month: u32,
}

/// Returns number of days in the current month (handles leap years automatically)
fn get_days_in_current_month() -> u32 {
    let now = Local::now();

    let year = now.year();
    let month = now.month();

    // Move to next month
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    // First day of next month
    let first_next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1).expect("Invalid date");

    // Last day of current month = previous day of next month
    let last_day = first_next_month
        .pred_opt()
        .expect("Failed to compute previous day");

    last_day.day()
}

/// Calculate budget based on 50-30-20 rule
pub fn calculate_budget(income: f64) -> Budget {
    let need_monthly = income * 0.5;
    let want_monthly = income * 0.3;

    let days = get_days_in_current_month() as f64;

    Budget {
        need_monthly,
        want_monthly,
        need_daily: need_monthly / days,
        want_daily: want_monthly / days,
        days_in_month: days as u32,
    }
}
