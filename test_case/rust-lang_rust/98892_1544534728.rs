rust
#![feature(inline_const)]

use std::mem::{swap, align_of, size_of};


#[no_mangle]
pub fn swap_strings(a: &mut String, b: &mut String){
    const {
        assert!(size_of::<String>() == 24);
        assert!(align_of::<String>() == 8);
    }
    swap(a, b);
}

pub struct Another{
    pub a: u16,
    pub b: u32,
    pub c: u16,
    pub d: u16
}

#[no_mangle]
pub fn swap_another(a: &mut Another, b: &mut Another){
    const {
        assert!(size_of::<Another>() == 12);
        assert!(align_of::<Another>() == 4);
    }
    swap(a, b);
}
