plain
[RUSTC-TIMING] rustc_hir_pretty test:false 0.511
error: unresolved link to `non_exhaustive`
    --> compiler/rustc_lint_defs/src/lib.rs:610:11
     |
610  |           $(#[$attr])*
     |
    ::: compiler/rustc_lint_defs/src/builtin.rs:3135:1
     |
     |
3135 | / declare_lint! {
3136 | |     /// The `repr_transparent_external_private_fields` lint
3137 | |     /// detects types marked #[repr(trasparent)] that (transitively)
3138 | |     /// contain an external ZST type marked #[non_exhaustive]
3182 | |     };
3183 | | }
     | |_- in this macro invocation
     |
     |
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
     = note: the link appears in this line:
             
             contain an external ZST type marked #[non_exhaustive]
     = note: no item named `non_exhaustive` in scope
     = note: no item named `non_exhaustive` in scope
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
     = note: this error originates in the macro `declare_lint` (in Nightly builds, run with -Z macro-backtrace for more info)
error: could not document `rustc_lint_defs`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_lint_defs compiler/rustc_lint_defs/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=2f005f6702b58fdd -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-7cc76f146040d077.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-efa696f6b84934b3.rmeta --extern rustc_error_messages=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_error_messages-a1eb5356127b99bc.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-38fbd0dc7655d693.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-19624a897b1135dd.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-0cad147a81a297ce.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-bccf670d3ad2026f.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a0ec6d36e31710e2.rmeta --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libserde-ffa37356025839ef.rmeta --extern-html-root-url 'serde=https://docs.rs/serde/1.0.125/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.64.0
  (75d51638b
  2022-07-11)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_errors test:false 1.724
Build completed unsuccessfully in 0:29:34
