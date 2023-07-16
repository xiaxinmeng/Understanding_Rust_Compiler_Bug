plain
[RUSTC-TIMING] rustc_errors test:false 1.441
error: unresolved link to `global_asm`
   --> compiler/rustc_hir/src/def.rs:116:20
    |
116 |     /// A use of [`global_asm!`].
    |                    ^^^^^^^^^^^ no item named `global_asm` in scope
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `rustc_hir`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_hir compiler/rustc_hir/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -C metadata=cb651872e5e3c0e8 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern odht=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libodht-f53637fb7cfae6f7.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-0a31cdb021f26106.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-753e4a36c5443b5d.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-3cc6c33bd708c8ea.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-d90d1aed05ab3bab.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-4c99b52135cc012d.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-04f5d07bbe443ab1.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-09d9ffe65b4c4966.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-e98343166915a6b4.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-be09b2194530a403.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-0704c0a21b2e9dae.rmeta --extern-html-root-url 'odht=https://docs.rs/odht/0.3.1/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.7.0/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.28/' -Zunstable-options -Zsymbol-mangling-version=v0 -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.59.0-nightly
  (1d2d4ea89
  2021-12-13)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_hir test:false 2.998
[RUSTC-TIMING] chalk_solve test:false 3.678
error: build failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-Zunstable-options" "-Zskip-rustdoc-fingerprint" "--no-deps" "-Zrustdoc-map" "-p" "rustc_mir_build" "-p" "rustc_interface" "-p" "rustc_arena" "-p" "rustc_plugin_impl" "-p" "rustc_ty_utils" "-p" "rustc_macros" "-p" "rustc_ast" "-p" "rustc_metadata" "-p" "rustc_typeck" "-p" "rustc_parse" "-p" "rustc_errors" "-p" "rustc_borrowck" "-p" "rustc_mir_dataflow" "-p" "rustc_serialize" "-p" "rustc_ast_passes" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_target" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_symbol_mangling" "-p" "rustc_ast_pretty" "-p" "rustc_index" "-p" "rustc_query_impl" "-p" "rustc_error_codes" "-p" "rustc_driver" "-p" "rustc_apfloat" "-p" "rustc_feature" "-p" "rustc_hir" "-p" "rustc_span" "-p" "rustc_data_structures" "-p" "rustc_query_system" "-p" "rustc_trait_selection" "-p" "rustc_hir_pretty" "-p" "rustc_resolve" "-p" "coverage_test_macros" "-p" "rustc_graphviz" "-p" "rustc_passes" "-p" "rustc_const_eval" "-p" "rustc_expand" "-p" "rustc_monomorphize" "-p" "rustc_mir_transform" "-p" "rustc_lexer" "-p" "rustc_session" "-p" "rustc_attr" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_traits" "-p" "rustc_llvm" "-p" "rustc_codegen_ssa" "-p" "rustc_save_analysis" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_incremental"


Build completed unsuccessfully in 0:32:52
