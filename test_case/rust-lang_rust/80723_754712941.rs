plain
   Compiling chalk-engine v0.36.0
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: call to noop method
   --> compiler/rustc_middle/src/ty/error.rs:537:36
537 |                   let target_spans = attrs
    |  ____________________________________^
538 | |                     .deref()
    | |____________________________^
    | |____________________________^
    |
    = note: `-D noop-method-call` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_middle`

