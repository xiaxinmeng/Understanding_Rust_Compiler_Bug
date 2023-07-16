plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
373 |                 lint.build(&format!("`#[{name}]` only has an effect on {}", supported_names))
    |
note: the lint level is defined here
   --> compiler/rustc_passes/src/lib.rs:9:9
    |
    |
9   | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
    |
    |
373 |                 lint.build(&format!("`#[{name}]` only has an effect on {}", supported_names))
    |
note: the lint level is defined here
   --> compiler/rustc_passes/src/lib.rs:8:9
    |
    |
8   | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
437 |             tcx.sess.span_err(span, &object_lifetime_default_reprs);


error: diagnostics should be created using translatable messages
    |
    |
437 |             tcx.sess.span_err(span, &object_lifetime_default_reprs);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
871 |                 let mut err = lint.build(fluent::passes::attr_crate_level);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
608  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
609  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1655:21
     |
1655 | /                     struct_span_err!(
1655 | /                     struct_span_err!(
1656 | |                         self.tcx.sess,
1657 | |                         hint.span(),
1658 | |                         E0552,
1659 | |                         "unrecognized representation hint"
     | |_____________________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
608  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
609  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1655:21
     |
1655 | /                     struct_span_err!(
1655 | /                     struct_span_err!(
1656 | |                         self.tcx.sess,
1657 | |                         hint.span(),
1658 | |                         E0552,
1659 | |                         "unrecognized representation hint"
     | |_____________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
608  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
609  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1667:13
     |
1667 | /             struct_span_err!(
1667 | /             struct_span_err!(
1668 | |                 self.tcx.sess,
1669 | |                 hint.span(),
1670 | |                 E0517,
1671 | |                 "{}",
1672 | |                 &format!("attribute should be applied to {article} {allowed_targets}")
     | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
608  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
609  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1667:13
     |
1667 | /             struct_span_err!(
1667 | /             struct_span_err!(
1668 | |                 self.tcx.sess,
1669 | |                 hint.span(),
1670 | |                 E0517,
1671 | |                 "{}",
1672 | |                 &format!("attribute should be applied to {article} {allowed_targets}")
     | |_____________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
608  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
609  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1685:13
     |
1685 | /             struct_span_err!(
1685 | /             struct_span_err!(
1686 | |                 self.tcx.sess,
1687 | |                 hint_spans,
1688 | |                 E0692,
1689 | |                 "transparent {} cannot have other repr hints",
1691 | |             )
     | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
608  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
609  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1685:13
     |
1685 | /             struct_span_err!(
1685 | /             struct_span_err!(
1686 | |                 self.tcx.sess,
1687 | |                 hint_spans,
1688 | |                 E0692,
1689 | |                 "transparent {} cannot have other repr hints",
1691 | |             )
     | |_____________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
609 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/check_const.rs:144:17
    |
    |
144 |                 struct_span_err!(tcx.sess, span, E0744, "{}", msg).emit();


error: diagnostics should be created using translatable messages
    |
608 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
609 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/check_const.rs:144:17
    |
    |
144 |                 struct_span_err!(tcx.sess, span, E0744, "{}", msg).emit();


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
194 |                         lint.build(&format!(
    |                              ^^^^^


error: diagnostics should be created using translatable messages
    |
194 |                         lint.build(&format!(
    |                              ^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
754 |                     let mut err = lint.build(&format!(
    |                                        ^^^^^


error: diagnostics should be created using translatable messages
    |
754 |                     let mut err = lint.build(&format!(
    |                                        ^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
  --> compiler/rustc_passes/src/debugger_visualizer.rs:59:26
59 |                         .struct_span_err(
   |                          ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
  --> compiler/rustc_passes/src/debugger_visualizer.rs:59:26
59 |                         .struct_span_err(
   |                          ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
   |
39 |                     .struct_span_err(span, &format!("duplicate diagnostic item found: `{name}`.")),


error: diagnostics should be created using translatable messages
   |
   |
39 |                     .struct_span_err(span, &format!("duplicate diagnostic item found: `{name}`.")),


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
40 |                 None => tcx.sess.struct_err(&format!(
   |                                  ^^^^^^^^^^


error: diagnostics should be created using translatable messages
   |
40 |                 None => tcx.sess.struct_err(&format!(
   |                                  ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
  --> compiler/rustc_passes/src/entry.rs:78:14
78 |             .struct_span_err(
   |              ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
  --> compiler/rustc_passes/src/entry.rs:78:14
78 |             .struct_span_err(
   |              ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/entry.rs:104:17
    |
104 | /                 struct_span_err!(
104 | /                 struct_span_err!(
105 | |                     ctxt.tcx.sess,
106 | |                     ctxt.tcx.def_span(id.def_id.to_def_id()),
107 | |                     E0137,
108 | |                     "multiple functions with a `#[rustc_main]` attribute"
    | |_________________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/entry.rs:104:17
    |
104 | /                 struct_span_err!(
104 | /                 struct_span_err!(
105 | |                     ctxt.tcx.sess,
106 | |                     ctxt.tcx.def_span(id.def_id.to_def_id()),
107 | |                     E0137,
108 | |                     "multiple functions with a `#[rustc_main]` attribute"
    | |_________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/entry.rs:122:17
    |
122 | /                 struct_span_err!(
122 | /                 struct_span_err!(
123 | |                     ctxt.tcx.sess,
124 | |                     ctxt.tcx.def_span(id.def_id.to_def_id()),
125 | |                     E0138,
126 | |                     "multiple `start` functions"
    | |_________________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/entry.rs:122:17
    |
122 | /                 struct_span_err!(
122 | /                 struct_span_err!(
123 | |                     ctxt.tcx.sess,
124 | |                     ctxt.tcx.def_span(id.def_id.to_def_id()),
125 | |                     E0138,
126 | |                     "multiple `start` functions"
    | |_________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_passes/src/entry.rs:146:22
146 |                     .struct_span_err(
    |                      ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_passes/src/entry.rs:146:22
146 |                     .struct_span_err(
    |                      ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/entry.rs:181:19
    |
181 |       let mut err = struct_span_err!(
---
186 | |         tcx.crate_name(LOCAL_CRATE)
187 | |     );
    | |_____- in this macro invocation

error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/entry.rs:181:19
    |
181 |       let mut err = struct_span_err!(
---
186 | |         tcx.crate_name(LOCAL_CRATE)
187 | |     );
    | |_____- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/lang_items.rs:68:35
    |
    |
68  |                       Some(span) => struct_span_err!(
69  | |                         self.tcx.sess,
70  | |                         span,
71  | |                         E0152,
71  | |                         E0152,
72  | |                         "found duplicate lang item `{}`",
74  | |                     ),
    | |_____________________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/lang_items.rs:68:35
    |
    |
68  |                       Some(span) => struct_span_err!(
69  | |                         self.tcx.sess,
70  | |                         span,
71  | |                         E0152,
71  | |                         E0152,
72  | |                         "found duplicate lang item `{}`",
74  | |                     ),
    | |_____________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
   |
77 | ...                   self.tcx.sess.struct_err(&format!(


error: diagnostics should be created using translatable messages
   |
   |
77 | ...                   self.tcx.sess.struct_err(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
   |
84 |                         _ => self.tcx.sess.struct_err(&format!(


error: diagnostics should be created using translatable messages
   |
   |
84 |                         _ => self.tcx.sess.struct_err(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/lang_items.rs:180:17
    |
180 | /                 struct_span_err!(
---
188 | |                     pluralized,
189 | |                 )
    | |_________________- in this macro invocation

error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/lang_items.rs:180:17
    |
180 | /                 struct_span_err!(
---
188 | |                     pluralized,
189 | |                 )
    | |_________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
38 |                         tcx.sess.span_err(
   |                                  ^^^^^^^^


error: diagnostics should be created using translatable messages
   |
38 |                         tcx.sess.span_err(
   |                                  ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
45 |                         tcx.sess.span_err(
   |                                  ^^^^^^^^


error: diagnostics should be created using translatable messages
   |
45 |                         tcx.sess.span_err(
   |                                  ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
52 |                         tcx.sess.span_err(
   |                                  ^^^^^^^^


error: diagnostics should be created using translatable messages
   |
52 |                         tcx.sess.span_err(
   |                                  ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
59 |                         tcx.sess.span_err(
   |                                  ^^^^^^^^


error: diagnostics should be created using translatable messages
   |
59 |                         tcx.sess.span_err(
   |                                  ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
73 |                         tcx.sess.span_err(
   |                                  ^^^^^^^^


error: diagnostics should be created using translatable messages
   |
73 |                         tcx.sess.span_err(
   |                                  ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
80 |                         tcx.sess.span_err(
   |                                  ^^^^^^^^


error: diagnostics should be created using translatable messages
   |
80 |                         tcx.sess.span_err(
   |                                  ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
90 |             tcx.sess.span_err(
   |                      ^^^^^^^^


error: diagnostics should be created using translatable messages
   |
90 |             tcx.sess.span_err(
   |                      ^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
609 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/lib_features.rs:129:9
    |
    |
129 |         struct_span_err!(self.tcx.sess, span, E0711, "{}", &msg).emit();


error: diagnostics should be created using translatable messages
    |
608 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
609 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/lib_features.rs:129:9
    |
    |
129 |         struct_span_err!(self.tcx.sess, span, E0711, "{}", &msg).emit();


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1327 |                     lint.build(&msg)


error: diagnostics should be created using translatable messages
     |
     |
1327 |                     lint.build(&msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1494 | ...                   lint.build(&format!(
     |                            ^^^^^


error: diagnostics should be created using translatable messages
     |
1494 | ...                   lint.build(&format!(
     |                            ^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1511 | ...                   lint.build(&format!("unused variable: `{}`", name))


error: diagnostics should be created using translatable messages
     |
     |
1511 | ...                   lint.build(&format!("unused variable: `{}`", name))


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1604 |                         lint.build(&format!("variable `{}` is assigned to, but never used", name))


error: diagnostics should be created using translatable messages
     |
     |
1604 |                         lint.build(&format!("variable `{}` is assigned to, but never used", name))


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1615 |                         let mut err = lint.build(&format!("unused variable: `{}`", name));


error: diagnostics should be created using translatable messages
     |
     |
1615 |                         let mut err = lint.build(&format!("unused variable: `{}`", name));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1664 | ...                   let mut err = lint.build(&format!("unused variable: `{}`", name));


error: diagnostics should be created using translatable messages
     |
     |
1664 | ...                   let mut err = lint.build(&format!("unused variable: `{}`", name));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1687 | ...                   let mut err = lint.build(&format!("unused variable: `{}`", name));


error: diagnostics should be created using translatable messages
     |
     |
1687 | ...                   let mut err = lint.build(&format!("unused variable: `{}`", name));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1761 |                     lint.build(&message(&name))


error: diagnostics should be created using translatable messages
     |
     |
1761 |                     lint.build(&message(&name))


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:119:43
    |
119 |   ...                   let mut err = struct_span_err!(
119 |   ...                   let mut err = struct_span_err!(
    |  _____________________________________-
120 | | ...                       self.sess,
121 | | ...                       e.span,
122 | | ...                       E0571,
123 | | ...                       "`break` with value from a `{}` loop",
124 | | ...                       kind.name()
    | |_______________________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:119:43
    |
119 |   ...                   let mut err = struct_span_err!(
119 |   ...                   let mut err = struct_span_err!(
    |  _____________________________________-
120 | | ...                       self.sess,
121 | | ...                       e.span,
122 | | ...                       E0571,
123 | | ...                       "`break` with value from a `{}` loop",
124 | | ...                       kind.name()
    | |_______________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:194:29
    |
194 | / ...                   struct_span_err!(
194 | / ...                   struct_span_err!(
195 | | ...                       self.sess,
196 | | ...                       e.span,
197 | | ...                       E0696,
198 | | ...                       "`continue` pointing to a labeled block"
    | |_______________________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:194:29
    |
194 | / ...                   struct_span_err!(
194 | / ...                   struct_span_err!(
195 | | ...                       self.sess,
196 | | ...                       e.span,
197 | | ...                       E0696,
198 | | ...                       "`continue` pointing to a labeled block"
    | |_______________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
609 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:230:13
    |
    |
230 |             struct_span_err!(self.sess, span, E0267, "`{}` inside of {} {}", name, article, ty)


error: diagnostics should be created using translatable messages
    |
608 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
609 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:230:13
    |
    |
230 |             struct_span_err!(self.sess, span, E0267, "`{}` inside of {} {}", name, article, ty)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
609 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:241:17
    |
    |
241 |                 struct_span_err!(self.sess, span, E0268, "`{}` outside of a loop", name)


error: diagnostics should be created using translatable messages
    |
608 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
609 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:241:17
    |
    |
241 |                 struct_span_err!(self.sess, span, E0268, "`{}` outside of a loop", name)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:256:17
    |
256 | /                 struct_span_err!(
---
261 | |                     cf_type
262 | |                 )
    | |_________________- in this macro invocation

error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:256:17
    |
256 | /                 struct_span_err!(
---
261 | |                     cf_type
262 | |                 )
    | |_________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:278:9
    |
278 | /         struct_span_err!(
278 | /         struct_span_err!(
279 | |             self.sess,
280 | |             span,
281 | |             E0590,
282 | |             "`break` or `continue` with no label in the condition of a `while` loop"
    | |_________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:278:9
    |
278 | /         struct_span_err!(
278 | /         struct_span_err!(
279 | |             self.sess,
280 | |             span,
281 | |             E0590,
282 | |             "`break` or `continue` with no label in the condition of a `while` loop"
    | |_________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
  --> compiler/rustc_passes/src/naked_functions.rs:59:18
   |
59 |         tcx.sess.struct_span_err(attr.span, "naked functions cannot be inlined").emit();


error: diagnostics should be created using translatable messages
  --> compiler/rustc_passes/src/naked_functions.rs:59:18
   |
59 |         tcx.sess.struct_span_err(attr.span, "naked functions cannot be inlined").emit();


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
  --> compiler/rustc_passes/src/naked_functions.rs:69:18
   |
69 |             lint.build("Rust ABI is unsupported in naked functions").emit();


error: diagnostics should be created using translatable messages
  --> compiler/rustc_passes/src/naked_functions.rs:69:18
   |
69 |             lint.build("Rust ABI is unsupported in naked functions").emit();


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
  --> compiler/rustc_passes/src/naked_functions.rs:82:22
82 |                     .struct_span_err(
   |                      ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
  --> compiler/rustc_passes/src/naked_functions.rs:82:22
82 |                     .struct_span_err(
   |                      ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_passes/src/naked_functions.rs:118:22
118 |                     .struct_span_err(
    |                      ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_passes/src/naked_functions.rs:118:22
118 |                     .struct_span_err(
    |                      ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/naked_functions.rs:138:24
    |
138 |           let mut diag = struct_span_err!(
138 |           let mut diag = struct_span_err!(
    |  ________________________-
139 | |             tcx.sess,
140 | |             tcx.def_span(def_id),
141 | |             E0787,
142 | |             "naked functions must contain a single asm block"
    | |_________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/naked_functions.rs:138:24
    |
138 |           let mut diag = struct_span_err!(
138 |           let mut diag = struct_span_err!(
    |  ________________________-
139 | |             tcx.sess,
140 | |             tcx.def_span(def_id),
141 | |             E0787,
142 | |             "naked functions must contain a single asm block"
    | |_________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/naked_functions.rs:250:13
    |
250 | /             struct_span_err!(
250 | /             struct_span_err!(
251 | |                 self.tcx.sess,
252 | |                 unsupported_operands,
253 | |                 E0787,
254 | |                 "only `const` and `sym` operands are supported in naked functions",
    | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/naked_functions.rs:250:13
    |
250 | /             struct_span_err!(
250 | /             struct_span_err!(
251 | |                 self.tcx.sess,
252 | |                 unsupported_operands,
253 | |                 E0787,
254 | |                 "only `const` and `sym` operands are supported in naked functions",
    | |_____________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/naked_functions.rs:272:13
    |
272 | /             struct_span_err!(
272 | /             struct_span_err!(
273 | |                 self.tcx.sess,
274 | |                 span,
275 | |                 E0787,
276 | |                 "asm options unsupported in naked functions: {}",
277 | |                 unsupported_options.join(", ")
    | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/naked_functions.rs:272:13
    |
272 | /             struct_span_err!(
272 | /             struct_span_err!(
273 | |                 self.tcx.sess,
274 | |                 span,
275 | |                 E0787,
276 | |                 "asm options unsupported in naked functions: {}",
277 | |                 unsupported_options.join(", ")
    | |_____________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/naked_functions.rs:289:13
    |
289 | /             struct_span_err!(
289 | /             struct_span_err!(
290 | |                 self.tcx.sess,
291 | |                 span,
292 | |                 E0787,
293 | |                 "asm in naked functions must use `noreturn` option"
    | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/naked_functions.rs:289:13
    |
289 | /             struct_span_err!(
289 | /             struct_span_err!(
290 | |                 self.tcx.sess,
291 | |                 span,
292 | |                 E0787,
293 | |                 "asm in naked functions must use `noreturn` option"
    | |_____________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
125 |                     lint.build("this `#[deprecated]` annotation has no effect")


error: diagnostics should be created using translatable messages
    |
    |
125 |                     lint.build("this `#[deprecated]` annotation has no effect")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/stability.rs:202:17
    |
202 | /                 struct_span_err!(
---
207 | |                     either stable or unstable attribute"
208 | |                 )
    | |_________________- in this macro invocation

error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/stability.rs:202:17
    |
202 | /                 struct_span_err!(
---
207 | |                     either stable or unstable attribute"
208 | |                 )
    | |_________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
225 |                 self.tcx.sess.struct_span_err(span,"this stability annotation is useless")


error: diagnostics should be created using translatable messages
    |
    |
225 |                 self.tcx.sess.struct_span_err(span,"this stability annotation is useless")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
244 | ...                   self.tcx.sess.struct_span_err(span, "invalid stability version found")


error: diagnostics should be created using translatable messages
    |
    |
244 | ...                   self.tcx.sess.struct_span_err(span, "invalid stability version found")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
253 | ...                   self.tcx.sess.struct_span_err(span, "an API can't be stabilized after it is deprecated")


error: diagnostics should be created using translatable messages
    |
    |
253 | ...                   self.tcx.sess.struct_span_err(span, "an API can't be stabilized after it is deprecated")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
264 | ...                   self.tcx.sess.struct_span_err(span, "invalid deprecation version found")


error: diagnostics should be created using translatable messages
    |
    |
264 | ...                   self.tcx.sess.struct_span_err(span, "invalid deprecation version found")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
536 |             self.tcx.sess.span_err(span, &format!("{} has missing stability attribute", descr));


error: diagnostics should be created using translatable messages
    |
    |
536 |             self.tcx.sess.span_err(span, &format!("{} has missing stability attribute", descr));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
556 |             self.tcx.sess.span_err(span, &format!("{descr} has missing const stability attribute"));


error: diagnostics should be created using translatable messages
    |
    |
556 |             self.tcx.sess.span_err(span, &format!("{descr} has missing const stability attribute"));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
758 | ...                   .build("an `#[unstable]` annotation here has no effect")


error: diagnostics should be created using translatable messages
    |
    |
758 | ...                   .build("an `#[unstable]` annotation here has no effect")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
773 | ...                   .struct_span_err(item.span, "trait implementations cannot be const stable yet")


error: diagnostics should be created using translatable messages
    |
    |
773 | ...                   .struct_span_err(item.span, "trait implementations cannot be const stable yet")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/stability.rs:944:13
    |
944 | /             struct_span_err!(
944 | /             struct_span_err!(
945 | |                 tcx.sess,
946 | |                 *span,
947 | |                 E0554,
948 | |                 "`#![feature]` may not be used on the {} release channel",
949 | |                 env!("CFG_RELEASE_CHANNEL")
    | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/stability.rs:944:13
    |
944 | /             struct_span_err!(
944 | /             struct_span_err!(
945 | |                 tcx.sess,
946 | |                 *span,
947 | |                 E0554,
948 | |                 "`#![feature]` may not be used on the {} release channel",
949 | |                 env!("CFG_RELEASE_CHANNEL")
    | |_____________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
608  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
609  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/stability.rs:1056:9
     |
     |
1056 |         struct_span_err!(tcx.sess, span, E0635, "unknown feature `{}`", feature).emit();


error: diagnostics should be created using translatable messages
     |
608  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
609  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/stability.rs:1056:9
     |
     |
1056 |         struct_span_err!(tcx.sess, span, E0635, "unknown feature `{}`", feature).emit();


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1068 |             .struct_span_err(
     |              ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
     |
1068 |             .struct_span_err(
     |              ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1087 |         lint.build(&format!(
     |              ^^^^^


error: diagnostics should be created using translatable messages
     |
1087 |         lint.build(&format!(
     |              ^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1111 |         lint.build(&format!(
     |              ^^^^^


error: diagnostics should be created using translatable messages
     |
1111 |         lint.build(&format!(
     |              ^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
608  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
609  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/stability.rs:1120:5
     |
     |
1120 |     struct_span_err!(sess, span, E0636, "the feature `{}` has already been declared", feature)


error: diagnostics should be created using translatable messages
     |
608  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
609  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/stability.rs:1120:5
     |
     |
1120 |     struct_span_err!(sess, span, E0636, "the feature `{}` has already been declared", feature)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1130 |         .struct_span_err(fn_sig_span, ERROR_MSG)


error: diagnostics should be created using translatable messages
     |
     |
1130 |         .struct_span_err(fn_sig_span, ERROR_MSG)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/weak_lang_items.rs:36:17
    |
36  | /                 struct_span_err!(
36  | /                 struct_span_err!(
37  | |                     tcx.sess,
38  | |                     span,
39  | |                     E0264,
40  | |                     "unknown external lang item: `{}`",
42  | |                 )
    | |_________________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
608 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
609 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/weak_lang_items.rs:36:17
    |
36  | /                 struct_span_err!(
36  | /                 struct_span_err!(
37  | |                     tcx.sess,
38  | |                     span,
39  | |                     E0264,
40  | |                     "unknown external lang item: `{}`",
42  | |                 )
    | |_________________- in this macro invocation

error: could not compile `rustc_passes` due to 137 previous errors
