plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
    |
    |
228 |     fn emit(self, err: &mut DiagnosticBuilder<'_>) {
    |                             ^^^^^^^^^^^^^^^^^ expected 1 generic argument
    |
note: struct defined here, with 1 generic parameter: `G`
    |
    |
19  | pub struct DiagnosticBuilder<'a, G: EmissionGuarantee> {
help: add missing generic argument
    |
    |
228 |     fn emit(self, err: &mut DiagnosticBuilder<'_, G>) {


error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
    |
234 |         err: &mut DiagnosticBuilder<'_>,
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
234 |         err: &mut DiagnosticBuilder<'_, G>,

For more information about this error, try `rustc --explain E0107`.
error: could not compile `rustc_parse` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
