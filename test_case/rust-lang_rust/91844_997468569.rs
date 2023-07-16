plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0616]: field `code` of struct `rustc_middle::traits::ObligationCause` is private
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:889:78
    |
889 |         if let ObligationCauseCode::AwaitableExpr(hir_id) = obligation.cause.code.peel_derives() {
    |
    |
help: a method `code` also exists, call it with parentheses
    |
889 |         if let ObligationCauseCode::AwaitableExpr(hir_id) = obligation.cause.code().peel_derives() {


error[E0599]: no method named `peel_derives` found for enum `Option` in the current scope
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:889:83
    |
889 |         if let ObligationCauseCode::AwaitableExpr(hir_id) = obligation.cause.code.peel_derives() {
    |                                                                                   ^^^^^^^^^^^^ method not found in `Option<Arc<rustc_middle::traits::ObligationCauseCode<'tcx>>>`
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
Some errors have detailed explanations: E0599, E0616.
For more information about an error, try `rustc --explain E0599`.
error: could not compile `rustc_trait_selection` due to 2 previous errors
