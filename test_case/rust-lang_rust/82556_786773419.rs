plain
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error: this file contains an unclosed delimiter
  --> compiler/rustc_target/src/spec/armv6_unknown_linux_dynmusleabihf.rs:12:10
3  | pub fn target() -> Target {
   |                           - unclosed delimiter
...
12 |     base
12 |     base
   |          ^

error: this file contains an unclosed delimiter
  --> compiler/rustc_target/src/spec/armv7_unknown_linux_dynmusleabihf.rs:12:10
3  | pub fn target() -> Target {
   |                           - unclosed delimiter
...
12 |     base
12 |     base
   |          ^

error[E0609]: no field `need_rpath` on type `TargetOptions`
 --> compiler/rustc_target/src/spec/mips_unknown_linux_dynmusl.rs:9:18
  |
9 |     base.options.need_rpath = true;
  |
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `need_rpath` on type `TargetOptions`
 --> compiler/rustc_target/src/spec/mipsel_unknown_linux_dynmusl.rs:9:18
  |
9 |     base.options.need_rpath = true;
  |
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `need_rpath` on type `TargetOptions`
 --> compiler/rustc_target/src/spec/powerpc_unknown_linux_dynmusl.rs:9:18
  |
9 |     base.options.need_rpath = true;
  |
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `need_rpath` on type `TargetOptions`
 --> compiler/rustc_target/src/spec/powerpc64_unknown_linux_dynmusl.rs:9:18
  |
9 |     base.options.need_rpath = true;
  |
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `need_rpath` on type `TargetOptions`
 --> compiler/rustc_target/src/spec/powerpc64le_unknown_linux_dynmusl.rs:9:18
  |
9 |     base.options.need_rpath = true;
  |
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `need_rpath` on type `TargetOptions`
 --> compiler/rustc_target/src/spec/s390x_unknown_linux_dynmusl.rs:9:18
  |
9 |     base.options.need_rpath = true;
  |
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `need_rpath` on type `TargetOptions`
  --> compiler/rustc_target/src/spec/armv6_unknown_linux_dynmusleabihf.rs:10:18
   |
10 |     base.options.need_rpath = true;
   |
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `need_rpath` on type `TargetOptions`
  --> compiler/rustc_target/src/spec/armv7_unknown_linux_dynmusleabihf.rs:10:18
   |
10 |     base.options.need_rpath = true;
   |
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `need_rpath` on type `TargetOptions`
 --> compiler/rustc_target/src/spec/aarch64_unknown_linux_dynmusl.rs:9:18
  |
9 |     base.options.need_rpath = true;
  |
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `need_rpath` on type `TargetOptions`
 --> compiler/rustc_target/src/spec/x86_64_unknown_linux_dynmusl.rs:9:18
  |
9 |     base.options.need_rpath = true;
  |
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `need_rpath` on type `TargetOptions`
  --> compiler/rustc_target/src/spec/i586_unknown_linux_dynmusl.rs:11:18
   |
11 |     base.options.need_rpath = true;
   |
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `need_rpath` on type `TargetOptions`
 --> compiler/rustc_target/src/spec/mips64_unknown_linux_dynmuslabi64.rs:9:18
  |
9 |     base.options.need_rpath = true;
  |
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `need_rpath` on type `TargetOptions`
 --> compiler/rustc_target/src/spec/mips64el_unknown_linux_dynmuslabi64.rs:9:18
  |
9 |     base.options.need_rpath = true;
  |
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error: aborting due to 15 previous errors

For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_target`
error: could not compile `rustc_target`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_feature" "-p" "rustc_target" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_lint_defs" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_error_codes" "-p" "rustc_save_analysis" "-p" "rustc_serialize" "-p" "rustc_hir_pretty" "-p" "rustc_parse" "-p" "rustc_errors" "-p" "rustc_lint" "-p" "rustc_attr" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_apfloat" "-p" "rustc_query_system" "-p" "rustc_plugin_impl" "-p" "rustc_interface" "-p" "rustc_typeck" "-p" "rustc_builtin_macros" "-p" "rustc_expand" "-p" "rustc_ty_utils" "-p" "rustc_ast_lowering" "-p" "rustc_passes" "-p" "rustc_incremental" "-p" "rustc_traits" "-p" "rustc_ast_passes" "-p" "rustc_symbol_mangling" "-p" "rustc_query_impl" "-p" "rustc_resolve" "-p" "rustc_mir_build" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_ast_pretty" "-p" "rustc_metadata" "-p" "rustc_hir" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:53
