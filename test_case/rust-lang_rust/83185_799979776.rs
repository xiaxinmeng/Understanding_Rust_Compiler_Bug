plain
    Checking rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
    Checking rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0425]: cannot find function `assert_send_val` in module `rustc_data_structures::sync`
    |
    |
328 | /         macro_rules! rustc_erase_owner {
329 | |             ($v:expr) => {{
330 | |                 let v = $v;
331 | |                 ::rustc_data_structures::sync::assert_send_val(&v);
    | |                                                ^^^^^^^^^^^^^^^ help: a function with a similar name exists: `assert_send`
332 | |                 v.erase_send_sync_owner()
334 | |         }
334 | |         }
    | |_________- in this expansion of `rustc_erase_owner!`
...
339 |   pub fn assert_send<T: ?Sized + Send>() {}
    |   -------------------------------------- similarly named function `assert_send` defined here
   ::: compiler/rustc_metadata/src/locator.rs:772:26
    |
    |
772 |                   Ok(_) => rustc_erase_owner!(OwningRef::new(inflated).map_owner_box()),


error[E0425]: cannot find function `assert_send_val` in module `rustc_data_structures::sync`
    |
    |
328 | /         macro_rules! rustc_erase_owner {
329 | |             ($v:expr) => {{
330 | |                 let v = $v;
331 | |                 ::rustc_data_structures::sync::assert_send_val(&v);
    | |                                                ^^^^^^^^^^^^^^^ help: a function with a similar name exists: `assert_send`
332 | |                 v.erase_send_sync_owner()
334 | |         }
334 | |         }
    | |_________- in this expansion of `rustc_erase_owner!`
...
339 |   pub fn assert_send<T: ?Sized + Send>() {}
    |   -------------------------------------- similarly named function `assert_send` defined here
   ::: compiler/rustc_metadata/src/locator.rs:786:13
    |
    |
786 |               rustc_erase_owner!(OwningRef::new(StableDerefMmap(mmap)).map_owner_box())

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_metadata`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_span" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_arena" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "rustc_error_codes" "-p" "rustc_mir_build" "-p" "rustc_attr" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_apfloat" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_ast" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_plugin_impl" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_target" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_feature" "-p" "rustc_save_analysis" "-p" "rustc_ast_pretty" "-p" "rustc_lint" "-p" "rustc_hir" "-p" "rustc_errors" "-p" "rustc_serialize" "-p" "rustc_typeck" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_interface" "-p" "rustc_builtin_macros" "-p" "rustc_ty_utils" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_query_impl" "-p" "rustc_privacy" "-p" "rustc_symbol_mangling" "-p" "rustc_incremental" "-p" "rustc_resolve" "-p" "rustc_traits" "-p" "rustc_passes" "-p" "rustc_data_structures" "-p" "rustc_hir_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:19
