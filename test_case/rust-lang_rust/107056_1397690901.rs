plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
 Documenting rustc_metadata v0.0.0 (/checkout/compiler/rustc_metadata)
 Documenting rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
 Documenting rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
 Documenting rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
error: unclosed HTML tag `AsyncReturn`
    |
    |
477 |     /// Creates a new Option<AsyncReturn> from an Option<(NodeId, Span)> that is typically
    |
    |
    = note: `-D rustdoc::invalid-html-tags` implied by `-D warnings`
    |
    |
477 |     /// Creates a new `Option<AsyncReturn>` from an Option<(NodeId, Span)> that is typically

error: could not document `rustc_ast_lowering`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_ast_lowering compiler/rustc_ast_lowering/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=3bdc59ab00f4bbc3 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_arena-d28e9ea4cc6b7ffe.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-28a7c2a6e82b2f2e.rmeta --extern rustc_ast_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_pretty-46e2a73f4e67f73b.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-49b7934135e2bb72.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-0811f826faa4de20.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-af11b69228b13569.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-1fddc05bc56ef872.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-035bbf709d0035bf.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-5e6dd757f9746a30.rmeta --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-992ebfefe936a0fc.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-037f79ead9409739.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-6b58ff6faca8e5ce.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-5f67fb198db30e09.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-809cfbb34629de00.rmeta --extern thin_vec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libthin_vec-6790925d5cf4225e.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-762b03051998eeed.rmeta --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.8.1/' --extern-html-root-url 'thin_vec=https://docs.rs/thin-vec/0.2.9/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options --cfg=bootstrap -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.68.0-nightly
  (1b65c2e6f
  2023-01-19)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
Build completed unsuccessfully in 0:00:36
