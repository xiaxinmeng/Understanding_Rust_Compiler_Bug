plain
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
 Documenting rustc_span v0.0.0 (/checkout/compiler/rustc_span)
[RUSTC-TIMING] rustc_type_ir test:false 0.515
[RUSTC-TIMING] serde_derive test:false 6.013
error: this URL is not a hyperlink
   --> compiler/rustc_arena/src/lib.rs:338:13
    |
338 |     /// see https://fitzgeraldnick.com/2019/11/01/always-bump-downwards.html.)
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://fitzgeraldnick.com/2019/11/01/always-bump-downwards.html.>`
    |
    = note: `-D rustdoc::bare-urls` implied by `-D warnings`
    = note: bare URLs are not automatically turned into clickable links
error: could not document `rustc_arena`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_arena compiler/rustc_arena/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-2c0667366df9694a.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-4db111a340340887.rmeta --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.7.0/' -Zunstable-options -Zsymbol-mangling-version=v0 -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.58.0
  (e058ef407
  2021-11-18)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_span test:false 2.671
[RUSTC-TIMING] serde test:false 5.096
error: build failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-Zunstable-options" "-Zskip-rustdoc-fingerprint" "--no-deps" "-Zrustdoc-map" "-p" "rustc_attr" "-p" "rustc_parse" "-p" "rustc_typeck" "-p" "rustc_driver" "-p" "rustc_infer" "-p" "rustc_resolve" "-p" "rustc_lexer" "-p" "rustc_plugin_impl" "-p" "rustc_incremental" "-p" "rustc_lint" "-p" "rustc_mir_dataflow" "-p" "rustc_borrowck" "-p" "rustc_traits" "-p" "rustc_error_codes" "-p" "rustc_apfloat" "-p" "rustc_ast" "-p" "rustc_data_structures" "-p" "rustc_passes" "-p" "rustc_feature" "-p" "rustc_serialize" "-p" "rustc_trait_selection" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_type_ir" "-p" "rustc_symbol_mangling" "-p" "rustc_graphviz" "-p" "rustc_middle" "-p" "rustc_codegen_ssa" "-p" "rustc_query_system" "-p" "rustc_span" "-p" "rustc_fs_util" "-p" "rustc_codegen_llvm" "-p" "rustc_mir_transform" "-p" "rustc_index" "-p" "rustc_ast_pretty" "-p" "rustc_const_eval" "-p" "rustc_mir_build" "-p" "rustc_interface" "-p" "rustc_query_impl" "-p" "rustc_arena" "-p" "rustc_monomorphize" "-p" "rustc_errors" "-p" "rustc_expand" "-p" "rustc_privacy" "-p" "rustc_session" "-p" "rustc_save_analysis" "-p" "rustc_parse_format" "-p" "rustc_builtin_macros" "-p" "rustc_macros" "-p" "rustc_ty_utils" "-p" "rustc_metadata" "-p" "rustc_ast_lowering" "-p" "coverage_test_macros" "-p" "rustc_hir" "-p" "rustc_target" "-p" "rustc_hir_pretty" "-p" "rustc_llvm"


Build completed unsuccessfully in 0:36:40
