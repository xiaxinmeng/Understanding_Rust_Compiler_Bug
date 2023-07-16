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
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.25
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0277]: the trait bound `Vec<_, _>: From<thin_vec::ThinVec<std::string::String>>` is not satisfied
 --> compiler/rustc_data_structures/src/thin_vec/tests.rs:5:16
  |
5 |     assert_eq!(Vec::from(std::iter::empty().collect::<ThinVec<String>>()), Vec::<String>::new());
  |                ^^^^^^^^^ the trait `From<thin_vec::ThinVec<std::string::String>>` is not implemented for `Vec<_, _>`
  = help: the following implementations were found:
  = help: the following implementations were found:
            <Vec<T, A> as From<Box<[T], A>>>
            <Vec<T> as From<&[T]>>
            <Vec<T> as From<&mut [T]>>
            <Vec<T> as From<BinaryHeap<T>>>
  = note: required by `from`

error: aborting due to previous error


For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_data_structures`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_target" "-p" "rustc_symbol_mangling" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_apfloat" "-p" "rustc_data_structures" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_serialize" "-p" "rustc_fs_util" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_macros" "-p" "rustc_session" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_driver" "-p" "rustc_error_codes" "-p" "rustc_typeck" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_lint" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_parse" "-p" "rustc_mir_build" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_resolve" "-p" "rustc_builtin_macros" "-p" "rustc_query_impl" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_hir_pretty" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:52
