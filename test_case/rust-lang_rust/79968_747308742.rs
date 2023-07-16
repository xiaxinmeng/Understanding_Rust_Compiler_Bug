plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +2a06ec12d96c59c6c95e8af223653aedb83f29c1:refs/remotes/pull/79968/merge
---
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0308]: mismatched types
  --> compiler/rustc_symbol_mangling/src/legacy.rs:63:19
   |
63 |                 &[ty]
   |                   ^^ expected struct `rustc_middle::ty::subst::GenericArg`, found enum `Option`
   |
   = note: expected struct `rustc_middle::ty::subst::GenericArg<'_>`
                found enum `Option<&TyS<'_>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_symbol_mangling`
error: could not compile `rustc_symbol_mangling`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_index" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_feature" "-p" "rustc_serialize" "-p" "rustc_target" "-p" "rustc_symbol_mangling" "-p" "rustc_hir" "-p" "rustc_errors" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_data_structures" "-p" "rustc_apfloat" "-p" "rustc_fs_util" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_macros" "-p" "rustc_driver" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_hir_pretty" "-p" "rustc_lint" "-p" "rustc_error_codes" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_privacy" "-p" "rustc_ast_lowering" "-p" "rustc_typeck" "-p" "rustc_builtin_macros" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_traits" "-p" "rustc_passes" "-p" "rustc_ty_utils" "-p" "rustc_mir_build" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:39
== clock drift check ==
  local time: Thu Dec 17 09:03:23 UTC 2020
