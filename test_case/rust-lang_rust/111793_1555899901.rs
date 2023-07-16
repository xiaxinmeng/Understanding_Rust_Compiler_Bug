plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error: variants `AfterConstProp` and `Final` are never constructed
 --> compiler/rustc_mir_transform/src/simplify_branches.rs:6:5
5 | pub enum SimplifyConstCondition {
  |          ---------------------- variants in this enum
6 |     AfterConstProp,
  |     ^^^^^^^^^^^^^^
  |     ^^^^^^^^^^^^^^
7 |     Final,
  |     ^^^^^
  |
  = note: `-D dead-code` implied by `-D warnings`
error: could not compile `rustc_mir_transform` (lib) due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_mir_transform` (lib test) due to previous error
Build completed unsuccessfully in 0:01:13
