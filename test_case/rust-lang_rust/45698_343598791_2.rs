
let local_def_id = local_def.def_id();
let local_node_id = self.tcx.hir.as_local_node_id(local_def_id).unwrap();
