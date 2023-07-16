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
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:757:25
    |
757 |                     err.note(msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:757:25
    |
757 |                     err.note(msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:822:21
    |
822 |                 err.span_label(*fn_decl_span, "consider calling this closure");


error: diagnostics should be created using translatable messages
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:822:21
    |
822 |                 err.span_label(*fn_decl_span, "consider calling this closure");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:835:21
    |
835 |                 err.span_label(ident.span, "consider calling this function");


error: diagnostics should be created using translatable messages
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:835:21
    |
835 |                 err.span_label(ident.span, "consider calling this function");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:867:17
    |
867 |             err.help(&format!("{}: `{}`", msg, snippet));


error: diagnostics should be created using translatable messages
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:867:17
    |
867 |             err.help(&format!("{}: `{}`", msg, snippet));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:946:33
    |
946 | ...                   err.note(&msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:946:33
    |
946 | ...                   err.note(&msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:956:29
956 |                         err.span_label(
    |                             ^^^^^^^^^^


error: diagnostics should be created using translatable messages
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:956:29
956 |                         err.span_label(
    |                             ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1124:33
1124 | ...                   err.span_label(
     |                           ^^^^^^^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1124:33
1124 | ...                   err.span_label(
     |                           ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1223:29
     |
1223 |                         err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1223:29
     |
1223 |                         err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1259:17
1259 |             err.span_label(
     |                 ^^^^^^^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1259:17
1259 |             err.span_label(
     |                 ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1462:17
     |
1462 |             err.note(impl_trait_msg);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1462:17
     |
1462 |             err.note(impl_trait_msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1489:21
     |
1489 |                 err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1489:21
     |
1489 |                 err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1494:17
     |
1494 |             err.note(trait_obj_msg);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1494:17
     |
1494 |             err.note(trait_obj_msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1495:17
     |
1495 |             err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1495:17
     |
1495 |             err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1500:17
     |
1500 |             err.note(impl_trait_msg);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1500:17
     |
1500 |             err.note(impl_trait_msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1501:17
     |
1501 |             err.note("you can create a new `enum` with a variant for each returned type");


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1501:17
     |
1501 |             err.note("you can create a new `enum` with a variant for each returned type");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1530:25
     |
1530 |                     err.span_label(expr.span, &format!("this returned value is of type `{}`", ty));


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1530:25
     |
1530 |                     err.span_label(expr.span, &format!("this returned value is of type `{}`", ty));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1577:23
     |
1577 |           let mut err = struct_span_err!(
1577 |           let mut err = struct_span_err!(
     |  _______________________-
1578 | |             self.tcx.sess,
1579 | |             span,
1580 | |             E0631,
1581 | |             "type mismatch in {argument_kind} arguments",
     | |_________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
579  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `struct_span_err!`
     |   ---------------------------- in this expansion of `struct_span_err!`
580  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581  |           $session.struct_span_err_with_code(
     |
    ::: compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1577:23
     |
1577 |           let mut err = struct_span_err!(
1577 |           let mut err = struct_span_err!(
     |  _______________________-
1578 | |             self.tcx.sess,
1579 | |             span,
1580 | |             E0631,
1581 | |             "type mismatch in {argument_kind} arguments",
     | |_________- in this macro invocation


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1610:21
     |
1610 |                 err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1610:21
     |
1610 |                 err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2027:21
2027 |                 err.span_note(
     |                     ^^^^^^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2027:21
2027 |                 err.span_note(
     |                     ^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2035:25
     |
2035 |                     err.span_note(span, &msg);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2035:25
     |
2035 |                     err.span_note(span, &msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2052:29
2052 |                         err.span_note(
     |                             ^^^^^^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2052:29
2052 |                         err.span_note(
     |                             ^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2109:41
2109 | ...                   err.span_help(
     |                           ^^^^^^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2109:41
2109 | ...                   err.span_help(
     |                           ^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2151:21
     |
2151 |                 err.span_note(span, &span_note);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2151:21
     |
2151 |                 err.span_note(span, &span_note);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2206:21
     |
2206 |                 err.note("slice and array elements must have `Sized` type");


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2206:21
     |
2206 |                 err.note("slice and array elements must have `Sized` type");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2209:21
     |
2209 |                 err.note("only the last element of a tuple may have a dynamically sized type");


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2209:21
     |
2209 |                 err.note("only the last element of a tuple may have a dynamically sized type");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2212:21
     |
2212 |                 err.note(&format!("required so that the projection `{}` is well-formed", data,));


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2212:21
     |
2212 |                 err.note(&format!("required so that the projection `{}` is well-formed", data,));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2215:21
     |
2215 |                 err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2215:21
     |
2215 |                 err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2221:21
     |
2221 |                 err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2221:21
     |
2221 |                 err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2248:25
     |
2248 |                     err.span_note(multispan, &descr);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2248:25
     |
2248 |                     err.span_note(multispan, &descr);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2250:25
     |
2250 |                     err.span_note(tcx.def_span(item_def_id), &descr);


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2250:25
     |
2250 |                     err.span_note(tcx.def_span(item_def_id), &descr);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2254:21
     |
2254 |                 err.note(&format!(


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2254:21
     |
2254 |                 err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2261:21
     |
2261 |                 err.note(&format!("required by cast to type `{}`", self.ty_to_string(target)));


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2261:21
     |
2261 |                 err.note(&format!("required by cast to type `{}`", self.ty_to_string(target)));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2264:21
2264 |                 err.note(
     |                     ^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2264:21
2264 |                 err.note(
     |                     ^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2269:25
2269 |                     err.help(
     |                         ^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2269:25
2269 |                     err.help(
     |                         ^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2277:25
2277 |                     err.help(
     |                         ^^^^


error: diagnostics should be created using translatable messages
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2277:25
2277 |                     err.help(
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
579 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
580 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/on_unimplemented.rs:45:20
    |
    |
45  |     let mut diag = struct_span_err!(tcx.sess, span, E0232, "{}", message);


error: diagnostics should be created using translatable messages
    |
579 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
580 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/on_unimplemented.rs:45:20
    |
    |
45  |     let mut diag = struct_span_err!(tcx.sess, span, E0232, "{}", message);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/on_unimplemented.rs:321:52
    |
321 |   ...                   let reported = struct_span_err!(
---
331 | | ...                       }
332 | | ...                   )
    | |_______________________- in this macro invocation

error: diagnostics should be created using translatable messages
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/on_unimplemented.rs:321:52
    |
321 |   ...                   let reported = struct_span_err!(
---
331 | | ...                       }
332 | | ...                   )
    | |_______________________- in this macro invocation

error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/on_unimplemented.rs:341:40
    |
341 |                           let reported = struct_span_err!(
---
345 | |                             "only named substitution parameters are allowed"
346 | |                         )
    | |_________________________- in this macro invocation

error: diagnostics should be created using translatable messages
    |
579 |   macro_rules! struct_span_err {
    |   ---------------------------- in this expansion of `struct_span_err!`
    |   ---------------------------- in this expansion of `struct_span_err!`
580 |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
581 |           $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_trait_selection/src/traits/on_unimplemented.rs:341:40
    |
341 |                           let reported = struct_span_err!(
---
345 | |                             "only named substitution parameters are allowed"
346 | |                         )
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

