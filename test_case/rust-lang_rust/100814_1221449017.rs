plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
56 |     err.note(
   |         ^^^^
   |
   |
note: the lint level is defined here
  --> compiler/rustc_trait_selection/src/lib.rs:25:9
   |
25 | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
   |
56 |     err.note(
   |         ^^^^
   |
   |
note: the lint level is defined here
  --> compiler/rustc_trait_selection/src/lib.rs:24:9
   |
24 | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
223 |                   .struct_span_fatal(
    |                    ^^^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
223 |                   .struct_span_fatal(
    |                    ^^^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
264 | ...                   err.build("cannot use constants which depend on generic parameters in types").emit();


error: diagnostics should be created using translatable messages
    |
    |
264 | ...                   err.build("cannot use constants which depend on generic parameters in types").emit();


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:760:25
    |
760 |                     err.note(msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:760:25
    |
760 |                     err.note(msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:825:21
    |
825 |                 err.span_label(*fn_decl_span, "consider calling this closure");


error: diagnostics should be created using translatable messages
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:825:21
    |
825 |                 err.span_label(*fn_decl_span, "consider calling this closure");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:838:21
    |
838 |                 err.span_label(ident.span, "consider calling this function");


error: diagnostics should be created using translatable messages
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:838:21
    |
838 |                 err.span_label(ident.span, "consider calling this function");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:870:17
    |
870 |             err.help(&format!("{}: `{}`", msg, snippet));


error: diagnostics should be created using translatable messages
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:870:17
    |
870 |             err.help(&format!("{}: `{}`", msg, snippet));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:966:33
    |
966 | ...                   err.note(&msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:966:33
    |
966 | ...                   err.note(&msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:976:29
976 |                         err.span_label(
    |                             ^^^^^^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:976:29
976 |                         err.span_label(
    |                             ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1145:33
1145 | ...                   err.span_label(
     |                           ^^^^^^^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1145:33
1145 | ...                   err.span_label(
     |                           ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1244:29
     |
1244 |                         err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1244:29
     |
1244 |                         err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1280:17
1280 |             err.span_label(
     |                 ^^^^^^^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1280:17
1280 |             err.span_label(
     |                 ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1483:17
     |
1483 |             err.note(impl_trait_msg);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1483:17
     |
1483 |             err.note(impl_trait_msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1510:21
     |
1510 |                 err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1510:21
     |
1510 |                 err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1515:17
     |
1515 |             err.note(trait_obj_msg);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1515:17
     |
1515 |             err.note(trait_obj_msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1516:17
     |
1516 |             err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1516:17
     |
1516 |             err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1521:17
     |
1521 |             err.note(impl_trait_msg);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1521:17
     |
1521 |             err.note(impl_trait_msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1522:17
     |
1522 |             err.note("you can create a new `enum` with a variant for each returned type");


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1522:17
     |
1522 |             err.note("you can create a new `enum` with a variant for each returned type");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1551:25
     |
1551 |                     err.span_label(expr.span, &format!("this returned value is of type `{}`", ty));


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1551:25
     |
1551 |                     err.span_label(expr.span, &format!("this returned value is of type `{}`", ty));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1598:23
     |
1598 |           let mut err = struct_span_err!(
1598 |           let mut err = struct_span_err!(
     |  _______________________-
1599 | |             self.tcx.sess,
1600 | |             span,
1601 | |             E0631,
1602 | |             "type mismatch in {argument_kind} arguments",
     | |_________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1598:23
     |
1598 |           let mut err = struct_span_err!(
1598 |           let mut err = struct_span_err!(
     |  _______________________-
1599 | |             self.tcx.sess,
1600 | |             span,
1601 | |             E0631,
1602 | |             "type mismatch in {argument_kind} arguments",
     | |_________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1631:21
     |
1631 |                 err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1631:21
     |
1631 |                 err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2048:21
2048 |                 err.span_note(
     |                     ^^^^^^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2048:21
2048 |                 err.span_note(
     |                     ^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2056:25
     |
2056 |                     err.span_note(span, &msg);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2056:25
     |
2056 |                     err.span_note(span, &msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2073:29
2073 |                         err.span_note(
     |                             ^^^^^^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2073:29
2073 |                         err.span_note(
     |                             ^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2130:41
2130 | ...                   err.span_help(
     |                           ^^^^^^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2130:41
2130 | ...                   err.span_help(
     |                           ^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2172:21
     |
2172 |                 err.span_note(span, &span_note);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2172:21
     |
2172 |                 err.span_note(span, &span_note);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2227:21
     |
2227 |                 err.note("slice and array elements must have `Sized` type");


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2227:21
     |
2227 |                 err.note("slice and array elements must have `Sized` type");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2230:21
     |
2230 |                 err.note("only the last element of a tuple may have a dynamically sized type");


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2230:21
     |
2230 |                 err.note("only the last element of a tuple may have a dynamically sized type");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2233:21
     |
2233 |                 err.note(&format!("required so that the projection `{}` is well-formed", data,));


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2233:21
     |
2233 |                 err.note(&format!("required so that the projection `{}` is well-formed", data,));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2236:21
     |
2236 |                 err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2236:21
     |
2236 |                 err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2242:21
     |
2242 |                 err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2242:21
     |
2242 |                 err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2269:25
     |
2269 |                     err.span_note(multispan, &descr);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2269:25
     |
2269 |                     err.span_note(multispan, &descr);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2271:25
     |
2271 |                     err.span_note(tcx.def_span(item_def_id), &descr);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2271:25
     |
2271 |                     err.span_note(tcx.def_span(item_def_id), &descr);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2275:21
     |
2275 |                 err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2275:21
     |
2275 |                 err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2282:21
     |
2282 |                 err.note(&format!("required by cast to type `{}`", self.ty_to_string(target)));


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2282:21
     |
2282 |                 err.note(&format!("required by cast to type `{}`", self.ty_to_string(target)));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2285:21
2285 |                 err.note(
     |                     ^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2285:21
2285 |                 err.note(
     |                     ^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2290:25
2290 |                     err.help(
     |                         ^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2290:25
2290 |                     err.help(
     |                         ^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2298:25
2298 |                     err.help(
     |                         ^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2298:25
2298 |                     err.help(
     |                         ^^^^


---
233 | |             predicate
234 | |         );
    | |_________- in this macro invocation

error: diagnostics should be created using translatable messages
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:228:23
    |
228 |           let mut err = struct_span_err!(
---
233 | |             predicate
234 | |         );
    | |_________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
283 |                 let mut err = self.tcx.sess.struct_span_err(
    |                                             ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
283 |                 let mut err = self.tcx.sess.struct_span_err(
    |                                             ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:376:39
    |
376 |                           let mut err = struct_span_err!(
---
401 | |                                 ))
402 | |                         );
    | |_________________________- in this macro invocation

error: diagnostics should be created using translatable messages
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:376:39
    |
376 |                           let mut err = struct_span_err!(
---
401 | |                                 ))
402 | |                         );
    | |_________________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:794:25
    |
794 | /                         struct_span_err!(
---
799 | |                             predicate
800 | |                         )
    | |_________________________- in this macro invocation

error: diagnostics should be created using translatable messages
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:794:25
    |
794 | /                         struct_span_err!(
---
799 | |                             predicate
800 | |                         )
    | |_________________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:811:39
    |
811 |                           let mut err = struct_span_err!(
---
818 | |                             found_kind
819 | |                         );
    | |_________________________- in this macro invocation

error: diagnostics should be created using translatable messages
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:811:39
    |
811 |                           let mut err = struct_span_err!(
---
818 | |                             found_kind
819 | |                         );
    | |_________________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
878 | ...                   self.tcx.sess.struct_span_err(
    |                                     ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
878 | ...                   self.tcx.sess.struct_span_err(
    |                                     ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1019 |                     let mut err = self.tcx.sess.struct_span_err(
     |                                                 ^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
     |
1019 |                     let mut err = self.tcx.sess.struct_span_err(
     |                                                 ^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1037 | ...                   self.tcx.sess.struct_span_err(span, "unconstrained generic constant");


error: diagnostics should be created using translatable messages
     |
     |
1037 | ...                   self.tcx.sess.struct_span_err(span, "unconstrained generic constant");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1177:23
     |
1177 |           let mut err = struct_span_err!(
---
1184 | |             found_str,
1185 | |         );
     | |_________- in this macro invocation

error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1177:23
     |
1177 |           let mut err = struct_span_err!(
---
1184 | |             found_str,
1185 | |         );
     | |_________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
580  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1585:28
     |
     |
1585 |             let mut diag = struct_span_err!(self.tcx.sess, obligation.cause.span, E0271, "{msg}");


error: diagnostics should be created using translatable messages
     |
579  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
580  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1585:28
     |
     |
1585 |             let mut diag = struct_span_err!(self.tcx.sess, obligation.cause.span, E0271, "{msg}");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1825 |             err.help(&format!(


error: diagnostics should be created using translatable messages
     |
     |
1825 |             err.help(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1958 |                 err.span_help(impl_span, "trait impl with same name found");


error: diagnostics should be created using translatable messages
     |
     |
1958 |                 err.span_help(impl_span, "trait impl with same name found");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1964 |                 err.note(&crate_msg);


error: diagnostics should be created using translatable messages
     |
     |
1964 |                 err.note(&crate_msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:2063:21
     |
2063 | /                     struct_span_err!(
2063 | /                     struct_span_err!(
2064 | |                         self.tcx.sess,
2065 | |                         span,
2066 | |                         E0283,
2067 | |                         "type annotations needed: cannot satisfy `{}`",
2069 | |                     )
     | |_____________________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:2063:21
     |
2063 | /                     struct_span_err!(
2063 | /                     struct_span_err!(
2064 | |                         self.tcx.sess,
2065 | |                         span,
2066 | |                         E0283,
2067 | |                         "type annotations needed: cannot satisfy `{}`",
2069 | |                     )
     | |_____________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
2201 |                         err = self.tcx.sess.struct_span_err_with_code(
     |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
     |
2201 |                         err = self.tcx.sess.struct_span_err_with_code(
     |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:2294:35
     |
2294 |                       let mut err = struct_span_err!(
2294 |                       let mut err = struct_span_err!(
     |  ___________________________________-
2295 | |                         self.tcx.sess,
2296 | |                         span,
2297 | |                         E0284,
2298 | |                         "type annotations needed: cannot satisfy `{}`",
2300 | |                     );
     | |_____________________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:2294:35
     |
2294 |                       let mut err = struct_span_err!(
2294 |                       let mut err = struct_span_err!(
     |  ___________________________________-
2295 | |                         self.tcx.sess,
2296 | |                         span,
2297 | |                         E0284,
2298 | |                         "type annotations needed: cannot satisfy `{}`",
2300 | |                     );
     | |_____________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:2322:35
     |
2322 |                       let mut err = struct_span_err!(
2322 |                       let mut err = struct_span_err!(
     |  ___________________________________-
2323 | |                         self.tcx.sess,
2324 | |                         span,
2325 | |                         E0284,
2326 | |                         "type annotations needed: cannot satisfy `{}`",
2328 | |                     );
     | |_____________________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:2322:35
     |
2322 |                       let mut err = struct_span_err!(
2322 |                       let mut err = struct_span_err!(
     |  ___________________________________-
2323 | |                         self.tcx.sess,
2324 | |                         span,
2325 | |                         E0284,
2326 | |                         "type annotations needed: cannot satisfy `{}`",
2328 | |                     );
     | |_____________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:2337:31
     |
2337 |                   let mut err = struct_span_err!(
2337 |                   let mut err = struct_span_err!(
     |  _______________________________-
2338 | |                     self.tcx.sess,
2339 | |                     span,
2340 | |                     E0284,
2341 | |                     "type annotations needed: cannot satisfy `{}`",
2343 | |                 );
     | |_________________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:2337:31
     |
2337 |                   let mut err = struct_span_err!(
2337 |                   let mut err = struct_span_err!(
     |  _______________________________-
2338 | |                     self.tcx.sess,
2339 | |                     span,
2340 | |                     E0284,
2341 | |                     "type annotations needed: cannot satisfy `{}`",
2343 | |                 );
     | |_________________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
2409 |                 err.note(&format!("cannot satisfy `{}`", predicate));


error: diagnostics should be created using translatable messages
     |
     |
2409 |                 err.note(&format!("cannot satisfy `{}`", predicate));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
2412 |                 err.note(&format!("{} in the `{}` crate{}", msg, crates[0], post,));


error: diagnostics should be created using translatable messages
     |
     |
2412 |                 err.note(&format!("{} in the `{}` crate{}", msg, crates[0], post,));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
2415 |                 err.note(&format!(


error: diagnostics should be created using translatable messages
     |
     |
2415 |                 err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
2424 |                 err.span_note(span, &msg);


error: diagnostics should be created using translatable messages
     |
     |
2424 |                 err.span_note(span, &msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
2428 |                 err.span_note(span, &msg);


error: diagnostics should be created using translatable messages
     |
     |
2428 |                 err.span_note(span, &msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
2429 |                 err.note(
     |                     ^^^^


error: diagnostics should be created using translatable messages
     |
2429 |                 err.note(
     |                     ^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
2435 |                 err.span_note(span, &msg);


error: diagnostics should be created using translatable messages
     |
     |
2435 |                 err.span_note(span, &msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
2436 |                 err.note(&format!(


error: diagnostics should be created using translatable messages
     |
     |
2436 |                 err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
2626 |         err.span_help(
     |             ^^^^^^^^^


error: diagnostics should be created using translatable messages
     |
2626 |         err.span_help(
     |             ^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
580  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:2713:9
     |
     |
2713 |         struct_span_err!(tcx.sess, span, E0072, "recursive type `{}` has infinite size", path);


error: diagnostics should be created using translatable messages
     |
579  | macro_rules! struct_span_err {
     | ---------------------------- in this expansion of `struct_span_err!`
     | ---------------------------- in this expansion of `struct_span_err!`
580  |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |         $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:2713:9
     |
     |
2713 |         struct_span_err!(tcx.sess, span, E0072, "recursive type `{}` has infinite size", path);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
167 |         let mut err = lint.build(&format!(
    |                            ^^^^^


error: diagnostics should be created using translatable messages
    |
167 |         let mut err = lint.build(&format!(
    |                            ^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/on_unimplemented.rs:288:52
    |
288 |   ...                   let reported = struct_span_err!(
---
298 | | ...                       }
299 | | ...                   )
    | |_______________________- in this macro invocation

error: diagnostics should be created using translatable messages
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/on_unimplemented.rs:288:52
    |
288 |   ...                   let reported = struct_span_err!(
---
298 | | ...                       }
299 | | ...                   )
    | |_______________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/on_unimplemented.rs:308:40
    |
308 |                           let reported = struct_span_err!(
---
312 | |                             "only named substitution parameters are allowed"
313 | |                         )
    | |_________________________- in this macro invocation

error: diagnostics should be created using translatable messages
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/on_unimplemented.rs:308:40
    |
308 |                           let reported = struct_span_err!(
---
312 | |                             "only named substitution parameters are allowed"
313 | |                         )
    | |_________________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs:825:46
825 | ...                   lint.build(&format!(
    |                            ^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs:825:46
825 | ...                   lint.build(&format!(
    |                            ^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
   |
66 |         err.note(&self.intercrate_ambiguity_hint());


error: diagnostics should be created using translatable messages
   |
   |
66 |         err.note(&self.intercrate_ambiguity_hint());


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/specialize/mod.rs:332:19
    |
332 |       let mut err = struct_span_err!(
332 |       let mut err = struct_span_err!(
    |  ___________________-
333 | |         tcx.sess,
334 | |         impl_span,
335 | |         E0751,
...   |
338 | |         overlap.self_desc.clone().map_or_else(String::new, |ty| format!(" for type `{}`", ty))
    | |_____- in this macro invocation


error: diagnostics should be created using translatable messages
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/specialize/mod.rs:332:19
    |
332 |       let mut err = struct_span_err!(
332 |       let mut err = struct_span_err!(
    |  ___________________-
333 | |         tcx.sess,
334 | |         impl_span,
335 | |         E0751,
...   |
338 | |         overlap.self_desc.clone().map_or_else(String::new, |ty| format!(" for type `{}`", ty))
    | |_____- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
393 |         let mut err = err.build(&msg);


error: diagnostics should be created using translatable messages
    |
    |
393 |         let mut err = err.build(&msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
579 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
580 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/specialize/mod.rs:430:27
    |
    |
430 |                 let err = struct_span_err!(tcx.sess, impl_span, E0119, "");


error: diagnostics should be created using translatable messages
    |
579 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
580 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/specialize/mod.rs:430:27
    |
    |
430 |                 let err = struct_span_err!(tcx.sess, impl_span, E0119, "");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
   |
47 |         diag.span_label(self.top().1, top_label);


error: diagnostics should be created using translatable messages
   |
   |
47 |         diag.span_label(self.top().1, top_label);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
   |
50 |                 diag.span_label(*sp, format!("referenced here ({})", use_desc));


error: diagnostics should be created using translatable messages
   |
   |
50 |                 diag.span_label(*sp, format!("referenced here ({})", use_desc));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
56 |             diag.span_label(
   |                  ^^^^^^^^^^


error: diagnostics should be created using translatable messages
   |
56 |             diag.span_label(
   |                  ^^^^^^^^^^

