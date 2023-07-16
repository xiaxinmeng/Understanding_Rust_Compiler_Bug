plain
    Checking chalk-solve v0.55.0
[RUSTC-TIMING] rustc_errors test:false 1.210
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
 Documenting rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error: unresolved link to `Map::opt_const_param_default_param_hir_id`
    --> compiler/rustc_hir/src/hir.rs:1427:38
     |
1427 | /// `const N: usize = { ... }` with [Map::opt_const_param_default_param_hir_id]
     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `Map` in scope
     |
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`

error: unresolved link to `Map::opt_const_param_default_param_hir_id`
    --> compiler/rustc_hir/src/hir.rs:1427:38
     |
1427 | /// `const N: usize = { ... }` with [Map::opt_const_param_default_param_hir_id]
     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `Map` in scope
error: aborting due to 2 previous errors

error: could not document `rustc_hir`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_hir compiler/rustc_hir/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-3916725f51ac2c31.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-e62f7b133d0f629d.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-79b4b8fe01f6a2b4.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-371e1565d9370dfc.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-7079f7ccd5750d15.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-caa2dd8dcfb9315e.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-29a5bd7555e0d6ff.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-1e5852cb47898325.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-943b89e095f064c5.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-7219816477d49636.rmeta -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.55.0-nightly
  (8e3359c9c
  2021-07-21)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout` (exit status: 1)
[RUSTC-TIMING] rustc_hir test:false 2.831
[RUSTC-TIMING] rustc_session test:false 1.549
[RUSTC-TIMING] chalk_solve test:false 3.502
error: build failed
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-Zskip-rustdoc-fingerprint" "--no-deps" "-p" "rustc_ast" "-p" "rustc_plugin_impl" "-p" "rustc_arena" "-p" "rustc_macros" "-p" "rustc_ast_lowering" "-p" "rustc_data_structures" "-p" "rustc_mir_build" "-p" "rustc_codegen_ssa" "-p" "rustc_type_ir" "-p" "rustc_target" "-p" "rustc_attr" "-p" "rustc_apfloat" "-p" "rustc_infer" "-p" "rustc_expand" "-p" "rustc_lexer" "-p" "rustc_traits" "-p" "rustc_save_analysis" "-p" "rustc_mir" "-p" "rustc_fs_util" "-p" "rustc_session" "-p" "rustc_lint" "-p" "rustc_llvm" "-p" "rustc_codegen_llvm" "-p" "rustc_privacy" "-p" "rustc_feature" "-p" "rustc_hir_pretty" "-p" "rustc_typeck" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_builtin_macros" "-p" "rustc_parse" "-p" "rustc_error_codes" "-p" "rustc_symbol_mangling" "-p" "rustc_serialize" "-p" "rustc_query_system" "-p" "rustc_ast_passes" "-p" "rustc_middle" "-p" "rustc_lint_defs" "-p" "rustc_driver" "-p" "rustc_query_impl" "-p" "rustc_metadata" "-p" "coverage_test_macros" "-p" "rustc_hir" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_ast_pretty" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_index" "-p" "rustc_incremental" "-p" "rustc_errors" "-p" "rustc_span" "-p" "rustc_graphviz"


Build completed unsuccessfully in 0:30:01
