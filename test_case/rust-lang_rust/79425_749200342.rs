plain
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
error: unused variable: `string`
    --> compiler/rustc_span/src/symbol.rs:1494:21
     |
1494 |                 let string = interner.get_dynamic(self);
     |                     ^^^^^^ help: if this is intentional, prefix it with an underscore: `_string`
     |
     = note: `-D unused-variables` implied by `-D warnings`
error: unused variable: `s`
    --> compiler/rustc_span/src/symbol.rs:1519:13
     |
     |
1519 |         let s: &str = &*self.as_str();
     |             ^ help: if this is intentional, prefix it with an underscore: `_s`
error: unused variable: `s`
    --> compiler/rustc_span/src/symbol.rs:1526:13
     |
     |
1526 |         let s: &str = &*self.as_str();
     |             ^ help: if this is intentional, prefix it with an underscore: `_s`
error: unused variable: `string`
    --> compiler/rustc_span/src/symbol.rs:1533:13
     |
     |
1533 |         let string: &str = &*self.as_str();
     |             ^^^^^^ help: if this is intentional, prefix it with an underscore: `_string`
error: aborting due to 4 previous errors

error: could not compile `rustc_span`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_attr" "-p" "rustc_hir_pretty" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_lint_defs" "-p" "rustc_interface" "-p" "rustc_mir_build" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_apfloat" "-p" "rustc_symbol_mangling" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_traits" "-p" "rustc_resolve" "-p" "rustc_ty_utils" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_passes" "-p" "rustc_privacy" "-p" "rustc_incremental" "-p" "rustc_typeck" "-p" "rustc_feature" "-p" "rustc_data_structures" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_lint" "-p" "rustc_hir" "-p" "rustc_target" "-p" "rustc_ast_pretty" "-p" "rustc_error_codes" "-p" "rustc_serialize" "-p" "rustc_plugin_impl" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_save_analysis" "-p" "rustc_parse" "-p" "rustc_errors" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:58
