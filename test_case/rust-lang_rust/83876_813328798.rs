plain
 Documenting rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
[RUSTC-TIMING] rustc_codegen_ssa test:false 3.029
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
 Documenting rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: public documentation for `ModuleData` links to private item `ModuleData::kind`
    |
    |
496 | /// You can use [`ModuleData::kind`] to determine the kind of module this is.
    |
    |
    = note: `-D rustdoc::private-intra-doc-links` implied by `-D warnings`
    = note: this link resolves only because you passed `--document-private-items`, but will break without
error: aborting due to previous error

error: could not document `rustc_resolve`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_resolve compiler/rustc_resolve/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7ec83cf720989e83.rmeta --extern rustc_arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_arena-e14df766a9a1d97d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-7ff7bb95d4c16229.rmeta --extern rustc_ast_lowering=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_lowering-2281b2f55822a76b.rmeta --extern rustc_ast_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_pretty-16fc105755939491.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-8faf4f50e38a7d30.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-055eae6340fba688.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-bd9f33b8039500ac.rmeta --extern rustc_expand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_expand-6ae81af4a5399ebe.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-b4709a95b05971d1.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-b0994d339cc6ad76.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-7016c32b6dbe287c.rmeta --extern rustc_metadata=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata-a586983d9479653f.rmeta --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-a1422b32a9ede3da.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-a82c5457cab381b1.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-2cfda9c12eeb510a.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-bfb7de8be7e4e175.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-d866109bfaeb9bb9.rmeta -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.53.0-nightly
  (bdc42c256
  2021-04-05)' --document-private-items --enable-index-page -Zunstable-options -Znormalize-docs` (exit code: 1)
[RUSTC-TIMING] rustc_query_impl test:false 5.149
[RUSTC-TIMING] rustc_resolve test:false 3.003
warning: could not parse code block as Rust code
  --> compiler/rustc_trait_selection/src/opaque_types.rs:45:9
---
[RUSTC-TIMING] rustc_trait_selection test:false 4.285
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--no-deps" "-p" "rustc_infer" "-p" "rustc_lint_defs" "-p" "rustc_parse" "-p" "rustc_mir_build" "-p" "rustc_parse_format" "-p" "rustc_traits" "-p" "rustc_type_ir" "-p" "rustc_typeck" "-p" "rustc_serialize" "-p" "rustc_ty_utils" "-p" "rustc_fs_util" "-p" "rustc_hir" "-p" "rustc_hir_pretty" "-p" "rustc_metadata" "-p" "rustc_trait_selection" "-p" "rustc_passes" "-p" "rustc_driver" "-p" "rustc_ast" "-p" "rustc_macros" "-p" "rustc_lint" "-p" "rustc_incremental" "-p" "rustc_llvm" "-p" "rustc_errors" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_apfloat" "-p" "rustc_error_codes" "-p" "rustc_save_analysis" "-p" "rustc_lexer" "-p" "rustc_codegen_ssa" "-p" "rustc_query_system" "-p" "rustc_span" "-p" "rustc_symbol_mangling" "-p" "rustc_builtin_macros" "-p" "rustc_resolve" "-p" "rustc_plugin_impl" "-p" "coverage_test_macros" "-p" "rustc_data_structures" "-p" "rustc_index" "-p" "rustc_ast_lowering" "-p" "rustc_query_impl" "-p" "rustc_expand" "-p" "rustc_graphviz" "-p" "rustc_interface" "-p" "rustc_session" "-p" "rustc_attr" "-p" "rustc_ast_passes" "-p" "rustc_privacy" "-p" "rustc_feature" "-p" "rustc_target" "-p" "rustc_codegen_llvm" "-p" "rustc_mir" "-p" "rustc_ast_pretty"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:07:48
