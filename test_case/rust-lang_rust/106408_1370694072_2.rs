
error: there is no argument named `x`
 --> src/main.rs:2:8
  |
2 |   dbg!(procmacro::foo!());
  |        ^^^^^^^^^^^^^^^^^
  |
  = note: did you intend to capture a variable `x` from the surrounding scope?
  = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
  = note: this error originates in the macro `procmacro::foo` (in Nightly builds, run with -Z macro-backtrace for more info)
