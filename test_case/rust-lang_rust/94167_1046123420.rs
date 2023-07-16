plain
 Documenting rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: unresolved link to `CFG`
   --> compiler/rustc_mir_transform/src/coverage/graph.rs:284:5
    |
284 |     /// A node in the [control-flow graph][CFG] of CoverageGraph.
    |
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = note: the link appears in this line:
            
            A node in the [control-flow graph][CFG] of CoverageGraph.
    = note: no item named `CFG` in scope
    = note: no item named `CFG` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `rustc_mir_transform`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_mir_transform compiler/rustc_mir_transform/src/lib.rs --target aarch64-unknown-linux-gnu -o /checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,future-incompat -C metadata=6ac58f764e91fefc -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/release/deps --extern itertools=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libitertools-953774d85bdeb50c.rmeta --extern rustc_ast=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_ast-0340e6623b1cf5e7.rmeta --extern rustc_attr=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_attr-0f02c7214e9a5ddc.rmeta --extern rustc_const_eval=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_const_eval-031a374aabc3c0dc.rmeta --extern rustc_data_structures=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_data_structures-a280f5a2980846e5.rmeta --extern rustc_errors=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_errors-cffacf2856b5995c.rmeta --extern rustc_hir=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_hir-e3003471921131e8.rmeta --extern rustc_index=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_index-b9a2ddb97a140439.rmeta --extern rustc_middle=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_middle-432f046369c7f9a3.rmeta --extern rustc_mir_dataflow=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_mir_dataflow-de027074fe12a85e.rmeta --extern rustc_query_system=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_query_system-12dd08cd24740a2f.rmeta --extern rustc_serialize=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_serialize-8e7a876d3e4cdc58.rmeta --extern rustc_session=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_session-e0521499c4b36fce.rmeta --extern rustc_span=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_span-d8395221facea914.rmeta --extern rustc_target=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_target-4a8a159b90580fce.rmeta --extern rustc_trait_selection=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_trait_selection-7d87b6152b966c29.rmeta --extern smallvec=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libsmallvec-880cb351865c273d.rmeta --extern tracing=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libtracing-62a879f7b3b5483c.rmeta --extern-html-root-url 'itertools=https://docs.rs/itertools/0.10.1/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.7.0/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.29/' -Zunstable-options -Csymbol-mangling-version=v0 -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.61.0-nightly
  (e1848123f
  2022-02-19)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_mir_transform test:false 5.183
[RUSTC-TIMING] rustc_borrowck test:false 7.690
[RUSTC-TIMING] rustc_typeck test:false 15.879
error: build failed
