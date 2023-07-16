plain
    Checking askama_shared v0.12.0
error: unresolved link to `HashMap`
  --> src/rustdoc-json-types/lib.rs:56:71
   |
56 |     /// [`String`] is currently `["alloc", "string", "String"]` and [`HashMap`] is
   |                                                                       ^^^^^^^ no item named `HashMap` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: could not document `rustdoc-json-types`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustdoc_json_types src/rustdoc-json-types/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=1d9268e6f09b20e4 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern rustc_hash=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_hash-5fa72dc46dd3b887.rmeta --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-ccb7310d6941ac0a.rmeta -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' '--check-cfg=values(emulate_second_only_system)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.69.0
  2023-03-04)' --document-private-items --enable-index-page --show-type-layout --generate-link-to-definition -Zunstable-options` (exit status: 1)
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustdoc_json_types test:false 3.782
[RUSTC-TIMING] askama_shared test:false 2.433
