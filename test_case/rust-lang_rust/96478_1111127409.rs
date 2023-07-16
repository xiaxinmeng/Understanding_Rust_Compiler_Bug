plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0425]: cannot find value `CRATE_DEF_INDEX` in this scope
   --> compiler/rustc_middle/src/middle/stability.rs:447:50
    |
447 |             self.lookup_stability(DefId { index: CRATE_DEF_INDEX, ..def_id }).is_some();
    |
help: consider importing one of these items
    |
4   | use rustc_hir::def_id::CRATE_DEF_INDEX;
