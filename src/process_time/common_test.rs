use std::{panic, time::Duration};

use crate::process_time::{common::months_counter, constants::MONTHS_OF_THE_LEAP};

use super::{year_days_perc, curr_leap_counter, epoch_to_years};

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
    const RULE: u32 = 15;

    // January
    assert_eq!(1, months_counter(&MONTHS_OF_THE_LEAP[0],  &31));
    // July
    assert_eq!(7, months_counter(&MONTHS_OF_THE_LEAP[0], &182));

    // February .. July
    for (index, month) in MONTHS_OF_THE_LEAP.iter().enumerate() {
        assert_eq!(
            2 + index as u8,
            months_counter(
                &MONTHS_OF_THE_LEAP[0],
                &(MONTHS_OF_THE_LEAP[0][index] - 1)
            )
        )
    }

    // February .. July
    for (index, month) in MONTHS_OF_THE_LEAP.iter().enumerate() {
        assert_eq!(
            2 + index as u8,
            months_counter(
                &MONTHS_OF_THE_LEAP[0],
                &(MONTHS_OF_THE_LEAP[0][index] - RULE)
            )
        )
    }

}


#[test]
fn months_of_the_leap_year_counter_second_half() {
    const RULE: u32 = 15;

    // July
    assert_eq!( 7, 5 + months_counter(&MONTHS_OF_THE_LEAP[1], &212));
    // August
    assert_eq!( 8, 5 + months_counter(&MONTHS_OF_THE_LEAP[1], &213));
    // December
    assert_eq!(12, 5 + months_counter(&MONTHS_OF_THE_LEAP[1], &335));

    // July .. December
    for (index, month) in MONTHS_OF_THE_LEAP.iter().enumerate() {
        assert_eq!(
            7 + index as u8,
            5 + months_counter(
                &MONTHS_OF_THE_LEAP[1],
                &(MONTHS_OF_THE_LEAP[1][index] - 1)
            )
        )
    }

    // July .. December
    for (index, month) in MONTHS_OF_THE_LEAP.iter().enumerate() {
        assert_eq!(
            7 + index as u8,
            5 + months_counter(
                &MONTHS_OF_THE_LEAP[1],
                &(MONTHS_OF_THE_LEAP[1][index] - RULE)
            )
        )
    }

    // August .. December
    for (index, month) in MONTHS_OF_THE_LEAP.iter().enumerate() {
        assert_eq!(
            8 + index as u8,
            5 + months_counter(
                &MONTHS_OF_THE_LEAP[1],
                &(MONTHS_OF_THE_LEAP[1][index] + RULE)
            )
        )
    }

}
