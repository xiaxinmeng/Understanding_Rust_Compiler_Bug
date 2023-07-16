plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0599]: no method named `canonicalize_hr_query_hack` found for reference `&'cx rustc_infer::infer::InferCtxt<'cx, 'tcx>` in the current scope
   --> compiler/rustc_trait_selection/src/traits/query/normalize.rs:252:22
    |
252 |                     .canonicalize_hr_query_hack(self.param_env.and(data), &mut orig_values);
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `canonicalize_query`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_trait_selection`
