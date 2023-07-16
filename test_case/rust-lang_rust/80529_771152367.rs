plain
warning: 1 warning emitted

    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
 Documenting rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: unresolved link to `Matrix::specialize_constructor`
    |
    |
203 | //! Specialization for the whole matrix is done in [`Matrix::specialize_constructor`]. Note that
    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the struct `Matrix` has no field or associated item named `specialize_constructor`
    |
    = note: `-D broken-intra-doc-links` implied by `-D warnings`

error: unresolved link to `PatStack::pop_head_constructor`
    |
    |
206 | //! [`PatStack::pop_head_constructor`]. The internals of how it's done mostly live in the
    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `PatStack` in scope

error: unresolved link to `is_useful`
    |
    |
266 | //! This computation is done in [`is_useful`]. In practice we don't care about the list of
    |                                  ^^^^^^^^^^^ no item named `is_useful` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: aborting due to 3 previous errors

error: could not document `rustc_mir_build`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_mir_build compiler/rustc_mir_build/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-e9bfb5f44b74579f.rmeta --extern rustc_arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_arena-e2f417d3f6c44169.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-af8907c51d0619b2.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-71589f82bb074ad7.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-900d11de835d34e6.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c9203ed2e6da36c2.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-d2c50c70ab8fb75c.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-984047fbd60b0386.rmeta --extern rustc_infer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_infer-a71eae28a3528f8b.rmeta --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-75bbdb0766b3579a.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-0f41645d10f64d62.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-8a2123a4515b8b32.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-3f54fec704da7e59.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a7d4facd2979b981.rmeta --extern rustc_trait_selection=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_trait_selection-0aca514d03ad15fe.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-e1ca8c1925105298.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-166f52f5b1e92350.rmeta -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.51.0-nightly
  (754e52478
  2021-02-01)' --document-private-items --enable-index-page -Zunstable-options -Znormalize-docs` (exit code: 1)
warning: could not parse code block as Rust code
   --> compiler/rustc_typeck/src/check/upvar.rs:366:9
    |
366 |       /// 