plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error[E0308]: `match` arms have incompatible types
    |
    |
431 |       let out = match ppm {
    |                 --------- `match` arms have incompatible types
...
434 | /             call_with_pp_support(&s, tcx.sess, Some(tcx), move |annotation| {
435 | |                 debug!("pretty printing source code {:?}", s);
436 | |                 let sess = annotation.sess();
437 | |                 let parse = &sess.parse_sess;
446 | |                 )
447 | |             })
    | |______________- this is found to be of type `std::string::String`
...
...
450 |           Hir(s) => call_with_pp_support_hir(&s, tcx, move |annotation, krate| {
    |  ___________________-
451 | |             debug!("pretty printing HIR {:?}", s);
452 | |             let sess = annotation.sess();
453 | |             let sm = sess.source_map();
454 | |             pprust_hir::print_crate(sm, krate, src_name, src, annotation.pp_ann())
455 | |         }),
    | |__________- this is found to be of type `std::string::String`
...
458 | /             call_with_pp_support_hir(&PpHirMode::Normal, tcx, move |_annotation, krate| {
459 | |                 debug!("pretty printing HIR tree");
460 | |                 format!("{:#?}", krate);
461 | |             });
    | |_______________^ expected struct `std::string::String`, found `()`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_driver`
error: could not compile `rustc_driver`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_symbol_mangling" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_feature" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_serialize" "-p" "rustc_target" "-p" "rustc_fs_util" "-p" "rustc_macros" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_hir" "-p" "rustc_index" "-p" "rustc_apfloat" "-p" "rustc_errors" "-p" "rustc_data_structures" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_ast" "-p" "rustc_driver" "-p" "rustc_error_codes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_hir_pretty" "-p" "rustc_interface" "-p" "rustc_typeck" "-p" "rustc_expand" "-p" "rustc_ast_lowering" "-p" "rustc_builtin_macros" "-p" "rustc_mir_build" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_ast_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_lint" "-p" "rustc_metadata" "-p" "rustc_save_analysis" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:07
