use std::time::Duration;
use super::constants::{
    COMMON_MIDYEAR,
    YEAR_SECONDS,
    LEAP_INTERVAL
};

pub fn
fetch_month_of_year(leap: bool, year_days: &u32, months: &[[u32; 5]; 2]) -> u8 {
    let leap: u32 = match leap {
        true => 1,
        _    => 0
    };
    let shim: u8 = 5;

    if *year_days < (COMMON_MIDYEAR + leap) {
        months_counter(&months[0], &year_days)
    } else {
        shim + months_counter(&months[1], &year_days)
    }
}

pub fn
months_counter(months: &[u32; 5], year_days: &u32) -> u8 {  
    if year_days > &(months[4] + 31) {
        panic!("Limit of year days exceeded!")
    }
    
    if *year_days <= 31 {
        return 1
    } 
    
    let mut month_count: u8 = 2;
    for days in months.iter() {
        if  year_days >= days {
            month_count += 1;
        }
    }
    month_count
}

pub fn
epoch_to_years(epoch: Duration) -> f64 {
    epoch.as_secs() as f64 / YEAR_SECONDS as f64
}

pub fn
curr_leap_counter(total_years: f64) -> f64 {
    total_years / LEAP_INTERVAL
}

pub fn
year_days_perc(total_years: f64) -> f64 {
    let year = total_years - total_years.trunc();

    if year >= 0.0 {
        year
    } else {
        panic!("Year Percentage was out of range.")
    }
}

#[cfg(test)]
#[path ="common_test.rs"]
pub mod common_test;