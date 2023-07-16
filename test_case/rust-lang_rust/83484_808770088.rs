plain
   Compiling rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0747]: inferred provided when a lifetime was expected
    --> compiler/rustc_query_impl/src/plumbing.rs:184:59
     |
181  | /          macro_rules! encode_queries {
182  | |              ($($query:ident,)*) => {
183  | |                  $(
184  | |                      on_disk_cache::encode_query_results::<_, super::queries::$query<'_>>(
...    |
190  | |              }
191  | |          }
191  | |          }
     | |__________- in this expansion of `encode_queries!` (#2)
192  | 
193  |            rustc_cached_queries!(encode_queries!);
     |            --------------------------------------- in this macro invocation (#1)
     | 
    ::: /checkout/compiler/rustc_middle/src/query/mod.rs:12:1
     |
12   |  / rustc_queries! {
13   |  |     query trigger_delay_span_bug(key: DefId) -> () {
14   |  |         desc { "trigger a delay span bug" }
...     |
1673 |  |     }
1674 |  | }
     |  | -
     |  | -
     |  | |
     |  |_in this expansion of `rustc_cached_queries!` (#1)
     |    in this macro invocation (#2)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0747`.
