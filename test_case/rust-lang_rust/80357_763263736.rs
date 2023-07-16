plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0624]: associated function `lower_scrutinee` is private
  --> compiler/rustc_mir_build/src/build/expr/looking_for_a_better_name.rs:40:55
   |
40 |         let expr_place = unpack!(block = self.builder.lower_scrutinee(block, expr.clone(), expr_span));
   |                                                       ^^^^^^^^^^^^^^^ private associated function

error[E0624]: associated function `new` is private
  --> compiler/rustc_mir_build/src/build/expr/looking_for_a_better_name.rs:41:46
   |
41 |         let mut guard_candidate = Candidate::new(expr_place, &pat, false);
   |                                              ^^^ private associated function

error[E0624]: associated function `new` is private
  --> compiler/rustc_mir_build/src/build/expr/looking_for_a_better_name.rs:43:50
   |
43 |         let mut otherwise_candidate = Candidate::new(expr_place, &wildcard, false);
   |                                                  ^^^ private associated function

error[E0624]: associated function `lower_match_tree` is private
  --> compiler/rustc_mir_build/src/build/expr/looking_for_a_better_name.rs:45:26
   |
45 |             self.builder.lower_match_tree(block, pat.span, false, &mut [&mut guard_candidate, &mut otherwise_candidate]);
   |                          ^^^^^^^^^^^^^^^^ private associated function

error[E0616]: field `pre_binding_block` of struct `Candidate` is private
  --> compiler/rustc_mir_build/src/build/expr/looking_for_a_better_name.rs:62:62
   |
62 |         let otherwise_post_guard_block = otherwise_candidate.pre_binding_block.unwrap();

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0616, E0624.
Some errors have detailed explanations: E0616, E0624.
For more information about an error, try `rustc --explain E0616`.
error: could not compile `rustc_mir_build`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_ast_pretty" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_type_ir" "-p" "rustc_apfloat" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_ty_utils" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_traits" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_fs_util" "-p" "rustc_trait_selection" "-p" "rustc_resolve" "-p" "rustc_symbol_mangling" "-p" "rustc_ast_lowering" "-p" "rustc_incremental" "-p" "rustc_privacy" "-p" "rustc_typeck" "-p" "rustc_mir_build" "-p" "rustc_span" "-p" "rustc_ast" "-p" "rustc_serialize" "-p" "rustc_target" "-p" "rustc_save_analysis" "-p" "rustc_hir" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_error_codes" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_hir_pretty" "-p" "rustc_lint" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_session" "-p" "rustc_feature" "-p" "rustc_data_structures" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:18
