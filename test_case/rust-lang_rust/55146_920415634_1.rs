   Compiling util_proc v0.1.0 ( .. util_proc)
   Compiling util v0.1.0 ( .. util)
error: expected one of `!`, `,`, `.`, `::`, `?`, `{`, `}`, or an operator, found reserved identifier `_`
   --> src/lib.rs:817:18
    |
817 |         #[derive(EnumError, Debug)]
    |                  ^^^^^^^^^
    |                  |
    |                  expected one of 8 possible tokens
    |                  while parsing the `match` arm starting here
    |
    = note: this error originates in the derive macro `EnumError` (in Nightly builds, run with -Z macro-backtrace for more info)

error: proc-macro derive produced unparseable tokens
   --> src/lib.rs:817:18
    |
817 |         #[derive(EnumError, Debug)]
    |                  ^^^^^^^^^

error: expected `,`, found `;`
   --> src/lib.rs:817:18
    |
817 |         #[derive(EnumError, Debug)]
    |                  ^^^^^^^^^ expected `,`
    |
    = note: this error originates in the derive macro `EnumError` (in Nightly builds, run with -Z macro-backtrace for more info)

error: format argument must be a string literal
   --> src/lib.rs:817:18
    |
817 |         #[derive(EnumError, Debug)]
    |                  ^^^^^^^^^
    |
    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
help: you might be missing a string literal to format with
    |
817 |         #[derive("{}", EnumError, Debug)]
    |                  +++++
