plain
[RUSTC-TIMING] rustc_expand test:false 1.876
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
 Documenting rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
[RUSTC-TIMING] rustc_builtin_macros test:false 1.789
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
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_middle compiler/rustc_middle/src/lib.rs --target aarch64-unknown-linux-gnu -o /checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,future-incompat -C metadata=432f046369c7f9a3 -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libbitflags-0410241138ddd382.rmeta --extern chalk_ir=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libchalk_ir-1df2501572cdf290.rmeta --extern either=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libeither-d64d7fec5c028f11.rmeta --extern gsgdt=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libgsgdt-966e0c89172d120e.rmeta --extern polonius_engine=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libpolonius_engine-2cb1212bef125339.rmeta --extern rand=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librand-1b9b15e46fc0f657.rmeta --extern rand_xoshiro=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librand_xoshiro-b5d8f9fc334e5622.rmeta --extern rustc_rayon=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_rayon-ae9fae48d9f3c2b6.rmeta --extern rustc_rayon_core=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_rayon_core-351dfab26e371dc9.rmeta --extern rustc_apfloat=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_apfloat-4334912c1dcabfdf.rmeta --extern rustc_arena=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_arena-b8a7603f170111ea.rmeta --extern rustc_ast=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_ast-0340e6623b1cf5e7.rmeta --extern rustc_attr=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_attr-0f02c7214e9a5ddc.rmeta --extern rustc_data_structures=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_data_structures-a280f5a2980846e5.rmeta --extern rustc_errors=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_errors-cffacf2856b5995c.rmeta --extern rustc_feature=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_feature-d778382a65a0ea37.rmeta --extern rustc_graphviz=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_graphviz-b50a397d5805453e.rmeta --extern rustc_hir=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_hir-e3003471921131e8.rmeta --extern rustc_index=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_index-b9a2ddb97a140439.rmeta --extern rustc_macros=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-d9a25e4a2c275e29.so --extern rustc_query_system=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_query_system-12dd08cd24740a2f.rmeta --extern rustc_serialize=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_serialize-8e7a876d3e4cdc58.rmeta --extern rustc_session=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_session-e0521499c4b36fce.rmeta --extern rustc_span=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_span-d8395221facea914.rmeta --extern rustc_target=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_target-4a8a159b90580fce.rmeta --extern rustc_type_ir=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_type_ir-5abd60916aab33b1.rmeta --extern smallvec=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libsmallvec-880cb351865c273d.rmeta --extern tracing=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libtracing-62a879f7b3b5483c.rmeta --extern-html-root-url 'bitflags=https://docs.rs/bitflags/1.2.1/' --extern-html-root-url 'chalk_ir=https://docs.rs/chalk-ir/0.76.0/' --extern-html-root-url 'either=https://docs.rs/either/1.6.0/' --extern-html-root-url 'gsgdt=https://docs.rs/gsgdt/0.1.2/' --extern-html-root-url 'polonius_engine=https://docs.rs/polonius-engine/0.13.0/' --extern-html-root-url 'rand=https://docs.rs/rand/0.8.4/' --extern-html-root-url 'rand_xoshiro=https://docs.rs/rand_xoshiro/0.6.0/' --extern-html-root-url 'rustc_rayon=https://docs.rs/rustc-rayon/0.3.2/' --extern-html-root-url 'rustc_rayon_core=https://docs.rs/rustc-rayon-core/0.3.2/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.7.0/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.29/' -Zunstable-options -Csymbol-mangling-version=v0 -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.61.0-nightly
  (0525b7fa5
  2022-02-20)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_middle test:false 26.912
error: build failed
Build completed unsuccessfully in 0:41:25
