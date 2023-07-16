plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0599]: no variant named `HasType` found for enum `RegionNameLabels<'_>`
    |
    |
123 |                 diag.subdiagnostic(RegionNameLabels::HasType {
    |                                                      ^^^^^^^ variant not found in `RegionNameLabels<'_>`
   ::: compiler/rustc_borrowck/src/session_diagnostics.rs:162:1
    |
    |
162 | pub(crate) enum RegionNameLabels<'a> {
    | ------------------------------------ variant `HasType` not found here

error[E0599]: no variant named `NameRefLifeTime` found for enum `RegionNameLabels<'_>`
    |
    |
131 |                 diag.subdiagnostic(RegionNameLabels::NameRefLifeTime {
    |                                                      ^^^^^^^^^^^^^^^ variant not found in `RegionNameLabels<'_>`
   ::: compiler/rustc_borrowck/src/session_diagnostics.rs:162:1
    |
    |
162 | pub(crate) enum RegionNameLabels<'a> {
    | ------------------------------------ variant `NameRefLifeTime` not found here
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_borrowck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_borrowck` due to 2 previous errors
