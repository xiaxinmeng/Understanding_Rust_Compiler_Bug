plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: `#[error(...)]` is not a valid attribute
  |
  |
6 | #[error(ast_lowering::generic_type_with_parentheses, code = "E0214")]
  |
  |
  = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  |
  |
6 | #[error(ast_lowering::generic_type_with_parentheses, code = "E0214")]
  |
  |
  = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
33 | #[error(ast_lowering::invalid_abi, code = "E0703")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
32 | #[help]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
43 | #[error(ast_lowering::assoc_ty_parentheses)]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
43 | #[error(ast_lowering::assoc_ty_parentheses)]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
75 | #[error(ast_lowering::misplaced_impl_trait, code = "E0562")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
75 | #[error(ast_lowering::misplaced_impl_trait, code = "E0562")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
83 | #[error(ast_lowering::rustc_box_attribute_error)]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
83 | #[error(ast_lowering::rustc_box_attribute_error)]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
90 | #[error(ast_lowering::underscore_expr_lhs_assign)]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
90 | #[error(ast_lowering::underscore_expr_lhs_assign)]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
98 | #[error(ast_lowering::base_expression_double_dot)]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
98 | #[error(ast_lowering::base_expression_double_dot)]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
106 | #[error(ast_lowering::await_only_in_async_fn_and_blocks, code = "E0728")]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
106 | #[error(ast_lowering::await_only_in_async_fn_and_blocks, code = "E0728")]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
116 | #[error(ast_lowering::generator_too_many_parameters, code = "E0628")]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
116 | #[error(ast_lowering::generator_too_many_parameters, code = "E0628")]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
123 | #[error(ast_lowering::closure_cannot_be_static, code = "E0697")]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
123 | #[error(ast_lowering::closure_cannot_be_static, code = "E0697")]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
131 | #[error(ast_lowering::async_non_move_closure_not_supported, code = "E0708")]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
130 | #[help]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
138 | #[error(ast_lowering::functional_record_update_destructuring_assignment)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
138 | #[error(ast_lowering::functional_record_update_destructuring_assignment)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
146 | #[error(ast_lowering::async_generators_not_supported, code = "E0727")]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
146 | #[error(ast_lowering::async_generators_not_supported, code = "E0727")]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
153 | #[error(ast_lowering::inline_asm_unsupported_target, code = "E0472")]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
153 | #[error(ast_lowering::inline_asm_unsupported_target, code = "E0472")]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
160 | #[error(ast_lowering::att_syntax_only_x86)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
160 | #[error(ast_lowering::att_syntax_only_x86)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
167 | #[error(ast_lowering::abi_specified_multiple_times)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
167 | #[error(ast_lowering::abi_specified_multiple_times)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
179 | #[error(ast_lowering::clobber_abi_not_supported)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
179 | #[error(ast_lowering::clobber_abi_not_supported)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
187 | #[error(ast_lowering::invalid_abi_clobber_abi)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
186 | #[note]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
195 | #[error(ast_lowering::invalid_register)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
195 | #[error(ast_lowering::invalid_register)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
204 | #[error(ast_lowering::invalid_register_class)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
204 | #[error(ast_lowering::invalid_register_class)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
213 | #[error(ast_lowering::invalid_asm_template_modifier_reg_class)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
213 | #[error(ast_lowering::invalid_asm_template_modifier_reg_class)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
233 | #[error(ast_lowering::invalid_asm_template_modifier_const)]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
233 | #[error(ast_lowering::invalid_asm_template_modifier_const)]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
243 | #[error(ast_lowering::invalid_asm_template_modifier_sym)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
243 | #[error(ast_lowering::invalid_asm_template_modifier_sym)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
253 | #[error(ast_lowering::register_class_only_clobber)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
253 | #[error(ast_lowering::register_class_only_clobber)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
261 | #[error(ast_lowering::register_conflict)]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
261 | #[error(ast_lowering::register_conflict)]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
276 | #[error(ast_lowering::sub_tuple_binding)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
275 | #[help]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
292 | #[error(ast_lowering::extra_double_dot)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
292 | #[error(ast_lowering::extra_double_dot)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
304 | #[error(ast_lowering::misplaced_double_dot)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
303 | #[note]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
311 | #[error(ast_lowering::misplaced_relax_trait_bound)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
311 | #[error(ast_lowering::misplaced_relax_trait_bound)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
318 | #[error(ast_lowering::not_supported_for_lifetime_binder_async_closure)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
318 | #[error(ast_lowering::not_supported_for_lifetime_binder_async_closure)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
325 | #[error(ast_lowering::arbitrary_expression_in_pattern)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
325 | #[error(ast_lowering::arbitrary_expression_in_pattern)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`
error: cannot find attribute `error` in this scope
---

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_lowering/src/errors.rs:243:3
    |
243 | #[error(ast_lowering::invalid_asm_template_modifier_sym)]
    |
note: `error` is imported here, but it is a function-like macro
   --> compiler/rustc_ast_lowering/src/lib.rs:39:1
    |
    |
39  | #[macro_use]
    | ^^^^^^^^^^^^

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_lowering/src/errors.rs:253:3
    |
253 | #[error(ast_lowering::register_class_only_clobber)]
    |
note: `error` is imported here, but it is a function-like macro
   --> compiler/rustc_ast_lowering/src/lib.rs:39:1
    |
---

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_lowering/src/errors.rs:276:3
    |
276 | #[error(ast_lowering::sub_tuple_binding)]
    |
note: `error` is imported here, but it is a function-like macro
   --> compiler/rustc_ast_lowering/src/lib.rs:39:1
    |
    |
39  | #[macro_use]
    | ^^^^^^^^^^^^

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_lowering/src/errors.rs:292:3
    |
292 | #[error(ast_lowering::extra_double_dot)]
    |
note: `error` is imported here, but it is a function-like macro
   --> compiler/rustc_ast_lowering/src/lib.rs:39:1
    |
    |
39  | #[macro_use]
    | ^^^^^^^^^^^^

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_lowering/src/errors.rs:304:3
    |
304 | #[error(ast_lowering::misplaced_double_dot)]
    |
note: `error` is imported here, but it is a function-like macro
   --> compiler/rustc_ast_lowering/src/lib.rs:39:1
    |
    |
39  | #[macro_use]
    | ^^^^^^^^^^^^

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_lowering/src/errors.rs:311:3
    |
311 | #[error(ast_lowering::misplaced_relax_trait_bound)]
    |
note: `error` is imported here, but it is a function-like macro
   --> compiler/rustc_ast_lowering/src/lib.rs:39:1
    |
    |
39  | #[macro_use]
    | ^^^^^^^^^^^^

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_lowering/src/errors.rs:318:3
    |
318 | #[error(ast_lowering::not_supported_for_lifetime_binder_async_closure)]
    |
note: `error` is imported here, but it is a function-like macro
   --> compiler/rustc_ast_lowering/src/lib.rs:39:1
    |
    |
39  | #[macro_use]
    | ^^^^^^^^^^^^

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_lowering/src/errors.rs:325:3
    |
325 | #[error(ast_lowering::arbitrary_expression_in_pattern)]
    |
note: `error` is imported here, but it is a function-like macro
   --> compiler/rustc_ast_lowering/src/lib.rs:39:1
    |
---
   |
15 | use rustc_data_structures::thin_vec::ThinVec;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `rustc_errors::struct_span_err`
  --> compiler/rustc_ast_lowering/src/expr.rs:16:5
   |
16 | use rustc_errors::struct_span_err;
16 | use rustc_errors::struct_span_err;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the trait bound `InlineAsmUnsupportedTarget: SessionDiagnostic<'_>` is not satisfied
    |
    |
35  |             self.tcx.sess.emit_err(InlineAsmUnsupportedTarget { span: sp });
    |                           -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `InlineAsmUnsupportedTarget`
    |                           required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `AttSyntaxOnlyX86: SessionDiagnostic<'_>` is not satisfied
    |
    |
62  |             self.tcx.sess.emit_err(AttSyntaxOnlyX86 { span: sp });
    |                           -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `AttSyntaxOnlyX86`
    |                           required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `AbiSpecifiedMultipleTimes: SessionDiagnostic<'_>` is not satisfied
    |
    |
89  |   ...                   self.tcx.sess.emit_err(AbiSpecifiedMultipleTimes {
    |  _____________________________________--------_^
    | |                                     required by a bound introduced by this call
    | |                                     required by a bound introduced by this call
90  | | ...                       abi_span: *abi_span,
91  | | ...                       prev_name: *prev_name,
92  | | ...                       prev_span: *prev_sp,
93  | | ...                       equivalent,
94  | | ...                   });
    | |_______________________^ the trait `SessionDiagnostic<'_>` is not implemented for `AbiSpecifiedMultipleTimes`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `ClobberAbiNotSupported: SessionDiagnostic<'_>` is not satisfied
    |
    |
102 |                         self.tcx.sess.emit_err(ClobberAbiNotSupported { abi_span: *abi_span });
    |                                       -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `ClobberAbiNotSupported`
    |                                       required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `InvalidAbiClobberAbi: SessionDiagnostic<'_>` is not satisfied
    |
    |
109 |                           self.tcx.sess.emit_err(InvalidAbiClobberAbi {
    |  _______________________________________--------_^
    | |                                       required by a bound introduced by this call
    | |                                       required by a bound introduced by this call
110 | |                             abi_span: *abi_span,
111 | |                             supported_abis: abis,
112 | |                         });
    | |_________________________^ the trait `SessionDiagnostic<'_>` is not implemented for `InvalidAbiClobberAbi`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `InvalidRegister<'_>: SessionDiagnostic<'_>` is not satisfied
    |
    |
130 | ...                   sess.emit_err(InvalidRegister { op_span: *op_sp, reg, error });
    |                            -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `InvalidRegister<'_>`
    |                            required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `InvalidRegisterClass<'_>: SessionDiagnostic<'_>` is not satisfied
    |
    |
141 |   ...                   sess.emit_err(InvalidRegisterClass {
    |  ____________________________--------_^
    | |                            required by a bound introduced by this call
    | |                            required by a bound introduced by this call
142 | | ...                       op_span: *op_sp,
143 | | ...                       reg_class,
145 | | ...                   });
145 | | ...                   });
    | |_______________________^ the trait `SessionDiagnostic<'_>` is not implemented for `InvalidRegisterClass<'_>`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `InvalidAsmTemplateModifierRegClass: SessionDiagnostic<'_>` is not satisfied
    |
    |
286 |   ...                   sess.emit_err(InvalidAsmTemplateModifierRegClass {
    |  ____________________________--------_^
    | |                            required by a bound introduced by this call
287 | | ...                       placeholder_span,
287 | | ...                       placeholder_span,
288 | | ...                       op_span: op_sp,
290 | | ...                   });
290 | | ...                   });
    | |_______________________^ the trait `SessionDiagnostic<'_>` is not implemented for `InvalidAsmTemplateModifierRegClass`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `InvalidAsmTemplateModifierConst: SessionDiagnostic<'_>` is not satisfied
    |
    |
294 |                           sess.emit_err(InvalidAsmTemplateModifierConst {
    |  ______________________________--------_^
    | |                              required by a bound introduced by this call
295 | |                             placeholder_span,
295 | |                             placeholder_span,
296 | |                             op_span: op_sp,
297 | |                         });
    | |_________________________^ the trait `SessionDiagnostic<'_>` is not implemented for `InvalidAsmTemplateModifierConst`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `InvalidAsmTemplateModifierSym: SessionDiagnostic<'_>` is not satisfied
    |
    |
301 |                           sess.emit_err(InvalidAsmTemplateModifierSym {
    |  ______________________________--------_^
    | |                              required by a bound introduced by this call
302 | |                             placeholder_span,
302 | |                             placeholder_span,
303 | |                             op_span: op_sp,
304 | |                         });
    | |_________________________^ the trait `SessionDiagnostic<'_>` is not implemented for `InvalidAsmTemplateModifierSym`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `RegisterClassOnlyClobber: SessionDiagnostic<'_>` is not satisfied
    |
    |
325 |                       sess.emit_err(RegisterClassOnlyClobber {
    |  __________________________--------_^
    | |                          required by a bound introduced by this call
    | |                          required by a bound introduced by this call
326 | |                         op_span: op_sp,
327 | |                         reg_class_name: reg_class.name(),
328 | |                     });
    | |_____________________^ the trait `SessionDiagnostic<'_>` is not implemented for `RegisterClassOnlyClobber`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `RegisterConflict<'_>: SessionDiagnostic<'_>` is not satisfied
    |
    |
384 |   ...                   sess.emit_err(RegisterConflict {
    |  ____________________________--------_^
    | |                            required by a bound introduced by this call
    | |                            required by a bound introduced by this call
385 | | ...                       op_span1: op_sp,
386 | | ...                       op_span2: op_sp2,
387 | | ...                       reg1_name: reg.name(),
388 | | ...                       reg2_name: reg2.name(),
389 | | ...                       in_out
390 | | ...                   });
    | |_______________________^ the trait `SessionDiagnostic<'_>` is not implemented for `RegisterConflict<'_>`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `RustcBoxAttributeError: SessionDiagnostic<'_>` is not satisfied
    |
    |
55  | ...                   self.tcx.sess.emit_err(RustcBoxAttributeError { span: e.span });
    |                                     -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `RustcBoxAttributeError`
    |                                     required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `UnderscoreExprLhsAssign: SessionDiagnostic<'_>` is not satisfied
    |
    |
215 |                     self.tcx.sess.emit_err(UnderscoreExprLhsAssign { span: e.span });
    |                                   -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `UnderscoreExprLhsAssign`
    |                                   required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `BaseExpressionDoubleDot: SessionDiagnostic<'_>` is not satisfied
    |
    |
247 | ...                   self.tcx.sess.emit_err(BaseExpressionDoubleDot { span: *sp });
    |                                     -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `BaseExpressionDoubleDot`
    |                                     required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `AwaitOnlyInAsyncFnAndBlocks: SessionDiagnostic<'_>` is not satisfied
    |
    |
656 |                   self.tcx.sess.emit_err(AwaitOnlyInAsyncFnAndBlocks {
    |  _______________________________--------_^
    | |                               required by a bound introduced by this call
657 | |                     dot_await_span,
658 | |                     item_span: self.current_item,
659 | |                 });
659 | |                 });
    | |_________________^ the trait `SessionDiagnostic<'_>` is not implemented for `AwaitOnlyInAsyncFnAndBlocks`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `GeneratorTooManyParameters: SessionDiagnostic<'_>` is not satisfied
    |
    |
879 |                     self.tcx.sess.emit_err(GeneratorTooManyParameters { fn_decl_span });
    |                                   -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `GeneratorTooManyParameters`
    |                                   required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `ClosureCannotBeStatic: SessionDiagnostic<'_>` is not satisfied
    |
    |
888 |                     self.tcx.sess.emit_err(ClosureCannotBeStatic { fn_decl_span });
    |                                   -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `ClosureCannotBeStatic`
    |                                   required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `NotSupportedForLifetimeBinderAsyncClosure: SessionDiagnostic<'_>` is not satisfied
    |
    |
921 |             self.tcx.sess.emit_err(NotSupportedForLifetimeBinderAsyncClosure { span });
    |                           -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `NotSupportedForLifetimeBinderAsyncClosure`
    |                           required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `AsyncNonMoveClosureNotSupported: SessionDiagnostic<'_>` is not satisfied
    |
    |
932 |                 this.tcx.sess.emit_err(AsyncNonMoveClosureNotSupported { fn_decl_span });
    |                               -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `AsyncNonMoveClosureNotSupported`
    |                               required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `FunctionalRecordUpdateDestructuringAssignemnt: SessionDiagnostic<'_>` is not satisfied
     |
     |
1172 |                           self.tcx.sess.emit_err(FunctionalRecordUpdateDestructuringAssignemnt {
     |  _______________________________________--------_^
     | |                                       required by a bound introduced by this call
     | |                                       required by a bound introduced by this call
1173 | |                             span: e.span,
1174 | |                         });
     | |_________________________^ the trait `SessionDiagnostic<'_>` is not implemented for `FunctionalRecordUpdateDestructuringAssignemnt`
     |
note: required by a bound in `Session::emit_err`
     |
     |
473  |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
     |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `AsyncGeneratorsNotSupported: SessionDiagnostic<'_>` is not satisfied
     |
     |
1371 |                 self.tcx.sess.emit_err(AsyncGeneratorsNotSupported { span });
     |                               -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `AsyncGeneratorsNotSupported`
     |                               required by a bound introduced by this call
     |
     |
note: required by a bound in `Session::emit_err`
     |
     |
473  |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
     |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `InvalidAbi: SessionDiagnostic<'_>` is not satisfied
     |
     |
1263 |           self.tcx.sess.emit_err(InvalidAbi {
     |  _______________________--------_^
     | |                       required by a bound introduced by this call
     | |                       required by a bound introduced by this call
1264 | |             span: abi.span,
1265 | |             abi: abi.symbol,
1266 | |             valid_abis: abi::all_names().join(", "),
1267 | |         });
     | |_________^ the trait `SessionDiagnostic<'_>` is not implemented for `InvalidAbi`
     |
note: required by a bound in `Session::emit_err`
     |
     |
473  |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
     |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `MisplacedRelaxTraitBound: SessionDiagnostic<'_>` is not satisfied
     |
     |
1342 |                     self.tcx.sess.emit_err(MisplacedRelaxTraitBound { span: bound.span() });
     |                                   -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `MisplacedRelaxTraitBound`
     |                                   required by a bound introduced by this call
     |
     |
note: required by a bound in `Session::emit_err`
     |
     |
473  |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
     |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `SubTupleBinding<'_>: SessionDiagnostic<'_>` is not satisfied
    |
    |
139 |                       self.tcx.sess.emit_err(SubTupleBinding {
    |  ___________________________________--------_^
    | |                                   required by a bound introduced by this call
140 | |                         span: sp,
141 | |                         ident_name: ident.name,
142 | |                         ident,
142 | |                         ident,
143 | |                         ctx,
144 | |                     });
    | |_____________________^ the trait `SessionDiagnostic<'_>` is not implemented for `SubTupleBinding<'_>`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `ExtraDoubleDot<'_>: SessionDiagnostic<'_>` is not satisfied
    |
    |
293 |         self.tcx.sess.emit_err(ExtraDoubleDot { span: sp, prev_span: prev_sp, ctx });
    |                       -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `ExtraDoubleDot<'_>`
    |                       required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `MisplacedDoubleDot: SessionDiagnostic<'_>` is not satisfied
    |
    |
298 |         self.tcx.sess.emit_err(MisplacedDoubleDot { span: sp });
    |                       -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `MisplacedDoubleDot`
    |                       required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `ArbitraryExpressionInPattern: SessionDiagnostic<'_>` is not satisfied
    |
    |
335 |                 self.tcx.sess.emit_err(ArbitraryExpressionInPattern { span: expr.span });
    |                               -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `ArbitraryExpressionInPattern`
    |                               required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `GenericTypeWithParentheses: SessionDiagnostic<'_>` is not satisfied
    |
    |
217 |                         self.tcx.sess.emit_err(GenericTypeWithParentheses { span: data.span, sub });
    |                                       -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `GenericTypeWithParentheses`
    |                                       required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `AssocTyParentheses: SessionDiagnostic<'_>` is not satisfied
     |
     |
1096 |         self.tcx.sess.emit_err(AssocTyParentheses { span: data.span, sub });
     |                       -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `AssocTyParentheses`
     |                       required by a bound introduced by this call
     |
     |
note: required by a bound in `Session::emit_err`
     |
     |
473  |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
     |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `MisplacedImplTrait<'_>: SessionDiagnostic<'_>` is not satisfied
     |
     |
1335 |                           self.tcx.sess.emit_err(MisplacedImplTrait {
     |  _______________________________________--------_^
     | |                                       required by a bound introduced by this call
1336 | |                             span: t.span,
1336 | |                             span: t.span,
1337 | |                             position: DiagnosticArgFromDisplay(&position),
1338 | |                         });
     | |_________________________^ the trait `SessionDiagnostic<'_>` is not implemented for `MisplacedImplTrait<'_>`
