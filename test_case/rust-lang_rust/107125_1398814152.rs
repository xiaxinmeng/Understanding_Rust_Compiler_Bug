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
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
 Documenting rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
 Documenting rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
error: unresolved link to `TraitItemKind::ExternCrate`
    --> compiler/rustc_hir/src/hir.rs:2291:21
     |
2291 |     /// Expect an [`TraitItemKind::ExternCrate`] or panic.
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^ the enum `TraitItemKind` has no variant or associated item named `ExternCrate`
     |
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`

error: unresolved link to `ImplItemKind::ExternCrate`
    --> compiler/rustc_hir/src/hir.rs:2374:21
     |
2374 |     /// Expect an [`ImplItemKind::ExternCrate`] or panic.
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^ the enum `ImplItemKind` has no variant or associated item named `ExternCrate`
error: could not document `rustc_hir`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_hir compiler/rustc_hir/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=af11b69228b13569 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern odht=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libodht-64c2d02b3069a553.rmeta --extern rustc_arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_arena-d28e9ea4cc6b7ffe.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-28a7c2a6e82b2f2e.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-49b7934135e2bb72.rmeta --extern rustc_error_messages=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_error_messages-53f53f7844e791f3.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-1fddc05bc56ef872.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-035bbf709d0035bf.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-e49f06d1076669f0.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-6b58ff6faca8e5ce.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-5f67fb198db30e09.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-809cfbb34629de00.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-762b03051998eeed.rmeta --extern-html-root-url 'odht=https://docs.rs/odht/0.3.1/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.8.1/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options --cfg=bootstrap -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.68.0-nightly
  (0dfa8f620
  2023-01-20)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
Build completed unsuccessfully in 0:00:13
