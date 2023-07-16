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
    Checking rustc_metadata v0.0.0 (/checkout/compiler/rustc_metadata)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0433]: failed to resolve: could not find `unix` in `os`
    --> compiler/rustc_codegen_ssa/src/back/link.rs:1624:26
     |
1624 |         let _ = std::os::unix::fs::symlink(lld_path.join("rust-lld"), lld_path.join("ld"));
     |                          ^^^^ could not find `unix` in `os`

error[E0433]: failed to resolve: could not find `unix` in `os`
    --> compiler/rustc_codegen_ssa/src/back/link.rs:1624:26
     |
1624 |         let _ = std::os::unix::fs::symlink(lld_path.join("rust-lld"), lld_path.join("ld"));
     |                          ^^^^ could not find `unix` in `os`
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0433`.
---
error: aborting due to previous error

For more information about this error, try `rustc --explain E0433`.
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_symbol_mangling" "-p" "rustc_target" "-p" "rustc_session" "-p" "rustc_feature" "-p" "rustc_lint_defs" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_fs_util" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_apfloat" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_data_structures" "-p" "rustc_hir" "-p" "rustc_span" "-p" "rustc_serialize" "-p" "rustc_errors" "-p" "rustc_driver" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_mir_build" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_hir_pretty" "-p" "rustc_interface" "-p" "rustc_traits" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_builtin_macros" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_passes" "-p" "rustc_query_impl" "-p" "rustc_parse" "-p" "rustc_save_analysis" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_error_codes" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:07
