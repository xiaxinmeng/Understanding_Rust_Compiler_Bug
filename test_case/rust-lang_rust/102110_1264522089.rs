plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
435 |                 tcx.sess.span_err(p.span, &repr);
    |
note: the lint level is defined here
   --> compiler/rustc_passes/src/lib.rs:9:9
    |
    |
9   | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
    |
    |
435 |                 tcx.sess.span_err(p.span, &repr);
    |
note: the lint level is defined here
   --> compiler/rustc_passes/src/lib.rs:8:9
    |
    |
8   | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
681  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
682  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1674:21
     |
1674 | /                     struct_span_err!(
1674 | /                     struct_span_err!(
1675 | |                         self.tcx.sess,
1676 | |                         hint.span(),
1677 | |                         E0552,
1678 | |                         "unrecognized representation hint"
     | |_____________________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
681  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
682  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1674:21
     |
1674 | /                     struct_span_err!(
1674 | /                     struct_span_err!(
1675 | |                         self.tcx.sess,
1676 | |                         hint.span(),
1677 | |                         E0552,
1678 | |                         "unrecognized representation hint"
     | |_____________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
681  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
682  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1688:13
     |
1688 | /             struct_span_err!(
1688 | /             struct_span_err!(
1689 | |                 self.tcx.sess,
1690 | |                 hint.span(),
1691 | |                 E0517,
1692 | |                 "{}",
1693 | |                 &format!("attribute should be applied to {article} {allowed_targets}")
     | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
681  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
682  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1688:13
     |
1688 | /             struct_span_err!(
1688 | /             struct_span_err!(
1689 | |                 self.tcx.sess,
1690 | |                 hint.span(),
1691 | |                 E0517,
1692 | |                 "{}",
1693 | |                 &format!("attribute should be applied to {article} {allowed_targets}")
     | |_____________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
681  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
682  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1706:13
     |
1706 | /             struct_span_err!(
1706 | /             struct_span_err!(
1707 | |                 self.tcx.sess,
1708 | |                 hint_spans,
1709 | |                 E0692,
1710 | |                 "transparent {} cannot have other repr hints",
1712 | |             )
     | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
681  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
682  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/check_attr.rs:1706:13
     |
1706 | /             struct_span_err!(
1706 | /             struct_span_err!(
1707 | |                 self.tcx.sess,
1708 | |                 hint_spans,
1709 | |                 E0692,
1710 | |                 "transparent {} cannot have other repr hints",
1712 | |             )
     | |_____________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
681 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
682 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683 |           $session.struct_span_err_with_code(
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
681 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
682 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683 |           $session.struct_span_err_with_code(
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
769 | ...                   .struct_span_err(item.span, "trait implementations cannot be const stable yet")


error: diagnostics should be created using translatable messages
    |
    |
769 | ...                   .struct_span_err(item.span, "trait implementations cannot be const stable yet")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
681 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
682 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/stability.rs:940:13
    |
940 | /             struct_span_err!(
940 | /             struct_span_err!(
941 | |                 tcx.sess,
942 | |                 *span,
943 | |                 E0554,
944 | |                 "`#![feature]` may not be used on the {} release channel",
945 | |                 env!("CFG_RELEASE_CHANNEL")
    | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
    |
681 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
682 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_passes/src/stability.rs:940:13
    |
940 | /             struct_span_err!(
940 | /             struct_span_err!(
941 | |                 tcx.sess,
942 | |                 *span,
943 | |                 E0554,
944 | |                 "`#![feature]` may not be used on the {} release channel",
945 | |                 env!("CFG_RELEASE_CHANNEL")
    | |_____________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
681  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
682  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/stability.rs:1052:9
     |
     |
1052 |         struct_span_err!(tcx.sess, span, E0635, "unknown feature `{}`", feature).emit();


error: diagnostics should be created using translatable messages
     |
681  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
682  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/stability.rs:1052:9
     |
     |
1052 |         struct_span_err!(tcx.sess, span, E0635, "unknown feature `{}`", feature).emit();


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1064 |             .struct_span_err(
     |              ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
     |
1064 |             .struct_span_err(
     |              ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
681  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
682  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/stability.rs:1125:5
     |
     |
1125 |     struct_span_err!(sess, span, E0636, "the feature `{}` has already been declared", feature)


error: diagnostics should be created using translatable messages
     |
681  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
682  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
683  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_passes/src/stability.rs:1125:5
     |
     |
1125 |     struct_span_err!(sess, span, E0636, "the feature `{}` has already been declared", feature)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1135 |         .struct_span_err(fn_sig_span, ERROR_MSG)


error: diagnostics should be created using translatable messages
     |
     |
1135 |         .struct_span_err(fn_sig_span, ERROR_MSG)

error: could not compile `rustc_passes` due to 34 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_passes` due to 34 previous errors
