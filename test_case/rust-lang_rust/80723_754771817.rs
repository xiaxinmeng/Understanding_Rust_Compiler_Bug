plain
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
   Compiling rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: call to noop method
   |
   |
54 |                 kind: TestKind::Eq { value, ty: match_pair.pattern.ty.clone() },
   |
   |
   = note: `-D noop-method-call` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_mir_build`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: call to noop method
   --> compiler/rustc_typeck/src/check/callee.rs:449:13
    |
449 |             fn_sig.output().clone(),
    |
    |
    = note: `-D noop-method-call` implied by `-D warnings`

error: call to noop method
    |
    |
713 |         let return_expr_ty = self.check_expr_with_hint(return_expr, ret_ty.clone());

error: aborting due to 2 previous errors

error: build failed
