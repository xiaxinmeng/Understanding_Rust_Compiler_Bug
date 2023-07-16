plain
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0277]: `impl EarlyLintPass + 'static` cannot be sent between threads safely
    |
378 |     passes.push(Box::new(builtin_lints));
378 |     passes.push(Box::new(builtin_lints));
    |                 ^^^^^^^^^^^^^^^^^^^^^^^ `impl EarlyLintPass + 'static` cannot be sent between threads safely
    |
    = note: required for the cast from `impl EarlyLintPass + 'static` to the object type `dyn EarlyLintPass + Send`
    |
    |
372 |     builtin_lints: impl EarlyLintPass + 'static + std::marker::Send,


error[E0277]: `T` cannot be sent between threads safely
    |
348 |     passes.push(Box::new(builtin_lints));
348 |     passes.push(Box::new(builtin_lints));
    |                 ^^^^^^^^^^^^^^^^^^^^^^^ `T` cannot be sent between threads safely
    |
    = note: required for the cast from `T` to the object type `dyn LateLintPass<'_> + Send`
    |
    |
329 | pub(super) fn late_lint_mod<'tcx, T: LateLintPass<'tcx> + 'tcx + std::marker::Send>(


error[E0277]: `T` cannot be sent between threads safely
    |
379 |     passes.push(Box::new(builtin_lints));
379 |     passes.push(Box::new(builtin_lints));
    |                 ^^^^^^^^^^^^^^^^^^^^^^^ `T` cannot be sent between threads safely
    |
    = note: required for the cast from `T` to the object type `dyn LateLintPass<'_> + Send`
    |
    |
364 | fn late_lint_crate<'tcx, T: LateLintPass<'tcx> + 'tcx + std::marker::Send>(tcx: TyCtxt<'tcx>, builtin_lints: T) {

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_lint` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
