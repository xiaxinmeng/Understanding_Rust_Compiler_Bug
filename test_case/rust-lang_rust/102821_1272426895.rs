plain
[RUSTC-TIMING] rustc_attr test:false 0.644
[RUSTC-TIMING] rustc_query_system test:false 1.327
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
 Documenting rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error: unclosed HTML tag `Diagnostic`
   |
   |
42 | /// A variant of 'panictry!' that works on a Vec<Diagnostic> instead of a single DiagnosticBuilder.
   |
   |
   = note: `-D rustdoc::invalid-html-tags` implied by `-D warnings`
help: try marking as source code
   |
42 | /// A variant of 'panictry!' that works on a `Vec<Diagnostic>` instead of a single DiagnosticBuilder.

error: could not document `rustc_parse`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_parse compiler/rustc_parse/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=0958a7cc7c087564 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-5bc17e3c8b9a21c5.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-ff524e956e158d99.rmeta --extern rustc_ast_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_pretty-73767f95a42fb461.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-1197f164b4ef6a83.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-7af6a2df0927dd16.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-970fc8c188a71539.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-7892113841a2a85c.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-d192b58067fba6ea.so --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-ec5984e8a4ac8b84.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-1febe9b9f0d27335.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-19bdec09876abbb4.rmeta --extern unicode_normalization=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libunicode_normalization-fc9cc15d1ebd791c.rmeta --extern unicode_width=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libunicode_width-50d49e50584d2a2c.rmeta --extern-html-root-url 'bitflags=https://docs.rs/bitflags/1.3.2/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' --extern-html-root-url 'unicode_normalization=https://docs.rs/unicode-normalization/0.1.22/' --extern-html-root-url 'unicode_width=https://docs.rs/unicode-width/0.1.10/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.66.0
  (ccd1edf04
  2022-10-09)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_parse test:false 4.013
[RUSTC-TIMING] rustc_parse test:false 4.013
error: unclosed HTML tag `INIT`
    |
    |
206 |         /// `let pat: ty = <INIT> else { <ELSE> }
    |
    |
    = note: `-D rustdoc::invalid-html-tags` implied by `-D warnings`

error: unclosed HTML tag `ELSE`
    |
    |
206 |         /// `let pat: ty = <INIT> else { <ELSE> }


error: unclosed HTML tag `T`
   --> compiler/rustc_middle/src/ty/adt.rs:335:38
    |
335 |     /// Returns `true` if this is Box<T>.
    |
help: try marking as source code
    |
    |
335 |     /// Returns `true` if this is `Box<T>`.


error: unclosed HTML tag `T`
   --> compiler/rustc_middle/src/ty/adt.rs:341:45
    |
341 |     /// Returns `true` if this is UnsafeCell<T>.
    |
help: try marking as source code
    |
    |
341 |     /// Returns `true` if this is `UnsafeCell<T>`.

error: could not document `rustc_middle`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_middle compiler/rustc_middle/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature, "rustc-rayon", "rustc-rayon-core", "rustc_use_parallel_compiler")' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=c6ef9c9428ed8410 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-5bc17e3c8b9a21c5.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-161af96d436c9f7d.rmeta --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-52375d5ba45e16cc.rmeta --extern gsgdt=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libgsgdt-19e7f17309dc1589.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-f6e9bbd3c7667bcb.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-2d7f6aa1a9595b3d.rmeta --extern rustc_arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_arena-c4ead2cf97eeccbc.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-ff524e956e158d99.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-c463ee419cef8205.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-1197f164b4ef6a83.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-7af6a2df0927dd16.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-970fc8c188a71539.rmeta --extern rustc_graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_graphviz-2510b5ee85d3dbab.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-558480e2583569bb.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-d91dbe02f5667352.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-d192b58067fba6ea.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-18de928b702adbb4.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-be0d8e3bdcdb7b02.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-ec5984e8a4ac8b84.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-1febe9b9f0d27335.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-c2ed24354e1057e6.rmeta --extern rustc_type_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_type_ir-f09d19418b8ca3ed.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-479463b75ca28467.rmeta --extern thin_vec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libthin_vec-6504724a248bec32.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-19bdec09876abbb4.rmeta --extern-html-root-url 'bitflags=https://docs.rs/bitflags/1.3.2/' --extern-html-root-url 'chalk_ir=https://docs.rs/chalk-ir/0.80.0/' --extern-html-root-url 'either=https://docs.rs/either/1.6.0/' --extern-html-root-url 'gsgdt=https://docs.rs/gsgdt/0.1.2/' --extern-html-root-url 'polonius_engine=https://docs.rs/polonius-engine/0.13.0/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.8.1/' --extern-html-root-url 'thin_vec=https://docs.rs/thin-vec/0.2.8/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.66.0
  (ccd1edf04
  2022-10-09)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
Build completed unsuccessfully in 0:30:49
