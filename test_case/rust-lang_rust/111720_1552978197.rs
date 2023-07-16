plain
 Documenting rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error: unresolved link to `Constructor::split`
  --> compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:40:39
   |
40 | //! Splitting is implemented in the [`Constructor::split`] function. We don't do splitting for
   |                                       ^^^^^^^^^^^^^^^^^^ the enum `Constructor` has no variant or associated item named `split`
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`

error: unresolved link to `SplitWildcard`
   |
   |
42 | //! wildcards, see [`SplitWildcard`]; for integer ranges, see [`SplitIntRange`]; for slices, see
   |                      ^^^^^^^^^^^^^ no item named `SplitWildcard` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `SplitWildcard`
    |
    |
660 |     /// for [`SplitWildcard`].
    |               ^^^^^^^^^^^^^ no item named `SplitWildcard` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `is_useful`
    |
    |
275 | //! This computation is done in [`is_useful`]. In practice we don't care about the list of
    |                                   ^^^^^^^^^ no item named `is_useful` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Constructor::split`
   --> compiler/rustc_mir_build/src/thir/pattern/usefulness.rs:290:60
    |
    |
290 | //! [`super::deconstruct_pat`]. Splitting is done by the [`Constructor::split`] function.
    |                                                            ^^^^^^^^^^^^^^^^^^ the enum `Constructor` has no variant or associated item named `split`
error: could not document `rustc_mir_build`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_mir_build compiler/rustc_mir_build/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=c63a31494c8e374b -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-d57b38b997967b78.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-1602ec632caac945.rmeta --extern rustc_arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_arena-13841b6bcde384be.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-c26dfa863d5d54a5.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-c29478f64130f4d1.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-5a884c0fbab204f7.rmeta --extern rustc_fluent_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_fluent_macro-d13f249698924795.so --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-7f4691e632e9f80a.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-fa4f8f1e19a83493.rmeta --extern rustc_infer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_infer-44c96499e2cc71c2.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-46578a62e90ab7e8.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-7ed5aad67adfca0c.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-53d6c6a2b68c2078.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-73b0f35a5e208c18.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-a3a62e175a7e014e.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-c3f55012650ae6f1.rmeta --extern rustc_trait_selection=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_trait_selection-cd5b5f2ab90b4be0.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-4d2ed8dbf5f2b6bc.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-98bd5c8b4fde9d1e.rmeta --extern-html-root-url 'either=https://docs.rs/either/1.6.0/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.10.0/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options --cfg=bootstrap --cfg=windows_raw_dylib -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' '--check-cfg=values(emulate_second_only_system)' '--check-cfg=values(windows_raw_dylib)' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.71.0-nightly
  (b68289368
  2023-05-18)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition '-Zcrate-attr=warn(rust_2018_idioms)' --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
Build completed unsuccessfully in 0:00:37
