rs
macro_rules! long_tuple_arg {
    ([$($t:tt)*]#$($h:tt)*) => {
        long_tuple_arg!{[$($t)*$($t)*]$($h)*}
    };
    ([$([$t:tt $y:tt])*]) => {
        pub fn _f(($($t,)*): ($($y,)*)) {}
    }
}

long_tuple_arg!{[[_ u8]]########## ###}
