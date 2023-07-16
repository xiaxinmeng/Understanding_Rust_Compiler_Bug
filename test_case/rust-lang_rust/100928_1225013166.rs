plain
  |
4 | use rustc_errors::struct_span_err;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
942 |             CrateError::NonAsciiName(crate_name) => sess.struct_span_err(
    |
note: the lint level is defined here
   --> compiler/rustc_metadata/src/lib.rs:19:9
    |
    |
19  | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
    |
    |
942 |             CrateError::NonAsciiName(crate_name) => sess.struct_span_err(
    |
note: the lint level is defined here
   --> compiler/rustc_metadata/src/lib.rs:18:9
    |
    |
18  | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
946 |             CrateError::ExternLocationNotExist(crate_name, loc) => sess.struct_span_err(


error: diagnostics should be created using translatable messages
    |
    |
946 |             CrateError::ExternLocationNotExist(crate_name, loc) => sess.struct_span_err(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
950 |             CrateError::ExternLocationNotFile(crate_name, loc) => sess.struct_span_err(


error: diagnostics should be created using translatable messages
    |
    |
950 |             CrateError::ExternLocationNotFile(crate_name, loc) => sess.struct_span_err(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
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

error: could not compile `rustc_metadata` due to 33 previous errors
