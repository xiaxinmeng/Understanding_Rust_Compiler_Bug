plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:44:35
   |
40 | / macro_rules! assert_eq {
41 | |     ($left:expr, $right:expr $(,)?) => ({
42 | |         match (&$left, &$right) {
43 | |             (left_val, right_val) => {
44 | |                 if !(*left_val == *right_val) {
   | |                                   ^^^^^^^^^^ expected struct `rustc_span::Symbol`, found `&str`
69 | |     });
70 | | }
70 | | }
   | |_- in this expansion of `assert_eq!`
  ::: compiler/rustc_ast/src/util/comments/tests.rs:9:9
   |
   |
9  |           assert_eq!(stripped, " Test \n*  Test\n   Test");

error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:44:35
   |
   |
40 | / macro_rules! assert_eq {
41 | |     ($left:expr, $right:expr $(,)?) => ({
42 | |         match (&$left, &$right) {
43 | |             (left_val, right_val) => {
44 | |                 if !(*left_val == *right_val) {
   | |                                   ^^^^^^^^^^ expected struct `rustc_span::Symbol`, found `&str`
69 | |     });
70 | | }
70 | | }
   | |_- in this expansion of `assert_eq!`
  ::: compiler/rustc_ast/src/util/comments/tests.rs:18:9
   |
   |
18 |           assert_eq!(stripped, " Test\n  Test");

error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:44:35
   |
   |
40 | / macro_rules! assert_eq {
41 | |     ($left:expr, $right:expr $(,)?) => ({
42 | |         match (&$left, &$right) {
43 | |             (left_val, right_val) => {
44 | |                 if !(*left_val == *right_val) {
   | |                                   ^^^^^^^^^^ expected struct `rustc_span::Symbol`, found `&str`
69 | |     });
70 | | }
70 | | }
   | |_- in this expansion of `assert_eq!`
  ::: compiler/rustc_ast/src/util/comments/tests.rs:27:9
   |
   |
27 |           assert_eq!(stripped, " let a: *i32;\n *a = 5;");

error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:44:35
   |
   |
40 | / macro_rules! assert_eq {
41 | |     ($left:expr, $right:expr $(,)?) => ({
42 | |         match (&$left, &$right) {
43 | |             (left_val, right_val) => {
44 | |                 if !(*left_val == *right_val) {
   | |                                   ^^^^^^^^^^ expected struct `rustc_span::Symbol`, found `&str`
69 | |     });
70 | | }
70 | | }
   | |_- in this expansion of `assert_eq!`
  ::: compiler/rustc_ast/src/util/comments/tests.rs:35:9
   |
   |
35 |           assert_eq!(stripped, " test");

error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:44:35
   |
   |
40 | / macro_rules! assert_eq {
41 | |     ($left:expr, $right:expr $(,)?) => ({
42 | |         match (&$left, &$right) {
43 | |             (left_val, right_val) => {
44 | |                 if !(*left_val == *right_val) {
   | |                                   ^^^^^^^^^^ expected struct `rustc_span::Symbol`, found `&str`
69 | |     });
70 | | }
70 | | }
   | |_- in this expansion of `assert_eq!`
  ::: compiler/rustc_ast/src/util/comments/tests.rs:37:9
   |
   |
37 |           assert_eq!(stripped, "! test");

error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:44:35
   |
   |
40 | / macro_rules! assert_eq {
41 | |     ($left:expr, $right:expr $(,)?) => ({
42 | |         match (&$left, &$right) {
43 | |             (left_val, right_val) => {
44 | |                 if !(*left_val == *right_val) {
   | |                                   ^^^^^^^^^^ expected struct `rustc_span::Symbol`, found `&str`
69 | |     });
70 | | }
70 | | }
   | |_- in this expansion of `assert_eq!`
  ::: compiler/rustc_ast/src/util/comments/tests.rs:39:9
   |
   |
39 |           assert_eq!(stripped, "test");

error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:44:35
   |
   |
40 | / macro_rules! assert_eq {
41 | |     ($left:expr, $right:expr $(,)?) => ({
42 | |         match (&$left, &$right) {
43 | |             (left_val, right_val) => {
44 | |                 if !(*left_val == *right_val) {
   | |                                   ^^^^^^^^^^ expected struct `rustc_span::Symbol`, found `&str`
69 | |     });
70 | | }
70 | | }
   | |_- in this expansion of `assert_eq!`
  ::: compiler/rustc_ast/src/util/comments/tests.rs:41:9
   |
   |
41 |           assert_eq!(stripped, "!test");

    Checking tracing-serde v0.1.2
    Checking gsgdt v0.1.2
    Checking rls-span v0.5.2
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_lint" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_parse_format" "-p" "rustc_ast_pretty" "-p" "rustc_middle" "-p" "rustc_apfloat" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_parse" "-p" "rustc_error_codes" "-p" "rustc_data_structures" "-p" "rustc_hir" "-p" "rustc_target" "-p" "rustc_save_analysis" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_span" "-p" "rustc_serialize" "-p" "rustc_interface" "-p" "rustc_traits" "-p" "rustc_incremental" "-p" "rustc_typeck" "-p" "rustc_privacy" "-p" "rustc_ty_utils" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_symbol_mangling" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_mir_build" "-p" "rustc_ast_lowering" "-p" "rustc_ast" "-p" "rustc_hir_pretty" "-p" "rustc_plugin_impl" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:19
