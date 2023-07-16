Rust
#![crate_type="lib"]

use std::slice::Iter;

// weird codegen (aliasing / None vs null bug?)
pub fn is_empty_1(xs: Iter<f32>) -> bool {
    {xs}.next().is_none()
}

// good codegen
pub fn is_empty_2(xs: Iter<f32>) -> bool {
    xs.map(|&x| x).next().is_none()
}
