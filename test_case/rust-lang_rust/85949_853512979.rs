plain
[RUSTC-TIMING] rustc_codegen_ssa test:false 2.959
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
 Documenting rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
[RUSTC-TIMING] rustc_query_impl test:false 4.376
error: this URL is not a hyperlink
   --> compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs:462:57
    |
462 |     /// Critically useful for this third-party project: https://github.com/hacspec/hacspec.
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://github.com/hacspec/hacspec.>`
    |
    = note: `-D rustdoc::bare-urls` implied by `-D warnings`
    = note: bare URLs are not automatically turned into clickable links

error: this URL is not a hyperlink
   --> compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs:463:13
    |
463 |     /// See https://github.com/rust-lang/rust/pull/85889 for context.
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://github.com/rust-lang/rust/pull/85889>`
    |
    = note: bare URLs are not automatically turned into clickable links
error: aborting due to 2 previous errors

error: could not document `rustc_metadata`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_metadata compiler/rustc_metadata/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-2e89c8340f21d8fd.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-cd8cc94a27a38859.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-e18fadc06654ecf3.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3c318052cc5e2988.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c09db6d261578a18.rmeta --extern rustc_expand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_expand-e9a27bf0f0143101.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-ff5b26a3cf2ef083.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-2b39a447feafa6fd.rmeta --extern rustc_hir_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir_pretty-fd2e72153cddbe01.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-696fd8ce4fa00664.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-91c19907dd1a36d3.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-0c96d8e2c6b3213e.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-7159eeb2d9f73c1f.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-cdc2f1c7ee949fe8.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-01a9fa73838c9beb.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-093e25e42c8fef38.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-bfb7de8be7e4e175.rmeta --extern snap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsnap-8efac53f71471102.rmeta --extern stable_deref_trait=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libstable_deref_trait-6cfb0f282a6d9c7a.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-7e5e9cf183cf4127.rmeta -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.54.0-nightly
  (3e5378074
  2021-06-03)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --cfg=parallel_compiler` (exit status: 1)
[RUSTC-TIMING] rustc_resolve test:false 2.824
[RUSTC-TIMING] rustc_codegen_llvm test:false 2.677
[RUSTC-TIMING] rustc_trait_selection test:false 3.840
error: build failed
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-Zskip-rustdoc-fingerprint" "--no-deps" "-p" "rustc_fs_util" "-p" "coverage_test_macros" "-p" "rustc_parse_format" "-p" "rustc_symbol_mangling" "-p" "rustc_interface" "-p" "rustc_llvm" "-p" "rustc_driver" "-p" "rustc_ast_passes" "-p" "rustc_hir" "-p" "rustc_errors" "-p" "rustc_save_analysis" "-p" "rustc_ast_lowering" "-p" "rustc_lint_defs" "-p" "rustc_target" "-p" "rustc_ast_pretty" "-p" "rustc_macros" "-p" "rustc_attr" "-p" "rustc_parse" "-p" "rustc_infer" "-p" "rustc_apfloat" "-p" "rustc_passes" "-p" "rustc_feature" "-p" "rustc_arena" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_error_codes" "-p" "rustc_expand" "-p" "rustc_mir_build" "-p" "rustc_index" "-p" "rustc_query_impl" "-p" "rustc_graphviz" "-p" "rustc_middle" "-p" "rustc_session" "-p" "rustc_type_ir" "-p" "rustc_plugin_impl" "-p" "rustc_trait_selection" "-p" "rustc_serialize" "-p" "rustc_span" "-p" "rustc_lexer" "-p" "rustc_metadata" "-p" "rustc_incremental" "-p" "rustc_builtin_macros" "-p" "rustc_hir_pretty" "-p" "rustc_ast" "-p" "rustc_mir" "-p" "rustc_lint" "-p" "rustc_ty_utils" "-p" "rustc_typeck" "-p" "rustc_data_structures" "-p" "rustc_query_system" "-p" "rustc_codegen_llvm" "-p" "rustc_resolve" "-p" "rustc_codegen_ssa"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:06:51
