plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:181:19
181 |         err: &mut DiagnosticBuilder<'_>,
    |                   ^^^^^^^^^^^^^^^^^ expected 1 generic argument
    |
    |
note: struct defined here, with 1 generic parameter: `G`
    |
    |
19  | pub struct DiagnosticBuilder<'a, G: EmissionGuarantee> {
help: add missing generic argument
    |
    |
181 |         err: &mut DiagnosticBuilder<'_, G>,


error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2512:19
2512 |         err: &mut DiagnosticBuilder<'_>,
     |                   ^^^^^^^^^^^^^^^^^ expected 1 generic argument
     |
     |
note: struct defined here, with 1 generic parameter: `G`
     |
     |
19   | pub struct DiagnosticBuilder<'a, G: EmissionGuarantee> {
help: add missing generic argument
     |
     |
2512 |         err: &mut DiagnosticBuilder<'_, G>,

For more information about this error, try `rustc --explain E0107`.
error: could not compile `rustc_trait_selection` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
