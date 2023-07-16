plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0405]: cannot find trait `AddToDiagnostic` in this scope
    |
    |
630 |         cb: impl Fn(Span) -> impl AddToDiagnostic,
    |
help: consider importing this trait
    |
3   | use rustc_errors::AddToDiagnostic;
3   | use rustc_errors::AddToDiagnostic;
    |

error[E0425]: cannot find function `f` in this scope
   --> compiler/rustc_borrowck/src/diagnostics/mod.rs:635:5
    |
635 |                 f(capture_kind_span);
    |
help: consider importing one of these items
    |
3   | use crate::diagnostics::sym::f;
---
    |
3   | use rustc_span::sym::f;
    |

error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `Fn` trait return
    |
    |
630 |         cb: impl Fn(Span) -> impl AddToDiagnostic,


error[E0599]: no method named `var_span_subdiag_label` found for enum `UseSpans` in the current scope
    |
    |
732 |         borrow_spans.var_span_subdiag_label(
    |                      ^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `var_span_label`
   ::: compiler/rustc_borrowck/src/diagnostics/mod.rs:513:1
    |
    |
513 | pub(super) enum UseSpans<'tcx> {
    | ------------------------------ method `var_span_subdiag_label` not found for this enum

error[E0559]: variant `CaptureVarKind::Here` has no field named `capture_kind_span`
    |
639 |                     capture_kind_span,
639 |                     capture_kind_span,
    |                     ^^^^^^^^^^^^^^^^^ `CaptureVarKind::Here` does not have this field
    |
    = note: available fields are: `kind_desc`, `kind_span`
Some errors have detailed explanations: E0405, E0425, E0559, E0562, E0599.
For more information about an error, try `rustc --explain E0405`.
error: could not compile `rustc_borrowck` due to 6 previous errors
warning: build failed, waiting for other jobs to finish...
