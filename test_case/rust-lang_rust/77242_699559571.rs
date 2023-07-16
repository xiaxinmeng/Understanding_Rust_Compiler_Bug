plain
warning: 4 warnings emitted

    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
 Documenting rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error: public documentation for `apply_switch_int_edge_effects` links to private item `SwitchIntEdgeEffects::apply`
   --> compiler/rustc_mir/src/dataflow/framework/mod.rs:189:10
    |
189 |     /// [`SwitchIntEdgeEffects::apply`]. The callback will be run once for each outgoing edge and
    |
    |
    = note: `-D broken-intra-doc-links` implied by `-D warnings`
    = note: this link resolves only because you passed `--document-private-items`, but will break without

error: public documentation for `apply_switch_int_edge_effects` links to private item `SwitchIntEdgeEffects::apply`
   --> compiler/rustc_mir/src/dataflow/framework/mod.rs:189:10
    |
189 |     /// [`SwitchIntEdgeEffects::apply`]. The callback will be run once for each outgoing edge and
    |
    |
    = note: this link resolves only because you passed `--document-private-items`, but will break without
warning: could not parse code block as Rust code
    --> compiler/rustc_mir/src/borrow_check/region_infer/mod.rs:1219:13
     |
1219 |     ///     <T as Foo<'0>>::Item: '1
---
    |     ^^^^^^^

error: aborting due to 2 previous errors; 2 warnings emitted

error: Could not document `rustc_mir`.
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_mir compiler/rustc_mir/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-0886f79aa9644b59.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libitertools-15ba3a79d5ec8f6f.rmeta --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-09a944a5749e95d8.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-9a99b9287362070b.rmeta --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libregex-32ea2f287d81478c.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-2bc88092b3516846.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-27ad5e09ebab896d.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-9bd7064b478506f7.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-5d3d1912340ee4c7.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-94b39b40ea1509ed.rmeta --extern rustc_graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_graphviz-e79b54e3fcce812e.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-98ac1e26a11e2929.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-78f7d4c857d61346.rmeta --extern rustc_infer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_infer-7cf4ec21cef32adb.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-d9c58c19eb967beb.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-526b2b3435deae8e.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-af60eb12c80f2671.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-ba853f2a3b898296.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-49085084dc014133.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-1ff4578e4d1f53ae.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-54d995fca71a3ae5.rmeta --extern rustc_trait_selection=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_trait_selection-8ce1ed9cb916064a.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-31c79deb030c1dbf.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-62a9fe09301d4da9.rmeta -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.48.0-nightly
  (b61b01669
  2020-09-26)' --document-private-items --enable-index-page -Zunstable-options` (exit code: 1)
error: build failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--no-deps" "-p" "rustc_ast_pretty" "-p" "rustc_macros" "-p" "rustc_lexer" "-p" "rustc_span" "-p" "rustc_ast" "-p" "rustc_query_system" "-p" "rustc_apfloat" "-p" "rustc_infer" "-p" "rustc_serialize" "-p" "rustc_expand" "-p" "rustc_hir" "-p" "rustc_traits" "-p" "rustc_codegen_ssa" "-p" "rustc_fs_util" "-p" "rustc_mir_build" "-p" "rustc_hir_pretty" "-p" "rustc_data_structures" "-p" "rustc_llvm" "-p" "rustc_save_analysis" "-p" "rustc_typeck" "-p" "rustc_ast_passes" "-p" "rustc_attr" "-p" "rustc_index" "-p" "rustc_parse_format" "-p" "rustc_ast_lowering" "-p" "rustc_middle" "-p" "rustc_plugin_impl" "-p" "rustc_driver" "-p" "rustc_incremental" "-p" "rustc_parse" "-p" "rustc_symbol_mangling" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_target" "-p" "rustc_session" "-p" "rustc_ty" "-p" "rustc_metadata" "-p" "rustc_privacy" "-p" "rustc_arena" "-p" "rustc_codegen_llvm" "-p" "rustc_mir" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_feature" "-p" "rustc_error_codes" "-p" "rustc_builtin_macros" "-p" "rustc_graphviz" "-p" "rustc_errors"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:21:32
Build completed unsuccessfully in 0:21:32
== clock drift check ==
  local time: Sat Sep 26 23:14:14 UTC 2020
  network time: Sat, 26 Sep 2020 23:14:14 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (5376) (python)
