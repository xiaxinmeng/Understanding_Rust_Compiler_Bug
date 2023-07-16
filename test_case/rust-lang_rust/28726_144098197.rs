 rust
#![feature(catch_panic)]

use std::thread;

pub unsafe fn try<R, F: FnOnce() -> R>(f: F) -> thread::Result<R> {
    let mut f = Some(f);
    let f = &mut f as *mut Option<F> as usize;
    thread::catch_panic(move || {
        (*(f as *mut Option<F>)).take().unwrap()()
    })
}
