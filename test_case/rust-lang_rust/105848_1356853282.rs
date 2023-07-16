plain
Successfully built fe1dac47528c
Successfully tagged rust-ci:latest
Built container sha256:fe1dac47528c0f15541a671116bfe9ec1b574a4de46ed46edc2219e946da0d92
Uploading finished image to https://ci-caches.rust-lang.org/docker/55db90084233c2c6d3f2ddcc4c9f8b7997aecabd635a628a2c66bcbdc007ca9a5c55a9b5bd41c20a325974553b8aaefb2756d07d1f18bc6caea3814fbff20c8c
upload failed: - to s3://rust-lang-ci-sccache2/docker/55db90084233c2c6d3f2ddcc4c9f8b7997aecabd635a628a2c66bcbdc007ca9a5c55a9b5bd41c20a325974553b8aaefb2756d07d1f18bc6caea3814fbff20c8c Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
 Documenting rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
error: unescaped backtick
  --> compiler/rustc_incremental/src/assert_module_sources.rs:21:5
   |
21 | //! `#![rustc_expected_cgu_reuse(module="spike", cfg="rpass2", kind="post-lto")]
   |
   |
   = note: `-D rustdoc::unescaped-backtick` implied by `-D warnings`
help: the closing backtick of an inline code may be missing
   |
21 | //! `#![rustc_expected_cgu_reuse(module="spike", cfg="rpass2", kind="post-lto")]`
   |                                                                                 +
help: a previous inline code might be longer than expected
   |
17 | //! The reason that we use `cfg=`...` and not `#[cfg_attr]` is so that
   |                                 +
help: if you meant to use a literal backtick, escape it
   |
21 | //! \`#![rustc_expected_cgu_reuse(module="spike", cfg="rpass2", kind="post-lto")]

error: could not document `rustc_incremental`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_incremental compiler/rustc_incremental/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=add2b14dbce4f41c -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librand-13fcb7ef3c6da78b.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-675636cef14c3f51.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-9d7dd652af2817a0.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-87889d082997933e.rmeta --extern rustc_fs_util=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_fs_util-b506e9eda9ffa392.rmeta --extern rustc_graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_graphviz-0154684f1f480fec.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-ea1118fcace03183.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-6a1453ab4ff51aaf.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-2642b864a82701c1.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-d7a734a6b7ad6062.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-82571e40130f3c80.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-c3d95f977d2916bc.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-cfbf65bd547f9394.rmeta --extern-html-root-url 'rand=https://docs.rs/rand/0.8.5/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.68.0-nightly
  (9b19c3505
  2022-12-18)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
error: unescaped backtick
error: unescaped backtick
  --> compiler/rustc_passes/src/liveness/rwu_table.rs:12:8
   |
12 | /// RWU`s can get very large, so it uses a more compact representation.
   |
   |
   = note: `-D rustdoc::unescaped-backtick` implied by `-D warnings`
help: the closing backtick of an inline code may be missing
   |
12 | /// RWU`s` can get very large, so it uses a more compact representation.
   |          +
help: if you meant to use a literal backtick, escape it
   |
12 | /// RWU\`s can get very large, so it uses a more compact representation.

error: could not document `rustc_passes`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_passes compiler/rustc_passes/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=8ac95926ab1ef6ad -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libitertools-7a3416b68222595d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-675636cef14c3f51.rmeta --extern rustc_ast_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_pretty-19d39d9146d7478f.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-af4435ff15462416.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-9d7dd652af2817a0.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-87889d082997933e.rmeta --extern rustc_expand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_expand-217f1a13e762bc0b.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-df78d2604c0a0802.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-ea1118fcace03183.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87828ee9fa475c71.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-b3248e1f30bf40a6.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-6a1453ab4ff51aaf.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-2642b864a82701c1.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-d7a734a6b7ad6062.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-82571e40130f3c80.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-c3d95f977d2916bc.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-4b7cd8ad8b9264a5.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-cfbf65bd547f9394.rmeta --extern-html-root-url 'itertools=https://docs.rs/itertools/0.10.5/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.68.0-nightly
  (9b19c3505
  2022-12-18)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
error: unescaped backtick
     |
     |
1593 |     /// filled, this function creates new definitions for `Param` and `Fresh` lifetimes, inserts the
     |
     |
     = help: the closing backtick of an inline code may be missing
     = note: `-D rustdoc::unescaped-backtick` implied by `-D warnings`
help: a previous inline code might be longer than expected
     |
1592 |     /// Given a `parent_def_id`, a list of `lifetimes_in_bounds` and a `remapping` hash to be
     |                                                                +
help: if you meant to use a literal backtick, escape it
     |
1593 |     /// filled, this function creates new definitions for `Param` and `Fresh\` lifetimes, inserts the

error: could not document `rustc_ast_lowering`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_ast_lowering compiler/rustc_ast_lowering/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=b949fb58060b1c14 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rustc_arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_arena-a944b458c0ed8963.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-675636cef14c3f51.rmeta --extern rustc_ast_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_pretty-19d39d9146d7478f.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-9d7dd652af2817a0.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-87889d082997933e.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-ea1118fcace03183.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87828ee9fa475c71.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-6a1453ab4ff51aaf.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-2642b864a82701c1.rmeta --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-cf515bf60bc91367.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-82571e40130f3c80.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-c3d95f977d2916bc.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-4b7cd8ad8b9264a5.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-514c0d16dcc4ec4a.rmeta --extern thin_vec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libthin_vec-b65ae049012eaced.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-cfbf65bd547f9394.rmeta --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.8.1/' --extern-html-root-url 'thin_vec=https://docs.rs/thin-vec/0.2.9/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.68.0-nightly
  (9b19c3505
  2022-12-18)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
error: unescaped backtick
  --> compiler/rustc_infer/src/infer/canonical/canonicalizer.rs:55:15
55 |     /// in `U2`.
   |               ^
   |
   |
   = help: the closing backtick of an inline code may be missing
   = note: `-D rustdoc::unescaped-backtick` implied by `-D warnings`
help: a previous inline code might be longer than expected
   |
53 |     /// example, canonicalizing `&'?0:` Trait<'?1>`, where `'?0` is in `U1` and
   |                                       +
help: if you meant to use a literal backtick, escape it
55 |     /// in `U2\`.
   |               +

error: unescaped backtick
error: unescaped backtick
  --> compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs:73:9
   |
73 |     /// `GenericBoundFailure(p, s, a)
   |
   |
help: the closing backtick of an inline code may be missing
   |
73 |     /// `GenericBoundFailure(p, s, a)`
   |                                      +
help: if you meant to use a literal backtick, escape it
   |
73 |     /// \`GenericBoundFailure(p, s, a)

error: unescaped backtick
error: unescaped backtick
  --> compiler/rustc_infer/src/infer/nll_relate/mod.rs:58:27
   |
58 |     /// - Invariant means `a == b.
   |
   |
help: the closing backtick of an inline code may be missing
   |
58 |     /// - Invariant means `a` == b.
   |                             +
help: if you meant to use a literal backtick, escape it
   |
58 |     /// - Invariant means \`a == b.

error: unescaped backtick
   --> compiler/rustc_infer/src/infer/region_constraints/mod.rs:255:56
    |
    |
255 | /// In the [`VerifyBound`], this struct is enclosed in `Binder to account
    |
    |
help: the closing backtick of an inline code may be missing
    |
255 | /// In the [`VerifyBound`], this struct is enclosed in `Binder` to account
    |                                                               +
help: a previous inline code might be longer than expected
    |
249 | /// If we have an obligation like `<T` as SomeTrait<'?x>>::Item: 'c`, then
    |                                      +
help: if you meant to use a literal backtick, escape it
    |
255 | /// In the [`VerifyBound`], this struct is enclosed in \`Binder to account

error: could not document `rustc_infer`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_infer compiler/rustc_infer/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=11e4c3cb05250913 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-9d7dd652af2817a0.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-87889d082997933e.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-ea1118fcace03183.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87828ee9fa475c71.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-6a1453ab4ff51aaf.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-2642b864a82701c1.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-d7a734a6b7ad6062.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-82571e40130f3c80.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-c3d95f977d2916bc.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-4b7cd8ad8b9264a5.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-514c0d16dcc4ec4a.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-cfbf65bd547f9394.rmeta --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.8.1/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.68.0-nightly
  (9b19c3505
  2022-12-18)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
