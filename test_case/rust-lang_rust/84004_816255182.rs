plain
   --> compiler/rustc_codegen_llvm/src/llvm_util.rs:232:47
    |
232 |       rustc_target_features.extend_from_slice(&[(
    |  _______________________________________________^
233 | |         "crt-static",
234 | |         "Enables C Run-time Libraries to be statically linked",
235 | |     )]);
    | |_____^ expected enum `Option`, found tuple
    |
    = note: expected enum `Option<(&str, &str)>`
              found tuple `(&'static str, &'static str)`
help: try using a variant of the expected enum
    |
232 |     rustc_target_features.extend_from_slice(&[Some((
233 |         "crt-static",
234 |         "Enables C Run-time Libraries to be statically linked",
235 |     ))]);


error[E0271]: type mismatch resolving `<std::slice::Iter<'_, Option<(&str, &str)>> as IntoIterator>::Item == &(&str, &str)`
   --> compiler/rustc_codegen_llvm/src/llvm_util.rs:238:10
    |
238 |         .chain(rustc_target_features.iter())
    |          ^^^^^ expected enum `Option`, found tuple
    |
    = note: expected reference `&Option<(&str, &str)>`
               found reference `&(&str, &str)`

error[E0599]: the method `map` exists for struct `std::iter::Chain<std::slice::Iter<'_, (&str, &str)>, std::slice::Iter<'_, Option<(&str, &str)>>>`, but its trait bounds were not satisfied
   --> compiler/rustc_codegen_llvm/src/llvm_util.rs:239:10
    |
239 |         .map(|(feature, _desc)| feature.len())
    |          ^^^ method cannot be called on `std::iter::Chain<std::slice::Iter<'_, (&str, &str)>, std::slice::Iter<'_, Option<(&str, &str)>>>` due to unsatisfied trait bounds
   ::: /checkout/library/core/src/iter/adapters/chain.rs:22:1
    |
    |
22  | pub struct Chain<A, B> {
    | ---------------------- doesn't satisfy `_: Iterator`
   ::: /checkout/library/core/src/slice/iter.rs:66:1
    |
    |
66  | pub struct Iter<'a, T: 'a> {
    | -------------------------- doesn't satisfy `<_ as Iterator>::Item = &(&str, &str)`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `<std::slice::Iter<'_, Option<(&str, &str)>> as Iterator>::Item = &(&str, &str)`
            which is required by `std::iter::Chain<std::slice::Iter<'_, (&str, &str)>, std::slice::Iter<'_, Option<(&str, &str)>>>: Iterator`
            `std::iter::Chain<std::slice::Iter<'_, (&str, &str)>, std::slice::Iter<'_, Option<(&str, &str)>>>: Iterator`
            which is required by `&mut std::iter::Chain<std::slice::Iter<'_, (&str, &str)>, std::slice::Iter<'_, Option<(&str, &str)>>>: Iterator`
error[E0308]: mismatched types
   --> compiler/rustc_codegen_llvm/src/llvm_util.rs:244:9
    |
    |
244 |     for (feature, desc) in &rustc_target_features {
    |         ^^^^^^^^^^^^^^^    ---------------------- this expression has type `&Option<(&str, &str)>`
    |         expected enum `Option`, found tuple
    |
    |
    = note: expected enum `Option<(&str, &str)>`
              found tuple `(_, _)`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0271, E0308, E0599.
For more information about an error, try `rustc --explain E0271`.
For more information about an error, try `rustc --explain E0271`.
error: could not compile `rustc_codegen_llvm`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_serialize" "-p" "rustc_target" "-p" "rustc_symbol_mangling" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_session" "-p" "rustc_feature" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_hir" "-p" "rustc_index" "-p" "rustc_apfloat" "-p" "rustc_macros" "-p" "rustc_data_structures" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_fs_util" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_driver" "-p" "rustc_mir_build" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_plugin_impl" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_lint" "-p" "rustc_interface" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_query_impl" "-p" "rustc_ty_utils" "-p" "rustc_builtin_macros" "-p" "rustc_resolve" "-p" "rustc_typeck" "-p" "rustc_hir_pretty" "-p" "rustc_error_codes" "-p" "rustc_save_analysis" "-p" "rustc_parse" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:16
