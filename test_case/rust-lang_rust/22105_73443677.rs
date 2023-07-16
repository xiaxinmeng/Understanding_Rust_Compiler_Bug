 rust
#![feature(std_misc)]

fn main() {
    use std::num::Float;

    // Rules:    
    // - If `self < other`: `0`
    // - Otherwise `self - other`

    assert_eq!((-3.0f64).abs_sub(2.0), 0.0);
    assert_eq!((-3.0f64).abs_sub(-2.0), 0.0);
    assert_eq!((-1.0f64).abs_sub(-2.0), 1.0);
}
