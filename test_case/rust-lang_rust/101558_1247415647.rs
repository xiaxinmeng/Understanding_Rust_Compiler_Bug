plain
[RUSTC-TIMING] rustc_lint_defs test:false 0.350
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
 Documenting rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
[RUSTC-TIMING] rustc_hir_pretty test:false 0.408
error: `Diagnostic` is both a struct and a derive macro
    |
    |
440 |     /// Expected [`Diagnostic`]s store a [`LintExpectationId`] as part of
    |                    ^^^^^^^^^^ ambiguous link
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
help: to link to the struct, prefix with `struct@`
    |
440 |     /// Expected [`struct@Diagnostic`]s store a [`LintExpectationId`] as part of
    |                    +++++++
help: to link to the derive macro, prefix with `derive@`
    |
440 |     /// Expected [`derive@Diagnostic`]s store a [`LintExpectationId`] as part of


error: `Diagnostic` is both a struct and a derive macro
    |
    |
444 |     /// replaced by a stable [`LintExpectationId`]. The [`Diagnostic`]s are the
    |                                                           ^^^^^^^^^^ ambiguous link
    |
help: to link to the struct, prefix with `struct@`
    |
444 |     /// replaced by a stable [`LintExpectationId`]. The [`struct@Diagnostic`]s are the
    |                                                           +++++++
help: to link to the derive macro, prefix with `derive@`
    |
444 |     /// replaced by a stable [`LintExpectationId`]. The [`derive@Diagnostic`]s are the

error: could not document `rustc_errors`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_errors compiler/rustc_errors/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=6929ef0939dc9a60 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern annotate_snippets=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libannotate_snippets-f5a2b06e341babbe.rmeta --extern atty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libatty-3f60af9ffd31d27b.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3fef9cc98f55f4c7.rmeta --extern rustc_error_messages=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_error_messages-3334f0a245401058.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-92c26aab30527b5c.rmeta --extern rustc_lint_defs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint_defs-126d2e485c9c4ff8.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-312220db1c4e5ee9.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-341cb7aea971ef26.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-3643dec4afe5173c.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-84ef2235447599ec.rmeta --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libserde-8a4772ee0e83914b.rmeta --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libserde_json-0b3276397c0f8403.rmeta --extern termcolor=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtermcolor-cde2cbe97738009c.rmeta --extern termize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtermize-25c196428b9517cb.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-4669a793b23f1f25.rmeta --extern unicode_width=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libunicode_width-7245566a99a8843b.rmeta --extern-html-root-url 'annotate_snippets=https://docs.rs/annotate-snippets/0.9.1/' --extern-html-root-url 'atty=https://docs.rs/atty/0.2.14/' --extern-html-root-url 'serde=https://docs.rs/serde/1.0.143/' --extern-html-root-url 'serde_json=https://docs.rs/serde_json/1.0.83/' --extern-html-root-url 'termcolor=https://docs.rs/termcolor/1.1.2/' --extern-html-root-url 'termize=https://docs.rs/termize/0.1.1/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' --extern-html-root-url 'unicode_width=https://docs.rs/unicode-width/0.1.8/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.65.0
  (09276acc0
  2022-09-14)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_errors test:false 1.376
Build completed unsuccessfully in 0:20:54
