plain
...
74 | pub use rustc_infer::traits::*;
   |         ---------------------- the name `select` in the type namespace is introduced by the glob reexport here
   |
   = note: `-D local-binding-shadows-glob-reexport` implied by `-D warnings`
error: local binding shadows glob re-export
  --> compiler/rustc_trait_selection/src/traits/mod.rs:20:1
   |
   |
20 | mod util;
   | ^^^^^^^^^ but the local binding here shadows the name `util` in the type namespace
74 | pub use rustc_infer::traits::*;
   |         ---------------------- the name `util` in the type namespace is introduced by the glob reexport here

   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
