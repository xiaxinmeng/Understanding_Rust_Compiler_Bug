plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: unused doc comment
  --> compiler/rustc_middle/src/thir.rs:36:1
   |
36 | /// An index to an [`Arm`] stored in [`Thir::arms`]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ rustdoc does not generate documentation for macro invocations
   |
   = note: `-D unused-doc-comments` implied by `-D warnings`
   = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion
error: unused doc comment
  --> compiler/rustc_middle/src/thir.rs:44:1
   |
   |
44 | /// An index to an [`Expr`] stored in [`Thir::exprs`]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ rustdoc does not generate documentation for macro invocations
   |
   = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion
error: unused doc comment
  --> compiler/rustc_middle/src/thir.rs:52:1
   |
   |
52 | /// An index to a [`Stmt`] stored in [`Thir::stmts`]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ rustdoc does not generate documentation for macro invocations
   |
   = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion
error: aborting due to 3 previous errors

error: could not compile `rustc_middle`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_macros" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_apfloat" "-p" "rustc_fs_util" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_incremental" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_index" "-p" "rustc_target" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_symbol_mangling" "-p" "rustc_span" "-p" "rustc_serialize" "-p" "rustc_session" "-p" "rustc_driver" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_hir_pretty" "-p" "rustc_interface" "-p" "rustc_query_impl" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_ast_lowering" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_traits" "-p" "rustc_ty_utils" "-p" "rustc_parse" "-p" "rustc_typeck" "-p" "rustc_save_analysis" "-p" "rustc_mir_build" "-p" "rustc_plugin_impl" "-p" "rustc_error_codes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_lint" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:00
