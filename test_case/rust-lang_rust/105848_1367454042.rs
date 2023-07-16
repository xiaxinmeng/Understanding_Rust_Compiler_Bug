plain
Successfully built 928870058724
Successfully tagged rust-ci:latest
Built container sha256:928870058724ad3a97e767a0efa30ca9868ba2952ff13de9cbbda1e3e80b0b20
Uploading finished image to https://ci-caches.rust-lang.org/docker/1a6e28604855eaed9434c7a940f6bb6921387e3670ba05e602dc2c343deeb61858fb5d83181978ebac2c97207591e9df2e69958bc73912324e7c2d75f6d42c21
upload failed: - to s3://rust-lang-ci-sccache2/docker/1a6e28604855eaed9434c7a940f6bb6921387e3670ba05e602dc2c343deeb61858fb5d83181978ebac2c97207591e9df2e69958bc73912324e7c2d75f6d42c21 Unable to locate credentials
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
   |
90 |     /// this case as projections as self types add `
   |                                                    ^
   |
   = help: the opening or closing backtick of an inline code may be missing
   = note: `-D rustdoc::unescaped-backticks` implied by `-D warnings`
help: if you meant to use a literal backtick, escape it
90 |     /// this case as projections as self types add \`
   |                                                    +

error: could not document `rustc_trait_selection`
error: could not document `rustc_trait_selection`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_trait_selection compiler/rustc_trait_selection/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=be3b87b0a2e79b46 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-a937334ae4c5d4dd.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-30645075bc11d890.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3720abc0888fadee.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-8ffba753ac164e46.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-88bfab01d637dbcb.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-625c2059ef193ab6.rmeta --extern rustc_infer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_infer-65c2ecbda5bd3065.rmeta --extern rustc_lint_defs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint_defs-b7cf924f043f1237.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-6a1453ab4ff51aaf.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-84f4bbc2a2c04e2b.rmeta --extern rustc_parse_format=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_parse_format-00516d49283ed502.rmeta --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-0ddcd09fe523fcdb.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-728b5b00e974e8ef.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-60d6f869c98d1e09.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-6286b294f509beea.rmeta --extern rustc_transmute=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_transmute-fa547ff9d23429b2.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-514c0d16dcc4ec4a.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-cfbf65bd547f9394.rmeta --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.8.1/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.68.0-nightly
  (d94a2807a
  2022-12-29)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
Build completed unsuccessfully in 0:08:53
