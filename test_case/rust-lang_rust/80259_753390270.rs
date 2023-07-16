plain
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0425]: cannot find function `check_generic_arg_count_for_call` in module `astconv::generics`
    --> compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:1232:36
     |
1232 |               } = astconv::generics::check_generic_arg_count_for_call(
     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `check_generic_arg_count`
    ::: compiler/rustc_typeck/src/astconv/generics.rs:338:1
     |
     |
338  | / pub(crate) fn check_generic_arg_count(
339  | |     tcx: TyCtxt<'_>,
340  | |     span: Span,
341  | |     def: &ty::Generics,
480  | |     }
481  | | }
481  | | }
     | |_- similarly named function `check_generic_arg_count` defined here

error[E0425]: cannot find function `check_generic_arg_count_for_call` in module `astconv::generics`
   --> compiler/rustc_typeck/src/check/method/confirm.rs:301:52
    |
301 |           let arg_count_correct = astconv::generics::check_generic_arg_count_for_call(
    |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `check_generic_arg_count`
   ::: compiler/rustc_typeck/src/astconv/generics.rs:338:1
    |
    |
338 | / pub(crate) fn check_generic_arg_count(
339 | |     tcx: TyCtxt<'_>,
340 | |     span: Span,
341 | |     def: &ty::Generics,
480 | |     }
481 | | }
481 | | }
    | |_- similarly named function `check_generic_arg_count` defined here
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_lint" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_attr" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_save_analysis" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_ast" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_apfloat" "-p" "rustc_type_ir" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_parse" "-p" "rustc_data_structures" "-p" "rustc_ast_pretty" "-p" "rustc_hir_pretty" "-p" "rustc_interface" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_typeck" "-p" "rustc_incremental" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_symbol_mangling" "-p" "rustc_mir_build" "-p" "rustc_ast_lowering" "-p" "rustc_ty_utils" "-p" "rustc_target" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_session" "-p" "rustc_hir" "-p" "rustc_serialize" "-p" "rustc_error_codes" "-p" "rustc_plugin_impl" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:51
