plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
210 |             mbcx.infcx.tcx.sess.diagnostic().struct_help(&match suggested.last().unwrap() {
    |
note: the lint level is defined here
   --> compiler/rustc_borrowck/src/diagnostics/mod.rs:2:9
    |
    |
2   | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
    |
    |
210 |             mbcx.infcx.tcx.sess.diagnostic().struct_help(&match suggested.last().unwrap() {
    |
note: the lint level is defined here
   --> compiler/rustc_borrowck/src/diagnostics/mod.rs:1:9
    |
    |
1   | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
228 |                 .struct_help("the following changes may resolve your lifetime errors");


error: diagnostics should be created using translatable messages
    |
    |
228 |                 .struct_help("the following changes may resolve your lifetime errors");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
109 |                 diag.span_label(*span, format!("lifetime `{self}` defined here"));


error: diagnostics should be created using translatable messages
    |
    |
109 |                 diag.span_label(*span, format!("lifetime `{self}` defined here"));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
112 |                 diag.span_label(*span, format!("lifetime `{self}` represents this closure's body"));


error: diagnostics should be created using translatable messages
    |
    |
112 |                 diag.span_label(*span, format!("lifetime `{self}` represents this closure's body"));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
113 |                 diag.note(*note);


error: diagnostics should be created using translatable messages
    |
    |
113 |                 diag.note(*note);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
119 |                 diag.span_label(*span, format!("has type `{type_name}`"));


error: diagnostics should be created using translatable messages
    |
    |
119 |                 diag.span_label(*span, format!("has type `{type_name}`"));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
124 |                 diag.span_label(
    |                      ^^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
124 |                 diag.span_label(
    |                      ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
136 |                 diag.span_label(*span, format!("let's call this `{self}`"));


error: diagnostics should be created using translatable messages
    |
    |
136 |                 diag.span_label(*span, format!("let's call this `{self}`"));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
142 |                 diag.span_label(
    |                      ^^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
142 |                 diag.span_label(
    |                      ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
151 |                 diag.span_label(
    |                      ^^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
151 |                 diag.span_label(
    |                      ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
159 |                 diag.span_label(
    |                      ^^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
159 |                 diag.span_label(
    |                      ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
168 |                 diag.span_label(*span, format!("return type{mir_description} is {type_name}"));


error: diagnostics should be created using translatable messages
    |
    |
168 |                 diag.span_label(*span, format!("return type{mir_description} is {type_name}"));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
171 |                 diag.span_label(*span, format!("yield type is {type_name}"));


error: diagnostics should be created using translatable messages
    |
    |
171 |                 diag.span_label(*span, format!("yield type is {type_name}"));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
174 |                 diag.span_label(
    |                      ^^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
174 |                 diag.span_label(
    |                      ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
608 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
609 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_borrowck/src/diagnostics/conflict_errors.rs:405:13
    |
    |
405 |             struct_span_err!(self, span, E0381, "{used} binding {desc}{isnt_initialized}");


error: diagnostics should be created using translatable messages
    |
608 | macro_rules! struct_span_err {
    | ---------------------------- in this expansion of `struct_span_err!`
    | ---------------------------- in this expansion of `struct_span_err!`
609 |     ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
610 |         $session.struct_span_err_with_code(
    |
   ::: compiler/rustc_borrowck/src/diagnostics/conflict_errors.rs:405:13
    |
    |
405 |             struct_span_err!(self, span, E0381, "{used} binding {desc}{isnt_initialized}");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
985 |         err.span_help(
    |             ^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
985 |         err.span_help(
    |             ^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
992 |         err.span_help(
    |             ^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
992 |         err.span_help(
    |             ^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1010 |             err.help(
     |                 ^^^^


error: diagnostics should be created using translatable messages
     |
1010 |             err.help(
     |                 ^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
2053 |                     err.note(&format!(


error: diagnostics should be created using translatable messages
     |
     |
2053 |                     err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
2057 |                     err.span_note(tcx.def_span(instance.def_id()), "deref defined here");


error: diagnostics should be created using translatable messages
     |
     |
2057 |                     err.span_note(tcx.def_span(instance.def_id()), "deref defined here");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
2559 |                 diag.span_label(
     |                      ^^^^^^^^^^


error: diagnostics should be created using translatable messages
     |
2559 |                 diag.span_label(
     |                      ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
2573 |                 diag.span_label(argument_span, format!("has type `{}`", argument_ty_name));


error: diagnostics should be created using translatable messages
     |
     |
2573 |                 diag.span_label(argument_span, format!("has type `{}`", argument_ty_name));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
2577 |                 diag.span_label(
     |                      ^^^^^^^^^^


error: diagnostics should be created using translatable messages
     |
2577 |                 diag.span_label(
     |                      ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
2586 |                 diag.note(
     |                      ^^^^


error: diagnostics should be created using translatable messages
     |
2586 |                 diag.note(
     |                      ^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
2589 |                 diag.note(
     |                      ^^^^


error: diagnostics should be created using translatable messages
     |
2589 |                 diag.note(
     |                      ^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
2600 |                     diag.span_label(*argument_span, format!("has lifetime `{}`", region_name));


error: diagnostics should be created using translatable messages
     |
     |
2600 |                     diag.span_label(*argument_span, format!("has lifetime `{}`", region_name));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
2603 |                 diag.span_label(*return_span, format!("also has lifetime `{}`", region_name,));


error: diagnostics should be created using translatable messages
     |
     |
2603 |                 diag.span_label(*return_span, format!("also has lifetime `{}`", region_name,));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
2605 |                 diag.help(&format!(


error: diagnostics should be created using translatable messages
     |
     |
2605 |                 diag.help(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
308 |                     err.span_label(
    |                         ^^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
308 |                     err.span_label(
    |                         ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
584 |             err.span_label(args_span, message);


error: diagnostics should be created using translatable messages
    |
    |
584 |             err.span_label(args_span, message);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
592 |             err.span_label(path_span, message);


error: diagnostics should be created using translatable messages
    |
    |
592 |             err.span_label(path_span, message);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
605 |                 err.span_label(capture_kind_span, message);


error: diagnostics should be created using translatable messages
    |
    |
605 |                 err.span_label(capture_kind_span, message);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
610 |                 err.span_label(capture_kind_span, capture_kind_label);


error: diagnostics should be created using translatable messages
    |
    |
610 |                 err.span_label(capture_kind_span, capture_kind_label);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
611 |                 err.span_label(path_span, path_label);


error: diagnostics should be created using translatable messages
    |
    |
611 |                 err.span_label(path_span, path_label);

error: could not compile `rustc_borrowck` due to 68 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_borrowck` due to 68 previous errors
