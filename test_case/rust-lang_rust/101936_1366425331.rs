plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: `#[subdiagnostic(...)]` is not a valid attribute
    |
    |
780 |     #[subdiagnostic(eager)]
    |
    |
    = help: `subdiagnostic` does not support nested attributes
[RUSTC-TIMING] rustc_symbol_mangling test:true 0.431
[RUSTC-TIMING] rustc_symbol_mangling test:false 0.486
[RUSTC-TIMING] rustc_save_analysis test:false 0.561
[RUSTC-TIMING] rustc_incremental test:false 0.640
