
error: there is no argument named `x`
  --> /home/archie/Projects/RustProjects/rust/src/test/ui/macros/unreachable-2021.rs:8:5
   |
LL |     unreachable!("x is {x}");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: did you intend to capture a variable `x` from the surrounding scope?
   = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
   = note: this error originates in the macro `$crate::concat` (in Nightly builds, run with -Z macro-backtrace for more info)
