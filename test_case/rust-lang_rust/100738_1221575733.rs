plain
    Checking rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: `#[error(...)]` is not a valid attribute
   |
92 | #[error(const_eval::max_num_nodes_in_const)]
   | ^
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
92 | #[error(const_eval::max_num_nodes_in_const)]
   | ^
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
100 | #[error(const_eval::unallowed_fn_pointer_call)]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
100 | #[error(const_eval::unallowed_fn_pointer_call)]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
108 | #[error(const_eval::unstable_const_fn)]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
108 | #[error(const_eval::unstable_const_fn)]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
116 | #[error(const_eval::unallowed_mutable_refs, code = "E0764")]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
116 | #[error(const_eval::unallowed_mutable_refs, code = "E0764")]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
126 | #[error(const_eval::unallowed_mutable_refs_raw, code = "E0764")]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
126 | #[error(const_eval::unallowed_mutable_refs_raw, code = "E0764")]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
135 | #[error(const_eval::non_const_fmt_macro_call, code = "E0015")]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
135 | #[error(const_eval::non_const_fmt_macro_call, code = "E0015")]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
143 | #[error(const_eval::non_const_fn_call, code = "E0015")]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
143 | #[error(const_eval::non_const_fn_call, code = "E0015")]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
152 | #[error(const_eval::unallowed_op_in_const_context)]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
152 | #[error(const_eval::unallowed_op_in_const_context)]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
160 | #[error(const_eval::unallowed_heap_allocations, code = "E0010")]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
160 | #[error(const_eval::unallowed_heap_allocations, code = "E0010")]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
171 | #[error(const_eval::unallowed_inline_asm, code = "E0015")]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
171 | #[error(const_eval::unallowed_inline_asm, code = "E0015")]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
179 | #[error(const_eval::interior_mutable_data_refer, code = "E0492")]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
179 | #[error(const_eval::interior_mutable_data_refer, code = "E0492")]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
192 | #[error(const_eval::interior_mutability_borrow)]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
192 | #[error(const_eval::interior_mutability_borrow)]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`
error: cannot find attribute `error` in this scope
  --> compiler/rustc_const_eval/src/errors.rs:92:3
   |
92 | #[error(const_eval::max_num_nodes_in_const)]
92 | #[error(const_eval::max_num_nodes_in_const)]
   |   ^^^^^
   |
   = help: have you added the `#[macro_use]` on the module/import?
error: cannot find attribute `error` in this scope
   --> compiler/rustc_const_eval/src/errors.rs:100:3
    |
100 | #[error(const_eval::unallowed_fn_pointer_call)]
100 | #[error(const_eval::unallowed_fn_pointer_call)]
    |   ^^^^^
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find attribute `error` in this scope
   --> compiler/rustc_const_eval/src/errors.rs:108:3
    |
108 | #[error(const_eval::unstable_const_fn)]
108 | #[error(const_eval::unstable_const_fn)]
    |   ^^^^^
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find attribute `error` in this scope
   --> compiler/rustc_const_eval/src/errors.rs:116:3
    |
116 | #[error(const_eval::unallowed_mutable_refs, code = "E0764")]
116 | #[error(const_eval::unallowed_mutable_refs, code = "E0764")]
    |   ^^^^^
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find attribute `error` in this scope
   --> compiler/rustc_const_eval/src/errors.rs:126:3
    |
    |
126 | #[error(const_eval::unallowed_mutable_refs_raw, code = "E0764")]
    |
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find attribute `error` in this scope
   --> compiler/rustc_const_eval/src/errors.rs:135:3
    |
135 | #[error(const_eval::non_const_fmt_macro_call, code = "E0015")]
135 | #[error(const_eval::non_const_fmt_macro_call, code = "E0015")]
    |   ^^^^^
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find attribute `error` in this scope
   --> compiler/rustc_const_eval/src/errors.rs:143:3
    |
143 | #[error(const_eval::non_const_fn_call, code = "E0015")]
143 | #[error(const_eval::non_const_fn_call, code = "E0015")]
    |   ^^^^^
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find attribute `error` in this scope
   --> compiler/rustc_const_eval/src/errors.rs:152:3
    |
152 | #[error(const_eval::unallowed_op_in_const_context)]
152 | #[error(const_eval::unallowed_op_in_const_context)]
    |   ^^^^^
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find attribute `error` in this scope
   --> compiler/rustc_const_eval/src/errors.rs:160:3
    |
160 | #[error(const_eval::unallowed_heap_allocations, code = "E0010")]
160 | #[error(const_eval::unallowed_heap_allocations, code = "E0010")]
    |   ^^^^^
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find attribute `error` in this scope
   --> compiler/rustc_const_eval/src/errors.rs:171:3
    |
171 | #[error(const_eval::unallowed_inline_asm, code = "E0015")]
171 | #[error(const_eval::unallowed_inline_asm, code = "E0015")]
    |   ^^^^^
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find attribute `error` in this scope
   --> compiler/rustc_const_eval/src/errors.rs:179:3
    |
179 | #[error(const_eval::interior_mutable_data_refer, code = "E0492")]
179 | #[error(const_eval::interior_mutable_data_refer, code = "E0492")]
    |   ^^^^^
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find attribute `error` in this scope
   --> compiler/rustc_const_eval/src/errors.rs:192:3
    |
192 | #[error(const_eval::interior_mutability_borrow)]
192 | #[error(const_eval::interior_mutability_borrow)]
    |   ^^^^^
    |
    = help: have you added the `#[macro_use]` on the module/import?

error[E0277]: the trait bound `MaxNumNodesInConstErr: SessionDiagnostic<'_>` is not satisfied
    |
    |
80  |                         Some(span) => tcx.sess.create_err(MaxNumNodesInConstErr { span, s }),
    |                                                ---------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `MaxNumNodesInConstErr`
    |                                                required by a bound introduced by this call
    |
note: required by a bound in `Session::create_err`
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
    |
460 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_err`

error[E0277]: the trait bound `UnallowedFnPointerCall: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_const_eval/src/transform/check_consts/ops.rs:103:33
    |
103 |         ccx.tcx.sess.create_err(UnallowedFnPointerCall { span, kind: ccx.const_kind() })
    |                      ---------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `UnallowedFnPointerCall`
    |                      required by a bound introduced by this call
    |
note: required by a bound in `Session::create_err`
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
    |
460 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_err`

error[E0277]: the trait bound `NonConstFmtMacroCall: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_const_eval/src/transform/check_consts/ops.rs:311:41
    |
311 |                 ccx.tcx.sess.create_err(NonConstFmtMacroCall { span, kind: ccx.const_kind() })
    |                              ---------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `NonConstFmtMacroCall`
    |                              required by a bound introduced by this call
    |
note: required by a bound in `Session::create_err`
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
    |
460 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_err`

error[E0277]: the trait bound `NonConstFnCall: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_const_eval/src/transform/check_consts/ops.rs:313:42
    |
313 |               _ => ccx.tcx.sess.create_err(NonConstFnCall {
    |  _______________________________----------_^
    | |                               required by a bound introduced by this call
314 | |                 span,
314 | |                 span,
315 | |                 def_path_str: ccx.tcx.def_path_str_with_substs(callee, substs),
316 | |                 kind: ccx.const_kind(),
317 | |             }),
    | |_____________^ the trait `SessionDiagnostic<'_>` is not implemented for `NonConstFnCall`
note: required by a bound in `Session::create_err`
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
    |
    |
460 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_err`

error[E0277]: the trait bound `UnstableConstFn: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_const_eval/src/transform/check_consts/ops.rs:351:25
    |
351 |             .create_err(UnstableConstFn { span, def_path: ccx.tcx.def_path_str(def_id) });
    |              ---------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `UnstableConstFn`
    |              required by a bound introduced by this call
    |
note: required by a bound in `Session::create_err`
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
    |
460 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_err`

error[E0277]: the trait bound `UnallowedOpInConstContext: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_const_eval/src/transform/check_consts/ops.rs:387:17
    |
386 |             ccx.tcx.sess.create_feature_err(
    |                          ------------------ required by a bound introduced by this call
387 |                 UnallowedOpInConstContext { span, msg },
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `UnallowedOpInConstContext`
note: required by a bound in `Session::create_feature_err`
   --> /checkout/compiler/rustc_session/src/session.rs:466:19
    |
    |
466 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_feature_err`

error[E0277]: the trait bound `UnallowedOpInConstContext: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_const_eval/src/transform/check_consts/ops.rs:391:37
    |
391 |             ccx.tcx.sess.create_err(UnallowedOpInConstContext { span, msg })
    |                          ---------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `UnallowedOpInConstContext`
    |                          required by a bound introduced by this call
    |
note: required by a bound in `Session::create_err`
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
    |
460 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_err`

error[E0277]: the trait bound `UnallowedHeapAllocations: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_const_eval/src/transform/check_consts/ops.rs:404:33
    |
404 |           ccx.tcx.sess.create_err(UnallowedHeapAllocations {
    |  ______________________----------_^
    | |                      required by a bound introduced by this call
405 | |             span,
405 | |             span,
406 | |             kind: ccx.const_kind(),
407 | |             teach: ccx.tcx.sess.teach(&error_code!(E0010)).then_some(()),
408 | |         })
    | |_________^ the trait `SessionDiagnostic<'_>` is not implemented for `UnallowedHeapAllocations`
note: required by a bound in `Session::create_err`
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
    |
    |
460 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_err`

error[E0277]: the trait bound `UnallowedInlineAsm: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_const_eval/src/transform/check_consts/ops.rs:420:33
    |
420 |         ccx.tcx.sess.create_err(UnallowedInlineAsm { span, kind: ccx.const_kind() })
    |                      ---------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `UnallowedInlineAsm`
    |                      required by a bound introduced by this call
    |
note: required by a bound in `Session::create_err`
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
    |
460 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_err`

error[E0277]: the trait bound `InteriorMutabilityBorrow: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_const_eval/src/transform/check_consts/ops.rs:466:41
    |
466 |         ccx.tcx.sess.create_feature_err(InteriorMutabilityBorrow { span }, sym::const_refs_to_cell)
    |                      ------------------ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `InteriorMutabilityBorrow`
    |                      required by a bound introduced by this call
    |
note: required by a bound in `Session::create_feature_err`
   --> /checkout/compiler/rustc_session/src/session.rs:466:19
   --> /checkout/compiler/rustc_session/src/session.rs:466:19
    |
466 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_feature_err`

error[E0277]: the trait bound `InteriorMutableDataRefer: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_const_eval/src/transform/check_consts/ops.rs:483:37
    |
483 |               ccx.tcx.sess.create_err(InteriorMutableDataRefer {
    |  __________________________----------_^
    | |                          required by a bound introduced by this call
484 | |                 span,
485 | |                 opt_help: Some(()),
485 | |                 opt_help: Some(()),
486 | |                 kind: ccx.const_kind(),
487 | |                 teach: ccx.tcx.sess.teach(&error_code!(E0492)).then_some(()),
488 | |             })
    | |_____________^ the trait `SessionDiagnostic<'_>` is not implemented for `InteriorMutableDataRefer`
note: required by a bound in `Session::create_err`
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
    |
    |
460 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_err`

error[E0277]: the trait bound `InteriorMutableDataRefer: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_const_eval/src/transform/check_consts/ops.rs:490:37
    |
490 |               ccx.tcx.sess.create_err(InteriorMutableDataRefer {
    |  __________________________----------_^
    | |                          required by a bound introduced by this call
491 | |                 span,
492 | |                 opt_help: None,
492 | |                 opt_help: None,
493 | |                 kind: ccx.const_kind(),
494 | |                 teach: ccx.tcx.sess.teach(&error_code!(E0492)).then_some(()),
495 | |             })
    | |_____________^ the trait `SessionDiagnostic<'_>` is not implemented for `InteriorMutableDataRefer`
note: required by a bound in `Session::create_err`
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
    |
    |
460 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_err`

error[E0277]: the trait bound `UnallowedMutableRefsRaw: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_const_eval/src/transform/check_consts/ops.rs:523:61
    |
523 |               hir::BorrowKind::Raw => ccx.tcx.sess.create_err(UnallowedMutableRefsRaw {
    |  __________________________________________________----------_^
    | |                                                  required by a bound introduced by this call
524 | |                 span,
524 | |                 span,
525 | |                 kind: ccx.const_kind(),
526 | |                 teach: ccx.tcx.sess.teach(&error_code!(E0764)).then_some(()),
527 | |             }),
    | |_____________^ the trait `SessionDiagnostic<'_>` is not implemented for `UnallowedMutableRefsRaw`
note: required by a bound in `Session::create_err`
   --> /checkout/compiler/rustc_session/src/session.rs:460:19
    |
    |
460 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_err`
