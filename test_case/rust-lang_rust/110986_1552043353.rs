plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error[E0277]: the trait bound `DiagnosticMessage: From<&std::string::String>` is not satisfied
   --> compiler/rustc_query_system/src/query/plumbing.rs:442:21
    |
440 |                   qcx.dep_context().sess().delay_span_bug(
441 |                       DUMMY_SP,
442 | /                     &format!(
442 | /                     &format!(
443 | |                         "Computed query value for {:?}({:?}) is inconsistent with fed value,\n\
444 | |                     computed={:#?}\nfed={:#?}",
445 | |                         query.dep_kind(),
448 | |                         formatter(&cached_result),
449 | |                     ),
449 | |                     ),
    | |_____________________^ the trait `From<&std::string::String>` is not implemented for `DiagnosticMessage`
    |
    = note: required for `&std::string::String` to implement `Into<DiagnosticMessage>`
note: required by a bound in `Session::delay_span_bug`
    |
    |
670 |         msg: impl Into<DiagnosticMessage>,
    |                   ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::delay_span_bug`
   --> /checkout/library/alloc/src/macros.rs:119:23
    |
119 |     ($($arg:tt)*) => {*{
    |                       +
