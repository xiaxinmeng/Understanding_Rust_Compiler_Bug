plain
   Compiling clippy v0.1.70 (/checkout/src/tools/clippy)
error[E0531]: cannot find unit struct, unit variant or constant `format_args_nl` in module `sym`
   --> src/tools/clippy/clippy_utils/src/macros.rs:394:93
    |
394 |                 .any(|name| matches!(name, sym::const_format_args | sym::format_args | sym::format_args_nl))
    |                                                                                             ^^^^^^^^^^^^^^ help: a constant with a similar name exists: `format_args`
   ::: /checkout/compiler/rustc_span/src/symbol.rs:22:1
    |
22  | symbols! {
    | -------- similarly named constant `format_args` defined here
