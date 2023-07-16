plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
579 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
580 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_attr/src/builtin.rs:832:29
    |
    |
832 | ...                   struct_span_err!(diagnostic, meta.span, E0551, "incorrect meta item")
    |
note: the lint level is defined here
   --> compiler/rustc_attr/src/lib.rs:9:9
    |
    |
9   | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
    |
579 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
580 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_attr/src/builtin.rs:832:29
    |
    |
832 | ...                   struct_span_err!(diagnostic, meta.span, E0551, "incorrect meta item")
    |
note: the lint level is defined here
   --> compiler/rustc_attr/src/lib.rs:8:9
    |
    |
8   | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_attr/src/builtin.rs:974:39
    |
974 |                           let mut err = struct_span_err!(
974 |                           let mut err = struct_span_err!(
    |  _______________________________________-
975 | |                             diagnostic,
976 | |                             item.span(),
977 | |                             E0589,
978 | |                             "invalid `repr(align)` attribute: `align` needs an argument"
    | |_________________________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_attr/src/builtin.rs:974:39
    |
974 |                           let mut err = struct_span_err!(
974 |                           let mut err = struct_span_err!(
    |  _______________________________________-
975 | |                             diagnostic,
976 | |                             item.span(),
977 | |                             E0589,
978 | |                             "invalid `repr(align)` attribute: `align` needs an argument"
    | |_________________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1015:21
     |
1015 | /                     struct_span_err!(
1015 | /                     struct_span_err!(
1016 | |                                 diagnostic,
1017 | |                                 item.span(),
1018 | |                                 E0552,
1019 | |                                 "invalid representation hint: `{}` does not take a parenthesized argument list",
1020 | |                                 name.to_ident_string(),
1021 | |                             ).emit();


error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1015:21
     |
1015 | /                     struct_span_err!(
1015 | /                     struct_span_err!(
1016 | |                                 diagnostic,
1017 | |                                 item.span(),
1018 | |                                 E0552,
1019 | |                                 "invalid representation hint: `{}` does not take a parenthesized argument list",
1020 | |                                 name.to_ident_string(),
1021 | |                             ).emit();


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1024:21
     |
1024 | /                     struct_span_err!(
1024 | /                     struct_span_err!(
1025 | |                         diagnostic,
1026 | |                         item.span(),
1027 | |                         E0589,
1030 | |                         literal_error
1031 | |                     )
     | |_____________________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1024:21
     |
1024 | /                     struct_span_err!(
1024 | /                     struct_span_err!(
1025 | |                         diagnostic,
1026 | |                         item.span(),
1027 | |                         E0589,
1030 | |                         literal_error
1031 | |                     )
     | |_____________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1039:39
     |
1039 |                           let mut err = struct_span_err!(
1039 |                           let mut err = struct_span_err!(
     |  _______________________________________-
1040 | |                             diagnostic,
1041 | |                             item.span(),
1042 | |                             E0693,
1043 | |                             "incorrect `repr({})` attribute format",
1045 | |                         );
     | |_________________________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1039:39
     |
1039 |                           let mut err = struct_span_err!(
1039 |                           let mut err = struct_span_err!(
     |  _______________________________________-
1040 | |                             diagnostic,
1041 | |                             item.span(),
1042 | |                             E0693,
1043 | |                             "incorrect `repr({})` attribute format",
1045 | |                         );
     | |_________________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1073:29
     |
1073 | / ...                   struct_span_err!(
1073 | / ...                   struct_span_err!(
1074 | | ...                       diagnostic,
1075 | | ...                       meta_item.span,
1076 | | ...                       E0552,
1077 | | ...                       "invalid representation hint: `{}` does not take a value",
1078 | | ...                       meta_item.name_or_empty().to_ident_string(),
     | |_______________________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1073:29
     |
1073 | / ...                   struct_span_err!(
1073 | / ...                   struct_span_err!(
1074 | | ...                       diagnostic,
1075 | | ...                       meta_item.span,
1076 | | ...                       E0552,
1077 | | ...                       "invalid representation hint: `{}` does not take a value",
1078 | | ...                       meta_item.name_or_empty().to_ident_string(),
     | |_______________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1086:25
     |
1086 | /                         struct_span_err!(
1086 | /                         struct_span_err!(
1087 | |                             diagnostic,
1088 | |                             meta_item.span,
1089 | |                             E0693,
1090 | |                             "incorrect `repr(align)` attribute format: \
1091 | |                                  `align` takes exactly one argument in parentheses"
     | |_________________________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1086:25
     |
1086 | /                         struct_span_err!(
1086 | /                         struct_span_err!(
1087 | |                             diagnostic,
1088 | |                             meta_item.span,
1089 | |                             E0693,
1090 | |                             "incorrect `repr(align)` attribute format: \
1091 | |                                  `align` takes exactly one argument in parentheses"
     | |_________________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1096:25
     |
1096 | /                         struct_span_err!(
---
1102 | |                                  or no parentheses at all"
1103 | |                         )
     | |_________________________- in this macro invocation

error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1096:25
     |
1096 | /                         struct_span_err!(
---
1102 | |                                  or no parentheses at all"
1103 | |                         )
     | |_________________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1111:25
     |
1111 | /                         struct_span_err!(
1111 | /                         struct_span_err!(
1112 | |                                 diagnostic,
1113 | |                                 meta_item.span,
1114 | |                                 E0552,
1115 | |                                 "invalid representation hint: `{}` does not take a parenthesized argument list",
1116 | |                                 meta_item.name_or_empty().to_ident_string(),
1117 | |                             ).emit();


error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_attr/src/builtin.rs:1111:25
     |
1111 | /                         struct_span_err!(
1111 | /                         struct_span_err!(
1112 | |                                 diagnostic,
1113 | |                                 meta_item.span,
1114 | |                                 E0552,
1115 | |                                 "invalid representation hint: `{}` does not take a parenthesized argument list",
1116 | |                                 meta_item.name_or_empty().to_ident_string(),
1117 | |                             ).emit();


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_attr/src/builtin.rs:1214:35
1214 |                 sess.diagnostic().span_err(
     |                                   ^^^^^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_attr/src/builtin.rs:1214:35
1214 |                 sess.diagnostic().span_err(
     |                                   ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_attr/src/builtin.rs:1226:31
1226 |             sess.diagnostic().span_err(
     |                               ^^^^^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_attr/src/builtin.rs:1226:31
1226 |             sess.diagnostic().span_err(
     |                               ^^^^^^^^

error: could not compile `rustc_attr` due to 22 previous errors
