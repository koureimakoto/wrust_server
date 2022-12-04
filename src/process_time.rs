use std::{time::{
    Duration,
    SystemTime
}};

use self::{common::*, constants::MONTHS_OF_THE_LEAP};

pub mod constants;
pub mod common;


/// Struct to parse Epoch time
pub struct
Date {
    utc    : Duration,
    year   : u32,
    months : u8,
    days   : u8,
    hour   : u8,
    minutes: u8,
    seconds: u8,
    millisecond: u8,
}

impl Date {
    pub fn new(now: Duration) -> Date {
        Date {
            utc    : now,
            year   : 0,
            months : 0,
            days   : 0,
            hour   : 0,
            minutes: 0,
            seconds: 0,
            millisecond: 0
        }
    }
    /// Parse Unix time second to Year/Months/Days/Hours/Minute/Seconds
    pub fn parse(&mut self) {
        let total_years          : f64 = epoch_to_years(self.utc);
        let truncated_total_years: f64 = total_years.trunc();
        let total_leap           : f64 = curr_leap_counter(total_years);
        
        let year_days : f64 = year_days_perc(total_years);
        let year_days : f64 = year_days * (365.0 - total_leap) as f64;

        self.set_curr_year(truncated_total_years);
        self.set_curr_mouths(&(year_days as u32));


    }

    fn set_curr_year(&mut self, total_years: f64) {
        self.year = 1970 + total_years as u32;
    }

    fn set_curr_mouths(&mut self, year_days: &u32) {
        self.months = if self.is_leap() {
            fetch_month_of_year(
                true,
                year_days, 
                &MONTHS_OF_THE_LEAP
            )
        } else {
            fetch_month_of_year(
                false,
                &(self.year as u32), 
                &MONTHS_OF_THE_LEAP
            )
        }

    }

    // fn set_curr_days(&mut self, year_days: &u32) {
    //     self.days = if self.is_leap() {
    //         if self.months >=
    //     } else {

    //     }
    // }

    fn is_leap(&self) -> bool {
        (self.year % 2) == 0
    }


}
