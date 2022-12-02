#![feature(external_doc)]

pub const YEAR_SECOND    :  u32 = 31536000;
pub const LEAP_INTERVAL  :  u32 = 4;
pub const COMMON_YEAR    :  u32 = 365;
pub const COMMON_MIDYEAR :  u32 = 211;
pub const FLOAT_MASK     :  f64 = f64::MAX ;

pub const MONTHS_OF_THE_LEAP : [[u32; 5]; 2]= [
    [60 , 91 , 121, 152, 182],
    [213, 244, 274, 305, 335]
];

pub const MONTHS_OF_THE_COMMON : [[u32; 5]; 2]= [
    [59 , 90 , 120, 151, 181],
    [212, 243, 273, 304, 334]
];

#[cfg(test)]
#[path ="constants_test.rs"]
pub mod constants_test;