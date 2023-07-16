plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: no rules expected the token `[`
     |
18   | / rustc_queries! {
19   | |     query trigger_delay_span_bug(key: DefId) -> () {
20   | |         desc { "trigger a delay span bug" }
20   | |         desc { "trigger a delay span bug" }
21   | |     }
...    |
2087 | |     }
2088 | | }
     | | ^
     | | |
     | |_no rules expected this token in macro call
     |
    ::: compiler/rustc_query_impl/src/profiling_support.rs:308:5
     |
308  |       macro_rules! alloc_once {
308  |       macro_rules! alloc_once {
     |       ----------------------- when calling this macro
...
323  |       rustc_query_append! { alloc_once! }

    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
error: could not compile `rustc_query_impl` due to previous error
