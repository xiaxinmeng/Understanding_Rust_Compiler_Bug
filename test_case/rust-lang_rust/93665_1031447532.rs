rust
error[[E0015]](https://doc.rust-lang.org/nightly/error-index.html#E0015): calls in constant functions are limited to constant functions, tuple structs and tuple variants
 [--> src/main.rs:2:20
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)  |
2 |     panic!("{:?}", 0);
  |                    ^
  |
  = note: this error originates in the macro `$crate::const_format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
