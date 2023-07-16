plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +da962ff8e9cd1d61c0fa355d05ed3df3c5cf4fdf:refs/remotes/pull/79790/merge
---
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0529]: expected an array or slice, found `Vec<TraitInfo>`
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1217:21
     |
1217 |                     [trait_info] if trait_info.def_id.is_local() => {
     |                     ^^^^^^^^^^^^ pattern cannot match with input type `Vec<TraitInfo>`

error[E0529]: expected an array or slice, found `Vec<TraitInfo>`
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1242:21
     |
1242 |                     [] => {}
     |                     ^^ pattern cannot match with input type `Vec<TraitInfo>`

error[E0529]: expected an array or slice, found `Vec<TraitInfo>`
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1243:21
     |
1243 |                     [trait_info] => {
     |                     ^^^^^^^^^^^^ pattern cannot match with input type `Vec<TraitInfo>`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0529`.
error: could not compile `rustc_typeck`
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_ast_pretty" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_save_analysis" "-p" "rustc_lexer" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_apfloat" "-p" "rustc_arena" "-p" "rustc_attr" "-p" "rustc_type_ir" "-p" "rustc_span" "-p" "rustc_plugin_impl" "-p" "rustc_ast" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_hir_pretty" "-p" "rustc_error_codes" "-p" "rustc_serialize" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "rustc_infer" "-p" "coverage_test_macros" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_lint" "-p" "rustc_feature" "-p" "rustc_hir" "-p" "rustc_interface" "-p" "rustc_mir_build" "-p" "rustc_symbol_mangling" "-p" "rustc_ty_utils" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_passes" "-p" "rustc_typeck" "-p" "rustc_incremental" "-p" "rustc_resolve" "-p" "rustc_errors" "-p" "rustc_target" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:57
== clock drift check ==
  local time: Tue Dec 15 19:22:04 UTC 2020
