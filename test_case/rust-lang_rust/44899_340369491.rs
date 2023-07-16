
#![feature(core_intrinsics)]

extern crate core;
use core::intrinsics::assume;

#[inline(always)]
pub fn from_u32(v: u32) -> Option<u32> {
    if v >= 1 && v <= 2u32 {
        Some(v)
    } else {
        None
    }
}

pub fn bugging_function(v: bool) -> u32 {
    let input: u32 = if v { 1 } else { 2 };
    if let Some(nval) =  from_u32(!input) {
        unsafe {
            assume(nval >= 1);
            assume(nval <= 2);
        }
        return nval;
    }
    0u32
}
