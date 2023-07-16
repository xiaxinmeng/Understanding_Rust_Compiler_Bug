rust
// crate A: compiled with -C target-feature=-sse
pub extern"C" fn foo(x: f32) -> f32 { x }

// crate B: compiled with -C target-feature=+sse
// this is now safe Rust code:
assert_eq!(A::foo(42.0), 42.0) }; // UB
