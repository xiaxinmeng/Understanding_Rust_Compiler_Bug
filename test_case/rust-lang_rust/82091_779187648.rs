plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: mismatched types
    --> compiler/rustc_middle/src/mir/visit.rs:154:34
     |
68   | / macro_rules! make_mir_visitor {
69   | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
70   | |         pub trait $visitor_trait_name<'tcx> {
71   | |             // Override these, and call `self.super_xxx` to revert back to the
...    |
154  | |                 self.super_place(place_ref, context, location);
     | |                                  ^^^^^^^^^ expected struct `mir::PlaceRef`, found `&mut mir::PlaceRef<'tcx>`
929  | |     }
930  | | }
930  | | }
     | |_- in this expansion of `make_mir_visitor!`
...
1084 |   make_mir_visitor!(Visitor,);

error[E0308]: mismatched types
    --> compiler/rustc_middle/src/mir/visit.rs:383:29
     |
     |
68   | / macro_rules! make_mir_visitor {
69   | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
70   | |         pub trait $visitor_trait_name<'tcx> {
71   | |             // Override these, and call `self.super_xxx` to revert back to the
383  | |                             &mut place.as_ref(),
383  | |                             &mut place.as_ref(),
     | |                             ^^^^^^^^^^^^^^^^^^^ expected struct `mir::PlaceRef`, found `&mir::Place<'_>`
929  | |     }
930  | | }
930  | | }
     | |_- in this expansion of `make_mir_visitor!`
...
1084 |   make_mir_visitor!(Visitor,);
     |
     |
     = note: expected mutable reference `&mut mir::PlaceRef<'tcx>`
                found mutable reference `&mut &mir::Place<'_>`
error[E0308]: mismatched types
    --> compiler/rustc_middle/src/mir/visit.rs:390:29
     |
     |
68   | / macro_rules! make_mir_visitor {
69   | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
70   | |         pub trait $visitor_trait_name<'tcx> {
71   | |             // Override these, and call `self.super_xxx` to revert back to the
390  | |                             &mut place.as_ref(),
390  | |                             &mut place.as_ref(),
     | |                             ^^^^^^^^^^^^^^^^^^^ expected struct `mir::PlaceRef`, found `&mir::Place<'_>`
929  | |     }
930  | | }
930  | | }
     | |_- in this expansion of `make_mir_visitor!`
...
1084 |   make_mir_visitor!(Visitor,);
     |
     |
     = note: expected mutable reference `&mut mir::PlaceRef<'tcx>`
                found mutable reference `&mut &mir::Place<'_>`
error[E0308]: mismatched types
    --> compiler/rustc_middle/src/mir/visit.rs:383:29
     |
     |
68   | / macro_rules! make_mir_visitor {
69   | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
70   | |         pub trait $visitor_trait_name<'tcx> {
71   | |             // Override these, and call `self.super_xxx` to revert back to the
383  | |                             &mut place.as_ref(),
383  | |                             &mut place.as_ref(),
     | |                             ^^^^^^^^^^^^^^^^^^^ expected struct `mir::PlaceRef`, found `&mir::Place<'_>`
929  | |     }
930  | | }
930  | | }
     | |_- in this expansion of `make_mir_visitor!`
...
1085 |   make_mir_visitor!(MutVisitor, mut);
     |
     |
     = note: expected mutable reference `&mut mir::PlaceRef<'tcx>`
                found mutable reference `&mut &mir::Place<'_>`
error[E0308]: mismatched types
    --> compiler/rustc_middle/src/mir/visit.rs:390:29
     |
     |
68   | / macro_rules! make_mir_visitor {
69   | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
70   | |         pub trait $visitor_trait_name<'tcx> {
71   | |             // Override these, and call `self.super_xxx` to revert back to the
390  | |                             &mut place.as_ref(),
390  | |                             &mut place.as_ref(),
     | |                             ^^^^^^^^^^^^^^^^^^^ expected struct `mir::PlaceRef`, found `&mir::Place<'_>`
929  | |     }
930  | | }
930  | | }
     | |_- in this expansion of `make_mir_visitor!`
...
1085 |   make_mir_visitor!(MutVisitor, mut);
     |
     |
     = note: expected mutable reference `&mut mir::PlaceRef<'tcx>`
                found mutable reference `&mut &mir::Place<'_>`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_middle`
error: could not compile `rustc_middle`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_serialize" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_middle" "-p" "rustc_feature" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_index" "-p" "rustc_hir" "-p" "rustc_apfloat" "-p" "rustc_macros" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_ast_pretty" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_target" "-p" "rustc_ast" "-p" "rustc_fs_util" "-p" "rustc_data_structures" "-p" "rustc_symbol_mangling" "-p" "rustc_errors" "-p" "rustc_driver" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_hir_pretty" "-p" "rustc_error_codes" "-p" "rustc_save_analysis" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_interface" "-p" "rustc_builtin_macros" "-p" "rustc_mir_build" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:22
