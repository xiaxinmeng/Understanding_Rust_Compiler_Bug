rust
#![feature(overlapping_marker_traits)]

pub trait F {}
impl<T> F for T where T: Copy {}
impl<T> F for T where T: 'static {}
