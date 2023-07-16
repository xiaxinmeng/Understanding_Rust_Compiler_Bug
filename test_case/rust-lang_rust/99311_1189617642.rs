plain
    Checking clippy_lints v0.1.64 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/methods/suspicious_map.rs:15:65
    |
15  |         if let Some(body_id) = cx.tcx.hir().maybe_body_owned_by(closure.hir_id);
    |                                             ------------------- ^^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
    |                                             arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:424:12
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:424:12
    |
424 |     pub fn maybe_body_owned_by(self, id: LocalDefId) -> Option<BodyId> {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:57
