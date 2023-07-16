`rust
use std::fmt::Debug;
use std::ptr;

fn main() {
    let a: *const dyn Debug = &1 as &dyn Debug;
    let b: *const dyn Debug = &1 as &dyn Debug;
    let _ = a < b;
}
