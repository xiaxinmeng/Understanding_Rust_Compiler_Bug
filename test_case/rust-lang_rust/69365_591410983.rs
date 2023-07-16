rust
///! This is a simple crate with a few functions, one of which is `#[inline]`.
#![some_crate_attribute]

#![inline] // typo: a ! snuck in
fn foo() {
    // ...
}

// ... other functions, which aren't intended to be #[inline]
