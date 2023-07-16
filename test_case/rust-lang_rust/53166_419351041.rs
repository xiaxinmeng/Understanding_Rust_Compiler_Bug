rust
#[cfg(feature = "std")]
use std::mem;
#[(cfg(not(feature = "std"))]
use core::mem;
