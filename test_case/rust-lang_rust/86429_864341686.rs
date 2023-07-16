plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0308]: mismatched types
  --> compiler/rustc_data_structures/src/sorted_map/tests.rs:17:31
   |
17 |     assert_eq!(set.get_by_key(&3).copied().collect::<Vec<_>>(), vec![0]);
   |                               |
   |                               |
   |                               expected integer, found `&{integer}`
   |                               help: consider removing the borrow: `3`
error[E0308]: mismatched types
  --> compiler/rustc_data_structures/src/sorted_map/tests.rs:18:28
   |
   |
18 |     assert!(set.get_by_key(&4).next().is_none());
   |                            |
   |                            |
   |                            expected integer, found `&{integer}`
   |                            help: consider removing the borrow: `4`
error[E0308]: mismatched types
  --> compiler/rustc_data_structures/src/sorted_map/tests.rs:21:50
   |
   |
21 |     let twos: Vec<_> = set.get_by_key_enumerated(&2).collect();
   |                                                  |
   |                                                  |
   |                                                  expected integer, found `&{integer}`
   |                                                  help: consider removing the borrow: `2`
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
error: aborting due to 3 previous errors


For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_data_structures`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_target" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_plugin_impl" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_lint_defs" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_attr" "-p" "rustc_hir" "-p" "rustc_ast_pretty" "-p" "rustc_errors" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_serialize" "-p" "rustc_error_codes" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_parse" "-p" "rustc_mir_build" "-p" "rustc_hir_pretty" "-p" "rustc_save_analysis" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_interface" "-p" "rustc_traits" "-p" "rustc_builtin_macros" "-p" "rustc_incremental" "-p" "rustc_query_impl" "-p" "rustc_passes" "-p" "rustc_privacy" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_symbol_mangling" "-p" "rustc_feature" "-p" "rustc_lint" "-p" "rustc_data_structures" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:45
