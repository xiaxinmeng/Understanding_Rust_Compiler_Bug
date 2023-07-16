plain
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error[E0603]: constant `TLV` is private
   --> compiler/rustc_middle/src/ty/context/tls.rs:54:28
    |
54  | use rustc_rayon_core::tlv::TLV;
    |
    |
note: the constant `TLV` is defined here
    |
151 | macro_rules! thread_local {
    | ------------------------- in this expansion of `thread_local!` (#1)
...
...
172 |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, $init);
...
181 | macro_rules! __thread_local_inner {
181 | macro_rules! __thread_local_inner {
    | --------------------------------- in this expansion of `$crate::__thread_local_inner!` (#2)
...
365 |         $(#[$attr])* $vis const $name: $crate::thread::LocalKey<$t> =
    |
    |
   ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/rustc-rayon-core-0.4.1/src/tlv.rs:6:1
    |
6   | thread_local!(pub(crate) static TLV: Cell<usize> = Cell::new(0));

    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
For more information about this error, try `rustc --explain E0603`.
