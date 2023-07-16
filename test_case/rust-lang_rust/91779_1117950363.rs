plain
test [debuginfo-gdb] src/test/debuginfo/generic-method-on-generic-struct.rs ... ok
test [debuginfo-gdb] src/test/debuginfo/issue-57822.rs ... ok
test [debuginfo-gdb] src/test/debuginfo/issue-13213.rs ... ok
test [debuginfo-gdb] src/test/debuginfo/lexical-scope-in-if.rs ... ok
test [debuginfo-gdb] src/test/debuginfo/msvc-embedded-natvis.rs ... ignored
test [debuginfo-gdb] src/test/debuginfo/msvc-scalarpair-params.rs ... ignored
test [debuginfo-gdb] src/test/debuginfo/lexical-scope-in-parameterless-closure.rs ... ok
test [debuginfo-gdb] src/test/debuginfo/lexical-scope-in-for-loop.rs ... ok
test [debuginfo-gdb] src/test/debuginfo/lexical-scope-in-unique-closure.rs ... ok
---
[RUSTC-TIMING] rustc_monomorphize test:false 1.258
 Documenting rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
[RUSTC-TIMING] rustc_mir_dataflow test:false 2.231
[RUSTC-TIMING] rustc_passes test:false 2.433
error: unresolved link to `debugger_visualizer`
 --> compiler/rustc_passes/src/debugger_visualizer.rs:1:30
  |
1 | //! Detecting usage of the #[debugger_visualizer] attribute.
  |                              ^^^^^^^^^^^^^^^^^^^ no item named `debugger_visualizer` in scope
  |
  = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `rustc_passes`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_passes compiler/rustc_passes/src/lib.rs --target aarch64-unknown-linux-gnu -o /checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=3639a27829aa0df1 -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/release/deps --extern rustc_ast=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_ast-a7232b1e4dff8ac2.rmeta --extern rustc_ast_pretty=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_ast_pretty-7940cdfdcc069c08.rmeta --extern rustc_attr=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_attr-9d9a8126f5de7f61.rmeta --extern rustc_data_structures=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_data_structures-ad0738d119a92993.rmeta --extern rustc_errors=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_errors-b4bef933801a8392.rmeta --extern rustc_expand=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_expand-ed7eded582770397.rmeta --extern rustc_feature=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_feature-b770637e6ddbff9e.rmeta --extern rustc_hir=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_hir-a754d593681fa910.rmeta --extern rustc_index=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_index-6b35857399c720d9.rmeta --extern rustc_lexer=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_lexer-0cbb6f88205ac83b.rmeta --extern rustc_middle=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_middle-6c670a221e8dda57.rmeta --extern rustc_parse=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_parse-943814e360e4ce08.rmeta --extern rustc_serialize=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_serialize-08f2bc344da9ea53.rmeta --extern rustc_session=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_session-c4148bcdd4d75dd8.rmeta --extern rustc_span=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_span-d2c22c7def731845.rmeta --extern rustc_target=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_target-3998be5e0fdcb1f4.rmeta --extern tracing=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libtracing-016725879ecff5df.rmeta --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.29/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.62.0-nightly
  (18b60c7b8
  2022-05-04)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_metadata test:false 3.494
[RUSTC-TIMING] rustc_infer test:false 6.433
[RUSTC-TIMING] rustc_query_impl test:false 9.538
Build completed unsuccessfully in 0:41:17
