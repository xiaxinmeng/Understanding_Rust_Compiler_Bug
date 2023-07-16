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
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 96 elements, found one with 68 elements
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
    ::: compiler/rustc_ast/src/ast.rs:2596:1
    ::: compiler/rustc_ast/src/ast.rs:2596:1
     |
2596 |   rustc_data_structures::static_assert_size!(Item<ItemKind>, 96);
     |   --------------------------------------------------------------- in this macro invocation
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
3    | / macro_rules! static_assert_size {
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 96 elements, found one with 68 elements
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
    ::: compiler/rustc_ast/src/ast.rs:2597:1
    ::: compiler/rustc_ast/src/ast.rs:2597:1
     |
2597 |   rustc_data_structures::static_assert_size!(Item<AssocItemKind>, 96);
     |   -------------------------------------------------------------------- in this macro invocation
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
3    | / macro_rules! static_assert_size {
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 96 elements, found one with 68 elements
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
    ::: compiler/rustc_ast/src/ast.rs:2598:1
    ::: compiler/rustc_ast/src/ast.rs:2598:1
     |
2598 |   rustc_data_structures::static_assert_size!(Item<ForeignItemKind>, 96);
     |   ---------------------------------------------------------------------- in this macro invocation
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
3    | / macro_rules! static_assert_size {
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 96 elements, found one with 68 elements
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
    ::: compiler/rustc_ast/src/ast.rs:2596:1
    ::: compiler/rustc_ast/src/ast.rs:2596:1
     |
2596 |   rustc_data_structures::static_assert_size!(Item<ItemKind>, 96);
     |   --------------------------------------------------------------- in this macro invocation
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
3    | / macro_rules! static_assert_size {
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 96 elements, found one with 68 elements
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
    ::: compiler/rustc_ast/src/ast.rs:2597:1
    ::: compiler/rustc_ast/src/ast.rs:2597:1
     |
2597 |   rustc_data_structures::static_assert_size!(Item<AssocItemKind>, 96);
     |   -------------------------------------------------------------------- in this macro invocation
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
3    | / macro_rules! static_assert_size {
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 96 elements, found one with 68 elements
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     | 
    ::: compiler/rustc_ast/src/ast.rs:2598:1
    ::: compiler/rustc_ast/src/ast.rs:2598:1
     |
2598 |   rustc_data_structures::static_assert_size!(Item<ForeignItemKind>, 96);
     |   ---------------------------------------------------------------------- in this macro invocation
    Checking tracing-serde v0.1.2
    Checking gsgdt v0.1.2
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
---
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_mir" "-p" "rustc_graphviz" "-p" "rustc_index" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_attr" "-p" "rustc_infer" "-p" "coverage_test_macros" "-p" "rustc_macros" "-p" "rustc_apfloat" "-p" "rustc_lexer" "-p" "rustc_lint" "-p" "rustc_error_codes" "-p" "rustc_plugin_impl" "-p" "rustc_interface" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_arena" "-p" "rustc_ast_passes" "-p" "rustc_privacy" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_symbol_mangling" "-p" "rustc_expand" "-p" "rustc_builtin_macros" "-p" "rustc_typeck" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_mir_build" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ty_utils" "-p" "rustc_save_analysis" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_parse" "-p" "rustc_errors" "-p" "rustc_serialize" "-p" "rustc_hir_pretty" "-p" "rustc_feature" "-p" "rustc_hir" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_data_structures" "-p" "rustc_ast" "-p" "rustc_target" "-p" "rustc_ast_pretty" "-p" "rustc_metadata" "-p" "rustc_span" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:16
