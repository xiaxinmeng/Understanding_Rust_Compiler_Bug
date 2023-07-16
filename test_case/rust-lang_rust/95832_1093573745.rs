plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0433]: failed to resolve: use of undeclared type `UintTy`
  --> compiler/rustc_codegen_ssa/src/glue.rs:43:39
   |
43 |                 bx.tcx().mk_mach_uint(UintTy::Usize),
   |                                       ^^^^^^ use of undeclared type `UintTy`
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0308]: mismatched types
  --> compiler/rustc_codegen_ssa/src/glue.rs:44:17
   |
   |
44 |                 unit.size.bytes(),
   |                 ^^^^^^^^^^^^^^^^^ expected associated type, found `u64`
   |
   = note: expected associated type `<Bx as BackendTypes>::Value`
                         found type `u64`
   = help: consider constraining the associated type `<Bx as BackendTypes>::Value` to `u64` or calling a method that returns `<Bx as BackendTypes>::Value`

error[E0308]: mismatched types
  --> compiler/rustc_codegen_ssa/src/glue.rs:48:52
   |
   |
48 |                 bx.select(overflow, bx.const_usize(usize::MAX), size),
   |                                                    ^^^^^^^^^^ expected `u64`, found `usize`
   |
help: you can convert a `usize` to a `u64` and panic if the converted value doesn't fit
   |
48 |                 bx.select(overflow, bx.const_usize(usize::MAX.try_into().unwrap()), size),

Some errors have detailed explanations: E0308, E0433.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_ssa` due to 3 previous errors
