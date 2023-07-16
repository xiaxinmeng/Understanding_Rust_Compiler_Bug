Rust
#![feature(thread_local_internals)]
use std::cell::RefCell;

pub const FOO: std::thread::LocalKey<RefCell<u32>> = __thread_local_inner!(@key pub BAR, RefCell<u32>, RefCell::new(1));

fn main() {
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });
    
    // BAR.with(|b| {
    //     assert_eq!(*b.borrow(), 1);
    //     *b.borrow_mut() = 2;
    // });
}
