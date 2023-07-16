plain
    Checking rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
    Checking rustc_metadata v0.0.0 (/checkout/compiler/rustc_metadata)
    Checking rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0425]: cannot find value `impl_candidate` in this scope
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:677:68
    |
677 | ...                   (e.span, format!("{}::{}({})", impl_candidate, segment.ident, path_segment.ident))
    |                                                      ^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `impl_candidates`

error[E0425]: cannot find value `candidates_len` in this scope
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:682:52
    |
682 | ...                   pluralize!(candidates_len),
    |                                  ^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `candidate_len`
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0061]: this function takes 4 arguments but 3 arguments were supplied
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:679:37
    |
    |
679 |   ...                   err.span_suggestions(
    |                             ^^^^^^^^^^^^^^^^ expected 4 arguments
680 | / ...                       &format!(
681 | | ...                           "use the fully qualified path for the potential candidate{}",
682 | | ...                           pluralize!(candidates_len),
683 | | ...                       ),
    | |___________________________-
684 |   ...                       suggestions,
    |                             -----------
685 |   ...                       Applicability::MaybeIncorrect,

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0061, E0425.
Some errors have detailed explanations: E0061, E0425.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_infer`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_middle" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_apfloat" "-p" "rustc_type_ir" "-p" "rustc_index" "-p" "rustc_query_system" "-p" "rustc_macros" "-p" "rustc_arena" "-p" "rustc_feature" "-p" "rustc_parse" "-p" "rustc_hir_pretty" "-p" "rustc_plugin_impl" "-p" "rustc_mir" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "coverage_test_macros" "-p" "rustc_graphviz" "-p" "rustc_infer" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_ast_passes" "-p" "rustc_privacy" "-p" "rustc_typeck" "-p" "rustc_mir_build" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_symbol_mangling" "-p" "rustc_incremental" "-p" "rustc_expand" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_metadata" "-p" "rustc_ast" "-p" "rustc_ast_pretty" "-p" "rustc_error_codes" "-p" "rustc_data_structures" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_errors" "-p" "rustc_target" "-p" "rustc_span" "-p" "rustc_serialize" "-p" "rustc_save_analysis" "-p" "rustc_hir" "-p" "rustc_lint" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:48
