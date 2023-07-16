plain
 Documenting rustc_target v0.0.0 (/checkout/compiler/rustc_target)
[RUSTC-TIMING] rustc_parse_format test:false 0.262
[RUSTC-TIMING] rustc_error_messages test:false 0.358
[RUSTC-TIMING] rustc_feature test:false 0.682
error: this URL is not a hyperlink
    |
    |
217 |     /// https://projectfluent.org/fluent/guide/hello.html
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://projectfluent.org/fluent/guide/hello.html>`
    |
    = note: `-D rustdoc::bare-urls` implied by `-D warnings`
    = note: bare URLs are not automatically turned into clickable links

error: this URL is not a hyperlink
    |
    |
218 |     /// https://projectfluent.org/fluent/guide/attributes.html
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://projectfluent.org/fluent/guide/attributes.html>`
    |
    = note: bare URLs are not automatically turned into clickable links
error: could not document `rustc_error_messages`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_error_messages compiler/rustc_error_messages/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=cccf00d7f26fe0ad -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern fluent_bundle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libfluent_bundle-472c5558645c5dd1.rmeta --extern fluent_syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libfluent_syntax-cd9e49d9b369ded0.rmeta --extern intl_memoizer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libintl_memoizer-4d301112e63e53e2.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a9b0afc859ec9545.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-082dcfbb56fe7ee4.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-8bbb02056853a4d1.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-8b8c99505d04c188.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-b96aad48c6084dcb.rmeta --extern unic_langid=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libunic_langid-98bacb8e6de2a602.rmeta --extern-html-root-url 'fluent_bundle=https://docs.rs/fluent-bundle/0.15.2/' --extern-html-root-url 'fluent_syntax=https://docs.rs/fluent-syntax/0.11.0/' --extern-html-root-url 'intl_memoizer=https://docs.rs/intl-memoizer/0.5.1/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.29/' --extern-html-root-url 'unic_langid=https://docs.rs/unic-langid/0.9.0/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.61.0-nightly
  (4c269a7b4
  2022-04-03)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] chalk_engine test:false 1.415
[RUSTC-TIMING] serde test:false 5.968
[RUSTC-TIMING] rustc_ast test:false 5.339
[RUSTC-TIMING] rustc_target test:false 5.554
