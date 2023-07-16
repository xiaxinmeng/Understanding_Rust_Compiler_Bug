plain
[RUSTC-TIMING] askama_derive test:false 1.207
    Checking askama v0.11.0
[RUSTC-TIMING] askama test:false 0.057
 Documenting rustdoc v0.0.0 (/checkout/src/librustdoc)
error: this URL is not a hyperlink
  --> src/librustdoc/json/import_finder.rs:14:9
14 | /// See https://github.com/rust-lang/rust/issues/100973 and
14 | /// See https://github.com/rust-lang/rust/issues/100973 and
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://github.com/rust-lang/rust/issues/100973>`
   |
   = note: `-D rustdoc::bare-urls` implied by `-D warnings`
   = note: bare URLs are not automatically turned into clickable links

error: this URL is not a hyperlink
  --> src/librustdoc/json/import_finder.rs:15:5
15 | /// https://github.com/rust-lang/rust/issues/101103 for times when this
15 | /// https://github.com/rust-lang/rust/issues/101103 for times when this
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://github.com/rust-lang/rust/issues/101103>`
   |
   = note: bare URLs are not automatically turned into clickable links
error: could not document `rustdoc`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustdoc src/librustdoc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature, "jemalloc")' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=d2bb835f568a7e90 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern arrayvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libarrayvec-ccba4aeb2ffcbfa5.rmeta --extern askama=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaskama-f4acc443a6667f4d.rmeta --extern atty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libatty-d3f9328515095276.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-714d2929cadc9eb9.rmeta --extern minifier=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-3aa0d6925a6aa374.rmeta --extern once_cell=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libonce_cell-cb44ef5ba3bd8a88.rmeta --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-be37c58e2e26eb19.rmeta --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-08e62cf91c6950e8.rmeta --extern rustdoc_json_types=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustdoc_json_types-c6f939367321b314.rmeta --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f617563fcc1c978a.rmeta --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-41130dc8c7445473.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-a9233ac415d71d85.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-879fb5a283714808.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-d7e27429edb87106.rmeta --extern tracing_subscriber=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_subscriber-9b0e823d11782cc5.rmeta --extern tracing_tree=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_tree-2df2574468d7f92e.rmeta -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(release)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.65.0
  2022-08-28)' --document-private-items --enable-index-page --show-type-layout --generate-link-to-definition -Zunstable-options` (exit status: 1)
Build completed unsuccessfully in 0:27:24
