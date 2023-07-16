plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: diagnostics should be created using translatable messages
   --> compiler/rustc_expand/src/tests.rs:157:17
    |
157 |         handler.span_err(msp, "foo");
    |
note: the lint level is defined here
   --> compiler/rustc_expand/src/lib.rs:13:9
    |
    |
13  | #![deny(rustc::untranslatable_diagnostic)]

error: could not compile `rustc_expand` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:20
