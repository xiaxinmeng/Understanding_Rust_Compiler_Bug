plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: 2 positional arguments in format string, but there is 1 argument
   --> compiler/rustc_metadata/src/locator.rs:921:55
    |
921 |                         let mut s = format!("\ncrate `{}`: {}", paths.next().unwrap().display());
    |                                                       ^^   ^^   -------------------------------
error[E0599]: no method named `write_fmt` found for struct `std::string::String` in the current scope
   --> /checkout/library/core/src/macros/mod.rs:478:39
    |
477 | / macro_rules! write {
477 | / macro_rules! write {
478 | |     ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))
479 | | }
    | |_- in this expansion of `write!`
    |
   ::: /checkout/library/core/src/fmt/mod.rs:185:8
   ::: /checkout/library/core/src/fmt/mod.rs:185:8
    |
185 |       fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> Result {
    |          --------- the method is available for `std::string::String` here
   ::: compiler/rustc_metadata/src/locator.rs:924:29
    |
    |
924 |   ...                   write!(s, "\n{:>padding$}", path, padding = padding).unwrap();
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use std::fmt::Write;`
            `use std::fmt::Write;`

error[E0277]: `PathBuf` doesn't implement `std::fmt::Display`
   --> compiler/rustc_metadata/src/locator.rs:924:57
    |
924 |   ...                   write!(s, "\n{:>padding$}", path, padding = padding).unwrap();
    |                         |                           |
    |                         |                           `PathBuf` cannot be formatted with the default formatter
    |                         in this macro invocation (#1)
    |
    |
   ::: /checkout/library/core/src/macros/mod.rs:477:1
    |
477 | / macro_rules! write {
478 | |     ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))
479 | | }
    | |_- in this expansion of `write!` (#1)
...
835 | /     macro_rules! format_args {
835 | /     macro_rules! format_args {
836 | |         ($fmt:expr) => {{ /* compiler built-in */ }};
837 | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |_____- in this expansion of `$crate::format_args!` (#2)
    |
    = help: the trait `std::fmt::Display` is not implemented for `PathBuf`
    = help: the trait `std::fmt::Display` is not implemented for `PathBuf`
    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    = note: required because of the requirements on the impl of `std::fmt::Display` for `&PathBuf`
   --> /checkout/library/core/src/fmt/mod.rs:738:5
    |
    |
738 |     fn fmt(&self, f: &mut Formatter<'_>) -> Result;

error[E0308]: mismatched types
   --> compiler/rustc_metadata/src/locator.rs:926:25
    |
    |
926 |                         s
    |                         ^
    |                         |
    |                         expected enum `Option`, found struct `std::string::String`
    |                         help: try using a variant of the expected enum: `Some(s)`
    = note: expected enum `Option<_>`
             found struct `std::string::String`

Some errors have detailed explanations: E0277, E0308, E0599.
