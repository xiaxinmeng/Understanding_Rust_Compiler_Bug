plain
    Checking cranelift-frontend v0.95.1
    Checking cranelift-native v0.95.1
    Checking cranelift-object v0.95.1
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0599]: `FnAbiError<'tcx>` doesn't implement `std::fmt::Display`
     |
     |
502  |             self.0.sess.span_fatal(span, err.to_string())
     |                                              ^^^^^^^^^ `FnAbiError<'tcx>` cannot be formatted with the default formatter
    ::: /checkout/compiler/rustc_middle/src/ty/layout.rs:1238:1
     |
     |
1238 | pub enum FnAbiError<'tcx> {
     | |
     | |
     | doesn't satisfy `FnAbiError<'tcx>: ToString`
     | doesn't satisfy `FnAbiError<'tcx>: std::fmt::Display`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `FnAbiError<'tcx>: std::fmt::Display`
             which is required by `FnAbiError<'tcx>: ToString`
note: the method `to_string` exists on the type `rustc_middle::ty::layout::LayoutError<'tcx>`
     |
2502 |     fn to_string(&self) -> String;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0277]: `FnAbiError<'tcx>` doesn't implement `std::fmt::Display`
    |
506 | /                     span_bug!(
507 | |                         span,
507 | |                         span,
508 | |                         "`fn_abi_of_fn_ptr({}, {:?})` failed: {}",
509 | |                         sig,
511 | |                         err
511 | |                         err
    | |                         ^^^ `FnAbiError<'tcx>` cannot be formatted with the default formatter
    | |_____________________- in this macro invocation (#1)
    |
   ::: /checkout/compiler/rustc_middle/src/macros.rs:30:1
    |
    |
30  |   macro_rules! span_bug {
    |   --------------------- in this expansion of `span_bug!` (#1)
...
34  |           $crate::util::bug::span_bug_fmt($span, ::std::format_args!($fmt, $($arg)+))
    |
   ::: /checkout/library/core/src/macros/mod.rs:876:5
    |
876 |       macro_rules! format_args {
876 |       macro_rules! format_args {
    |       ------------------------ in this expansion of `::std::format_args!` (#2)
    |
    = help: the trait `std::fmt::Display` is not implemented for `FnAbiError<'tcx>`


error[E0277]: `FnAbiError<'tcx>` doesn't implement `std::fmt::Display`
    |
515 | /                     span_bug!(
516 | |                         span,
516 | |                         span,
517 | |                         "`fn_abi_of_instance({}, {:?})` failed: {}",
519 | |                         extra_args,
520 | |                         err
520 | |                         err
    | |                         ^^^ `FnAbiError<'tcx>` cannot be formatted with the default formatter
    | |_____________________- in this macro invocation (#1)
    |
   ::: /checkout/compiler/rustc_middle/src/macros.rs:30:1
    |
    |
30  |   macro_rules! span_bug {
    |   --------------------- in this expansion of `span_bug!` (#1)
...
34  |           $crate::util::bug::span_bug_fmt($span, ::std::format_args!($fmt, $($arg)+))
    |
   ::: /checkout/library/core/src/macros/mod.rs:876:5
    |
876 |       macro_rules! format_args {
876 |       macro_rules! format_args {
    |       ------------------------ in this expansion of `::std::format_args!` (#2)
    |
    = help: the trait `std::fmt::Display` is not implemented for `FnAbiError<'tcx>`

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_codegen_cranelift` (lib) due to 3 previous errors
