plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.6.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.63 (/checkout/src/tools/clippy/clippy_lints)
error[E0599]: no variant or associated item named `LetElse` found for enum `DesugaringKind` in the current scope
    |
    |
204 |                 if expr.span.desugaring_kind() != Some(DesugaringKind::LetElse) {
    |                                                                        ^^^^^^^ variant or associated item not found in `DesugaringKind`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:04:09
