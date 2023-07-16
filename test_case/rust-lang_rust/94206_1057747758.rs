plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.61 (/checkout/src/tools/clippy/clippy_lints)
error: passing `Ty<'tcx>` by reference
   --> src/tools/clippy/clippy_lints/src/significant_drop_in_scrutinee.rs:189:69
    |
189 | fn has_significant_drop_attribute<'tcx>(cx: &LateContext<'tcx>, ty: &Ty<'tcx>) -> bool {
    |                                                                     ^^^^^^^^^ help: try passing by value: `Ty<'tcx>`
    |
    = note: `-D rustc::pass-by-value` implied by `-D warnings`
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:12
