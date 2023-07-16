plain
    Checking clippy_lints v0.1.64 (/checkout/src/tools/clippy/clippy_lints)
error[E0599]: no variant or associated item named `Binding` found for enum `rustc_hir::Node` in the current scope
  --> src/tools/clippy/clippy_lints/src/manual_rem_euclid.rs:75:31
   |
75 |             && let Some(Node::Binding(_)) = cx.tcx.hir().find(hir_id) {
   |                               ^^^^^^^ variant or associated item not found in `rustc_hir::Node<'_>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:54
