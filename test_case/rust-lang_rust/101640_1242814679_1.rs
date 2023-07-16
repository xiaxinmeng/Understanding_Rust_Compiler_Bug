
$ cargo +stable build --target thumbv6m-none-eabi
   Compiling once_cell v1.14.0 (/home/jan/tmp/once_cell)
error[E0463]: can't find crate for `std`
  |
  = note: the `thumbv6m-none-eabi` target may not support the standard library
  = note: `std` is required by `once_cell` because it does not declare `#![no_std]`

error[E0463]: can't find crate for `std`
   --> src/lib.rs:354:9
    |
354 |     use std::panic::{RefUnwindSafe, UnwindSafe};
