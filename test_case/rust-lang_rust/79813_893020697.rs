
warning: trailing semicolon in macro used in expression position
   --> src/macros.rs:5:63
    |
5   |         return $crate::private::Err($crate::format_err!($msg));
    |                                                               ^
    |
   ::: src/utils/date.rs:100:18
    |
100 |             _ => bail!("Invalid Month"),
    |                  ---------------------- in this macro invocation
    |
    = note: `#[warn(semicolon_in_expressions_from_macros)]` on by default
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
    = note: this warning originates in the macro `bail` (in Nightly builds, run with -Z macro-backtrace for more info)
