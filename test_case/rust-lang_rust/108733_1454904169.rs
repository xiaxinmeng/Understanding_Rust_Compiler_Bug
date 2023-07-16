rust
let parent = tcx.hir().parent_id(expr.hir_id);
let parent = tcx.hir().get_parent(parent);
if !matches!(parent, hir::Node::Local(..)) {
    // ...
}
