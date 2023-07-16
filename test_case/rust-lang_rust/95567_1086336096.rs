plain
 Documenting rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: unresolved link to `rustc_middle::ty::TyS`
   --> compiler/rustc_query_impl/src/plumbing.rs:268:45
    |
268 |             $(#[allow(nonstandard_style)] $(#[$attr])*
    |
   ::: compiler/rustc_query_impl/src/lib.rs:55:1
    |
    |
55  | rustc_query_append! { [define_queries!][<'tcx>] }
    |
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = note: the link appears in this line:
            
            Returns the [`Ty`][rustc_middle::ty::TyS] of the given [`DefId`]. If the [`DefId`] points
    = note: no item named `TyS` in module `ty`
    = note: this error originates in the macro `define_queries` (in Nightly builds, run with -Z macro-backtrace for more info)

error: unresolved link to `rustc_middle::ty::TyS`
error: unresolved link to `rustc_middle::ty::TyS`
   --> compiler/rustc_query_impl/src/plumbing.rs:485:17
    |
485 |             $($(#[$attr])*  $name: QueryState<query_keys::$name<$tcx>>,)*
    |
   ::: compiler/rustc_query_impl/src/lib.rs:55:1
    |
    |
55  | rustc_query_append! { [define_queries!][<'tcx>] }
    |
    = note: the link appears in this line:
            
            
            Returns the [`Ty`][rustc_middle::ty::TyS] of the given [`DefId`]. If the [`DefId`] points
    = note: no item named `TyS` in module `ty`
    = note: this error originates in the macro `define_queries_struct` (in Nightly builds, run with -Z macro-backtrace for more info)

error: unresolved link to `rustc_middle::ty::TyS`
error: unresolved link to `rustc_middle::ty::TyS`
   --> compiler/rustc_query_impl/src/plumbing.rs:533:17
    |
533 |             $($(#[$attr])*
    |
   ::: compiler/rustc_query_impl/src/lib.rs:55:1
    |
    |
55  | rustc_query_append! { [define_queries!][<'tcx>] }
    |
    = note: the link appears in this line:
            
            
            Returns the [`Ty`][rustc_middle::ty::TyS] of the given [`DefId`]. If the [`DefId`] points
    = note: no item named `TyS` in module `ty`
    = note: this error originates in the macro `define_queries_struct` (in Nightly builds, run with -Z macro-backtrace for more info)

error: could not document `rustc_query_impl`
error: could not document `rustc_query_impl`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_query_impl compiler/rustc_query_impl/src/lib.rs --target aarch64-unknown-linux-gnu -o /checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=a8790594a3458556 -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/release/deps --extern measureme=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/libmeasureme-97261aa370912d0f.rmeta --extern rustc_ast=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_ast-2d337973636fc2ea.rmeta --extern rustc_data_structures=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_data_structures-abca0a8f9995dcd4.rmeta --extern rustc_errors=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_errors-294c0c68448c7e72.rmeta --extern rustc_hir=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_hir-8483462c86bd4c42.rmeta --extern rustc_index=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_index-33e4452436cade9b.rmeta --extern rustc_macros=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-d9a25e4a2c275e29.so --extern rustc_middle=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_middle-e08b48a6392b4ba6.rmeta --extern rustc_query_system=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_query_system-0084c4b9adba4302.rmeta --extern rustc_serialize=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_serialize-4f5a742a80f150f2.rmeta --extern rustc_session=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_session-28d86d45ebf809c5.rmeta --extern rustc_span=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-rustc/aarch64-unknown-linux-gnu/release/deps/librustc_span-30179793ed0be8e1.rmeta --extern-html-root-url 'measureme=https://docs.rs/measureme/10.0.0/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.61.0-nightly
  (578e7df23
  2022-04-01)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_infer test:false 6.545
[RUSTC-TIMING] rustc_resolve test:false 5.048
[RUSTC-TIMING] rustc_codegen_ssa test:false 5.137
[RUSTC-TIMING] rustc_query_impl test:false 9.848
