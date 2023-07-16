plain
[RUSTC-TIMING] rustc_expand test:false 1.876
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
 Documenting rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
[RUSTC-TIMING] rustc_builtin_macros test:false 1.819
error: unresolved link to `wrapper`
     |
     |
1844 | /     /// A [newtype'd][wrapper] index type in the MIR [control-flow graph][CFG]
1845 | |     ///
1846 | |     /// A field (e.g., `f` in `_1.f`) is one variant of [`ProjectionElem`]. Conceptually,
1847 | |     /// rustc can identify that a field projection refers to either two different regions of memory
...    |
1851 | |     /// [CFG]: https://rustc-dev-guide.rust-lang.org/appendix/background.html#cfg
1852 | |     /// [mir-datatypes]: https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types
     |
     |
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
     = note: the link appears in this line:
             
             A [newtype'd][wrapper] index type in the MIR [control-flow graph][CFG]
     = note: no item named `wrapper` in scope
     = note: no item named `wrapper` in scope
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `CFG`
    --> compiler/rustc_middle/src/mir/mod.rs:1844:5
     |
     |
1844 | /     /// A [newtype'd][wrapper] index type in the MIR [control-flow graph][CFG]
1845 | |     ///
1846 | |     /// A field (e.g., `f` in `_1.f`) is one variant of [`ProjectionElem`]. Conceptually,
1847 | |     /// rustc can identify that a field projection refers to either two different regions of memory
...    |
1851 | |     /// [CFG]: https://rustc-dev-guide.rust-lang.org/appendix/background.html#cfg
1852 | |     /// [mir-datatypes]: https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types
     |
     = note: the link appears in this line:
             
             
             A [newtype'd][wrapper] index type in the MIR [control-flow graph][CFG]
     = note: no item named `CFG` in scope
     = note: no item named `CFG` in scope
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: this URL is not a hyperlink
     |
     |
1844 | /     /// A [newtype'd][wrapper] index type in the MIR [control-flow graph][CFG]
1845 | |     ///
1846 | |     /// A field (e.g., `f` in `_1.f`) is one variant of [`ProjectionElem`]. Conceptually,
1847 | |     /// rustc can identify that a field projection refers to either two different regions of memory
...    |
1851 | |     /// [CFG]: https://rustc-dev-guide.rust-lang.org/appendix/background.html#cfg
1852 | |     /// [mir-datatypes]: https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types
     | |____________________________________________________________________________________________^ help: use an automatic link instead: `<https://rustc-dev-guide.rust-lang.org/appendix/glossary.html#newtype>`
     |
     = note: `-D rustdoc::bare-urls` implied by `-D warnings`
     = note: bare URLs are not automatically turned into clickable links

error: this URL is not a hyperlink
     |
     |
1844 | /     /// A [newtype'd][wrapper] index type in the MIR [control-flow graph][CFG]
1845 | |     ///
1846 | |     /// A field (e.g., `f` in `_1.f`) is one variant of [`ProjectionElem`]. Conceptually,
1847 | |     /// rustc can identify that a field projection refers to either two different regions of memory
...    |
1851 | |     /// [CFG]: https://rustc-dev-guide.rust-lang.org/appendix/background.html#cfg
1852 | |     /// [mir-datatypes]: https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types
     | |____________________________________________________________________________________________^ help: use an automatic link instead: `<https://rustc-dev-guide.rust-lang.org/appendix/background.html#cfg>`
     |
     = note: bare URLs are not automatically turned into clickable links

error: this URL is not a hyperlink
     |
     |
1844 | /     /// A [newtype'd][wrapper] index type in the MIR [control-flow graph][CFG]
1845 | |     ///
1846 | |     /// A field (e.g., `f` in `_1.f`) is one variant of [`ProjectionElem`]. Conceptually,
1847 | |     /// rustc can identify that a field projection refers to either two different regions of memory
...    |
1851 | |     /// [CFG]: https://rustc-dev-guide.rust-lang.org/appendix/background.html#cfg
1852 | |     /// [mir-datatypes]: https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types
     | |____________________________________________________________________________________________^ help: use an automatic link instead: `<https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types>`
     |
     = note: bare URLs are not automatically turned into clickable links
error: could not document `rustc_middle`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_middle compiler/rustc_middle/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,future-incompat -C metadata=1b73d1ddb2dc71c9 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-17521ded70cd206c.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-2e112d3b4bd3a943.rmeta --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-5c06d7d1a4a7c2ac.rmeta --extern gsgdt=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libgsgdt-ca234ec367d1272e.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-ddd207219af52877.rmeta --extern rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librand-cf1485162b26a25a.rmeta --extern rand_xoshiro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librand_xoshiro-4c5698e56ae2b836.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-13636ceca7dce130.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-14a7223e4d89800c.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-234297eac9bfc556.rmeta --extern rustc_arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_arena-2641742aea1db5aa.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-6bb237822b00488f.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-289b18167731059d.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a86d4359906a94a8.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e56e7cbc8ae9db50.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-8785cc43b8977e56.rmeta --extern rustc_graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_graphviz-802a340a7a993eee.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-41674eea628ce851.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-b2be072e02e654c1.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-ba86bb43634aaa81.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-b8db69b00aeeec18.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-bbea91bef2fd8c04.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-4fb56829f368d330.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-e31ca7c57949b75c.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-325d08b4a94d7190.rmeta --extern rustc_type_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_type_ir-6fdd790f76ddcc49.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-66481e7c74444ad9.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-f9b89a91db3bd4be.rmeta --extern-html-root-url 'bitflags=https://docs.rs/bitflags/1.2.1/' --extern-html-root-url 'chalk_ir=https://docs.rs/chalk-ir/0.76.0/' --extern-html-root-url 'either=https://docs.rs/either/1.6.0/' --extern-html-root-url 'gsgdt=https://docs.rs/gsgdt/0.1.2/' --extern-html-root-url 'polonius_engine=https://docs.rs/polonius-engine/0.13.0/' --extern-html-root-url 'rand=https://docs.rs/rand/0.8.4/' --extern-html-root-url 'rand_xoshiro=https://docs.rs/rand_xoshiro/0.6.0/' --extern-html-root-url 'rustc_rayon=https://docs.rs/rustc-rayon/0.3.2/' --extern-html-root-url 'rustc_rayon_core=https://docs.rs/rustc-rayon-core/0.3.2/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.7.0/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.29/' -Zunstable-options -Csymbol-mangling-version=v0 -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.60.0
  (74f494b90
  2022-02-19)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_middle test:false 25.575
error: build failed
Build completed unsuccessfully in 0:38:01
