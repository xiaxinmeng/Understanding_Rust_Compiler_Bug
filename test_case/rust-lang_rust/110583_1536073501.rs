plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0277]: the trait bound `SubdiagnosticMessage: From<&std::string::String>` is not satisfied
    --> compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs:1125:21
1123 |                 err.span_suggestion_verbose(
     |                     ----------------------- required by a bound introduced by this call
1124 |                     span,
1124 |                     span,
1125 |                     &format!("consider {changing} this binding's type"),
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<&std::string::String>` is not implemented for `SubdiagnosticMessage`
     |
     = note: required for `&std::string::String` to implement `Into<SubdiagnosticMessage>`
note: required by a bound in `rustc_errors::DiagnosticBuilder::<'a, G>::span_suggestion_verbose`
     |
     |
725  |         msg: impl Into<SubdiagnosticMessage>,
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `DiagnosticBuilder::<'a, G>::span_suggestion_verbose`
    -->  /checkout/library/alloc/src/macros.rs:119:23
     |
119  |     ($($arg:tt)*) => {*{
     |                       +
     |                       +

error[E0277]: the trait bound `SubdiagnosticMessage: From<&std::string::String>` is not satisfied
    --> compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs:1132:21
1130 |                   err.span_label(
     |                       ---------- required by a bound introduced by this call
1131 |                       err_label_span,
1132 | /                     &format!(
1132 | /                     &format!(
1133 | |                         "consider changing this binding's type to be: `{message}`"
1134 | |                     ),
     | |_____________________^ the trait `From<&std::string::String>` is not implemented for `SubdiagnosticMessage`
     |
     = note: required for `&std::string::String` to implement `Into<SubdiagnosticMessage>`
note: required by a bound in `rustc_errors::DiagnosticBuilder::<'a, G>::span_label`
     |
     |
613  |     pub fn span_label(&mut self, span: Span, label: impl Into<SubdiagnosticMessage>) -> &mut Self);
     |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `DiagnosticBuilder::<'a, G>::span_label`
    -->  /checkout/library/alloc/src/macros.rs:119:23
     |
119  |     ($($arg:tt)*) => {*{
     |                       +
