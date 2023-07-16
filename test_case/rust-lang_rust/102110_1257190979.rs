plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
372 |                 lint.build(&format!("`#[{name}]` only has an effect on {}", supported_names))
    |
note: the lint level is defined here
   --> compiler/rustc_passes/src/lib.rs:9:9
    |
    |
9   | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
    |
    |
372 |                 lint.build(&format!("`#[{name}]` only has an effect on {}", supported_names))
    |
note: the lint level is defined here
   --> compiler/rustc_passes/src/lib.rs:8:9
    |
    |
8   | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
430 |                 tcx.sess.span_err(p.span, &repr);


error: diagnostics should be created using translatable messages
    |
    |
430 |                 tcx.sess.span_err(p.span, &repr);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
879 |                 let mut err = lint.build(fluent::passes::attr_crate_level);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
668  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
669  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1663:21
     |
1663 | /                     struct_span_err!(
1663 | /                     struct_span_err!(
1664 | |                         self.tcx.sess,
1665 | |                         hint.span(),
1666 | |                         E0552,
1667 | |                         "unrecognized representation hint"
     | |_____________________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
668  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
669  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1663:21
     |
1663 | /                     struct_span_err!(
1663 | /                     struct_span_err!(
1664 | |                         self.tcx.sess,
1665 | |                         hint.span(),
1666 | |                         E0552,
1667 | |                         "unrecognized representation hint"
     | |_____________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
668  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
669  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1677:13
     |
1677 | /             struct_span_err!(
1677 | /             struct_span_err!(
1678 | |                 self.tcx.sess,
1679 | |                 hint.span(),
1680 | |                 E0517,
1681 | |                 "{}",
1682 | |                 &format!("attribute should be applied to {article} {allowed_targets}")
     | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
668  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
669  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1677:13
     |
1677 | /             struct_span_err!(
1677 | /             struct_span_err!(
1678 | |                 self.tcx.sess,
1679 | |                 hint.span(),
1680 | |                 E0517,
1681 | |                 "{}",
1682 | |                 &format!("attribute should be applied to {article} {allowed_targets}")
     | |_____________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
668  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
669  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1695:13
     |
1695 | /             struct_span_err!(
1695 | /             struct_span_err!(
1696 | |                 self.tcx.sess,
1697 | |                 hint_spans,
1698 | |                 E0692,
1699 | |                 "transparent {} cannot have other repr hints",
1701 | |             )
     | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
668  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
669  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1695:13
     |
1695 | /             struct_span_err!(
1695 | /             struct_span_err!(
1696 | |                 self.tcx.sess,
1697 | |                 hint_spans,
1698 | |                 E0692,
1699 | |                 "transparent {} cannot have other repr hints",
1701 | |             )
     | |_____________- in this macro invocation


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
748 |                     let mut err = lint.build(&format!(
    |                                        ^^^^^


error: diagnostics should be created using translatable messages
    |
748 |                     let mut err = lint.build(&format!(
    |                                        ^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
  --> compiler/rustc_passes/src/entry.rs:79:14
   |
79 |             .struct_span_err(attr.span, &format!("`{}` attribute {}", sym, details))


error: diagnostics should be created using translatable messages
  --> compiler/rustc_passes/src/entry.rs:79:14
   |
79 |             .struct_span_err(attr.span, &format!("`{}` attribute {}", sym, details))


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/entry.rs:123:17
    |
123 | /                 struct_span_err!(
123 | /                 struct_span_err!(
124 | |                     ctxt.tcx.sess,
125 | |                     ctxt.tcx.def_span(id.def_id.to_def_id()),
126 | |                     E0138,
127 | |                     "multiple `start` functions"
    | |_________________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/entry.rs:123:17
    |
123 | /                 struct_span_err!(
123 | /                 struct_span_err!(
124 | |                     ctxt.tcx.sess,
125 | |                     ctxt.tcx.def_span(id.def_id.to_def_id()),
126 | |                     E0138,
127 | |                     "multiple `start` functions"
    | |_________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_passes/src/entry.rs:148:22
148 |                     .struct_span_err(
    |                      ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_passes/src/entry.rs:148:22
148 |                     .struct_span_err(
    |                      ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_passes/src/entry.rs:186:22
186 |                     .struct_span_err(
    |                      ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_passes/src/entry.rs:186:22
186 |                     .struct_span_err(
    |                      ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/entry.rs:209:19
    |
209 |       let mut err = struct_span_err!(
---
214 | |         tcx.crate_name(LOCAL_CRATE)
215 | |     );
    | |_____- in this macro invocation

error: diagnostics should be created using translatable messages
    |
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/entry.rs:209:19
    |
209 |       let mut err = struct_span_err!(
---
214 | |         tcx.crate_name(LOCAL_CRATE)
215 | |     );
    | |_____- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
     |
1328 |                     lint.build(&msg)


error: diagnostics should be created using translatable messages
     |
     |
1328 |                     lint.build(&msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1495 | ...                   lint.build(&format!(
     |                            ^^^^^


error: diagnostics should be created using translatable messages
     |
1495 | ...                   lint.build(&format!(
     |                            ^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1512 | ...                   lint.build(&format!("unused variable: `{}`", name))


error: diagnostics should be created using translatable messages
     |
     |
1512 | ...                   lint.build(&format!("unused variable: `{}`", name))


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1605 |                         lint.build(&format!("variable `{}` is assigned to, but never used", name))


error: diagnostics should be created using translatable messages
     |
     |
1605 |                         lint.build(&format!("variable `{}` is assigned to, but never used", name))


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1616 |                         let mut err = lint.build(&format!("unused variable: `{}`", name));


error: diagnostics should be created using translatable messages
     |
     |
1616 |                         let mut err = lint.build(&format!("unused variable: `{}`", name));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1665 | ...                   let mut err = lint.build(&format!("unused variable: `{}`", name));


error: diagnostics should be created using translatable messages
     |
     |
1665 | ...                   let mut err = lint.build(&format!("unused variable: `{}`", name));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1688 | ...                   let mut err = lint.build(&format!("unused variable: `{}`", name));


error: diagnostics should be created using translatable messages
     |
     |
1688 | ...                   let mut err = lint.build(&format!("unused variable: `{}`", name));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1762 |                     lint.build(&message(&name))


error: diagnostics should be created using translatable messages
     |
     |
1762 |                     lint.build(&message(&name))


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
669 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:230:13
    |
    |
230 |             struct_span_err!(self.sess, span, E0267, "`{}` inside of {} {}", name, article, ty)


error: diagnostics should be created using translatable messages
    |
668 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
669 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:230:13
    |
    |
230 |             struct_span_err!(self.sess, span, E0267, "`{}` inside of {} {}", name, article, ty)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
668 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
669 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:241:17
    |
    |
241 |                 struct_span_err!(self.sess, span, E0268, "`{}` outside of a loop", name)


error: diagnostics should be created using translatable messages
    |
668 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
669 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/loops.rs:241:17
    |
    |
241 |                 struct_span_err!(self.sess, span, E0268, "`{}` outside of a loop", name)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
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
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/stability.rs:200:17
    |
200 | /                 struct_span_err!(
---
205 | |                     either stable or unstable attribute"
206 | |                 )
    | |_________________- in this macro invocation

error: diagnostics should be created using translatable messages
    |
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/stability.rs:200:17
    |
200 | /                 struct_span_err!(
---
205 | |                     either stable or unstable attribute"
206 | |                 )
    | |_________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
223 |                 self.tcx.sess.struct_span_err(span,"this stability annotation is useless")


error: diagnostics should be created using translatable messages
    |
    |
223 |                 self.tcx.sess.struct_span_err(span,"this stability annotation is useless")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
242 | ...                   self.tcx.sess.struct_span_err(span, "invalid stability version found")


error: diagnostics should be created using translatable messages
    |
    |
242 | ...                   self.tcx.sess.struct_span_err(span, "invalid stability version found")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
251 | ...                   self.tcx.sess.struct_span_err(span, "an API can't be stabilized after it is deprecated")


error: diagnostics should be created using translatable messages
    |
    |
251 | ...                   self.tcx.sess.struct_span_err(span, "an API can't be stabilized after it is deprecated")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
262 | ...                   self.tcx.sess.struct_span_err(span, "invalid deprecation version found")


error: diagnostics should be created using translatable messages
    |
    |
262 | ...                   self.tcx.sess.struct_span_err(span, "invalid deprecation version found")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
534 |             self.tcx.sess.span_err(span, &format!("{} has missing stability attribute", descr));


error: diagnostics should be created using translatable messages
    |
    |
534 |             self.tcx.sess.span_err(span, &format!("{} has missing stability attribute", descr));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
554 |             self.tcx.sess.span_err(span, &format!("{descr} has missing const stability attribute"));


error: diagnostics should be created using translatable messages
    |
    |
554 |             self.tcx.sess.span_err(span, &format!("{descr} has missing const stability attribute"));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
756 | ...                   .build("an `#[unstable]` annotation here has no effect")


error: diagnostics should be created using translatable messages
    |
    |
756 | ...                   .build("an `#[unstable]` annotation here has no effect")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
771 | ...                   .struct_span_err(item.span, "trait implementations cannot be const stable yet")


error: diagnostics should be created using translatable messages
    |
    |
771 | ...                   .struct_span_err(item.span, "trait implementations cannot be const stable yet")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/stability.rs:942:13
    |
942 | /             struct_span_err!(
942 | /             struct_span_err!(
943 | |                 tcx.sess,
944 | |                 *span,
945 | |                 E0554,
946 | |                 "`#![feature]` may not be used on the {} release channel",
947 | |                 env!("CFG_RELEASE_CHANNEL")
    | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
668 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
669 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/stability.rs:942:13
    |
942 | /             struct_span_err!(
942 | /             struct_span_err!(
943 | |                 tcx.sess,
944 | |                 *span,
945 | |                 E0554,
946 | |                 "`#![feature]` may not be used on the {} release channel",
947 | |                 env!("CFG_RELEASE_CHANNEL")
    | |_____________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
668  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
669  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/stability.rs:1054:9
     |
     |
1054 |         struct_span_err!(tcx.sess, span, E0635, "unknown feature `{}`", feature).emit();


error: diagnostics should be created using translatable messages
     |
668  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
669  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/stability.rs:1054:9
     |
     |
1054 |         struct_span_err!(tcx.sess, span, E0635, "unknown feature `{}`", feature).emit();


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1066 |             .struct_span_err(
     |              ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
     |
1066 |             .struct_span_err(
     |              ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1085 |         lint.build(&format!(
     |              ^^^^^


error: diagnostics should be created using translatable messages
     |
1085 |         lint.build(&format!(
     |              ^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1117 |         lint.build(&format!(
     |              ^^^^^


error: diagnostics should be created using translatable messages
     |
1117 |         lint.build(&format!(
     |              ^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
668  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
669  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/stability.rs:1126:5
     |
     |
1126 |     struct_span_err!(sess, span, E0636, "the feature `{}` has already been declared", feature)


error: diagnostics should be created using translatable messages
     |
668  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
669  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
670  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/stability.rs:1126:5
     |
     |
1126 |     struct_span_err!(sess, span, E0636, "the feature `{}` has already been declared", feature)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1136 |         .struct_span_err(fn_sig_span, ERROR_MSG)


error: diagnostics should be created using translatable messages
     |
     |
1136 |         .struct_span_err(fn_sig_span, ERROR_MSG)

error: could not compile `rustc_passes` due to 111 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_passes` due to 111 previous errors
