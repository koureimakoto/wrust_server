use std::time::Duration;
use super::constants::{
    COMMON_MIDYEAR,
    YEAR_SECONDS,
    LEAP_INTERVAL, MAX_MONTHS_SIZE
};

#[deprecated = "Inefficient! You should use fetch_month_and_day_of_the_year() instead"]
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

#[deprecated = "Inefficient! You should use fetch_month_and_day_of_the_year() instead"]
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
fetch_month_and_day_of_the_year(leap: bool, months: &[u32; 12], year_days: &u32) -> (u8, u8) {
    if year_days > &months[MAX_MONTHS_SIZE] {
        panic!("Max Limit of year days exceeded!")
    } else
    if *year_days < 1 {
        panic!("Min value of year days is 01!")
    }

    let leap: u32 = match leap {
        true => 1,
        _    => 0
    };

    let mut month: usize = 0;

    'found: for (index, days) in months.iter().enumerate() {
        if year_days < &(days + leap){
            month = index;
            break 'found
        } 
    }

    match month {
        0 => ((month + 1) as u8, *year_days as u8),
        1 => ((month + 1) as u8, (year_days - &months[ month - 1 ]) as u8),
        _ => ((month + 1) as u8, (year_days - &months[ month - 1 ] - leap)  as u8 )
    }
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