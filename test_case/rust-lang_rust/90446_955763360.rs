plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0382]: borrow of moved value: `spans`
    --> compiler/rustc_resolve/src/late/diagnostics.rs:1970:13
     |
1963 |         let mut spans: Vec<_> = lifetime_refs.iter().map(|lt| lt.span).collect();
     |             --------- move occurs because `spans` has type `Vec<rustc_span::Span>`, which does not implement the `Copy` trait
1969 |             spans,
     |             ----- value moved here
     |             ----- value moved here
1970 |             |lint| {
     |             ^^^^^^ value borrowed here after move
...
1973 |                 let mut spans_dedup = spans.clone();
     |                                       ----- borrow occurs due to use in closure
For more information about this error, try `rustc --explain E0382`.
error: could not compile `rustc_resolve` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
