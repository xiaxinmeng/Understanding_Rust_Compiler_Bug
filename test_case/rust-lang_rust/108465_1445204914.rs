plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0425]: cannot find value `infcx` in this scope
  --> compiler/rustc_trait_selection/src/solve/trait_goals/structural_traits.rs:29:28
   |
29 |         ty::Str => Ok(vec![infcx.tcx.mk_slice(infcx.tcx.types.u8)]),

error[E0425]: cannot find value `infcx` in this scope
  --> compiler/rustc_trait_selection/src/solve/trait_goals/structural_traits.rs:29:47
   |
   |
29 |         ty::Str => Ok(vec![infcx.tcx.mk_slice(infcx.tcx.types.u8)]),

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_trait_selection` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
