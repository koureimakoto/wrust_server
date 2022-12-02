//! Doc test to preserve the constants definitions
use super::{
    YEAR_SECONDS,
    COMMON_YEAR,
    LEAP_INTERVAL,
    COMMON_MIDYEAR, 
    MONTHS_OF_THE_LEAP,
    MONTHS_OF_THE_COMMON
};

#[test]
fn year_in_second_format() {
    assert_eq!(YEAR_SECONDS, 31536000)
}

#[test]
fn leap_interval_pattern() {
    assert_eq!(LEAP_INTERVAL, 4.0)
}

#[test]
fn common_year_days_pattern() {
    assert_eq!(COMMON_YEAR, 365)
}

#[test]
fn common_midyear_days() {
    assert_eq!(COMMON_MIDYEAR, 211)
}

#[test]
fn common_year_day_count_list() {
    let days_count_of_the_month_of_the_common_year : [[u32; 5]; 2]= [
        [59 , 90 , 120, 151, 181], [212, 243, 273, 304, 334]
    ];

    for days in MONTHS_OF_THE_COMMON[0].iter().enumerate() {
        assert_eq!(
            days.1, 
            &days_count_of_the_month_of_the_common_year[0][days.0]
        )
    }
}

#[test]
fn leap_year_day_count_list() {
    let days_count_of_the_month_of_the_leap_year : [[u32; 5]; 2]= [
        [60 , 91 , 121, 152, 182], [213, 244, 274, 305, 335]
    ];
    
    for days in MONTHS_OF_THE_LEAP[0].iter().enumerate() {
        assert_eq!(
            days.1, 
            &days_count_of_the_month_of_the_leap_year[0][days.0]
        )
    }
}