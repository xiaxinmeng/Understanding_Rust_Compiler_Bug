
error: expected type, found `{`
 --> src/main.rs:3:5
  |
2 |     println!("x"): // note the accidental colon
  |                  - tried to parse a type due to this type ascription
3 |     println!("y");
  |     ^^^^^^^^^^^^^
  |     |
  |     expected type
  |     in this macro invocation
  |     this macro call doesn't expand to a type
  |
  = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
  = note: for more information, see https://github.com/rust-lang/rust/issues/23416
  = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
