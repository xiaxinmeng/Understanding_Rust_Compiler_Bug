plain
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
   --> compiler/rustc_resolve/src/late/lifetimes.rs:280:1
    |
280 | const ROOT_SCOPE: ScopeRef<'static> = &Scope::Root;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0492`.
