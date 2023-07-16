plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0658]: `try` expression is experimental
  --> library/core/tests/future.rs:91:34
   |
91 |         assert!(Option::is_none(&try { join!(maybe_fut?, async { unreachable!() }) }));
   |
   = note: see issue #31436 <https://github.com/rust-lang/rust/issues/31436> for more information
   = help: add `#![feature(try_blocks)]` to the crate attributes to enable

