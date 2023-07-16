plain
[RUSTC-TIMING] rustc_expand test:false 1.989
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
 Documenting rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
[RUSTC-TIMING] rustc_builtin_macros test:false 1.999
error: unresolved link to `terminator::TerminatorKind::DropAndReplace`
   |
   |
56 |     /// * [`TerminatorKind::DropAndReplace`](terminator::TerminatorKind::DropAndReplace)
   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `terminator` in scope
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`

error: unresolved link to `terminator::TerminatorKind::FalseUnwind`
   |
   |
57 |     /// * [`TerminatorKind::FalseUnwind`](terminator::TerminatorKind::FalseUnwind)
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `terminator` in scope

error: unresolved link to `terminator::TerminatorKind::FalseEdge`
   |
   |
58 |     /// * [`TerminatorKind::FalseEdge`](terminator::TerminatorKind::FalseEdge)
   |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `terminator` in scope
error: unresolved link to `terminator::TerminatorKind::Yield`
  --> compiler/rustc_middle/src/mir/syntax.rs:89:37
   |
   |
89 |     /// * [`TerminatorKind::Yield`](terminator::TerminatorKind::Yield)
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `terminator` in scope
error: unresolved link to `terminator::TerminatorKind::GeneratorDrop`
  --> compiler/rustc_middle/src/mir/syntax.rs:90:45
   |
   |
90 |     /// * [`TerminatorKind::GeneratorDrop`](terminator::TerminatorKind::GeneratorDrop)
   |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `terminator` in scope

error: unresolved link to `tcx::PlaceTy`
    |
    |
715 | ///  3. The type of the place and an optional variant index. See [`PlaceTy`][tcx::PlaceTy].
    |                                                                              ^^^^^^^^^^^^ no item named `tcx` in scope
error: could not document `rustc_middle`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_middle compiler/rustc_middle/src/lib.rs --target aarch64-unknown-linux-gnu -o /checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature, "rustc-rayon", "rustc-rayon-core", "rustc_use_parallel_compiler")' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=9a789795c64c45da -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libbitflags-0410241138ddd382.rmeta --extern chalk_ir=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libchalk_ir-b036709b0e4971af.rmeta --extern either=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libeither-d64d7fec5c028f11.rmeta --extern gsgdt=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libgsgdt-72ccf04c7035a6be.rmeta --extern polonius_engine=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libpolonius_engine-2cb1212bef125339.rmeta --extern rand=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librand-79250d319d79c195.rmeta --extern rand_xoshiro=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librand_xoshiro-59d63a5169ea9873.rmeta --extern rustc_apfloat=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_apfloat-b77413dbf9e42b9c.rmeta --extern rustc_arena=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_arena-2d75e75ef9739da5.rmeta --extern rustc_ast=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_ast-593e1bfe34f1954f.rmeta --extern rustc_attr=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_attr-6b8f8fa6d214e00d.rmeta --extern rustc_data_structures=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_data_structures-56814c4d8c93246d.rmeta --extern rustc_errors=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_errors-09fe49c99f800086.rmeta --extern rustc_feature=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_feature-dd8464297ef7525d.rmeta --extern rustc_graphviz=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_graphviz-b50a397d5805453e.rmeta --extern rustc_hir=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_hir-75c97c47dfd2493c.rmeta --extern rustc_index=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_index-e776b3c3799810ed.rmeta --extern rustc_macros=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-77a23d2b6fe629a0.so --extern rustc_query_system=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_query_system-0a4d1fa6455180e8.rmeta --extern rustc_serialize=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_serialize-2b5d40fcf7bcf0a1.rmeta --extern rustc_session=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_session-e0c37e1ea6e43c95.rmeta --extern rustc_span=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_span-6e7a3390881d0b7d.rmeta --extern rustc_target=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_target-2fcffbc16da4c607.rmeta --extern rustc_type_ir=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_type_ir-e99d1f3563e689d3.rmeta --extern smallvec=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libsmallvec-ec66cb750b733b82.rmeta --extern tracing=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libtracing-016725879ecff5df.rmeta --extern-html-root-url 'bitflags=https://docs.rs/bitflags/1.2.1/' --extern-html-root-url 'chalk_ir=https://docs.rs/chalk-ir/0.80.0/' --extern-html-root-url 'either=https://docs.rs/either/1.6.0/' --extern-html-root-url 'gsgdt=https://docs.rs/gsgdt/0.1.2/' --extern-html-root-url 'polonius_engine=https://docs.rs/polonius-engine/0.13.0/' --extern-html-root-url 'rand=https://docs.rs/rand/0.8.5/' --extern-html-root-url 'rand_xoshiro=https://docs.rs/rand_xoshiro/0.6.0/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.8.1/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.29/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.64.0-nightly
  (9789394f4
  2022-06-29)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_middle test:false 27.442
Build completed unsuccessfully in 0:41:55
