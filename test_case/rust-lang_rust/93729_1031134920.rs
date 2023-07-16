plain
169 |             1..255 => Self::FAILURE,
    |             ^^^^^^
    |
    = note: see issue #37854 <https://github.com/rust-lang/rust/issues/37854> for more information
    = help: add `#![feature(exclusive_range_pattern)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
[RUSTC-TIMING] std test:false 1.271
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to previous error; 1 warning emitted
