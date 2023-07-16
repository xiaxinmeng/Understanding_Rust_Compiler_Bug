plain
test [ui] src/test/ui/pub/pub-ident-struct.rs ... ok
test [ui] src/test/ui/pub/pub-restricted-error.rs ... ok
test [ui] src/test/ui/pub/pub-restricted.rs ... ok
test [ui] src/test/ui/qualified/qualified-path-params-2.rs ... ok
test [ui] src/test/ui/query-system/fn-sig-cycle-arity.rs ... ok
test [ui] src/test/ui/query-system/issue-83479.rs ... ok
test [ui] src/test/ui/qualified/qualified-path-params.rs ... ok
test [ui] src/test/ui/range/exclusive-range-patterns-2021.rs ... ok
test [ui] src/test/ui/range/issue-54505-no-std.rs ... ok
---
test [assembly] src/test/assembly/asm/bpf-types.rs ... ok
test [assembly] src/test/assembly/asm/msp430-types.rs ... ok
test [assembly] src/test/assembly/asm/nvptx-types.rs ... ok
test [assembly] src/test/assembly/asm/avr-types.rs ... ok
test [assembly] src/test/assembly/asm/aarch64-el2vmsa.rs ... ok
test [assembly] src/test/assembly/asm/aarch64-modifiers.rs ... ok
test [assembly] src/test/assembly/asm/mips-types.rs#mips32 ... ok
test [assembly] src/test/assembly/asm/mips-types.rs#mips64 ... ok
test [assembly] src/test/assembly/align_offset.rs ... ok
---
[RUSTC-TIMING] rustc_resolve test:false 4.965
error: unresolved link to `At::trace_exp`
   --> compiler/rustc_trait_selection/src/traits/engine.rs:117:15
    |
117 |     /// See [`At::trace_exp`] and [`Trace::eq`] for a version of
    |               ^^^^^^^^^^^^^ no item named `At` in scope
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: unresolved link to `Trace::eq`
   --> compiler/rustc_trait_selection/src/traits/engine.rs:117:37
    |
    |
117 |     /// See [`At::trace_exp`] and [`Trace::eq`] for a version of
    |                                     ^^^^^^^^^ no item named `Trace` in scope
[RUSTC-TIMING] rustc_query_impl test:false 11.659
error: could not document `rustc_trait_selection`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_trait_selection compiler/rustc_trait_selection/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=1ecbef6cd6fbd598 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-37e654d48981118e.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-3c1ca7b460fa7fba.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-48cbf1b1ecc02c85.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-41b3637034d7637a.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-ab33218bc5dfe0de.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-e4df45d295066797.rmeta --extern rustc_infer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_infer-0a2ca4cc3bd71225.rmeta --extern rustc_lint_defs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint_defs-e26a2ee10826b261.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-6a1453ab4ff51aaf.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-c76eb41d5b50bd25.rmeta --extern rustc_parse_format=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_parse_format-5b12dbf479ce3891.rmeta --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-7f71e3440cf7f9bf.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-ee882ea42a0810c1.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-ecf4b20a9d04c720.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-39a08677932984fd.rmeta --extern rustc_transmute=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_transmute-11ee650a416bb0ba.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-514c0d16dcc4ec4a.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-cfbf65bd547f9394.rmeta --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.8.1/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.67.0-nightly
  (3031f187a
  2022-12-02)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_trait_selection test:false 7.634
Build completed unsuccessfully in 0:25:57
