plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +ea3c925217c562b5bbfc5911172edc93a1bfbaed:refs/remotes/pull/80085/merge
---
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking chalk-engine v0.36.0
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error: the feature `iter_is_partitioned` has been stable since 1.49.0 and no longer requires an attribute to enable
  |
8 | #![feature(iter_is_partitioned)]
  |            ^^^^^^^^^^^^^^^^^^^
  |
  |
  = note: `-D stable-features` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_ast_passes`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_target" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_data_structures" "-p" "rustc_macros" "-p" "rustc_symbol_mangling" "-p" "rustc_fs_util" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_arena" "-p" "rustc_feature" "-p" "rustc_query_system" "-p" "rustc_index" "-p" "rustc_apfloat" "-p" "rustc_hir" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_span" "-p" "rustc_ast" "-p" "rustc_serialize" "-p" "rustc_session" "-p" "rustc_driver" "-p" "rustc_save_analysis" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_plugin_impl" "-p" "rustc_error_codes" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_typeck" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_builtin_macros" "-p" "rustc_resolve" "-p" "rustc_mir_build" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_hir_pretty" "-p" "rustc_parse" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:42
== clock drift check ==
  local time: Wed Dec 16 18:29:10 UTC 2020
