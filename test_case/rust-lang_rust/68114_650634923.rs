rust
#![feature(allow_internal_unstable)]

#[allow_internal_unstable(const_transmute)]
macro_rules! transmute {
    ($e: expr) => {
        std::mem::transmute($e)
    };
}

#[allow_internal_unstable(wrapping_next_power_of_two)]
macro_rules! wrapping_next_power_of_two {
    ($e: expr) => {
        $e.wrapping_next_power_of_two()
    };
}


fn g(a: u64) -> u64 {
    wrapping_next_power_of_two!(a)
}

const fn h(a: u64) -> i64 {
    unsafe { transmute!(a) }
}
