rust
#[macro_export]
#[doc(hidden)]
macro_rules! foo {
    () => {};
}

pub use crate::foo as Macro; // this one isn't displayed, as expected
#[doc(inline)]
pub use crate::foo as Macro2; // this one isn't displayed but should be!
