rust
fn def_id_corresponds_to_hir_dep_node(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
    let hir_id = tcx.hir.as_local_hir_id(def_id).unwrap();
    match tcx.hir.get(hir_id) {
        Node::Item(_) |
        Node::TraitItem(_) |
        Node::ImplItem(_) |
        Node::Crate => true,

        Node::Param(_) |
        Node::ForeignItem(_) |
        Node::Variant(_) |
        Node::Field(_) |
        Node::AnonConst(_|
        Node::Expr(_) |
        Node::Stmt(_) |
        Node::PathSegment(_) |
        Node::Ty(_) |
        Node::TraitRef(_) |
        Node::Binding(_) |
        Node::Pat(_) |
        Node::Arm(_) |
        Node::Block(_) |
        Node::Local(_) |
        Node::MacroDef(_) |
        Node::Ctor(_) |
        Node::Lifetime(_|
        Node::GenericParam(_) |
        Node::Visibility(_) => false
    }
}
