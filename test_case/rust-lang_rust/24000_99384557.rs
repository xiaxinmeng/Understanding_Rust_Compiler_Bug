 rust
fn test<T: Copy>(x: T) { x.clone(); }
fn main() { test([0; 1000]); }
