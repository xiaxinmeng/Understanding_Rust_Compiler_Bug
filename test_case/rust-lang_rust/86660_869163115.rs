plain
    |
18  | macro_rules! declare_features {
    | ----------------------------- when calling this macro
...
671 |     (active, type_changing_struct_update, "1.55.0", Some(86618), None)
    |                                                                       ^ missing tokens in macro arguments
error[E0432]: unresolved import `crate::Features`
 --> compiler/rustc_feature/src/builtin_attrs.rs:6:13
  |
  |
6 | use crate::{Features, Stability};
  |             |
  |             no `Features` in the root
  |             help: a similar name exists in the module: `Feature`


error[E0432]: unresolved imports `active::Features`, `active::ACTIVE_FEATURES`
   --> compiler/rustc_feature/src/lib.rs:149:18
    |
149 | pub use active::{Features, ACTIVE_FEATURES, INCOMPATIBLE_FEATURES, INCOMPLETE_FEATURES};
    |                  ^^^^^^^^  ^^^^^^^^^^^^^^^ no `ACTIVE_FEATURES` in `active`
    |                  |
    |                  no `Features` in `active`
    |                  help: a similar name exists in the module: `Feature`
error[E0412]: cannot find type `Features` in this scope
  --> compiler/rustc_feature/src/active.rs:76:38
   |
   |
76 |     pub fn set(&self, features: &mut Features, span: Span) {
   |                                      ^^^^^^^^ help: a struct with a similar name exists: `Feature`
  ::: compiler/rustc_feature/src/lib.rs:48:1
   |
48 | pub struct Feature {
   | ------------------ similarly named struct `Feature` defined here
   | ------------------ similarly named struct `Feature` defined here

error: unused import: `to_nonzero`
 --> compiler/rustc_feature/src/active.rs:3:13
  |
3 | use super::{to_nonzero, Feature, State};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `rustc_span::edition::Edition`
 --> compiler/rustc_feature/src/active.rs:5:5
  |
5 | use rustc_span::edition::Edition;
5 | use rustc_span::edition::Edition;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused macro definition
  --> compiler/rustc_feature/src/active.rs:9:1
   |
9  | / macro_rules! set {
10 | |     ($field: ident) => {{
11 | |         fn f(features: &mut Features, _: Span) {
12 | |             features.$field = true;
15 | |     }};
16 | | }
   | |_^
   |
   |
   = note: `-D unused-macros` implied by `-D warnings`
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0412, E0432.
For more information about an error, try `rustc --explain E0412`.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `rustc_feature`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_macros" "-p" "rustc_middle" "-p" "rustc_feature" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_hir" "-p" "rustc_symbol_mangling" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_errors" "-p" "rustc_serialize" "-p" "rustc_index" "-p" "rustc_fs_util" "-p" "rustc_apfloat" "-p" "rustc_incremental" "-p" "rustc_span" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_target" "-p" "rustc_driver" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_error_codes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_mir_build" "-p" "rustc_hir_pretty" "-p" "rustc_parse" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_interface" "-p" "rustc_builtin_macros" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_query_impl" "-p" "rustc_privacy" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:47
