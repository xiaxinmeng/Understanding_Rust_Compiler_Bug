plain
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: the feature `extended_key_value_attributes` has been stable since 1.53.0 and no longer requires an attribute to enable
   |
11 | #![feature(extended_key_value_attributes)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `-D stable-features` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_codegen_llvm`

