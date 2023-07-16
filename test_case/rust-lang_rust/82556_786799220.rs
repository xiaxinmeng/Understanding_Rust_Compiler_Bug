plain
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/mips_unknown_linux_dynmusl.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/mips_unknown_linux_dynmusl.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/mipsel_unknown_linux_dynmusl.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/mipsel_unknown_linux_dynmusl.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
    Checking tracing-serde v0.1.2
    Checking rls-span v0.5.3
    Checking gsgdt v0.1.2
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/powerpc_unknown_linux_dynmusl.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/powerpc_unknown_linux_dynmusl.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/powerpc64_unknown_linux_dynmusl.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/powerpc64_unknown_linux_dynmusl.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/powerpc64le_unknown_linux_dynmusl.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
    Checking rls-data v0.19.1
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/powerpc64le_unknown_linux_dynmusl.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/s390x_unknown_linux_dynmusl.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/s390x_unknown_linux_dynmusl.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/arm_unknown_linux_dynmusleabihf.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/arm_unknown_linux_dynmusleabihf.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/armv7_unknown_linux_dynmusleabihf.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/armv7_unknown_linux_dynmusleabihf.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/aarch64_unknown_linux_dynmusl.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
    Checking tracing-subscriber v0.2.13
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/aarch64_unknown_linux_dynmusl.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/x86_64_unknown_linux_dynmusl.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/x86_64_unknown_linux_dynmusl.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/i686_unknown_linux_dynmusl.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/i686_unknown_linux_dynmusl.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/i586_unknown_linux_dynmusl.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/i586_unknown_linux_dynmusl.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/mips64_unknown_linux_dynmuslabi64.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/mips64_unknown_linux_dynmuslabi64.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/mips64el_unknown_linux_dynmuslabi64.rs:8:46
  |
8 |     base.options.pre_link_objects_fallback = None;
  |                                              ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/mips64el_unknown_linux_dynmuslabi64.rs:9:47
  |
9 |     base.options.post_link_objects_fallback = None;
  |                                               ^^^^ expected struct `BTreeMap`, found enum `Option`
  |
  = note: expected struct `BTreeMap<LinkOutputKind, Vec<std::string::String>>`
               found enum `Option<_>`
error: aborting due to 28 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_target`
error: could not compile `rustc_target`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_target" "-p" "rustc_middle" "-p" "rustc_feature" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_apfloat" "-p" "rustc_serialize" "-p" "rustc_incremental" "-p" "rustc_symbol_mangling" "-p" "rustc_fs_util" "-p" "rustc_session" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_hir" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_expand" "-p" "rustc_typeck" "-p" "rustc_infer" "-p" "rustc_privacy" "-p" "rustc_query_impl" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_ast_passes" "-p" "rustc_traits" "-p" "rustc_mir_build" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_builtin_macros" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_metadata" "-p" "rustc_lint" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_save_analysis" "-p" "rustc_error_codes" "-p" "rustc_plugin_impl" "-p" "rustc_hir_pretty" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:03
