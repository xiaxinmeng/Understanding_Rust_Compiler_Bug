 rust
// src/lib.rs
#![no_std]
#[cfg(feature = "foo")]
extern crate std;

use core::{ ... };

#[cfg(feature = "foo")]
mod foo;

// src/foo.rs

use std::prelude::v1::*;

// .. code using libstd
