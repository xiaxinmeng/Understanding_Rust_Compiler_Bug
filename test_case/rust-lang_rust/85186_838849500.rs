plain
    Checking matchers v0.0.1
    Checking tempfile v3.2.0
    Checking synstructure v0.12.4
 Documenting rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
thread 'rustc' panicked at 'invoked `update_reached_depth` with something under this stack: self.depth=25 reached_depth=32', compiler/rustc_trait_selection/src/traits/select/mod.rs:2164:9

error: internal compiler error: unexpected panic

error: Unrecognized option: 'crate-version'
error: Unrecognized option: 'crate-version'

error: could not document `rustc_macros`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type proc-macro --crate-name rustc_macros compiler/rustc_macros/src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern proc_macro2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/libproc_macro2-3e6746836238bc53.rmeta --extern quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/libquote-95c7b82e9536f67e.rmeta --extern syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/libsyn-957e21fa6f00b3a8.rmeta --extern synstructure=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/libsynstructure-f6e0bf1f33792190.rmeta --extern proc_macro --crate-version 0.1.0` (exit code: 1)
error: build failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--no-deps" "-p" "rustc_error_codes" "-p" "rustc_lint" "-p" "rustc_codegen_llvm" "-p" "rustc_symbol_mangling" "-p" "rustc_apfloat" "-p" "rustc_traits" "-p" "rustc_plugin_impl" "-p" "rustc_query_impl" "-p" "rustc_builtin_macros" "-p" "rustc_ty_utils" "-p" "rustc_llvm" "-p" "rustc_ast_pretty" "-p" "rustc_hir" "-p" "rustc_codegen_ssa" "-p" "rustc_errors" "-p" "rustc_privacy" "-p" "rustc_data_structures" "-p" "rustc_index" "-p" "rustc_lint_defs" "-p" "rustc_metadata" "-p" "rustc_ast" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "coverage_test_macros" "-p" "rustc_graphviz" "-p" "rustc_ast_passes" "-p" "rustc_target" "-p" "rustc_lexer" "-p" "rustc_macros" "-p" "rustc_passes" "-p" "rustc_mir" "-p" "rustc_span" "-p" "rustc_middle" "-p" "rustc_attr" "-p" "rustc_expand" "-p" "rustc_driver" "-p" "rustc_session" "-p" "rustc_save_analysis" "-p" "rustc_incremental" "-p" "rustc_resolve" "-p" "rustc_fs_util" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_interface" "-p" "rustc_query_system" "-p" "rustc_hir_pretty" "-p" "rustc_parse" "-p" "rustc_feature" "-p" "rustc_ast_lowering" "-p" "rustc_serialize" "-p" "rustc_mir_build" "-p" "rustc_typeck"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:06:21
