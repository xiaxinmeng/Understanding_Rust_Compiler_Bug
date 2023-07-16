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

#[repr(C, packed)]
pub struct LargePacked {
    a: bool,
    b: u64,
    c: u32,
    d: u64,
    e: [bool;2],
    f: u16,
    g: (usize, usize),
}

#[no_mangle]
pub fn swap_large_packed(a: &mut LargePacked, b: &mut LargePacked){
    const {
        assert!(size_of::<LargePacked>() == 41);
        assert!(align_of::<LargePacked>() == 1);
    }
    swap(a, b);
}

#[repr(C, packed)]
pub struct SmallPacked {
    a: bool,
    b: u64,
}

#[no_mangle]
pub fn swap_small_packed(a: &mut SmallPacked, b: &mut SmallPacked){
    const {
        assert!(size_of::<SmallPacked>() == 9);
        assert!(align_of::<SmallPacked>() == 1);
    }
    swap(a, b);
}
