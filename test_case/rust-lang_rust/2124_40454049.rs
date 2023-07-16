 rust
#[phase(syntax)] extern crate bindgen;

#[bindgen("foo.h")] // fills the module with the appropriate `extern` block
mod foo {}
