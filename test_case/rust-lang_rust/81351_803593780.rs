plain
    Checking rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 1 field
    |
    |
736 |             ty::ConstKind::Unevaluated(def, substs, promoted)
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 field, found 3
   ::: /checkout/compiler/rustc_middle/src/ty/consts/kind.rs:42:5
    |
    |
42  |     Unevaluated(Unevaluated<'tcx>),
    |     ------------------------------ tuple variant defined here
error[E0061]: this function takes 1 argument but 3 arguments were supplied
   --> compiler/rustc_infer/src/infer/combine.rs:743:26
    |
    |
743 |                     val: ty::ConstKind::Unevaluated(def, substs, promoted),
    |                          |
    |                          expected 1 argument


error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 1 field
    |
    |
964 |             ty::ConstKind::Unevaluated(def, substs, promoted)
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 field, found 3
   ::: /checkout/compiler/rustc_middle/src/ty/consts/kind.rs:42:5
    |
    |
42  |     Unevaluated(Unevaluated<'tcx>),
    |     ------------------------------ tuple variant defined here
error[E0061]: this function takes 1 argument but 3 arguments were supplied
   --> compiler/rustc_infer/src/infer/combine.rs:971:26
    |
    |
971 |                     val: ty::ConstKind::Unevaluated(def, substs, promoted),
    |                          |
    |                          expected 1 argument

error: aborting due to 4 previous errors
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_macros" "-p" "rustc_target" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_fs_util" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_span" "-p" "rustc_ast" "-p" "rustc_symbol_mangling" "-p" "rustc_index" "-p" "rustc_incremental" "-p" "rustc_apfloat" "-p" "rustc_hir" "-p" "rustc_serialize" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_traits" "-p" "rustc_ast_lowering" "-p" "rustc_privacy" "-p" "rustc_expand" "-p" "rustc_ty_utils" "-p" "rustc_ast_passes" "-p" "rustc_passes" "-p" "rustc_query_impl" "-p" "rustc_mir_build" "-p" "rustc_hir_pretty" "-p" "rustc_parse" "-p" "rustc_lint" "-p" "rustc_typeck" "-p" "rustc_plugin_impl" "-p" "rustc_save_analysis" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_error_codes" "-p" "rustc_metadata" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:33
