plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: `#[label]` is not a valid attribute
    |
203 |     #[label]
    |     ^
    |
    |
    = help: only `primary_span`, `applicability` and `skip_arg` are valid field attributes
error: unreachable statement
   --> compiler/rustc_resolve/src/errors.rs:199:10
    |
199 |   #[derive(SessionSubdiagnostic)]
---
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(SessionSubdiagnostic)]`
    |
    = note: `-D unreachable-code` implied by `-D warnings`
error: could not compile `rustc_resolve` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_resolve` due to 2 previous errors
Build completed unsuccessfully in 0:02:05
