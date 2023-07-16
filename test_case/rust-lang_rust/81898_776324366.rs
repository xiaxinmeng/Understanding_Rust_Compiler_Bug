plain
    Checking rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
    Checking rustc_metadata v0.0.0 (/checkout/compiler/rustc_metadata)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0425]: cannot find value `cx` in this scope
   --> compiler/rustc_codegen_ssa/src/mir/debuginfo.rs:344:32
    |
344 | ...                   if cx.sess().target.is_like_msvc {
    |
    |
help: you might have meant to use the available field
    |
344 |                             if self.cx.sess().target.is_like_msvc {
help: consider importing this unit variant
    |
    |
1   | use rustc_target::asm::X86InlineAsmReg::cx;

    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0308]: `if` and `else` have incompatible types
   --> compiler/rustc_codegen_ssa/src/mir/debuginfo.rs:365:33
    |
344 |  / ...                   if cx.sess().target.is_like_msvc {
345 |  | ...                       // Rust compiler decomposes every &str or slice argument into two components:
346 |  | ...                       // a pointer to the memory address where the data is stored and a usize representing
347 |  | ...                       // the length of the str (or slice). These components will later be used to reconstruct
...    |
354 | /| ...                       match *var_ty.kind() {
355 | || ...                           ty::Ref(_, inner_type, _) => match *inner_type.kind() {
356 | || ...                               ty::Slice(_) | ty::Str => VariableKind::LocalVariable,
357 | || ...                               _ => VariableKind::ArgumentVariable(arg_index + 1),
358 | || ...                           },
359 | || ...                           _ => VariableKind::ArgumentVariable(arg_index + 1),
360 | || ...                       }
    | ||___________________________- expected because of this
...    |
365 |  | ...                       VariableKind::ArgumentVariable(arg_index + 1);
    |  |                           |                                            |
    |  |                           |                                            help: consider removing this semicolon
    |  |                           |                                            help: consider removing this semicolon
    |  |                           expected enum `VariableKind`, found `()`
366 |  | ...                   }
    |  |_______________________- `if` and `else` have incompatible types
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_ssa`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_feature" "-p" "rustc_ast_pretty" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_apfloat" "-p" "rustc_serialize" "-p" "rustc_index" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_ast" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_session" "-p" "rustc_hir" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_incremental" "-p" "rustc_symbol_mangling" "-p" "rustc_target" "-p" "rustc_macros" "-p" "rustc_driver" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_save_analysis" "-p" "rustc_interface" "-p" "rustc_ast_lowering" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_mir_build" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_expand" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_ast_passes" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_hir_pretty" "-p" "rustc_error_codes" "-p" "rustc_metadata" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:37
