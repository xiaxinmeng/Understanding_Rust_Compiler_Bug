plain
    Checking tracing-subscriber v0.2.13
error[E0282]: type annotations needed
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.2.13/src/filter/env/mod.rs:273:29
    |
273 |                     warning.style_ref_mut().is_bold = true;
    |
    |
    = note: type must be known at this point

error[E0599]: no method named `style_ref_mut` found for struct `ANSIGenericString<'_, _>` in the current scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.2.13/src/filter/env/mod.rs:273:29
    |
273 |                     warning.style_ref_mut().is_bold = true;
    |                             ^^^^^^^^^^^^^ method not found in `ANSIGenericString<'_, _>`
    |
    = note: `warning` is a function, perhaps you wish to call it

error[E0599]: no method named `style_ref_mut` found for struct `ANSIGenericString<'_, _>` in the current scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.2.13/src/filter/env/mod.rs:284:27
    |
284 |                     equal.style_ref_mut().is_bold = true;
    |                           ^^^^^^^^^^^^^ method not found in `ANSIGenericString<'_, _>`
    |
    = note: `equal` is a function, perhaps you wish to call it

error[E0599]: no method named `style_ref_mut` found for struct `ANSIGenericString<'_, _>` in the current scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.2.13/src/filter/env/mod.rs:297:26
    |
297 |                     pipe.style_ref_mut().is_bold = true;
    |                          ^^^^^^^^^^^^^ method not found in `ANSIGenericString<'_, _>`
    |
    = note: `pipe` is a function, perhaps you wish to call it
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
error: aborting due to 4 previous errors
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0282, E0599.
For more information about an error, try `rustc --explain E0282`.
error: could not compile `tracing-subscriber`
To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_index" "-p" "rustc_apfloat" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_macros" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_lint_defs" "-p" "rustc_hir_pretty" "-p" "rustc_parse" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_errors" "-p" "rustc_target" "-p" "rustc_serialize" "-p" "rustc_ast" "-p" "rustc_hir" "-p" "rustc_span" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_feature" "-p" "rustc_interface" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_privacy" "-p" "rustc_ty_utils" "-p" "rustc_symbol_mangling" "-p" "rustc_resolve" "-p" "rustc_query_impl" "-p" "rustc_builtin_macros" "-p" "rustc_incremental" "-p" "rustc_typeck" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_mir_build" "-p" "rustc_traits" "-p" "rustc_error_codes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_data_structures" "-p" "rustc_ast_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:06
