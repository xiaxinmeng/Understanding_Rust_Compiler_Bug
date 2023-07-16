 rust
// Re-export `{mem, ptr}` from either `std` or `core` as appropriate. This
// keeps the conditional logic on one place.
#[cfg(feature = "lib_no_std")]
pub use core::{mem, ptr};
#[cfg(not(feature = "lib_no_std"))]
pub use std::{mem, ptr};
