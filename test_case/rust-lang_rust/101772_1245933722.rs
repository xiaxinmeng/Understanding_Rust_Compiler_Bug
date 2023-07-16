plain
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error[E0308]: mismatched types
  --> compiler/rustc_attr/src/builtin.rs:26:5
   |
24 | pub fn rust_version_symbol() -> Symbol {
   |                                 ------ expected `Symbol` because of return type
25 |     let version = option_env!("CFG_VERSION").unwrap_or("<current>");
26 |     version.split(' ').next().unwrap()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Symbol`, found `&str`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_attr` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_attr` due to previous error
