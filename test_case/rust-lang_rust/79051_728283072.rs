plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +3c3e63e0e76d594be89ab9e4dca5686ca064c803:refs/remotes/pull/79051/merge
---
    Checking rustc_ty v0.0.0 (/checkout/compiler/rustc_ty)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
error[E0308]: mismatched types
   --> compiler/rustc_mir_build/src/thir/pattern/check_match.rs:175:45
    |
175 |                 check_if_let_guard(&mut cx, &tpat, pat.hir_id);
    |                                             ^^^^^ expected struct `thir::pattern::Pat`, found tuple
    |
    = note: expected reference `&thir::pattern::Pat<'_>`
               found reference `&(&thir::pattern::Pat<'_>, &TyS<'_>)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_mir_build`
error: could not compile `rustc_mir_build`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_plugin_impl" "-p" "rustc_metadata" "-p" "rustc_macros" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lexer" "-p" "rustc_attr" "-p" "rustc_index" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_parse_format" "-p" "rustc_ast" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_hir_pretty" "-p" "rustc_hir" "-p" "rustc_save_analysis" "-p" "rustc_error_codes" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_ast_pretty" "-p" "rustc_serialize" "-p" "rustc_feature" "-p" "rustc_data_structures" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_interface" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_traits" "-p" "rustc_ast_lowering" "-p" "rustc_incremental" "-p" "rustc_resolve" "-p" "rustc_mir_build" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_typeck" "-p" "rustc_ty" "-p" "rustc_symbol_mangling" "-p" "rustc_privacy" "-p" "rustc_target" "-p" "rustc_parse" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:14
== clock drift check ==
  local time: Mon Nov 16 19:43:05 UTC 2020
