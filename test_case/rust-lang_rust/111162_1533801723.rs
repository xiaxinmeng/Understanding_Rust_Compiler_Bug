plain
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
error[E0277]: the trait bound `DiagnosticMessage: From<&std::string::String>` is not satisfied
   --> compiler/rustc_symbol_mangling/src/typeid/typeid_itanium_cxx_abi.rs:537:33
535 | ...                   .struct_span_err(
    |                        --------------- required by a bound introduced by this call
    |                        --------------- required by a bound introduced by this call
536 | ...                       cfi_encoding.span,
537 | ...                       &format!("invalid `cfi_encoding` for `{:?}`", ty.kind()),
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<&std::string::String>` is not implemented for `DiagnosticMessage`
    |
    = note: required for `&std::string::String` to implement `Into<DiagnosticMessage>`
note: required by a bound in `Session::struct_span_err`
    |
    |
409 |         msg: impl Into<DiagnosticMessage>,
    |                   ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::struct_span_err`
   --> /checkout/library/alloc/src/macros.rs:119:23
    |
119 |     ($($arg:tt)*) => {*{
    |                       +
    |                       +

error[E0277]: the trait bound `DiagnosticMessage: From<&std::string::String>` is not satisfied
   --> compiler/rustc_symbol_mangling/src/typeid/typeid_itanium_cxx_abi.rs:589:33
587 | ...                   .struct_span_err(
    |                        --------------- required by a bound introduced by this call
    |                        --------------- required by a bound introduced by this call
588 | ...                       cfi_encoding.span,
589 | ...                       &format!("invalid `cfi_encoding` for `{:?}`", ty.kind()),
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<&std::string::String>` is not implemented for `DiagnosticMessage`
    |
    = note: required for `&std::string::String` to implement `Into<DiagnosticMessage>`
note: required by a bound in `Session::struct_span_err`
    |
    |
409 |         msg: impl Into<DiagnosticMessage>,
    |                   ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::struct_span_err`
   --> /checkout/library/alloc/src/macros.rs:119:23
    |
119 |     ($($arg:tt)*) => {*{
    |                       +
