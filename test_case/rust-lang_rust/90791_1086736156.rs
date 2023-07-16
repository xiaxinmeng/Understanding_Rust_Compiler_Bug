plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0004]: non-exhaustive patterns: `&_` not covered
    |
    |
332 |                     match self.cx.sess().target.arch.as_str() {
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `&_` not covered
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `&str`

For more information about this error, try `rustc --explain E0004`.
