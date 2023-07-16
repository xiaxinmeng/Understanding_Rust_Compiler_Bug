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
error: unreachable statement
    --> compiler/rustc_codegen_ssa/src/back/link.rs:1756:9
     |
1756 | /         let tools_path =
1757 | |             sess.host_filesearch(PathKind::All).get_tools_search_paths(crt_objects_fallback);
     | |_____________________________________________________________________________________________^ unreachable statement
    ::: /checkout/library/core/src/panic.rs:24:9
     |
24   |           $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+))
     |           ------------------------------------------------------------------ any code following this expression is unreachable
     |           ------------------------------------------------------------------ any code following this expression is unreachable
     |
     = note: `-D unreachable-code` implied by `-D warnings`
error: unreachable statement
    --> compiler/rustc_codegen_ssa/src/back/link.rs:1756:9
     |
1756 | /         let tools_path =
1756 | /         let tools_path =
1757 | |             sess.host_filesearch(PathKind::All).get_tools_search_paths(crt_objects_fallback);
     | |_____________________________________________________________________________________________^ unreachable statement
    ::: /checkout/library/core/src/panic.rs:24:9
     |
24   |           $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+))
     |           ------------------------------------------------------------------ any code following this expression is unreachable
     |           ------------------------------------------------------------------ any code following this expression is unreachable
     |
     = note: `-D unreachable-code` implied by `-D warnings`
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: aborting due to previous error

error: could not compile `rustc_codegen_ssa`
error: could not compile `rustc_codegen_ssa`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: aborting due to previous error

error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_save_analysis" "-p" "rustc_lexer" "-p" "rustc_hir" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_ast_pretty" "-p" "rustc_hir_pretty" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_attr" "-p" "rustc_arena" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_errors" "-p" "rustc_ast" "-p" "rustc_data_structures" "-p" "rustc_plugin_impl" "-p" "rustc_interface" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_incremental" "-p" "rustc_passes" "-p" "rustc_symbol_mangling" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_ty_utils" "-p" "rustc_builtin_macros" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_lint" "-p" "rustc_target" "-p" "rustc_error_codes" "-p" "rustc_session" "-p" "rustc_feature" "-p" "rustc_typeck" "-p" "rustc_span" "-p" "rustc_parse" "-p" "rustc_serialize" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:16
