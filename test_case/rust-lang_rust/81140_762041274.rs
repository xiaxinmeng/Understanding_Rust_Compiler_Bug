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
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 72 elements, found one with 44 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    | 
   ::: compiler/rustc_middle/src/mir/terminator.rs:278:1
   ::: compiler/rustc_middle/src/mir/terminator.rs:278:1
    |
278 |   rustc_data_structures::static_assert_size!(Terminator<'static>, 72);
    |   -------------------------------------------------------------------- in this macro invocation
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 64 elements, found one with 36 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    | 
   ::: compiler/rustc_middle/src/mir/terminator.rs:279:1
   ::: compiler/rustc_middle/src/mir/terminator.rs:279:1
    |
279 |   rustc_data_structures::static_assert_size!(InlineAsmTerminator<'static>, 64);
    |   ----------------------------------------------------------------------------- in this macro invocation
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 88 elements, found one with 52 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    | 
   ::: compiler/rustc_middle/src/mir/terminator.rs:280:1
   ::: compiler/rustc_middle/src/mir/terminator.rs:280:1
    |
280 |   rustc_data_structures::static_assert_size!(CallTerminator<'static>, 88);
    |   ------------------------------------------------------------------------ in this macro invocation
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 96 elements, found one with 52 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    | 
   ::: compiler/rustc_middle/src/mir/terminator.rs:281:1
   ::: compiler/rustc_middle/src/mir/terminator.rs:281:1
    |
281 |   rustc_data_structures::static_assert_size!(AssertTerminator<'static>, 96);
    |   -------------------------------------------------------------------------- in this macro invocation
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 72 elements, found one with 44 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    | 
   ::: compiler/rustc_middle/src/mir/terminator.rs:278:1
   ::: compiler/rustc_middle/src/mir/terminator.rs:278:1
    |
278 |   rustc_data_structures::static_assert_size!(Terminator<'static>, 72);
    |   -------------------------------------------------------------------- in this macro invocation
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 80 elements, found one with 56 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    | 
   ::: compiler/rustc_middle/src/mir/terminator.rs:282:1
   ::: compiler/rustc_middle/src/mir/terminator.rs:282:1
    |
282 |   rustc_data_structures::static_assert_size!(SwitchIntTerminator<'static>, 80);
    |   ----------------------------------------------------------------------------- in this macro invocation
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 64 elements, found one with 36 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    | 
   ::: compiler/rustc_middle/src/mir/terminator.rs:279:1
   ::: compiler/rustc_middle/src/mir/terminator.rs:279:1
    |
279 |   rustc_data_structures::static_assert_size!(InlineAsmTerminator<'static>, 64);
    |   ----------------------------------------------------------------------------- in this macro invocation
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 88 elements, found one with 52 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    | 
   ::: compiler/rustc_middle/src/mir/terminator.rs:280:1
   ::: compiler/rustc_middle/src/mir/terminator.rs:280:1
    |
280 |   rustc_data_structures::static_assert_size!(CallTerminator<'static>, 88);
    |   ------------------------------------------------------------------------ in this macro invocation
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 96 elements, found one with 52 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    | 
   ::: compiler/rustc_middle/src/mir/terminator.rs:281:1
   ::: compiler/rustc_middle/src/mir/terminator.rs:281:1
    |
281 |   rustc_data_structures::static_assert_size!(AssertTerminator<'static>, 96);
    |   -------------------------------------------------------------------------- in this macro invocation
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 80 elements, found one with 56 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    | 
   ::: compiler/rustc_middle/src/mir/terminator.rs:282:1
   ::: compiler/rustc_middle/src/mir/terminator.rs:282:1
    |
282 |   rustc_data_structures::static_assert_size!(SwitchIntTerminator<'static>, 80);
    |   ----------------------------------------------------------------------------- in this macro invocation
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_middle`
error: could not compile `rustc_middle`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0308`.
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_macros" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_hir" "-p" "rustc_plugin_impl" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_hir_pretty" "-p" "rustc_serialize" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_target" "-p" "rustc_save_analysis" "-p" "rustc_data_structures" "-p" "rustc_mir" "-p" "rustc_apfloat" "-p" "coverage_test_macros" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_symbol_mangling" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_passes" "-p" "rustc_mir_build" "-p" "rustc_incremental" "-p" "rustc_traits" "-p" "rustc_ast_lowering" "-p" "rustc_builtin_macros" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_typeck" "-p" "rustc_ast" "-p" "rustc_parse" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_error_codes" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:21
