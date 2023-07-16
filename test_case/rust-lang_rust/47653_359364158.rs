rust
#![feature(box_into_raw_non_null)]
use std::ptr::NonNull;

fn main() {
    // Creating thin pointers
    let a: NonNull<[u8; 10]> = Box::into_raw_non_null(Box::new([42; 10]));
    let b: NonNull<String> = Box::into_raw_non_null(Box::new(String::new()));

    // Coercing to fat pointers to dynamically-sized types
    let _a: NonNull<[u8]> = a;
    let _b: NonNull<std::fmt::Display> = b;
}
