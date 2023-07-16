plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0599]: no variant or associated item named `Misc` found for enum `DefPathData` in the current scope
   --> compiler/rustc_symbol_mangling/src/typeid/typeid_itanium_cxx_abi.rs:341:46
    |
341 |             | hir::definitions::DefPathData::Misc => {
    |                                              ^^^^ variant or associated item not found in `DefPathData`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_symbol_mangling` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_symbol_mangling` due to previous error
