plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/76260/merge:refs/remotes/pull/76260/merge
---
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
error[E0609]: no field `has_thumb_interworking` on type `&spec::Target`
    --> compiler/rustc_target/src/spec/mod.rs:1640:21
     |
1640 |         target_val!(has_thumb_interworking);
     |
     |
     = note: available fields are: `llvm_target`, `target_endian`, `target_pointer_width`, `target_c_int_width`, `target_os` ... and 6 others

error[E0609]: no field `has_thumb_interworking` on type `&spec::Target`
    --> compiler/rustc_target/src/spec/mod.rs:1640:21
     |
1640 |         target_val!(has_thumb_interworking);
     |
     |
     = note: available fields are: `llvm_target`, `target_endian`, `target_pointer_width`, `target_c_int_width`, `target_os` ... and 6 others
error: aborting due to previous error

For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_target`.
error: could not compile `rustc_target`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: aborting due to previous error

For more information about this error, try `rustc --explain E0609`.
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_plugin_impl" "-p" "rustc_errors" "-p" "rustc_macros" "-p" "rustc_ast" "-p" "rustc_index" "-p" "rustc_lexer" "-p" "rustc_metadata" "-p" "rustc_attr" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_parse" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_serialize" "-p" "rustc_save_analysis" "-p" "rustc_mir" "-p" "rustc_graphviz" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_apfloat" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_lint" "-p" "rustc_data_structures" "-p" "rustc_hir_pretty" "-p" "rustc_ast_pretty" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_error_codes" "-p" "rustc_interface" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_mir_build" "-p" "rustc_typeck" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_incremental" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_ty" "-p" "rustc_symbol_mangling" "-p" "rustc_builtin_macros" "-p" "rustc_target" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:04
== clock drift check ==
  local time: Tue Sep 15 21:48:05 UTC 2020
  local time: Tue Sep 15 21:48:05 UTC 2020
  network time: Tue, 15 Sep 2020 21:48:05 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (3904) (python)
