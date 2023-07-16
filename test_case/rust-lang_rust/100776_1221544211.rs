plain
    Checking rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: `#[error(...)]` is not a valid attribute
  |
  |
7 | #[error(lint::overruled_attribute, code = "E0453")]
  |
  |
  = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  |
  |
7 | #[error(lint::overruled_attribute, code = "E0453")]
  |
  |
  = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
46 | #[error(lint::malformed_attribute, code = "E0452")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
46 | #[error(lint::malformed_attribute, code = "E0452")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
65 | #[error(lint::unknown_tool_in_scoped_lint, code = "E0710")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
65 | #[error(lint::unknown_tool_in_scoped_lint, code = "E0710")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
76 | #[error(lint::builtin_ellipsis_inclusive_range_patterns, code = "E0783")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
76 | #[error(lint::builtin_ellipsis_inclusive_range_patterns, code = "E0783")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
111 | #[error(lint::unsupported_group, code = "E0602")]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
111 | #[error(lint::unsupported_group, code = "E0602")]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
140 | #[error(lint::check_name_unknown_tool, code = "E0602")]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
140 | #[error(lint::check_name_unknown_tool, code = "E0602")]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[warning(...)]` is not a valid attribute
    |
    |
148 | #[warning(lint::check_name_warning)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
148 | #[warning(lint::check_name_warning)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[warning(...)]` is not a valid attribute
    |
    |
156 | #[warning(lint::check_name_deprecated)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
156 | #[warning(lint::check_name_deprecated)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`
error: cannot find attribute `error` in this scope
 --> compiler/rustc_lint/src/errors.rs:7:3
  |
  |
7 | #[error(lint::overruled_attribute, code = "E0453")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_lint/src/errors.rs:46:3
   |
   |
46 | #[error(lint::malformed_attribute, code = "E0452")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_lint/src/errors.rs:65:3
   |
   |
65 | #[error(lint::unknown_tool_in_scoped_lint, code = "E0710")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_lint/src/errors.rs:76:3
   |
   |
76 | #[error(lint::builtin_ellipsis_inclusive_range_patterns, code = "E0783")]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_lint/src/errors.rs:111:3
    |
    |
111 | #[error(lint::unsupported_group, code = "E0602")]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_lint/src/errors.rs:140:3
    |
    |
140 | #[error(lint::check_name_unknown_tool, code = "E0602")]


error: cannot find attribute `warning` in this scope
    |
    |
148 | #[warning(lint::check_name_warning)]


error: cannot find attribute `warning` in this scope
    |
    |
156 | #[warning(lint::check_name_deprecated)]


error[E0277]: the trait bound `BuiltinEllpisisInclusiveRangePatterns: SessionDiagnostic<'_>` is not satisfied
     |
     |
1767 |                       cx.sess().emit_err(BuiltinEllpisisInclusiveRangePatterns {
     |  _______________________________--------_^
     | |                               required by a bound introduced by this call
     | |                               required by a bound introduced by this call
1768 | |                         span: pat.span,
1769 | |                         suggestion: pat.span,
1771 | |                     });
1771 | |                     });
     | |_____________________^ the trait `SessionDiagnostic<'_>` is not implemented for `BuiltinEllpisisInclusiveRangePatterns`
     |
note: required by a bound in `Session::emit_err`
     |
     |
473  |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
     |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `BuiltinEllpisisInclusiveRangePatterns: SessionDiagnostic<'_>` is not satisfied
     |
     |
1787 |                       cx.sess().emit_err(BuiltinEllpisisInclusiveRangePatterns {
     |  _______________________________--------_^
     | |                               required by a bound introduced by this call
     | |                               required by a bound introduced by this call
1788 | |                         span: pat.span,
1789 | |                         suggestion: join,
1790 | |                         replace: replace.to_string(),
1791 | |                     });
     | |_____________________^ the trait `SessionDiagnostic<'_>` is not implemented for `BuiltinEllpisisInclusiveRangePatterns`
     |
note: required by a bound in `Session::emit_err`
     |
     |
473  |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
     |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `UnsupportedGroup: SessionDiagnostic<'_>` is not satisfied
    |
    |
336 |             sess.emit_err(UnsupportedGroup { lint_group: crate::WARNINGS.name_lower() });
    |                  -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `UnsupportedGroup`
    |                  required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `CheckNameWarning: SessionDiagnostic<'_, ()>` is not satisfied
    |
    |
342 |                   sess.emit_warning(CheckNameWarning {
    |  ______________________------------_^
    | |                      required by a bound introduced by this call
343 | |                     msg,
343 | |                     msg,
344 | |                     sub: RequestedLevel { level, lint_name },
345 | |                 });
    | |_________________^ the trait `SessionDiagnostic<'_, ()>` is not implemented for `CheckNameWarning`
    |
note: required by a bound in `Session::emit_warning`
    |
    |
482 |     pub fn emit_warning<'a>(&'a self, warning: impl SessionDiagnostic<'a, ()>) {
    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_warning`

error[E0277]: the trait bound `CheckNameDeprecated: SessionDiagnostic<'_, ()>` is not satisfied
    |
    |
356 |                       sess.emit_warning(CheckNameDeprecated {
    |  __________________________------------_^
    | |                          required by a bound introduced by this call
357 | |                         lint_name: lint_name.clone(),
358 | |                         new_name,
358 | |                         new_name,
359 | |                         sub: RequestedLevel { level, lint_name },
360 | |                     });
    | |_____________________^ the trait `SessionDiagnostic<'_, ()>` is not implemented for `CheckNameDeprecated`
    |
note: required by a bound in `Session::emit_warning`
    |
    |
482 |     pub fn emit_warning<'a>(&'a self, warning: impl SessionDiagnostic<'a, ()>) {
    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_warning`

error[E0277]: the trait bound `CheckNameUnknownTool: SessionDiagnostic<'_>` is not satisfied
    |
    |
364 |                   sess.emit_err(CheckNameUnknownTool {
    |  ______________________--------_^
    | |                      required by a bound introduced by this call
365 | |                     tool_name: tool_name.unwrap(),
365 | |                     tool_name: tool_name.unwrap(),
366 | |                     sub: RequestedLevel { level, lint_name },
367 | |                 });
    | |_________________^ the trait `SessionDiagnostic<'_>` is not implemented for `CheckNameUnknownTool`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `OverruledAttribute: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_lint/src/levels.rs:197:40
    |
197 |                       self.sess.emit_err(OverruledAttribute {
    |  _______________________________--------_^
    | |                               required by a bound introduced by this call
    | |                               required by a bound introduced by this call
198 | |                         span: src.span(),
199 | |                         overruled: src.span(),
200 | |                         lint_level: level.as_str().to_string(),
215 | |                         },
216 | |                     });
216 | |                     });
    | |_____________________^ the trait `SessionDiagnostic<'_>` is not implemented for `OverruledAttribute`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `MalformedAttribute: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_lint/src/levels.rs:337:47
    |
337 |   ...                   sess.emit_err(MalformedAttribute {
    |  ____________________________--------_^
    | |                            required by a bound introduced by this call
338 | | ...                       span: name_value.span,
338 | | ...                       span: name_value.span,
339 | | ...                       sub: MalformedAttributeSub::ReasonMustBeStringLiteral(
340 | | ...                           name_value.span,
342 | | ...                   });
342 | | ...                   });
    | |_______________________^ the trait `SessionDiagnostic<'_>` is not implemented for `MalformedAttribute`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `MalformedAttribute: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_lint/src/levels.rs:347:43
    |
347 |   ...                   sess.emit_err(MalformedAttribute {
    |  ____________________________--------_^
    | |                            required by a bound introduced by this call
    | |                            required by a bound introduced by this call
348 | | ...                       span: item.span,
349 | | ...                       sub: MalformedAttributeSub::BadAttributeArgument(item.span),
350 | | ...                   });
    | |_______________________^ the trait `SessionDiagnostic<'_>` is not implemented for `MalformedAttribute`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `MalformedAttribute: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_lint/src/levels.rs:354:39
    |
354 |                           sess.emit_err(MalformedAttribute {
    |  ______________________________--------_^
    | |                              required by a bound introduced by this call
    | |                              required by a bound introduced by this call
355 | |                             span: item.span,
356 | |                             sub: MalformedAttributeSub::BadAttributeArgument(item.span),
357 | |                         });
    | |_________________________^ the trait `SessionDiagnostic<'_>` is not implemented for `MalformedAttribute`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `MalformedAttribute: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_lint/src/levels.rs:378:51
    |
378 |   ...                   sess.emit_err(MalformedAttribute {
    |  ____________________________--------_^
    | |                            required by a bound introduced by this call
379 | | ...                       span: sp,
379 | | ...                       span: sp,
380 | | ...                       sub: MalformedAttributeSub::ReasonMustComeLast(sp),
381 | | ...                   });
    | |_______________________^ the trait `SessionDiagnostic<'_>` is not implemented for `MalformedAttribute`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `MalformedAttribute: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_lint/src/levels.rs:386:39
    |
386 |                           sess.emit_err(MalformedAttribute {
    |  ______________________________--------_^
    | |                              required by a bound introduced by this call
387 | |                             span: sp,
387 | |                             span: sp,
388 | |                             sub: MalformedAttributeSub::BadAttributeArgument(sp),
389 | |                         });
    | |_________________________^ the trait `SessionDiagnostic<'_>` is not implemented for `MalformedAttribute`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `UnknownToolInScopedLint: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_lint/src/levels.rs:513:39
    |
513 |                           sess.emit_err(UnknownToolInScopedLint {
    |  ______________________________--------_^
    | |                              required by a bound introduced by this call
    | |                              required by a bound introduced by this call
514 | |                             span: tool_ident.map(|ident| ident.span),
515 | |                             tool_name: tool_name.unwrap(),
516 | |                             lint_name: pprust::path_to_string(&meta_item.path),
517 | |                             is_nightly_build: sess.is_nightly_build().then_some(()),
518 | |                         });
    | |_________________________^ the trait `SessionDiagnostic<'_>` is not implemented for `UnknownToolInScopedLint`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_lint` due to 37 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_lint` due to 37 previous errors
