plain
[RUSTC-TIMING] rustc_incremental test:false 1.270
 Documenting rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
[RUSTC-TIMING] rustc_monomorphize test:false 1.301
 Documenting rustc_mir_dataflow v0.0.0 (/checkout/compiler/rustc_mir_dataflow)
error: unresolved link to `rustc_incremental::persist::save::build_dep_graph`
   --> compiler/rustc_incremental/src/persist/fs.rs:143:11
    |
143 | /// see [`rustc_incremental::persist::save::build_dep_graph`].
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `rustc_incremental` in scope
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`

error: unresolved link to `DepGraph`
   --> compiler/rustc_incremental/src/persist/fs.rs:196:67
    |
196 | /// If loading fails for some reason, we fallback to a disabled [`DepGraph`].
    |                                                                   ^^^^^^^^ no item named `DepGraph` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
[RUSTC-TIMING] rustc_mir_dataflow test:false 2.190
error: could not document `rustc_incremental`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_incremental compiler/rustc_incremental/src/lib.rs --target aarch64-unknown-linux-gnu -o /checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -C metadata=4417e124d42c6188 -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/release/deps --extern rand=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librand-5db77235b2cfedde.rmeta --extern rustc_ast=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_ast-1ba410238aec7dc2.rmeta --extern rustc_data_structures=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_data_structures-d513ff27df0cac76.rmeta --extern rustc_errors=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_errors-ee4be6447727d5ee.rmeta --extern rustc_fs_util=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_fs_util-5c759bced7af0dce.rmeta --extern rustc_graphviz=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_graphviz-b50a397d5805453e.rmeta --extern rustc_hir=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_hir-12fd03e2c6512588.rmeta --extern rustc_macros=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-6777ca596ba94fcc.so --extern rustc_middle=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_middle-5599b6493e95c3dc.rmeta --extern rustc_serialize=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_serialize-cbfb0cc45e66dba3.rmeta --extern rustc_session=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_session-d84fdae80b6e656d.rmeta --extern rustc_span=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_span-fdba9d3248eed195.rmeta --extern tracing=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libtracing-8eb19f6f60afcda5.rmeta --extern-html-root-url 'rand=https://docs.rs/rand/0.7.3/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.28/' -Zunstable-options -Zsymbol-mangling-version=v0 -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.59.0-nightly
  (75d07e6bd
  2021-12-07)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_passes test:false 2.584
[RUSTC-TIMING] rustc_metadata test:false 3.679
[RUSTC-TIMING] rustc_infer test:false 6.214
[RUSTC-TIMING] rustc_query_impl test:false 10.891
[RUSTC-TIMING] rustc_query_impl test:false 10.891
error: build failed


command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "aarch64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "14" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-Zunstable-options" "-Zskip-rustdoc-fingerprint" "--no-deps" "-Zrustdoc-map" "-p" "rustc_feature" "-p" "rustc_borrowck" "-p" "rustc_ty_utils" "-p" "rustc_mir_transform" "-p" "rustc_expand" "-p" "rustc_symbol_mangling" "-p" "rustc_privacy" "-p" "rustc_trait_selection" "-p" "rustc_data_structures" "-p" "rustc_hir" "-p" "rustc_attr" "-p" "rustc_type_ir" "-p" "rustc_lint_defs" "-p" "rustc_codegen_ssa" "-p" "rustc_interface" "-p" "rustc_index" "-p" "rustc_traits" "-p" "rustc_ast_passes" "-p" "rustc_passes" "-p" "rustc_error_codes" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_mir_dataflow" "-p" "rustc_serialize" "-p" "rustc_const_eval" "-p" "rustc_infer" "-p" "rustc_resolve" "-p" "rustc_fs_util" "-p" "rustc_builtin_macros" "-p" "rustc_hir_pretty" "-p" "rustc_ast_lowering" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_macros" "-p" "rustc_errors" "-p" "coverage_test_macros" "-p" "rustc_middle" "-p" "rustc_apfloat" "-p" "rustc_codegen_llvm" "-p" "rustc_monomorphize" "-p" "rustc_driver" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_metadata" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_query_system" "-p" "rustc_query_impl" "-p" "rustc_arena" "-p" "rustc_span" "-p" "rustc_target" "-p" "rustc_session" "-p" "rustc_mir_build" "-p" "rustc_typeck" "-p" "rustc_llvm" "-p" "rustc_ast"


Build completed unsuccessfully in 0:43:20
