rust
#![feature(core_intrinsics)]

use std::sync::atomic::{AtomicUsize, AtomicIsize};
use std::sync::atomic::Ordering::*;
use std::intrinsics::atomic_nand;

#[test]
fn uint_nand() {
    let mut x = AtomicUsize::new(0xf731);
    unsafe {
        assert_eq!(atomic_nand(x.get_mut() as *mut usize, 0x137f), 0xf731);
    }
    assert_eq!(x.load(SeqCst), !(0xf731 & 0x137f));
}

#[test]
fn int_nand() {
    let mut x = AtomicIsize::new(0xf731);
    unsafe {
        assert_eq!(atomic_nand(x.get_mut() as *mut isize, 0x137f), 0xf731);
    }
    assert_eq!(x.load(SeqCst), !(0xf731 & 0x137f));
}
