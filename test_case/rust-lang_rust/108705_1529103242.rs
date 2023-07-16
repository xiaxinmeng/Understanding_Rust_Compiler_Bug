plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: method `add_error_maybe` is never used
   --> compiler/rustc_mir_build/src/thir/pattern/check_match.rs:500:8
    |
184 | impl<'p, 'tcx> MatchVisitor<'_, 'p, 'tcx> {
    | ----------------------------------------- method in this implementation
...
500 |     fn add_error_maybe(&mut self, e: Result<(), ErrorGuaranteed>) {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: could not compile `rustc_mir_build` (lib test) due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_mir_build` (lib) due to previous error
Build completed unsuccessfully in 0:00:59
