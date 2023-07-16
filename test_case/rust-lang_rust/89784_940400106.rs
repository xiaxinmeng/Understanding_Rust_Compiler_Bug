plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0425]: cannot find value `hits` in this scope
   |
   |
63 |         eprintln!("\nQuery cache hit rate: {}", hits as f64 / (hits + results) as f64);


error[E0425]: cannot find value `hits` in this scope
   |
   |
63 |         eprintln!("\nQuery cache hit rate: {}", hits as f64 / (hits + results) as f64);

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
For more information about this error, try `rustc --explain E0425`.
