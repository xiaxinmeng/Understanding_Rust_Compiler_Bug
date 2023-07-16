plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
  --> compiler/rustc_metadata/src/native_libs.rs:69:34
   |
69 | ...                   sess.span_err(item.span(), msg);
   |
note: the lint level is defined here
  --> compiler/rustc_metadata/src/lib.rs:19:9
   |
   |
19 | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
  --> compiler/rustc_metadata/src/native_libs.rs:69:34
   |
69 | ...                   sess.span_err(item.span(), msg);
   |
note: the lint level is defined here
  --> compiler/rustc_metadata/src/lib.rs:18:9
   |
   |
18 | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
  --> compiler/rustc_metadata/src/native_libs.rs:74:34
   |
74 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
  --> compiler/rustc_metadata/src/native_libs.rs:74:34
   |
74 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
609 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
610 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_metadata/src/native_libs.rs:79:29
    |
    |
79  | ...                   struct_span_err!(sess, span, E0454, "link name must not be empty")


error: diagnostics should be created using translatable messages
    |
609 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
610 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_metadata/src/native_libs.rs:79:29
    |
    |
79  | ...                   struct_span_err!(sess, span, E0454, "link name must not be empty")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
  --> compiler/rustc_metadata/src/native_libs.rs:88:34
   |
88 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
  --> compiler/rustc_metadata/src/native_libs.rs:88:34
   |
88 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
  --> compiler/rustc_metadata/src/native_libs.rs:93:34
   |
93 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
  --> compiler/rustc_metadata/src/native_libs.rs:93:34
   |
93 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
609 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
610 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_metadata/src/native_libs.rs:103:37
    |
103 | / ...                   struct_span_err!(
103 | / ...                   struct_span_err!(
104 | | ...                       sess,
105 | | ...                       span,
106 | | ...                       E0455,
107 | | ...                       "link kind `framework` is only supported on Apple targets"
    | |_______________________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
609 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
610 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_metadata/src/native_libs.rs:103:37
    |
103 | / ...                   struct_span_err!(
103 | / ...                   struct_span_err!(
104 | | ...                       sess,
105 | | ...                       span,
106 | | ...                       E0455,
107 | | ...                       "link kind `framework` is only supported on Apple targets"
    | |_______________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
609 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
610 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_metadata/src/native_libs.rs:115:37
    |
115 | / ...                   struct_span_err!(
115 | / ...                   struct_span_err!(
116 | | ...                       sess,
117 | | ...                       span,
118 | | ...                       E0455,
119 | | ...                       "link kind `raw-dylib` is only supported on Windows targets"
    | |_______________________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
609 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
610 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_metadata/src/native_libs.rs:115:37
    |
115 | / ...                   struct_span_err!(
115 | / ...                   struct_span_err!(
116 | | ...                       sess,
117 | | ...                       span,
118 | | ...                       E0455,
119 | | ...                       "link kind `raw-dylib` is only supported on Windows targets"
    | |_______________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
609 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
610 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_metadata/src/native_libs.rs:138:33
    |
    |
138 | ...                   struct_span_err!(sess, span, E0458, "{}", msg)


error: diagnostics should be created using translatable messages
    |
609 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
610 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_metadata/src/native_libs.rs:138:33
    |
    |
138 | ...                   struct_span_err!(sess, span, E0458, "{}", msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:150:34
    |
150 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:150:34
    |
150 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:155:34
    |
155 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:155:34
    |
155 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:163:34
    |
163 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:163:34
    |
163 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:168:34
    |
168 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:168:34
    |
168 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:173:34
    |
173 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:173:34
    |
173 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:191:34
    |
191 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:191:34
    |
191 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:197:34
    |
197 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:197:34
    |
197 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:205:30
    |
205 |                         sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:205:30
    |
205 |                         sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:217:34
217 | ...                   sess.span_err(
    |                            ^^^^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:217:34
217 | ...                   sess.span_err(
    |                            ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:242:34
    |
242 | ...                   sess.span_err(span, &msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:242:34
    |
242 | ...                   sess.span_err(span, &msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:252:34
252 | ...                   sess.span_err(
    |                            ^^^^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:252:34
252 | ...                   sess.span_err(
    |                            ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:268:34
268 | ...                   sess.span_err(
    |                            ^^^^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:268:34
268 | ...                   sess.span_err(
    |                            ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:281:34
281 | ...                   sess.span_err(
    |                            ^^^^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:281:34
281 | ...                   sess.span_err(
    |                            ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:289:34
289 | ...                   sess.span_err(
    |                            ^^^^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:289:34
289 | ...                   sess.span_err(
    |                            ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:305:26
    |
305 |                     sess.span_err(span, msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:305:26
    |
305 |                     sess.span_err(span, msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
609 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
610 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_metadata/src/native_libs.rs:308:17
    |
308 | /                 struct_span_err!(
308 | /                 struct_span_err!(
309 | |                     sess,
310 | |                     m.span,
311 | |                     E0459,
312 | |                     "`#[link]` attribute requires a `name = \"string\"` argument"
    | |_________________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
609 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
610 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_metadata/src/native_libs.rs:308:17
    |
308 | /                 struct_span_err!(
308 | /                 struct_span_err!(
309 | |                     sess,
310 | |                     m.span,
311 | |                     E0459,
312 | |                     "`#[link]` attribute requires a `name = \"string\"` argument"
    | |_________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:321:30
321 |                         sess.span_err(
    |                              ^^^^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:321:30
321 |                         sess.span_err(
    |                              ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:347:34
347 | ...                   sess.span_err(
    |                            ^^^^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:347:34
347 | ...                   sess.span_err(
    |                            ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:376:31
    |
376 |                 self.tcx.sess.err("library kind `framework` is only supported on Apple targets");


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:376:31
    |
376 |                 self.tcx.sess.err("library kind `framework` is only supported on Apple targets");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:385:35
    |
385 |                     self.tcx.sess.err(format!(


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:385:35
    |
385 |                     self.tcx.sess.err(format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:390:35
    |
390 |                     self.tcx.sess.err(format!(


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:390:35
    |
390 |                     self.tcx.sess.err(format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:397:35
    |
397 |                     self.tcx.sess.err(format!(


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:397:35
    |
397 |                     self.tcx.sess.err(format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:428:67
    |
428 | ...                   Some(def_id) => self.tcx.sess.span_err(self.tcx.def_span(def_id), msg),


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:428:67
    |
428 | ...                   Some(def_id) => self.tcx.sess.span_err(self.tcx.def_span(def_id), msg),


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:429:59
---
961 | |                     crate_name,
962 | |                 );
    | |_________________- in this macro invocation

error: diagnostics should be created using translatable messages
    |
609 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
610 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_metadata/src/locator.rs:955:31
    |
955 |                   let mut err = struct_span_err!(
---
961 | |                     crate_name,
962 | |                 );
    | |_________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
609 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
610 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_metadata/src/locator.rs:969:31
    |
969 |                   let mut err = struct_span_err!(
969 |                   let mut err = struct_span_err!(
    |  _______________________________-
970 | |                     sess,
971 | |                     span,
972 | |                     E0464,
973 | |                     "multiple matching crates for `{}`",
975 | |                 );
    | |_________________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
609 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
610 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_metadata/src/locator.rs:969:31
    |
969 |                   let mut err = struct_span_err!(
969 |                   let mut err = struct_span_err!(
    |  _______________________________-
970 | |                     sess,
971 | |                     span,
972 | |                     E0464,
973 | |                     "multiple matching crates for `{}`",
975 | |                 );
    | |_________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1006:62
     |
     |
1006 |               CrateError::SymbolConflictsCurrent(root_name) => struct_span_err!(
1007 | |                 sess,
1008 | |                 span,
1009 | |                 E0519,
...    |
...    |
1013 | |                 root_name,
1014 | |             ),
     | |_____________- in this macro invocation

error: diagnostics should be created using translatable messages
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1006:62
     |
     |
1006 |               CrateError::SymbolConflictsCurrent(root_name) => struct_span_err!(
1007 | |                 sess,
1008 | |                 span,
1009 | |                 E0519,
...    |
...    |
1013 | |                 root_name,
1014 | |             ),
     | |_____________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1015:61
     |
     |
1015 |               CrateError::SymbolConflictsOthers(root_name) => struct_span_err!(
1016 | |                 sess,
1017 | |                 span,
1018 | |                 E0523,
...    |
...    |
1021 | |                 root_name,
1022 | |             ),
     | |_____________- in this macro invocation

error: diagnostics should be created using translatable messages
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1015:61
     |
     |
1015 |               CrateError::SymbolConflictsOthers(root_name) => struct_span_err!(
1016 | |                 sess,
1017 | |                 span,
1018 | |                 E0523,
...    |
...    |
1021 | |                 root_name,
1022 | |             ),
     | |_____________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1028 |                 sess.struct_span_err(span, &msg)


error: diagnostics should be created using translatable messages
     |
     |
1028 |                 sess.struct_span_err(span, &msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1030 |             CrateError::DlOpen(s) | CrateError::DlSym(s) => sess.struct_span_err(span, &s),


error: diagnostics should be created using translatable messages
     |
     |
1030 |             CrateError::DlOpen(s) | CrateError::DlSym(s) => sess.struct_span_err(span, &s),


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1039:35
     |
1039 |                       let mut err = struct_span_err!(
---
1045 | |                         add,
1046 | |                     );
     | |_____________________- in this macro invocation

error: diagnostics should be created using translatable messages
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1039:35
     |
1039 |                       let mut err = struct_span_err!(
---
1045 | |                         add,
1046 | |                     );
     | |_____________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1060:35
     |
1060 |                       let mut err = struct_span_err!(
---
1067 | |                         add,
1068 | |                     );
     | |_____________________- in this macro invocation

error: diagnostics should be created using translatable messages
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1060:35
     |
1060 |                       let mut err = struct_span_err!(
---
1067 | |                         add,
1068 | |                     );
     | |_____________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1081:35
     |
1081 |                       let mut err = struct_span_err!(
---
1087 | |                         add,
1088 | |                     );
     | |_____________________- in this macro invocation

error: diagnostics should be created using translatable messages
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1081:35
     |
1081 |                       let mut err = struct_span_err!(
---
1087 | |                         add,
1088 | |                     );
     | |_____________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1097:35
     |
1097 |                       let mut err = struct_span_err!(
---
1103 | |                         add,
1104 | |                     );
     | |_____________________- in this macro invocation

error: diagnostics should be created using translatable messages
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1097:35
     |
1097 |                       let mut err = struct_span_err!(
---
1103 | |                         add,
1104 | |                     );
     | |_____________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1122:35
     |
1122 |                       let mut err = struct_span_err!(
---
1128 | |                         add,
1129 | |                     );
     | |_____________________- in this macro invocation

error: diagnostics should be created using translatable messages
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1122:35
     |
1122 |                       let mut err = struct_span_err!(
---
1128 | |                         add,
1129 | |                     );
     | |_____________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1135:35
     |
1135 |                       let mut err = struct_span_err!(
---
1141 | |                         add,
1142 | |                     );
     | |_____________________- in this macro invocation

error: diagnostics should be created using translatable messages
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1135:35
     |
1135 |                       let mut err = struct_span_err!(
---
1141 | |                         add,
1142 | |                     );
     | |_____________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1211:55
     |
     |
1211 |               CrateError::NonDylibPlugin(crate_name) => struct_span_err!(
1212 | |                 sess,
1213 | |                 span,
1214 | |                 E0457,
1214 | |                 E0457,
1215 | |                 "plugin `{}` only found in rlib format, but must be available in dylib format",
1217 | |             ),
     | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
609  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
610  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
611  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_metadata/src/locator.rs:1211:55
     |
     |
1211 |               CrateError::NonDylibPlugin(crate_name) => struct_span_err!(
1212 | |                 sess,
1213 | |                 span,
1214 | |                 E0457,
1214 | |                 E0457,
1215 | |                 "plugin `{}` only found in rlib format, but must be available in dylib format",
1217 | |             ),
     | |_____________- in this macro invocation

error: could not compile `rustc_metadata` due to 132 previous errors
