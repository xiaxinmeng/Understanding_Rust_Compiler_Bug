plain
[RUSTC-TIMING] build_script_build test:false 1.200
[RUSTC-TIMING] build_script_build test:false 0.417
[RUSTC-TIMING] bitflags test:false 0.076
[RUSTC-TIMING] build_script_build test:false 1.296
error: internal compiler error: compiler/rustc_traits/src/normalize_erasing_regions.rs:54:32: could not fully normalize `<[E] as std::borrow::ToOwned>::Owned`

thread 'rustc' panicked at 'Box<Any>', /rustc/70501f5ac34563ee7bbe2ee98f40efacb09a1f93/library/std/src/panic.rs:59:5

error: Unrecognized option: 'crate-version'

error: aborting due to previous error
---
[RUSTC-TIMING] rustc_lexer test:false 0.583
error: could not document `rustc_graphviz`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_graphviz compiler/rustc_graphviz/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.53.0-nightly
  (70501f5ac
  2021-04-30)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout` (exit code: 1)
[RUSTC-TIMING] build_script_build test:false 0.898
[RUSTC-TIMING] sharded_slab test:false 1.091
[RUSTC-TIMING] build_script_build test:false 0.943
[RUSTC-TIMING] itertools test:false 1.125
---
[RUSTC-TIMING] typenum test:false 1.128
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--no-deps" "-p" "rustc_session" "-p" "rustc_codegen_llvm" "-p" "rustc_ast_pretty" "-p" "rustc_lint" "-p" "rustc_save_analysis" "-p" "rustc_expand" "-p" "rustc_interface" "-p" "rustc_fs_util" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_typeck" "-p" "rustc_graphviz" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "rustc_ty_utils" "-p" "rustc_parse_format" "-p" "rustc_span" "-p" "rustc_trait_selection" "-p" "rustc_resolve" "-p" "rustc_index" "-p" "rustc_ast" "-p" "rustc_hir_pretty" "-p" "rustc_privacy" "-p" "rustc_metadata" "-p" "rustc_query_impl" "-p" "rustc_middle" "-p" "rustc_incremental" "-p" "rustc_plugin_impl" "-p" "rustc_symbol_mangling" "-p" "rustc_traits" "-p" "rustc_arena" "-p" "rustc_serialize" "-p" "rustc_llvm" "-p" "rustc_hir" "-p" "rustc_data_structures" "-p" "rustc_driver" "-p" "rustc_passes" "-p" "rustc_query_system" "-p" "rustc_infer" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_attr" "-p" "rustc_feature" "-p" "rustc_type_ir" "-p" "rustc_mir" "-p" "rustc_ast_lowering" "-p" "rustc_error_codes" "-p" "rustc_target" "-p" "rustc_macros" "-p" "rustc_builtin_macros" "-p" "rustc_mir_build" "-p" "rustc_codegen_ssa" "-p" "rustc_errors"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:06:45
