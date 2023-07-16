`
 Documenting rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
[RUSTC-TIMING] rustc_resolve test:false 4.965
error: unresolved link to `At::trace_exp`
   --> compiler/rustc_trait_selection/src/traits/engine.rs:117:15
    |
117 |     /// See [`At::trace_exp`] and [`Trace::eq`] for a version of
    |               ^^^^^^^^^^^^^ no item named `At` in scope
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`

error: unresolved link to `Trace::eq`
   --> compiler/rustc_trait_selection/src/traits/engine.rs:117:37
    |
117 |     /// See [`At::trace_exp`] and [`Trace::eq`] for a version of
    |                                     ^^^^^^^^^ no item named `Trace` in scope

[RUSTC-TIMING] rustc_query_impl test:false 11.659
error: could not document `rustc_trait_selection`

