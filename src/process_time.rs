use std::{time::{
    Duration,
    SystemTime
}};

use self::common::*;

pub mod constants;
pub mod common;


/// Struct to parse Epoch time
pub struct
Date {
    utc    : Duration,
    year   : u16,
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
        let total_years          : f64 = utc_to_years(self.utc);
        let truncated_total_years: f64 = total_years.trunc();
        let total_leap           : f64 = curr_leap_count(total_years);
        

        self.set_curr_year(truncated_total_years)


    }

    fn set_curr_year(&mut self, total_years: f64) {
        self.year = 1970 + total_years as u16;
    }

    fn set_curr_mouths(&mut self, total_years: f64) {
        let leap_count: f64 = curr_leap_count(total_years);
        let year_days : f64 = year_days_perc(total_years);
        let year_days : f64 = year_days * (365.0 - leap_count) as f64;

        if self.is_leap() {

        } 

    }

    fn is_leap(&self) -> bool {
        (self.year % 2) == 0
    }


}
