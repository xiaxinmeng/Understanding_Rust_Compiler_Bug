rust
#![feature(atomic_integers, const_fn)]
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

const unsafe fn transmute(x: *mut u8) -> usize {
    union T {
        a: *mut u8,
        b: usize
    }
    T { a: x }.b
}

const BAR: *mut u8 = ((|| 3) as fn() -> i32) as *mut u8;
const FOO: AtomicUsize = AtomicUsize::new(unsafe { transmute(BAR) });
// static FOO: AtomicUsize = AtomicUsize::new(unsafe { transmute(BAR) }); // ALSO OK

fn main() {
    let l = FOO.load(Ordering::Relaxed);
    let l: fn() -> i32 = unsafe { std::mem::transmute(l) };
    assert_eq!(l(), 3);
}
