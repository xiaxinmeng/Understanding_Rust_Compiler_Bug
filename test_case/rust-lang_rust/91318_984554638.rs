plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMKFrXW/Ko4X
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
[RUSTC-TIMING] chalk_engine test:false 2.519
    Checking rustc_parse v0.0.0 (/Users/runner/work/rust/rust/compiler/rustc_parse)
[RUSTC-TIMING] rustc_query_system test:false 2.181
 Documenting rustc_parse v0.0.0 (/Users/runner/work/rust/rust/compiler/rustc_parse)
error: couldn't generate documentation: stream did not contain valid UTF-8
  |
  = note: failed to create or modify "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/doc/search-index.js"
error: could not document `rustc_session`

Caused by:
Caused by:
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_session compiler/rustc_session/src/lib.rs --target x86_64-apple-darwin -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps -L dependency=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/release/deps --extern getopts=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps/libgetopts-2ff56373c38cf350.rmeta --extern num_cpus=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps/libnum_cpus-e1a8753e97447fd0.rmeta --extern rustc_ast=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps/librustc_ast-4e75d221da689f49.rmeta --extern rustc_data_structures=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps/librustc_data_structures-6c8d797292753b07.rmeta --extern rustc_errors=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps/librustc_errors-59edd5fe9d85a5cf.rmeta --extern rustc_feature=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps/librustc_feature-b92c0c85839b7abd.rmeta --extern rustc_fs_util=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps/librustc_fs_util-30e70bfc94a9a667.rmeta --extern rustc_hir=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps/librustc_hir-0812c6efdcbcd380.rmeta --extern rustc_lint_defs=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps/librustc_lint_defs-05ed8cb757efd4ae.rmeta --extern rustc_macros=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/release/deps/librustc_macros-e10457cb33b66a07.dylib --extern rustc_serialize=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps/librustc_serialize-e85b74e55ce58e02.rmeta --extern rustc_span=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps/librustc_span-fc5029242af7d85b.rmeta --extern rustc_target=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps/librustc_target-09bced2149876958.rmeta --extern tracing=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-rustc/x86_64-apple-darwin/release/deps/libtracing-dd19761d1ff5badb.rmeta --extern-html-root-url 'getopts=https://docs.rs/getopts/0.2.21/' --extern-html-root-url 'num_cpus=https://docs.rs/num_cpus/1.13.0/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.28/' -Zunstable-options -Zsymbol-mangling-version=v0 -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.59.0-nightly
  (65b0a7bc0
  2021-12-02)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_parse test:false 3.468
error: build failed



command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "doc" "--target" "x86_64-apple-darwin" "-Zbinary-dep-depinfo" "-j" "3" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/Users/runner/work/rust/rust/compiler/rustc/Cargo.toml" "-Zunstable-options" "-Zskip-rustdoc-fingerprint" "--no-deps" "-Zrustdoc-map" "-p" "rustc_llvm" "-p" "rustc_parse" "-p" "rustc_ast" "-p" "rustc_lint" "-p" "rustc_borrowck" "-p" "rustc_span" "-p" "rustc_infer" "-p" "rustc_privacy" "-p" "coverage_test_macros" "-p" "rustc_graphviz" "-p" "rustc_traits" "-p" "rustc_session" "-p" "rustc_mir_build" "-p" "rustc_lexer" "-p" "rustc_arena" "-p" "rustc_errors" "-p" "rustc_hir" "-p" "rustc_serialize" "-p" "rustc_apfloat" "-p" "rustc_mir_transform" "-p" "rustc_ast_lowering" "-p" "rustc_target" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_ty_utils" "-p" "rustc_metadata" "-p" "rustc_symbol_mangling" "-p" "rustc_ast_pretty" "-p" "rustc_macros" "-p" "rustc_query_system" "-p" "rustc_error_codes" "-p" "rustc_const_eval" "-p" "rustc_data_structures" "-p" "rustc_driver" "-p" "rustc_monomorphize" "-p" "rustc_trait_selection" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_feature" "-p" "rustc_middle" "-p" "rustc_mir_dataflow" "-p" "rustc_incremental" "-p" "rustc_plugin_impl" "-p" "rustc_parse_format" "-p" "rustc_hir_pretty" "-p" "rustc_builtin_macros" "-p" "rustc_type_ir" "-p" "rustc_query_impl" "-p" "rustc_passes" "-p" "rustc_interface" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_save_analysis" "-p" "rustc_codegen_ssa" "-p" "rustc_typeck" "-p" "rustc_fs_util" "-p" "rustc_lint_defs"


Build completed unsuccessfully in 2:38:05
