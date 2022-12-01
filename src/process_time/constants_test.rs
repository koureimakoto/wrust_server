
// pub const FLOAT_MASK     :  f64 = f64::MAX ;

// pub const MOUTHS_OF_THE_LEAP : [[u32; 5]; 2]= [
//     [60 , 91 , 121, 152, 182],
//     [213, 244, 274, 305, 335]
// ];

// pub const MOUTHS_OF_THE_COMMON : [[u32; 5]; 2]= [
//     [59 , 90 , 120, 151, 181],
//     [212, 243, 273, 304, 334]
// ];

use super::{
    YEAR_SECOND,
    LEAP_INTERVAL,
    COMMON_YEAR,
    COMMON_MIDYEAR
};

#[test]
fn year_in_second_format() {
    assert_eq!(YEAR_SECOND, 31536000)
}

#[test]
fn leap_interval_pattern() {
    assert_eq!(LEAP_INTERVAL, 4)
}

#[test]
fn common_year_days_pattern() {
    assert_eq!(COMMON_YEAR, 365)
}

#[test]
fn common_midyear_days() {
    assert_eq!(COMMON_MIDYEAR, 211)
}