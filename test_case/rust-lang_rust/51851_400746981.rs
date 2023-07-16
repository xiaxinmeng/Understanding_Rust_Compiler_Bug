rust
#![feature(never_type)]

use std::sync::atomic;

struct ArcInner<T: ?Sized> {
    strong: atomic::AtomicUsize,

    // the value usize::MAX acts as a sentinel for temporarily "locking" the
    // ability to upgrade weak pointers or downgrade strong ones; this is used
    // to avoid races in `make_mut` and `get_mut`.
    weak: atomic::AtomicUsize,

    data: T,
}

fn main() {
    println!("{}", std::mem::size_of::<ArcInner<!>>());
}