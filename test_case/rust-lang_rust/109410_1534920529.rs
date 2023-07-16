plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0277]: the trait bound `DiagnosticMessage: From<&std::string::String>` is not satisfied
     |
1354 |               tcx.sess.delay_span_bug(
     |                        -------------- required by a bound introduced by this call
1355 |                   cause.span,
1355 |                   cause.span,
1356 | /                 &format!(
1357 | |                     "{self_ty:?} was a subtype of {impl_ty:?} during selection but now it is not"
1358 | |                 ),
     | |_________________^ the trait `From<&std::string::String>` is not implemented for `DiagnosticMessage`
     |
     = note: required for `&std::string::String` to implement `Into<DiagnosticMessage>`
note: required by a bound in `Session::delay_span_bug`
     |
     |
670  |         msg: impl Into<DiagnosticMessage>,
     |                   ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::delay_span_bug`
    -->  /checkout/library/alloc/src/macros.rs:119:23
     |
119  |     ($($arg:tt)*) => {*{
     |                       +
