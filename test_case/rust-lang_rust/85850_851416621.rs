plain
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error[E0658]: use of unstable library feature 'array_windows'
   --> compiler/rustc_parse/src/parser/attr_wrapper.rs:128:84
    |
128 |                 for [(range, tokens), (next_range, next_tokens)] in replace_ranges.array_windows() {
    |
    = note: see issue #75027 <https://github.com/rust-lang/rust/issues/75027> for more information
    = help: add `#![feature(array_windows)]` to the crate attributes to enable

