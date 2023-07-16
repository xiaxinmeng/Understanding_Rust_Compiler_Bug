plain
 Documenting rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error: unresolved link to `Ty`
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
            
            Returns the [`Ty`] of the given [`DefId`]. If the [`DefId`] points to an alias, it will
    = note: no item named `Ty` in scope
    = note: no item named `Ty` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in the macro `define_queries` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unresolved link to `DefId`
   --> compiler/rustc_query_impl/src/plumbing.rs:268:45
    |
    |
268 |             $(#[allow(nonstandard_style)] $(#[$attr])*
    |
   ::: compiler/rustc_query_impl/src/lib.rs:55:1
    |
    |
55  | rustc_query_append! { [define_queries!][<'tcx>] }
    |
    = note: the link appears in this line:
            
            
            Returns the [`Ty`] of the given [`DefId`]. If the [`DefId`] points to an alias, it will
    = note: no item named `DefId` in scope
    = note: no item named `DefId` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in the macro `define_queries` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unresolved link to `Ty`
   --> compiler/rustc_query_impl/src/plumbing.rs:485:17
    |
    |
485 |             $($(#[$attr])*  $name: QueryState<query_keys::$name<$tcx>>,)*
    |
   ::: compiler/rustc_query_impl/src/lib.rs:55:1
    |
    |
55  | rustc_query_append! { [define_queries!][<'tcx>] }
    |
    = note: the link appears in this line:
            
            
            Returns the [`Ty`] of the given [`DefId`]. If the [`DefId`] points to an alias, it will
    = note: no item named `Ty` in scope
    = note: no item named `Ty` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in the macro `define_queries_struct` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unresolved link to `DefId`
   --> compiler/rustc_query_impl/src/plumbing.rs:485:17
    |
    |
485 |             $($(#[$attr])*  $name: QueryState<query_keys::$name<$tcx>>,)*
    |
   ::: compiler/rustc_query_impl/src/lib.rs:55:1
    |
    |
55  | rustc_query_append! { [define_queries!][<'tcx>] }
    |
    = note: the link appears in this line:
            
            
            Returns the [`Ty`] of the given [`DefId`]. If the [`DefId`] points to an alias, it will
    = note: no item named `DefId` in scope
    = note: no item named `DefId` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in the macro `define_queries_struct` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unresolved link to `Ty`
   --> compiler/rustc_query_impl/src/plumbing.rs:533:17
    |
    |
533 |             $($(#[$attr])*
    |
   ::: compiler/rustc_query_impl/src/lib.rs:55:1
    |
    |
55  | rustc_query_append! { [define_queries!][<'tcx>] }
    |
    = note: the link appears in this line:
            
            
            Returns the [`Ty`] of the given [`DefId`]. If the [`DefId`] points to an alias, it will
    = note: no item named `Ty` in scope
    = note: no item named `Ty` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in the macro `define_queries_struct` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unresolved link to `DefId`
   --> compiler/rustc_query_impl/src/plumbing.rs:533:17
    |
    |
533 |             $($(#[$attr])*
    |
   ::: compiler/rustc_query_impl/src/lib.rs:55:1
    |
    |
55  | rustc_query_append! { [define_queries!][<'tcx>] }
    |
    = note: the link appears in this line:
            
            
            Returns the [`Ty`] of the given [`DefId`]. If the [`DefId`] points to an alias, it will
    = note: no item named `DefId` in scope
    = note: no item named `DefId` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in the macro `define_queries_struct` (in Nightly builds, run with -Z macro-backtrace for more info)
error: could not document `rustc_query_impl`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_query_impl compiler/rustc_query_impl/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=7cc47ce3fefffc96 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-d811de57fd8922da.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-91e8f3e059a76e36.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a77587f7740aa1e9.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-80411235d75714ef.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-d77349e7181a902f.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-033285bb1596a93a.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-b21fba65432a8859.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-8dc8a68eaafa5cc9.rmeta --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-4c6b8629a2a7fa60.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-497373ff6b2f9c27.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-4dfc049b76f713a8.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-fc026f39dfffa0a7.rmeta --extern-html-root-url 'measureme=https://docs.rs/measureme/10.0.0/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.61.0
  (4dfbfbaf2
  2022-04-01)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_infer test:false 6.233
[RUSTC-TIMING] rustc_resolve test:false 4.788
[RUSTC-TIMING] rustc_codegen_ssa test:false 5.192
[RUSTC-TIMING] rustc_query_impl test:false 9.033
