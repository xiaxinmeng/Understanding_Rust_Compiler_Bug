plain
test [ui] src/test/ui/rfc-2497-if-let-chains/issue-93150.rs ... ok
test [ui] src/test/ui/rfc-2091-track-caller/std-panic-locations.rs#mir-opt ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/no-double-assigments.rs ... ok
test [ui] src/test/ui/rfc-2126-extern-absolute-paths/single-segment.rs ... ok
test [ui] src/test/ui/rfc-2627-raw-dylib/import-name-type-invalid-format.rs ... ignored
test [ui] src/test/ui/rfc-2627-raw-dylib/import-name-type-multiple.rs ... ignored
test [ui] src/test/ui/rfc-2627-raw-dylib/import-name-type-unknown-value.rs ... ignored
test [ui] src/test/ui/rfc-2627-raw-dylib/import-name-type-unsupported-link-kind.rs ... ignored
test [ui] src/test/ui/rfc-2627-raw-dylib/import-name-type-x86-only.rs ... ignored
test [ui] src/test/ui/rfc-2565-param-attrs/param-attrs-2018.rs ... ok
test [ui] src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs ... ok
test [ui] src/test/ui/rfc-2627-raw-dylib/link-ordinal-multiple.rs ... ignored
test [ui] src/test/ui/rfc-2565-param-attrs/param-attrs-cfg.rs ... ok
---
[RUSTC-TIMING] rustc_hir_pretty test:false 0.553
[RUSTC-TIMING] rustc_errors test:false 1.751
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
 Documenting rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error: this URL is not a hyperlink
  --> compiler/rustc_session/src/cstore.rs:85:10
   |
85 | /// From https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#import-name-type
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#import-name-type>`
   |
   = note: `-D rustdoc::bare-urls` implied by `-D warnings`
   = note: bare URLs are not automatically turned into clickable links
error: could not document `rustc_session`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_session compiler/rustc_session/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=62d3437117abb2b7 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern getopts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libgetopts-2ff1602040d63687.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-be6ec747bbca83d8.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-c150b6a753802082.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-be7d8b07df29cfc7.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-0940d46b1338747a.rmeta --extern rustc_fs_util=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_fs_util-b506e9eda9ffa392.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-652f8f0320de2c50.rmeta --extern rustc_lint_defs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint_defs-58a806415c9c79cc.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-d31ad75a21ab5644.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-88771ab453f8ca4a.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-9d27021d9b49c76e.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-c5a5d26dc851c31d.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-b063042510468333.rmeta --extern-html-root-url 'getopts=https://docs.rs/getopts/0.2.21/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.29/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.65.0-nightly
  (b771848c3
  2022-08-26)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_session test:false 2.102
Build completed unsuccessfully in 0:24:19
