
#![feature(const_fn_union)]

union Transmute {
    e: u32,
}

pub const fn transpose() -> u32 {
    unsafe { Transmute { e: 0 }.e }
}
