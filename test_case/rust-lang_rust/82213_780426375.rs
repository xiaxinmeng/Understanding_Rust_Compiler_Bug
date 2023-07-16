plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0308]: `match` arms have incompatible types
   |
   |
63 |           let arch_clobbers = match &self.sess().target.arch[..] {
   |  _____________________________-
64 | |             "x86" | "x86_64" => &["~{dirflag}", "~{fpsr}", "~{flags}"],
   | |                                 -------------------------------------- this is found to be of type `&[&str; 3]`
65 | |             "mips" | "mips64" => &["~{$1}"],
   | |                                  ^^^^^^^^^^ expected an array with a fixed size of 3 elements, found one with 1 element
66 | |             _ => &[],
67 | |         };
   | |_________- `match` arms have incompatible types
   |
   = note:   expected type `&[&str; 3]`
           found reference `&[&str; 1]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_llvm`
error: could not compile `rustc_codegen_llvm`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_session" "-p" "rustc_macros" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_ast_pretty" "-p" "rustc_plugin_impl" "-p" "rustc_serialize" "-p" "rustc_hir_pretty" "-p" "rustc_errors" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "rustc_feature" "-p" "rustc_error_codes" "-p" "rustc_target" "-p" "rustc_index" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_hir" "-p" "rustc_ast" "-p" "rustc_lint" "-p" "rustc_attr" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_parse_format" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_apfloat" "-p" "rustc_type_ir" "-p" "rustc_interface" "-p" "rustc_ast_passes" "-p" "rustc_mir_build" "-p" "rustc_builtin_macros" "-p" "rustc_expand" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_passes" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_typeck" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_symbol_mangling" "-p" "rustc_incremental" "-p" "rustc_ty_utils" "-p" "rustc_data_structures" "-p" "rustc_save_analysis" "-p" "rustc_metadata" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:43
