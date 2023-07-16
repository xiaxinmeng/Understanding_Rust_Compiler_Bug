plain
 Documenting rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: unresolved link to `2`
  --> compiler/rustc_borrowck/src/places_conflict.rs:10:19
   |
10 | //! BORROW:  (*x1[2].y).z.a
   |                   ^ no item named `2` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: unresolved link to `i`
  --> compiler/rustc_borrowck/src/places_conflict.rs:11:19
   |
   |
11 | //! ACCESS:  (*x1[i].y).w.b
   |                   ^ no item named `i` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `2`
  --> compiler/rustc_borrowck/src/places_conflict.rs:15:14
   |
   |
15 | //!       x1[2]      |   x1[i]       -- equal or disjoint (disjoint if indexes differ)
   |              ^ no item named `2` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `i`
  --> compiler/rustc_borrowck/src/places_conflict.rs:15:29
   |
   |
15 | //!       x1[2]      |   x1[i]       -- equal or disjoint (disjoint if indexes differ)
   |                             ^ no item named `i` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `2`
  --> compiler/rustc_borrowck/src/places_conflict.rs:16:14
   |
   |
16 | //!       x1[2].y    |   x1[i].y     -- equal or disjoint
   |              ^ no item named `2` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `i`
  --> compiler/rustc_borrowck/src/places_conflict.rs:16:29
   |
   |
16 | //!       x1[2].y    |   x1[i].y     -- equal or disjoint
   |                             ^ no item named `i` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `2`
  --> compiler/rustc_borrowck/src/places_conflict.rs:17:14
   |
   |
17 | //!      *x1[2].y    |  *x1[i].y     -- equal or disjoint
   |              ^ no item named `2` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `i`
  --> compiler/rustc_borrowck/src/places_conflict.rs:17:29
   |
   |
17 | //!      *x1[2].y    |  *x1[i].y     -- equal or disjoint
   |                             ^ no item named `i` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `2`
  --> compiler/rustc_borrowck/src/places_conflict.rs:18:14
   |
   |
18 | //!     (*x1[2].y).z | (*x1[i].y).w  -- we are disjoint and don't need to check more!
   |              ^ no item named `2` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `i`
  --> compiler/rustc_borrowck/src/places_conflict.rs:18:29
   |
   |
18 | //!     (*x1[2].y).z | (*x1[i].y).w  -- we are disjoint and don't need to check more!
   |                             ^ no item named `i` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `2`
  --> compiler/rustc_borrowck/src/places_conflict.rs:23:19
   |
   |
23 | //! BORROW:  (*x1[2].y).z.a
   |                   ^ no item named `2` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `i`
  --> compiler/rustc_borrowck/src/places_conflict.rs:24:17
   |
   |
24 | //! ACCESS:  x1[i].y
   |                 ^ no item named `i` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `2`
  --> compiler/rustc_borrowck/src/places_conflict.rs:28:14
   |
   |
28 | //!       x1[2]      |   x1[i]       -- equal or disjoint (disjoint if indexes differ)
   |              ^ no item named `2` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `i`
  --> compiler/rustc_borrowck/src/places_conflict.rs:28:29
   |
   |
28 | //!       x1[2]      |   x1[i]       -- equal or disjoint (disjoint if indexes differ)
   |                             ^ no item named `i` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `2`
  --> compiler/rustc_borrowck/src/places_conflict.rs:29:14
   |
   |
29 | //!       x1[2].y    |   x1[i].y     -- equal or disjoint
   |              ^ no item named `2` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `i`
  --> compiler/rustc_borrowck/src/places_conflict.rs:29:29
   |
   |
29 | //!       x1[2].y    |   x1[i].y     -- equal or disjoint
   |                             ^ no item named `i` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not parse code block as Rust code
  --> compiler/rustc_borrowck/src/places_conflict.rs:35:9
   |
   |
35 |   //!       x1[2].y    | (*x1[i].y)    -- a deref! the access can't get past this, so we
36 | | //!                                     are disjoint
   | |____________________________________________________^
   |
   |
   = note: error from rustc: prefix `can` is unknown
   = note: `-D rustdoc::invalid-rust-codeblocks` implied by `-D warnings`
error: could not document `rustc_borrowck`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_borrowck compiler/rustc_borrowck/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=1714b094337771d1 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-d57b38b997967b78.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libitertools-ad1fa586eb45f42f.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-6b33756c543d2a6f.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-c29478f64130f4d1.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-5a884c0fbab204f7.rmeta --extern rustc_fluent_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_fluent_macro-d13f249698924795.so --extern rustc_graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_graphviz-6fa933368128c19c.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-7f4691e632e9f80a.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-fa4f8f1e19a83493.rmeta --extern rustc_infer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_infer-44c96499e2cc71c2.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-cceae88582b6da09.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-46578a62e90ab7e8.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-7ed5aad67adfca0c.rmeta --extern rustc_mir_dataflow=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir_dataflow-7860bbf2c4bbcb56.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-53d6c6a2b68c2078.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-73b0f35a5e208c18.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-a3a62e175a7e014e.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-c3f55012650ae6f1.rmeta --extern rustc_trait_selection=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_trait_selection-cd5b5f2ab90b4be0.rmeta --extern rustc_traits=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_traits-7b06d4596e253d7c.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-4d2ed8dbf5f2b6bc.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-98bd5c8b4fde9d1e.rmeta --extern-html-root-url 'either=https://docs.rs/either/1.6.0/' --extern-html-root-url 'itertools=https://docs.rs/itertools/0.10.5/' --extern-html-root-url 'polonius_engine=https://docs.rs/polonius-engine/0.13.0/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.10.0/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options --cfg=bootstrap --cfg=windows_raw_dylib -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' '--check-cfg=values(emulate_second_only_system)' '--check-cfg=values(windows_raw_dylib)' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.71.0-nightly
  (4b3995f39
  2023-05-19)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition '-Zcrate-attr=warn(rust_2018_idioms)' --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
Build completed unsuccessfully in 0:00:37
