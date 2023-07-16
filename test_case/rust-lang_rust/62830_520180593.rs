rust
pub mod x {}
pub fn x() {}

/// [module@x]
///
/// [module@x]: crate::rustdoc_workaround::x_module
pub struct S;

// https://github.com/rust-lang/rust/issues/62830
mod rustdoc_workaround {
    pub use crate::x::{self as x_module};
    //             ^^^^^^^^ no ambiguity with function
}
