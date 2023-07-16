
.../work/rocket/rust-lang-66251 (master=) ♥ RUSTFLAGS='--remap-path-prefix=/home/jeb/work=oops' cargo build
   Compiling crate2 v0.1.0 (/home/jeb/work/rocket/rust-lang-66251/crate2)
   Compiling rust-lang-66251 v0.1.0 (/home/jeb/work/rocket/rust-lang-66251)
error[E0277]: `*const ()` doesn't implement `std::fmt::Display`
 --> src/main.rs:2:27
  |
2 |     crate2::wants_display(&() as *const ());
  |                           ^^^^^^^^^^^^^^^^ `*const ()` cannot be formatted with the default formatter
  |
  = help: the trait `std::fmt::Display` is not implemented for `*const ()`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rust-lang-66251`

To learn more, run the command again with --verbose.
