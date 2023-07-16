plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0425]: cannot find value `send_trait` in module `sym`
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1997:53
     |
1997 |                 if self.tcx.is_diagnostic_item(sym::send_trait, item_def_id)
     |                                                     ^^^^^^^^^^ help: a constant with a similar name exists: `dyn_trait`
    ::: /checkout/compiler/rustc_span/src/symbol.rs:23:1
     |
23   | symbols! {
23   | symbols! {
     | -------- similarly named constant `dyn_trait` defined here
error[E0425]: cannot find value `sync_trait` in module `sym`
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1998:57
     |
     |
1998 |                     || self.tcx.is_diagnostic_item(sym::sync_trait, item_def_id)
     |                                                         ^^^^^^^^^^ help: a constant with a similar name exists: `dyn_trait`
    ::: /checkout/compiler/rustc_span/src/symbol.rs:23:1
     |
23   | symbols! {
23   | symbols! {
     | -------- similarly named constant `dyn_trait` defined here
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_trait_selection` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
