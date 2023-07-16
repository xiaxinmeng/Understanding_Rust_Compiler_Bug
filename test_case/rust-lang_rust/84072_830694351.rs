plain
 Documenting rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
 Documenting rustc_target v0.0.0 (/checkout/compiler/rustc_target)
[RUSTC-TIMING] rustc_parse_format test:false 0.214
[RUSTC-TIMING] rustc_feature test:false 0.534
error: this URL is not a hyperlink
    --> compiler/rustc_target/src/spec/mod.rs:1049:13
     |
1049 |     /// See https://doc.rust-lang.org/reference/conditional-compilation.html#target_family
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://doc.rust-lang.org/reference/conditional-compilation.html#target_family>`
     |
     = note: `-D rustdoc::bare-urls` implied by `-D warnings`
     = note: bare URLs are not automatically turned into clickable links
error: aborting due to previous error

error: could not document `rustc_target`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_target compiler/rustc_target/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-c9d97e461f5936b6.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-4eefe2716bd21502.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-de9c06658b73bb6c.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-e23a8cf21214b03f.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-6a0fe2c0c859b1a2.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-fa0fbe29e7ebfc26.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-bdae349256a5f9fe.rmeta -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.54.0-nightly
  (716ea2c85
  2021-05-01)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs` (exit code: 1)
[RUSTC-TIMING] serde test:false 4.259
[RUSTC-TIMING] rustc_target test:false 3.809
[RUSTC-TIMING] rustc_ast test:false 4.827
error: build failed
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--no-deps" "-p" "rustc_query_system" "-p" "rustc_hir" "-p" "rustc_codegen_ssa" "-p" "rustc_privacy" "-p" "rustc_apfloat" "-p" "rustc_hir_pretty" "-p" "rustc_fs_util" "-p" "rustc_interface" "-p" "rustc_index" "-p" "rustc_save_analysis" "-p" "rustc_llvm" "-p" "rustc_data_structures" "-p" "rustc_plugin_impl" "-p" "rustc_attr" "-p" "rustc_mir" "-p" "rustc_serialize" "-p" "rustc_query_impl" "-p" "rustc_lint" "-p" "rustc_lexer" "-p" "rustc_error_codes" "-p" "rustc_middle" "-p" "rustc_macros" "-p" "rustc_arena" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_span" "-p" "rustc_ast" "-p" "rustc_ast_lowering" "-p" "rustc_mir_build" "-p" "rustc_graphviz" "-p" "rustc_traits" "-p" "rustc_ast_pretty" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_expand" "-p" "rustc_passes" "-p" "rustc_lint_defs" "-p" "rustc_errors" "-p" "rustc_type_ir" "-p" "rustc_incremental" "-p" "rustc_codegen_llvm" "-p" "rustc_target" "-p" "rustc_driver" "-p" "rustc_metadata" "-p" "rustc_resolve" "-p" "rustc_ast_passes" "-p" "rustc_builtin_macros" "-p" "rustc_feature" "-p" "coverage_test_macros" "-p" "rustc_infer" "-p" "rustc_session" "-p" "rustc_symbol_mangling" "-p" "rustc_parse"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:30:24
