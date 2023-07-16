plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
error[E0277]: `&std::boxed::Box<[&TyS<'_>]>` is not an iterator
   --> compiler/rustc_mir_build/src/build/expr/into.rs:299:44
    |
299 |                     iter::zip(field_names, field_types)
    |                                            ^^^^^^^^^^^ `&std::boxed::Box<[&TyS<'_>]>` is not an iterator
   ::: /checkout/library/core/src/iter/adapters/zip.rs:62:8
    |
62  |     B: IntoIterator,
62  |     B: IntoIterator,
    |        ------------ required by this bound in `std::iter::zip`
    |
    = help: the trait `Iterator` is not implemented for `&std::boxed::Box<[&TyS<'_>]>`
    = note: required because of the requirements on the impl of `IntoIterator` for `&std::boxed::Box<[&TyS<'_>]>`

error[E0599]: the method `map` exists for struct `Zip<std::vec::IntoIter<rustc_middle::mir::Field>, &std::boxed::Box<[&TyS<'_>]>>`, but its trait bounds were not satisfied
   --> compiler/rustc_mir_build/src/build/expr/into.rs:300:26
    |
300 |                         .map(|(n, ty)| match fields_map.get(&n) {
    |                          ^^^ method cannot be called on `Zip<std::vec::IntoIter<rustc_middle::mir::Field>, &std::boxed::Box<[&TyS<'_>]>>` due to unsatisfied trait bounds
   ::: /checkout/library/core/src/iter/adapters/zip.rs:13:1
    |
    |
13  | pub struct Zip<A, B> {
    | -------------------- doesn't satisfy `_: Iterator`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `&std::boxed::Box<[&TyS<'_>]>: Iterator`
            which is required by `Zip<std::vec::IntoIter<rustc_middle::mir::Field>, &std::boxed::Box<[&TyS<'_>]>>: Iterator`
            `Zip<std::vec::IntoIter<rustc_middle::mir::Field>, &std::boxed::Box<[&TyS<'_>]>>: Iterator`
            which is required by `&mut Zip<std::vec::IntoIter<rustc_middle::mir::Field>, &std::boxed::Box<[&TyS<'_>]>>: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_mir_build`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_serialize" "-p" "rustc_macros" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_privacy" "-p" "rustc_attr" "-p" "rustc_ast_passes" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_index" "-p" "rustc_builtin_macros" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_traits" "-p" "rustc_ast_lowering" "-p" "rustc_arena" "-p" "rustc_resolve" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_passes" "-p" "rustc_symbol_mangling" "-p" "rustc_save_analysis" "-p" "rustc_hir" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_apfloat" "-p" "rustc_parse" "-p" "rustc_hir_pretty" "-p" "rustc_lint" "-p" "rustc_target" "-p" "rustc_plugin_impl" "-p" "rustc_mir_build" "-p" "rustc_errors" "-p" "rustc_span" "-p" "rustc_ast" "-p" "rustc_feature" "-p" "rustc_ast_pretty" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_error_codes" "-p" "rustc_typeck" "-p" "rustc_session" "-p" "rustc_data_structures" "-p" "rustc_metadata" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:42
