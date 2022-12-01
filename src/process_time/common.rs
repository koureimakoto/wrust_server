use std::time::Duration;
use super::constants::{
    COMMON_MIDYEAR,
    FLOAT_MASK,
    YEAR_SECOND
};

pub fn
    fetch_mouth_of_the_common_year(year_days: u32, months: &[[u32; 5]; 2]) -> u8 {
    let mut month_count: u8 = 0;
    let days = year_days;

    if year_days < COMMON_MIDYEAR {
        month_count = months_counter(&months[0], &year_days)
    } else {
        month_count = months_counter(&months[1], &year_days)
    }
    month_count
}

pub fn
    months_counter(months: &[u32; 5], year_days: &u32) -> u8 {
    let mut month_count: u8 = 0;
    for days in months.iter() {
        if days < year_days {
            month_count += 1;
        }
    }
    month_count
}

pub fn
utc_to_years(utc: Duration) -> f64 {
    utc.as_secs() as f64 / YEAR_SECOND as f64
}

pub fn
curr_leap_count(total_years: f64) -> f64 {
    total_years / 4.0
}

pub fn
year_days_perc(total_years: f64) -> f64 {
    ( total_years as u64 & FLOAT_MASK as u64 ) as f64
}
