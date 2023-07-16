plain
 Documenting rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking tracing-serde v0.1.2
    Checking gsgdt v0.1.2
    Checking rls-span v0.5.3
error: unresolved link to `outer_attr`
   --> compiler/rustc_ast/src/tokenstream.rs:205:48
    |
205 |     /// back to a `TokenStream` of the form `#[outer_attr] attr_target`.
    |                                                ^^^^^^^^^^ no item named `outer_attr` in scope
    |
    = note: `-D broken-intra-doc-links` implied by `-D warnings`
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    Checking rls-data v0.19.1
error: aborting due to previous error

error: could not document `rustc_ast`
error: could not document `rustc_ast`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_ast compiler/rustc_ast/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-2f4ad6ed99e8eeca.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3c6b54bd6576736a.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-53c75d98d366c3dd.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-63fdfb72cdb44d2f.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-e1634dc0d7ab121f.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-b9a24f71da7b43c7.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-df9f93c2134540eb.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-9fb44e28278df6b0.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-c382490ed46d3134.rmeta -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.52.0-nightly
  (18c0527ae
  2021-03-02)' --document-private-items --enable-index-page -Zunstable-options -Znormalize-docs` (exit code: 1)
error: build failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--no-deps" "-p" "rustc_builtin_macros" "-p" "rustc_index" "-p" "rustc_parse_format" "-p" "rustc_error_codes" "-p" "rustc_span" "-p" "rustc_target" "-p" "rustc_data_structures" "-p" "rustc_expand" "-p" "rustc_resolve" "-p" "rustc_fs_util" "-p" "rustc_save_analysis" "-p" "rustc_lint" "-p" "rustc_ast_pretty" "-p" "rustc_metadata" "-p" "rustc_ty_utils" "-p" "rustc_parse" "-p" "rustc_middle" "-p" "rustc_driver" "-p" "rustc_mir" "-p" "rustc_privacy" "-p" "rustc_serialize" "-p" "rustc_ast_lowering" "-p" "rustc_passes" "-p" "rustc_graphviz" "-p" "rustc_session" "-p" "rustc_llvm" "-p" "rustc_symbol_mangling" "-p" "rustc_feature" "-p" "rustc_arena" "-p" "rustc_attr" "-p" "rustc_apfloat" "-p" "rustc_ast" "-p" "rustc_errors" "-p" "rustc_infer" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_hir" "-p" "rustc_codegen_llvm" "-p" "rustc_query_impl" "-p" "rustc_plugin_impl" "-p" "rustc_hir_pretty" "-p" "rustc_codegen_ssa" "-p" "rustc_macros" "-p" "rustc_trait_selection" "-p" "rustc_type_ir" "-p" "rustc_mir_build" "-p" "rustc_interface" "-p" "coverage_test_macros" "-p" "rustc_typeck" "-p" "rustc_lexer" "-p" "rustc_traits" "-p" "rustc_query_system" "-p" "rustc_incremental"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:06:38
