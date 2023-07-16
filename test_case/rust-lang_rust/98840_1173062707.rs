plain
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
[RUSTC-TIMING] rustc_target test:false 37.617
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0433]: failed to resolve: `Self` is only available in impls, traits, and type definitions
   ::: compiler/rustc_query_impl/src/plumbing.rs:319:13
    |
    |
319 |               rustc_query_description! { $name<$tcx> }
    |
   ::: compiler/rustc_query_impl/src/lib.rs:56:1
    |
56  |   rustc_query_append! { [define_queries!][<'tcx>] }
