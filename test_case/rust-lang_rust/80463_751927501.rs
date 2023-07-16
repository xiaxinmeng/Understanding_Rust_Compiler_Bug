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
   Compiling chalk-derive v0.36.0
    Checking chalk-ir v0.36.0
    Checking tracing v0.1.19
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error[E0425]: cannot find function `write_signed_leb128` in this scope
   --> compiler/rustc_serialize/tests/leb128.rs:36:9
    |
36  |         write_signed_leb128(&mut stream, x);
    |         ^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `read_signed_leb128`
   ::: /checkout/compiler/rustc_serialize/src/leb128.rs:123:1
    |
    |
123 | pub fn read_signed_leb128(data: &[u8], start_position: usize) -> (i128, usize) {
    | ------------------------------------------------------------------------------ similarly named function `read_signed_leb128` defined here
error[E0308]: mismatched types
  --> compiler/rustc_serialize/tests/leb128.rs:10:32
   |
   |
3  | / macro_rules! impl_test_unsigned_leb128 {
4  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
5  | |         #[test]
6  | |         fn $test_name() {
...  |
10 | |                 $write_fn_name(&mut stream, (3u64 << x) as $int_ty);
   | |                                ^^^^^^^^^^^ expected array of 3 elements, found struct `Vec`
22 | |     };
23 | | }
23 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
24 | 
25 |   impl_test_unsigned_leb128!(test_u16_leb128, write_u16_leb128, read_u16_leb128, u16);
   |   ------------------------------------------------------------------------------------ in this macro invocation
   |
   = note: expected mutable reference `&mut [MaybeUninit<u8>; 3]`
              found mutable reference `&mut Vec<_>`
error[E0308]: mismatched types
  --> compiler/rustc_serialize/tests/leb128.rs:10:32
   |
   |
3  | / macro_rules! impl_test_unsigned_leb128 {
4  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
5  | |         #[test]
6  | |         fn $test_name() {
...  |
10 | |                 $write_fn_name(&mut stream, (3u64 << x) as $int_ty);
   | |                                ^^^^^^^^^^^ expected array of 5 elements, found struct `Vec`
22 | |     };
23 | | }
23 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
26 |   impl_test_unsigned_leb128!(test_u32_leb128, write_u32_leb128, read_u32_leb128, u32);
   |   ------------------------------------------------------------------------------------ in this macro invocation
   |
   = note: expected mutable reference `&mut [MaybeUninit<u8>; 5]`
              found mutable reference `&mut Vec<_>`
error[E0308]: mismatched types
  --> compiler/rustc_serialize/tests/leb128.rs:10:32
   |
   |
3  | / macro_rules! impl_test_unsigned_leb128 {
4  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
5  | |         #[test]
6  | |         fn $test_name() {
...  |
10 | |                 $write_fn_name(&mut stream, (3u64 << x) as $int_ty);
   | |                                ^^^^^^^^^^^ expected array of 10 elements, found struct `Vec`
22 | |     };
23 | | }
23 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
27 |   impl_test_unsigned_leb128!(test_u64_leb128, write_u64_leb128, read_u64_leb128, u64);
   |   ------------------------------------------------------------------------------------ in this macro invocation
   |
   = note: expected mutable reference `&mut [MaybeUninit<u8>; 10]`
              found mutable reference `&mut Vec<_>`
error[E0308]: mismatched types
  --> compiler/rustc_serialize/tests/leb128.rs:10:32
   |
   |
3  | / macro_rules! impl_test_unsigned_leb128 {
4  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
5  | |         #[test]
6  | |         fn $test_name() {
...  |
10 | |                 $write_fn_name(&mut stream, (3u64 << x) as $int_ty);
   | |                                ^^^^^^^^^^^ expected array of 19 elements, found struct `Vec`
22 | |     };
23 | | }
23 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
28 |   impl_test_unsigned_leb128!(test_u128_leb128, write_u128_leb128, read_u128_leb128, u128);
   |   ---------------------------------------------------------------------------------------- in this macro invocation
   |
   = note: expected mutable reference `&mut [MaybeUninit<u8>; 19]`
              found mutable reference `&mut Vec<_>`
error[E0308]: mismatched types
  --> compiler/rustc_serialize/tests/leb128.rs:10:32
   |
   |
3  | / macro_rules! impl_test_unsigned_leb128 {
4  | |     ($test_name:ident, $write_fn_name:ident, $read_fn_name:ident, $int_ty:ident) => {
5  | |         #[test]
6  | |         fn $test_name() {
...  |
10 | |                 $write_fn_name(&mut stream, (3u64 << x) as $int_ty);
   | |                                ^^^^^^^^^^^ expected array of 5 elements, found struct `Vec`
22 | |     };
23 | | }
23 | | }
   | |_- in this expansion of `impl_test_unsigned_leb128!`
...
29 |   impl_test_unsigned_leb128!(test_usize_leb128, write_usize_leb128, read_usize_leb128, usize);
   |   -------------------------------------------------------------------------------------------- in this macro invocation
   |
   = note: expected mutable reference `&mut [MaybeUninit<u8>; 5]`
              found mutable reference `&mut Vec<_>`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_serialize`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "rustc_lint" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_parse_format" "-p" "rustc_attr" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_serialize" "-p" "rustc_data_structures" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_apfloat" "-p" "rustc_type_ir" "-p" "rustc_target" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_hir" "-p" "rustc_error_codes" "-p" "rustc_save_analysis" "-p" "rustc_interface" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_resolve" "-p" "rustc_builtin_macros" "-p" "rustc_symbol_mangling" "-p" "rustc_mir_build" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_incremental" "-p" "rustc_privacy" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_ast" "-p" "rustc_hir_pretty" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_plugin_impl" "-p" "rustc_ast_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:12
