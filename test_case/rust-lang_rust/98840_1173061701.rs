plain
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0433]: failed to resolve: `Self` is only available in impls, traits, and type definitions
    ::: compiler/rustc_query_impl/src/lib.rs:56:1
     |
56   |   rustc_query_append! { [define_queries!][<'tcx>] }
     |   ------------------------------------------------- in this macro invocation (#1)
     |   ------------------------------------------------- in this macro invocation (#1)
     |
    ::: compiler/rustc_query_impl/src/plumbing.rs:319:13
     |
319  |               rustc_query_description! { $name<$tcx> }
    --> /checkout/compiler/rustc_middle/src/query/mod.rs:18:1
     |
18   | / rustc_queries! {
19   | |     query trigger_delay_span_bug(key: DefId) -> () {
19   | |     query trigger_delay_span_bug(key: DefId) -> () {
20   | |         desc { "trigger a delay span bug" }
21   | |     }
...    |
2047 | |     }
2048 | | }
     | | ^
     | | |
     | | `Self` is only available in impls, traits, and type definitions
     | |_in this expansion of `rustc_query_append!` (#1)
     |   in this expansion of `rustc_query_description!` (#2)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_query_impl` due to 274 previous errors
