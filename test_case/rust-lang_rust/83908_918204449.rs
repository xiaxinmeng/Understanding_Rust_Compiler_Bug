plain
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: no rules expected the token `EnumIntrinsicsNonEnums`
    |
150 | / macro_rules! late_lint_passes {
150 | / macro_rules! late_lint_passes {
151 | |     ($macro:path, $args:tt) => {
152 | |         $macro!(
153 | |             $args,
...   |
174 | |                 EnumIntrinsicsNonEnums: EnumIntrinsicsNonEnums,
    | |                 ^^^^^^^^^^^^^^^^^^^^^^ no rules expected this token in macro call
179 | |     };
180 | | }
180 | | }
    | |_- in this expansion of `late_lint_passes!`
...
224 |   late_lint_passes!(declare_combined_late_pass, [pub BuiltinCombinedLateLintPass]);
    |
   ::: compiler/rustc_lint/src/passes.rs:122:1
    |
122 |   macro_rules! declare_combined_late_lint_pass {
122 |   macro_rules! declare_combined_late_lint_pass {
    |   -------------------------------------------- when calling this macro

error: no rules expected the token `EnumIntrinsicsNonEnums`
    |
150 | / macro_rules! late_lint_passes {
150 | / macro_rules! late_lint_passes {
151 | |     ($macro:path, $args:tt) => {
152 | |         $macro!(
153 | |             $args,
...   |
174 | |                 EnumIntrinsicsNonEnums: EnumIntrinsicsNonEnums,
    | |                 ^^^^^^^^^^^^^^^^^^^^^^ no rules expected this token in macro call
179 | |     };
180 | | }
180 | | }
    | |_- in this expansion of `late_lint_passes!`
256 |       macro_rules! register_passes {
    |       ---------------------------- when calling this macro
...
...
267 |           late_lint_passes!(register_passes, register_late_pass);


error[E0433]: failed to resolve: use of undeclared type `BuiltinCombinedLateLintPass`
    |
    |
273 |         store.register_lints(&BuiltinCombinedLateLintPass::get_lints());
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `BuiltinCombinedLateLintPass`

error: unused import: `fold::TypeFoldable`
 --> compiler/rustc_lint/src/enum_intrinsics_non_enums.rs:3:24
  |
3 | use rustc_middle::ty::{fold::TypeFoldable, Ty};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `array_into_iter::ArrayIntoIter`
  --> compiler/rustc_lint/src/lib.rs:79:5
   |
79 | use array_into_iter::ArrayIntoIter;
79 | use array_into_iter::ArrayIntoIter;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `enum_intrinsics_non_enums::EnumIntrinsicsNonEnums`
   |
   |
81 | use enum_intrinsics_non_enums::EnumIntrinsicsNonEnums;


error: unused import: `methods::*`
   |
83 | use methods::*;
   |     ^^^^^^^^^^


error: unused import: `non_fmt_panic::NonPanicFmt`
   |
   |
85 | use non_fmt_panic::NonPanicFmt;


error: unused import: `noop_method_call::*`
   |
87 | use noop_method_call::*;
   |     ^^^^^^^^^^^^^^^^^^^


error: unused import: `traits::*`
  --> compiler/rustc_lint/src/lib.rs:89:5
   |
89 | use traits::*;
   |     ^^^^^^^^^

error[E0599]: no method named `needs_subst` found for reference `&TyS<'_>` in the current scope
  --> compiler/rustc_lint/src/enum_intrinsics_non_enums.rs:41:24
   |
41 |     !t.is_enum() && !t.needs_subst()
   |                        ^^^^^^^^^^^ method not found in `&TyS<'_>`
Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `rustc_lint` due to 11 previous errors
warning: build failed, waiting for other jobs to finish...
