plain
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0309]: the parameter type `Bx` may not live long enough
    |
405 |         .collect::<Vec<_>>();
405 |         .collect::<Vec<_>>();
    |          ^^^^^^^ ...so that the type `Bx` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
    |
    |
279 | fn arg_local_refs<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx> + 'a>(

error: lifetime may not live long enough
   --> compiler/rustc_codegen_ssa/src/mir/mod.rs:417:13
    |
    |
279 | fn arg_local_refs<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx>>(
    |                   --  ---- lifetime `'tcx` defined here
    |                   |
    |                   lifetime `'a` defined here
...
417 |             "#[track_caller] instance {:?} must have 1 more argument in their ABI than in their MIR",
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'tcx` must outlive `'a`
    |
    = help: consider adding the following bound: `'tcx: 'a`
error: lifetime may not live long enough
   --> compiler/rustc_codegen_ssa/src/mir/block.rs:768:23
    |
    |
293 | impl<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx>> FunctionCx<'a, 'tcx, Bx> {
    |      --  ---- lifetime `'tcx` defined here
    |      |
    |      lifetime `'a` defined here
...
768 |             _ => bug!("{} is not callable", callee.layout.ty),
    |                       ^^^^^^^^^^^^^^^^^^^^ requires that `'tcx` must outlive `'a`
    |
    = help: consider adding the following bound: `'tcx: 'a`

error[E0309]: the parameter type `Bx` may not live long enough
    |
885 |                 Self::codegen_intrinsic_call(
885 |                 Self::codegen_intrinsic_call(
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `Bx` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
    |
    |
293 | impl<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx> + 'a> FunctionCx<'a, 'tcx, Bx> {

error: lifetime may not live long enough
   --> compiler/rustc_codegen_ssa/src/mir/debuginfo.rs:271:59
    |
    |
208 | impl<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx>> FunctionCx<'a, 'tcx, Bx> {
    |      --  ---- lifetime `'tcx` defined here
    |      |
    |      lifetime `'a` defined here
...
271 |         let full_debug_info = bx.sess().opts.debuginfo == DebugInfo::Full;
    |                                                           ^^^^^^^^^^^^^^^ requires that `'tcx` must outlive `'a`
    |
    = help: consider adding the following bound: `'tcx: 'a`

error[E0309]: the parameter type `Bx` may not live long enough
    |
    |
375 |                 Self::spill_operand_to_stack(operand, name, bx)
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `Bx` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
    |
    |
208 | impl<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx> + 'a> FunctionCx<'a, 'tcx, Bx> {

error: lifetime may not live long enough
   --> compiler/rustc_codegen_ssa/src/mir/debuginfo.rs:470:64
    |
    |
208 | impl<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx>> FunctionCx<'a, 'tcx, Bx> {
    |      --  ---- lifetime `'tcx` defined here
    |      |
    |      lifetime `'a` defined here
...
470 |         let full_debug_info = self.cx.sess().opts.debuginfo == DebugInfo::Full;
    |                                                                ^^^^^^^^^^^^^^^ requires that `'tcx` must outlive `'a`
    |
    = help: consider adding the following bound: `'tcx: 'a`

error[E0309]: the parameter type `Bx` may not live long enough
    |
549 | ...                   let base = Self::spill_operand_to_stack(
549 | ...                   let base = Self::spill_operand_to_stack(
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `Bx` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
    |
    |
208 | impl<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx> + 'a> FunctionCx<'a, 'tcx, Bx> {

For more information about this error, try `rustc --explain E0309`.
error: could not compile `rustc_codegen_ssa` (lib) due to 8 previous errors
warning: build failed, waiting for other jobs to finish...
