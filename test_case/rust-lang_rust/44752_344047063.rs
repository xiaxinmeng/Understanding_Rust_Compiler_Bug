
let node_id = hir.as_local_node_id(def_id).unwrap(); // guarnateed to be local
let hir_lifetime: &hir::Lifetime = match hir.get(node_id) {
    hir_map::NodeLifetime(l) => l,
    _ => bug!()
};
let span = hir_lifetime.span;
