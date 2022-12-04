pub const YEAR_SECONDS   :  u32 = 31536000;
pub const COMMON_YEAR    :  u32 = 365;
pub const COMMON_MIDYEAR :  u32 = 211;
pub const LEAP_INTERVAL  :  f64 = 4.0;
pub const MAX_MONTHS_SIZE:  usize = 11;
pub const MONTHS_OF_THE_LEAP : [[u32; 5]; 2]= [
    [60 , 91 , 121, 152, 182],
    [213, 244, 274, 305, 335]
];

pub const MONTHS_OF_THE_COMMON : [[u32; 5]; 2]= [
    [59 , 90 , 120, 151, 181],
    [212, 243, 273, 304, 334]
];

pub const MONTHS_OF_THE_COMMON_YEAR : [u32; 12]= [
    31,   59,  90, 120, 151, 181,
    212, 243, 273, 304, 334, 365
];


#[cfg(test)]
#[path ="constants_test.rs"]
pub mod constants_test;