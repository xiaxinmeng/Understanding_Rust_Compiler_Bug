rust
#![feature(const_panic)]
#![feature(const_precise_live_drops)] 

pub const fn unwrap<T>(this: Option<T>) -> T {
    match this {
        Some(val) => val,
        None => panic!("called `Option::unwrap()` on a `None` value"),
    }
}
