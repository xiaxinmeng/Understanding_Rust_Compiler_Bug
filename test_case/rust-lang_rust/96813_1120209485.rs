plain
[RUSTC-TIMING] rustc_hir_pretty test:false 0.470
error: unresolved link to `unstable_expect_diagnostics`
   --> compiler/rustc_errors/src/lib.rs:429:15
    |
429 |     /// The [`unstable_expect_diagnostics`] should be empty when this struct is
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `unstable_expect_diagnostics` in scope
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `unstable_expect_diagnostics`
   --> compiler/rustc_errors/src/lib.rs:432:20
    |
    |
432 |     /// check if [`unstable_expect_diagnostics`] is empty, if the expectation ids
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `unstable_expect_diagnostics` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `rustc_errors`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_errors compiler/rustc_errors/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=88ffe50e24515a02 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern annotate_snippets=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libannotate_snippets-585631eaa61c91fd.rmeta --extern atty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libatty-4b841e53df2d393d.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a196ff7ad209c777.rmeta --extern rustc_error_messages=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_error_messages-1484af816b3a4a33.rmeta --extern rustc_lint_defs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint_defs-06dac9b5edabea0e.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-2fcf84fe61976a37.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-f57cf13be9a3d814.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-5b8d910a52afbae0.rmeta --extern termcolor=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtermcolor-fc1a8388ab8165c6.rmeta --extern termize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtermize-228e37007101642e.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-40f2c02ee42fe18f.rmeta --extern unicode_width=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libunicode_width-8bf457a793c7c502.rmeta --extern-html-root-url 'annotate_snippets=https://docs.rs/annotate-snippets/0.8.0/' --extern-html-root-url 'atty=https://docs.rs/atty/0.2.14/' --extern-html-root-url 'termcolor=https://docs.rs/termcolor/1.1.2/' --extern-html-root-url 'termize=https://docs.rs/termize/0.1.1/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.29/' --extern-html-root-url 'unicode_width=https://docs.rs/unicode-width/0.1.8/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.62.0-nightly
  (020349e62
  2022-05-07)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_errors test:false 1.541
Build completed unsuccessfully in 0:33:36
