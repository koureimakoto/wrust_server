use std::{panic, time::Duration};

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

    let float_cmp = 52.955483701 as f64 - epoch_to_years(Duration::new(1670004134, 0));
    assert!( float_cmp < f64::EPSILON );
}
