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
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.25
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0369]: binary operation `==` cannot be applied to type `thin_vec::ThinVec<_>`
   |
35 | / macro_rules! assert_eq {
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
39 | |                 if !(*left_val == *right_val) {
   | |                      --------- ^^ ---------- &[_; 0]
   | |                      thin_vec::ThinVec<_>
...  |
61 | |     });
62 | | }
62 | | }
   | |_- in this expansion of `assert_eq!`
   | 
  ::: compiler/rustc_data_structures/src/thin_vec/tests.rs:5:5
   |
5  |       assert_eq!(std::iter::empty().collect::<ThinVec<_>>(), &[]);
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `thin_vec::ThinVec<_>`

error[E0369]: binary operation `==` cannot be applied to type `thin_vec::ThinVec<{integer}>`
   |
35 | / macro_rules! assert_eq {
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
39 | |                 if !(*left_val == *right_val) {
   | |                      --------- ^^ ---------- &[{integer}; 1]
   | |                      |
   | |                      thin_vec::ThinVec<{integer}>
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   | 
   | 
  ::: compiler/rustc_data_structures/src/thin_vec/tests.rs:6:5
   |
6  |       assert_eq!(std::iter::once(42).collect::<ThinVec<_>>(), &[42]);
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `thin_vec::ThinVec<{integer}>`

error[E0369]: binary operation `==` cannot be applied to type `thin_vec::ThinVec<{integer}>`
   |
35 | / macro_rules! assert_eq {
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
39 | |                 if !(*left_val == *right_val) {
   | |                      --------- ^^ ---------- &[{integer}; 2]
   | |                      |
   | |                      thin_vec::ThinVec<{integer}>
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   | 
   | 
  ::: compiler/rustc_data_structures/src/thin_vec/tests.rs:7:5
   |
7  |       assert_eq!(vec![1, 2].into_iter().collect::<ThinVec<_>>(), &[1, 2]);
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `thin_vec::ThinVec<{integer}>`

error[E0369]: binary operation `==` cannot be applied to type `thin_vec::ThinVec<{integer}>`
   |
35 | / macro_rules! assert_eq {
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
39 | |                 if !(*left_val == *right_val) {
   | |                      --------- ^^ ---------- &[{integer}; 3]
   | |                      |
   | |                      thin_vec::ThinVec<{integer}>
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   | 
   | 
  ::: compiler/rustc_data_structures/src/thin_vec/tests.rs:8:5
   |
8  |       assert_eq!(vec![1, 2, 3].into_iter().collect::<ThinVec<_>>(), &[1, 2, 3]);
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `thin_vec::ThinVec<{integer}>`
error[E0282]: type annotations needed
  --> compiler/rustc_data_structures/src/thin_vec/tests.rs:13:16
   |
   |
13 |     assert_eq!(ThinVec::new().into_iter().collect::<Vec<_>>(), &[]);
   |                |
   |                |
   |                this method call resolves to `<Self as IntoIterator>::IntoIter`
   |                cannot infer type for type parameter `T`
error[E0282]: type annotations needed
  --> compiler/rustc_data_structures/src/thin_vec/tests.rs:21:16
   |
   |
21 |     assert_eq!(ThinVec::new().iter().collect::<Vec<_>>(), &[]);
   |                |
   |                |
   |                this method call resolves to `IntoIterRef<'_, T>`
   |                cannot infer type for type parameter `T`
error[E0282]: type annotations needed
  --> compiler/rustc_data_structures/src/thin_vec/tests.rs:29:16
   |
   |
29 |     assert_eq!(ThinVec::new().iter_mut().collect::<Vec<_>>(), &[]);
   |                |
   |                |
   |                this method call resolves to `IntoIterRefMut<'_, T>`
   |                cannot infer type for type parameter `T`
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0282, E0369.
For more information about an error, try `rustc --explain E0282`.
For more information about an error, try `rustc --explain E0282`.
error: could not compile `rustc_data_structures`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_data_structures" "-p" "rustc_index" "-p" "rustc_graphviz" "-p" "rustc_macros" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_target" "-p" "rustc_mir" "-p" "rustc_apfloat" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "coverage_test_macros" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_parse" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_typeck" "-p" "rustc_serialize" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_ast_passes" "-p" "rustc_ast_pretty" "-p" "rustc_ast" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_save_analysis" "-p" "rustc_lint" "-p" "rustc_interface" "-p" "rustc_passes" "-p" "rustc_incremental" "-p" "rustc_symbol_mangling" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_builtin_macros" "-p" "rustc_traits" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ty_utils" "-p" "rustc_query_impl" "-p" "rustc_error_codes" "-p" "rustc_plugin_impl" "-p" "rustc_mir_build" "-p" "rustc_hir" "-p" "rustc_hir_pretty" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:06
