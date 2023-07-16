plain
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
   --> compiler/rustc_resolve/src/late/diagnostics.rs:177:22
    |
177 |                     .map_or_else(|_| String::new(), |res| format!("{} ", res.descr()));
    |                      ^^^^^^^^^^^ --- takes 1 argument
    |                      expected closure that takes 0 arguments


error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> compiler/rustc_resolve/src/late/diagnostics.rs:139:14
    |
139 |         let (base_msg, fallback_label, base_span, could_be_expr) = if let Some(res) = res {
    |              ^^^^^^^^ doesn't have a size known at compile-time
    = help: the trait `std::marker::Sized` is not implemented for `str`
    = help: the trait `std::marker::Sized` is not implemented for `str`
    = note: all local variables must have a statically known size
    = help: unsized locals are gated as an unstable feature

error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> compiler/rustc_resolve/src/late/diagnostics.rs:139:24
    |
139 |         let (base_msg, fallback_label, base_span, could_be_expr) = if let Some(res) = res {
    |                        ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    = help: the trait `std::marker::Sized` is not implemented for `str`
    = help: the trait `std::marker::Sized` is not implemented for `str`
    = note: all local variables must have a statically known size
    = help: unsized locals are gated as an unstable feature

error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> compiler/rustc_resolve/src/late/diagnostics.rs:542:43
    |
542 |                 err.span_label(base_span, fallback_label);
    |                                           ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    = help: the trait `std::marker::Sized` is not implemented for `str`


error[E0277]: the trait bound `std::string::String: From<str>` is not satisfied
   --> compiler/rustc_resolve/src/late/diagnostics.rs:542:21
    |
542 |                 err.span_label(base_span, fallback_label);
    |                     ^^^^^^^^^^ the trait `From<str>` is not implemented for `std::string::String`
    = help: the following implementations were found:
    = help: the following implementations were found:
              <std::string::String as From<&mut str>>
              <std::string::String as From<&std::string::String>>
              <std::string::String as From<&str>>
              <std::string::String as From<Cow<'a, str>>>
            and 2 others
    = note: required because of the requirements on the impl of `Into<std::string::String>` for `str`
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0277, E0593.
Some errors have detailed explanations: E0277, E0593.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_resolve`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_expand" "-p" "rustc_macros" "-p" "rustc_lexer" "-p" "rustc_lint_defs" "-p" "rustc_typeck" "-p" "rustc_arena" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_traits" "-p" "rustc_symbol_mangling" "-p" "rustc_resolve" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_ast_lowering" "-p" "rustc_attr" "-p" "rustc_ty_utils" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_passes" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_privacy" "-p" "rustc_index" "-p" "rustc_ast_passes" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_builtin_macros" "-p" "rustc_data_structures" "-p" "rustc_plugin_impl" "-p" "rustc_error_codes" "-p" "rustc_lint" "-p" "rustc_ast_pretty" "-p" "rustc_metadata" "-p" "rustc_hir_pretty" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_span" "-p" "rustc_save_analysis" "-p" "rustc_serialize" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_session" "-p" "rustc_target" "-p" "rustc_errors" "-p" "rustc_ast" "-p" "rustc_hir" "-p" "rustc_parse" "-p" "rustc_feature" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:34
