plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_borrowck/src/borrowck_errors.rs:50:23
    |
50  |           let mut err = struct_span_err!(
50  |           let mut err = struct_span_err!(
    |  _______________________-
51  | |             self,
52  | |             new_loan_span,
53  | |             E0499,
...   |
56  | |             via(opt_via),
    | |_________- in this macro invocation
    |
note: the lint level is defined here
   --> compiler/rustc_borrowck/src/borrowck_errors.rs:2:9
   --> compiler/rustc_borrowck/src/borrowck_errors.rs:2:9
    |
2   | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_borrowck/src/borrowck_errors.rs:50:23
    |
50  |           let mut err = struct_span_err!(
50  |           let mut err = struct_span_err!(
    |  _______________________-
51  | |             self,
52  | |             new_loan_span,
53  | |             E0499,
...   |
56  | |             via(opt_via),
    | |_________- in this macro invocation
    |
note: the lint level is defined here
   --> compiler/rustc_borrowck/src/borrowck_errors.rs:1:9
   --> compiler/rustc_borrowck/src/borrowck_errors.rs:1:9
    |
1   | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_borrowck/src/borrowck_errors.rs:177:23
    |
177 |           let mut err = struct_span_err!(
177 |           let mut err = struct_span_err!(
    |  _______________________-
178 | |             self,
179 | |             span,
180 | |             E0502,
...   |
187 | |             via(msg_old),
    | |_________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_borrowck/src/borrowck_errors.rs:177:23
    |
177 |           let mut err = struct_span_err!(
177 |           let mut err = struct_span_err!(
    |  _______________________-
178 | |             self,
179 | |             span,
180 | |             E0502,
...   |
187 | |             via(msg_old),
    | |_________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
609 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_borrowck/src/borrowck_errors.rs:301:9
    |
    |
301 |         struct_span_err!(self, span, E0596, "cannot borrow {} as mutable{}", path, reason,)


error: diagnostics should be created using translatable messages
    |
608 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
609 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_borrowck/src/borrowck_errors.rs:301:9
    |
    |
301 |         struct_span_err!(self, span, E0596, "cannot borrow {} as mutable{}", path, reason,)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_borrowck/src/borrowck_errors.rs:395:29
    |
395 |         self.infcx.tcx.sess.struct_span_err_with_code(sp, msg, code)


error: diagnostics should be created using translatable messages
   --> compiler/rustc_borrowck/src/borrowck_errors.rs:395:29
    |
395 |         self.infcx.tcx.sess.struct_span_err_with_code(sp, msg, code)

error: could not compile `rustc_borrowck` due to 8 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_borrowck` due to 8 previous errors
