plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0004]: non-exhaustive patterns: `Bool`, `Char`, `Str` and 130 more not covered
   --> compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:843:19
843 |             match lang_item {
843 |             match lang_item {
    |                   ^^^^^^^^^ patterns `Bool`, `Char`, `Str` and 130 more not covered
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_hir::LangItem`

For more information about this error, try `rustc --explain E0004`.
