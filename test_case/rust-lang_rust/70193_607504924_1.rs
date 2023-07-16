rust
//! A library for getting aligned vectors, slices, and arrays of bytes or any other type
//!
//! The `A*` structs are aligned as indicated and hold a single array of bytes of the same size
//! These implement an `Alignment` trait that can be used for always aligned byte arrays for faster
//! memory copies or copies by hardware. The arrays are accessible as slices using `as_bytes()` or
//! `as_bytes_mut()` or by dereferencing the struct.
