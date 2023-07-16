rust
// no_std
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core; // this core is hygienic
#[macro_use]
extern crate compiler_builtins;  // this compiler_builtins is hygienic
