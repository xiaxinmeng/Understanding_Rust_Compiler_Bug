rust
// if the current type is an opaque type that originates from an async function
if let hir::Node::Item(item) = self.tcx.hir().get(parent_hir_id) {
    if let hir::ItemKind::OpaqueTy(ref opaque) = item.kind {
        if opaque.origin == hir::OpaqueTyOrigin::AsyncFn {
            // check that the lifetimes used in the opaque type are already declared in the current scope. 
            // The defined_by map is created at the beginning of this method to collect all lifetimes in the scope
            for p in opaque.generics.params {
                if defined_by.contains_key(&p.name) {
                    continue 'lifetimes;
                }
            }
        }
    }
}
