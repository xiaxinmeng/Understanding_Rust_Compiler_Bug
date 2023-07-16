plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error[E0433]: failed to resolve: use of undeclared type `ImplementationMissing`
   --> compiler/rustc_typeck/src/check/op.rs:973:20
    |
973 |             if let ImplementationMissing::BinOp(binary_op_trait) = missing_trait {
    |                    ^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `ImplementationMissing`
error: unused import: `std::borrow::Borrow`
  --> compiler/rustc_typeck/src/check/op.rs:22:5
   |
22 | use std::borrow::Borrow;
22 | use std::borrow::Borrow;
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0308]: `match` arms have incompatible types
   --> compiler/rustc_typeck/src/check/op.rs:386:25
    |
266 |                   let (mut err, missing_trait, use_output, involves_fn) = match is_assign {
    |  _________________________________________________________________________-
267 | |                     IsAssign::Yes => {
268 | |                         let mut err = struct_span_err!(
269 | |                             self.tcx.sess,
...   |
280 | |                         (err, Some(STDImplementationMissing::BinOpAssign(op.node)), false, false)
    | |                         ------------------------------------------------------------------------- this is found to be of type `(DiagnosticBuilder<'_>, Option<STDImplementationMissing>, bool, bool)`
...   |
386 | |                         (err, missing_trait, use_output, involves_fn)
    | |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `STDImplementationMissing`, found enum `rustc_hir::BinOpKind`
388 | |                 };
388 | |                 };
    | |_________________- `match` arms have incompatible types
    |
    = note: expected type `(DiagnosticBuilder<'_>, Option<STDImplementationMissing>, _, _)`
              found tuple `(DiagnosticBuilder<'_>, Option<rustc_hir::BinOpKind>, _, _)`
error[E0308]: mismatched types
   --> compiler/rustc_typeck/src/check/op.rs:705:33
    |
    |
705 | ...                   &STDImplementationMissing::Unop(op),
    |                       |
    |                       |
    |                       expected enum `STDImplementationMissing`, found `&STDImplementationMissing`
    |                       help: consider removing the borrow: `STDImplementationMissing::Unop(op)`

error[E0277]: `STDImplementationMissing` doesn't implement `std::fmt::Display`
   --> compiler/rustc_typeck/src/check/op.rs:969:21
    |
967 |                   err.note(&format!(
    |  ___________________________-
968 | |                     "an implementation of `{}` might be missing for `{}`",
969 | |                     missing_trait, ty
    | |                     ^^^^^^^^^^^^^ `STDImplementationMissing` cannot be formatted with the default formatter
    | |_________________- in this macro invocation (#1)
    | 
   ::: /checkout/library/alloc/src/macros.rs:109:1
    |
---
    | |_- in this expansion of `format!` (#1)
    | 
   ::: /checkout/library/core/src/macros/mod.rs:834:5
    |
834 | /     macro_rules! format_args {
835 | |         ($fmt:expr) => {{ /* compiler built-in */ }};
836 | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |_____- in this expansion of `$crate::__export::format_args!` (#2)
    |
    |
    = help: the trait `std::fmt::Display` is not implemented for `STDImplementationMissing`
    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
Some errors have detailed explanations: E0277, E0308, E0433.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_typeck` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
