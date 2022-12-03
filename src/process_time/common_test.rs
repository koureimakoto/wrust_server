use std::{panic, time::Duration};

use crate::process_time::{common::months_counter, constants::{MONTHS_OF_THE_LEAP, MONTHS_OF_THE_COMMON}};

use super::{year_days_perc, curr_leap_counter, epoch_to_years, fetch_month_of_year};

#[test]
fn mask_of_year_days_percentage_max_limit() {
    let fake_year = 50.5515148521;
    let fake_year = year_days_perc(fake_year);
    assert!( fake_year < 1.0)
}

#[test]
fn mask_of_year_days_percentage_min_limit() {
    let fake_year = 50.5515148521;
    let fake_year = year_days_perc(fake_year);
    assert!( fake_year > 0.0)
}

#[test]
#[should_panic]
fn mask_of_year_days_percentage_min_limit_error() {
    let fake_year = -1.5;
    year_days_perc(fake_year);
}

#[test]
fn leap_years_counter_for_any_input() {
    assert_eq!((100.3 / 4.0), curr_leap_counter(100.3));
    assert_eq!(( 55.4 / 4.0), curr_leap_counter( 55.4));
    assert_eq!((13.85), curr_leap_counter( 55.4));
}

#[test]
fn convert_epoch_to_year() {
    let fake_year_seconds = 31536000;
    assert_eq!(
        (1670004134 as f64 / fake_year_seconds as f64),
        epoch_to_years(Duration::new(1670004134, 0))
    );

    let precision = 52.955483701 as f64 - epoch_to_years(Duration::new(1670004134, 0));
    assert!( precision < f64::EPSILON );
}

#[test]
fn months_of_the_leap_year_counter_first_half() {
    let months = &MONTHS_OF_THE_LEAP[0];
    let rule: u32 = 15;

    // January
    assert_eq!(1, months_counter(months,  &31));
    // July
    assert_eq!(7, months_counter(months, &212));

    // February .. July
    for (index, month) in months.iter().enumerate() {
        assert_eq!(
            2 + index as u8,
            months_counter(months, &(month - rule))
        )
    }
}


#[test]
fn months_of_the_leap_year_counter_second_half() {
    let months = &MONTHS_OF_THE_LEAP[1];
    let rule: u32 = 15;
    let shim:  u8 =  5;

    // July
    assert_eq!( 7, shim + months_counter(months, &212));
    // August
    assert_eq!( 8, shim + months_counter(months, &213));
    // December
    assert_eq!(12, shim + months_counter(months, &335));

    // July .. December
    for (index, month) in months.iter().enumerate() {
        assert_eq!(
            7 + index as u8,
            shim + months_counter(months, &(month - rule))
        )
    }

    // August .. December
    for (index, month) in months.iter().enumerate() {
        assert_eq!(
            8 + index as u8,
            shim + months_counter(months, &(month + rule))
        )
    }
}



#[test]
fn months_of_the_common_year_counter_first_half() {
    let months = &MONTHS_OF_THE_COMMON[0];
    let rule: u32 = 15;

    // January
    assert_eq!(1, months_counter(months,  &31));
    // July
    assert_eq!(7, months_counter(months, &211));

    // February .. July
    for (index, month) in months.iter().enumerate() {
        assert_eq!(
            2 + index as u8,
            months_counter(months, &(month - rule))
        )
    }
}


#[test]
fn months_of_the_leap_common_counter_second_half() {
    let months = &MONTHS_OF_THE_COMMON[1];
    let rule: u32 = 15;
    let shim:  u8 =  5;

    // July
    assert_eq!( 7, shim + months_counter(months, &211));
    // August
    assert_eq!( 8, shim + months_counter(months, &212));
    // December
    assert_eq!(12, shim + months_counter(months, &334));

    // July .. December
    for (index, month) in months.iter().enumerate() {
        assert_eq!(
            7 + index as u8,
            shim + months_counter(months, &(month - rule))
        )
    }

    // August .. December
    for (index, month) in months.iter().enumerate() {
        assert_eq!(
            8 + index as u8,
            shim + months_counter(months, &(month + rule))
        )
    }
}



#[test]
fn fetch_just_months_of_the_leap_year() {
    let months = &MONTHS_OF_THE_LEAP;

    assert_eq!( 1, fetch_month_of_year(true, &28, months));
    assert_ne!( 2, fetch_month_of_year(true, &28, months));

    assert_eq!( 7, fetch_month_of_year(true, &212, months));
    assert_ne!( 8, fetch_month_of_year(true, &212, months));

    assert_eq!( 8, fetch_month_of_year(true, &213, months));
    assert_ne!( 9, fetch_month_of_year(true, &213, months));

    assert_eq!( 12, fetch_month_of_year(true, &335, months));
    assert_ne!( 12, fetch_month_of_year(true, &334, months));
    assert_eq!( 12, fetch_month_of_year(true, &366, months));
}

#[test]
fn fetch_just_months_of_the_common_year() {
    let months = &MONTHS_OF_THE_COMMON;

    assert_eq!( 1, fetch_month_of_year(false, &28, months));
    assert_ne!( 2, fetch_month_of_year(false, &28, months));

    assert_eq!( 7, fetch_month_of_year(false, &211, months));
    assert_ne!( 8, fetch_month_of_year(false, &211, months));

    assert_eq!( 8, fetch_month_of_year(false, &212, months));
    assert_ne!( 9, fetch_month_of_year(false, &212, months));

    assert_eq!( 12, fetch_month_of_year(false, &334, months));
    assert_ne!( 12, fetch_month_of_year(false, &333, months));
    assert_eq!( 12, fetch_month_of_year(false, &365, months));
}

#[test]
#[should_panic]
fn fetch_just_panic_of_the_leap_year() {
    assert_eq!( 12, fetch_month_of_year(true, &367, &MONTHS_OF_THE_LEAP));
}

#[test]
#[should_panic]
fn fetch_just_panic_of_the_common_year() {
    assert_eq!( 12, fetch_month_of_year(true, &366, &MONTHS_OF_THE_COMMON));
}