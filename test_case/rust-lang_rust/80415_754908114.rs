rust
use rustc_middle::ty::DefIdTree;
let def_id = self.tcx.hir().local_def_id(macro_def.hir_id);
assert_eq!(
    Some(self.tcx.parent_module(macro_def.hir_id).to_def_id()),
    self.tcx.parent(def_id.to_def_id())
);
