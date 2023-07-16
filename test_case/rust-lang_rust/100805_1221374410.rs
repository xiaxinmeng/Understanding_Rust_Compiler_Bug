plain
test [ui] src/test/ui/abi/mir/mir_codegen_calls_variadic.rs ... ok
test [ui] src/test/ui/abi/nullable-pointer-ffi-compat.rs ... ok
test [ui] src/test/ui/abi/foreign/invoke-external-foreign.rs ... ok
test [ui] src/test/ui/abi/numbers-arithmetic/i128-ffi.rs ... ok
test [ui] src/test/ui/abi/issues/issue-97463-broken-abi-leaked-uninit-data.rs ... ok
test [ui] src/test/ui/abi/unsupported.rs#arm ... ok
test [ui] src/test/ui/abi/foreign/foreign-dupe.rs ... ok
test [ui] src/test/ui/abi/unsupported.rs#i686 ... ok
test [ui] src/test/ui/abi/unsupported.rs#x64 ... ok
---
test [codegen] src/test/codegen/slice-iter-len-eq-zero.rs ... ok
test [codegen] src/test/codegen/simd-intrinsic/simd-intrinsic-float-powi.rs ... ok
test [codegen] src/test/codegen/simd-intrinsic/simd-intrinsic-generic-scatter.rs ... ok
test [codegen] src/test/codegen/simd-intrinsic/simd-intrinsic-generic-select.rs ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#aarch64-linux ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#aarch64-apple ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#arm ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#aarch64-windows ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#riscv ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#i686 ... ok
test [codegen] src/test/codegen/slice-ref-equality.rs ... ok
test [codegen] src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#x86_64 ... ok
test [codegen] src/test/codegen/some-global-nonnull.rs ... ok
---
 Documenting rustc_target v0.0.0 (/checkout/compiler/rustc_target)
[RUSTC-TIMING] rls_data test:false 0.922
[RUSTC-TIMING] rustc_error_messages test:false 0.479
[RUSTC-TIMING] rustc_feature test:false 0.514
error: this URL is not a hyperlink
   |
   |
15 | ...o: https://developer.apple.com/documentation/xcode/writing-arm64-code-for-apple-platforms#Pass-Arguments-to-Functions-Correctly
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://developer.apple.com/documentation/xcode/writing-arm64-code-for-apple-platforms#Pass-Arguments-to-Functions-Correctly>`
   |
   = note: `-D rustdoc::bare-urls` implied by `-D warnings`
   = note: bare URLs are not automatically turned into clickable links
error: could not document `rustc_target`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_target compiler/rustc_target/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=c8bf51d0752adb5a -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-c08d16d3f9bfbebf.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-806b043cf0c87648.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-ca47da41c14119eb.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-cc95cdafb9cb787b.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-aead69bc012483ad.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-c73f945117cabe2d.rmeta --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libserde_json-f0d32bda1fa908e3.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-3bd0a935b6e18a2f.rmeta --extern-html-root-url 'bitflags=https://docs.rs/bitflags/1.3.2/' --extern-html-root-url 'serde_json=https://docs.rs/serde_json/1.0.82/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.29/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.65.0
  (5f91df5ae
  2022-08-20)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_ast test:false 2.858
[RUSTC-TIMING] rustc_target test:false 3.890
Build completed unsuccessfully in 0:23:16
