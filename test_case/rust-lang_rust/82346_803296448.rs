plain
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error[E0599]: no method named `reserve` found for struct `BTreeMap<u32, VariableKind<ChalkRustInterner<'_>>>` in the current scope
   --> compiler/rustc_traits/src/chalk/lowering.rs:819:16
    |
819 |     parameters.reserve(named_parameters.len());
    |                ^^^^^^^ method not found in `BTreeMap<u32, VariableKind<ChalkRustInterner<'_>>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_traits`
