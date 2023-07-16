rust
#![allow(ellipsis_inclusive_range_patterns)]

pub fn f() -> bool {
    let x = 123;
    match x {
        0...100 => true,
        _ => false,
    }
}
