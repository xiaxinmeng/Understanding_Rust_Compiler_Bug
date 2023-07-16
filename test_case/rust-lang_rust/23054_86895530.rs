 rust
#![crate_type="lib"]

pub use inner::blah as testing;

mod inner {
    pub fn blah() {}
}
