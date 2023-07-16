rust
// compile-flags: --document-private-items --document-hidden-items
#![feature(no_core)]
#![no_core]

pub trait TheTrait {}

#[doc(hidden)]
struct Value {}

impl TheTrait for Value {}
