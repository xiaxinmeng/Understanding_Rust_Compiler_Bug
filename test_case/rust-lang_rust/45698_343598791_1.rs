rust
// Convert the def-id into a node-id. node-ids are only valid for
// the local code in the current crate, so this returns an `Option` in case
// the closure comes from another crate. But in that case we wouldn't
// be borrowck'ing it, so we can just unwrap:
let node_id = self.tcx.hir.as_local_node_id(closure_def_id).unwrap();

let local_def: hir::Def = self.tcx.with_freevars(node_id, |fv| fv[i].def);
