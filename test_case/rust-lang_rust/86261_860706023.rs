
warning: attribute should be applied to a function or static
 --> src/ffi/mod.rs:3:1
  |
3 | #[no_mangle]
  | ^^^^^^^^^^^^
4 | #[cfg(feature="std")]
5 | use std::{thread,panic, io, boxed, any, string};
  | ------------------------------------------------ not a function or static
  |
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
