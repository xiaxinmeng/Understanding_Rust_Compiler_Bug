plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error[E0277]: `rustc_span::Span` is not an iterator
   --> compiler/rustc_mir_build/src/thir/pattern/check_match.rs:764:56
    |
764 |             for span in unreachable_subpatterns.iter().flatten().copied() {
    |                                                        ^^^^^^^ `rustc_span::Span` is not an iterator
    |
    = help: the trait `Iterator` is not implemented for `rustc_span::Span`
    = note: required because of the requirements on the impl of `IntoIterator` for `rustc_span::Span`

error[E0599]: no method named `copied` found for struct `Flatten<impl Iterator+Captures<'_>>` in the current scope
   --> compiler/rustc_mir_build/src/thir/pattern/check_match.rs:764:66
    |
764 |             for span in unreachable_subpatterns.iter().flatten().copied() {
    |                                                                  ^^^^^^ method not found in `Flatten<impl Iterator+Captures<'_>>`
   ::: /checkout/library/core/src/iter/adapters/flatten.rs:126:1
    |
    |
126 | pub struct Flatten<I: Iterator<Item: IntoIterator>> {
    | --------------------------------------------------- doesn't satisfy `Flatten<impl Iterator+Captures<'_>>: Iterator`
   ::: /checkout/compiler/rustc_span/src/span_encoding.rs:59:1
    |
59  | pub struct Span {
    | ---------------
    | ---------------
    | |
    | doesn't satisfy `<rustc_span::Span as IntoIterator>::IntoIter = _`
    | doesn't satisfy `<rustc_span::Span as IntoIterator>::Item = _`
    | doesn't satisfy `rustc_span::Span: IntoIterator`
    |
    = note: the method `copied` exists but the following trait bounds were not satisfied:
            `<rustc_span::Span as IntoIterator>::IntoIter = _`
            which is required by `Flatten<impl Iterator+Captures<'_>>: Iterator`
            `<rustc_span::Span as IntoIterator>::Item = _`
            which is required by `Flatten<impl Iterator+Captures<'_>>: Iterator`
            `rustc_span::Span: IntoIterator`
            which is required by `Flatten<impl Iterator+Captures<'_>>: Iterator`
            `Flatten<impl Iterator+Captures<'_>>: Iterator`
            which is required by `&mut Flatten<impl Iterator+Captures<'_>>: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_mir_build`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_serialize" "-p" "rustc_macros" "-p" "rustc_mir" "-p" "rustc_attr" "-p" "rustc_apfloat" "-p" "rustc_infer" "-p" "rustc_index" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_graphviz" "-p" "coverage_test_macros" "-p" "rustc_ast_pretty" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_lint" "-p" "rustc_save_analysis" "-p" "rustc_interface" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_mir_build" "-p" "rustc_incremental" "-p" "rustc_ast_passes" "-p" "rustc_expand" "-p" "rustc_ty_utils" "-p" "rustc_builtin_macros" "-p" "rustc_typeck" "-p" "rustc_ast_lowering" "-p" "rustc_passes" "-p" "rustc_symbol_mangling" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_ast" "-p" "rustc_data_structures" "-p" "rustc_feature" "-p" "rustc_session" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_hir_pretty" "-p" "rustc_span" "-p" "rustc_hir" "-p" "rustc_error_codes" "-p" "rustc_target" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:48
