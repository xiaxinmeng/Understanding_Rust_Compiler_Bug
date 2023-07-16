
warning: panic message is not a string literal
  --> scratchpad.rs:23:27
   |
23 | assert!(version == "1.0", format!("test failure text: {}", version));
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
   = note: the panic!() macro supports formatting, so there's no need for the format!() macro here
help: remove the `format!(..)` macro call
   |
23 | assert!(version == "1.0", "test failure text: {}", version);
   |                          --                              --
