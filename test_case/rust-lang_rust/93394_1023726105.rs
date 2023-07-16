plain
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: invalid reference to positional argument 1 (no arguments were given)
   --> compiler/rustc_resolve/src/late/diagnostics.rs:962:60
    |
962 |                         format!("{path_str} {{{pad}{fields}{pad}}}"),
    |
    = note: positional arguments are zero-based

error: could not compile `rustc_resolve` due to previous error
