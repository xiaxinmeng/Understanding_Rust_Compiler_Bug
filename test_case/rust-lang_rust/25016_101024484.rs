 rust
#![feature(core)]

use std::slice;

fn foo<T>(v: &[T]) -> Option<&[T]> {
    let mut it = v.iter();
    for _ in 0..5 {
        let _ = it.next();
    }
    Some(it.as_slice())
}

fn main() {
    let v: &[()] = unsafe { slice::from_raw_parts(-5isize as *const (), 10) };
    assert!(foo(v).is_some());
}
