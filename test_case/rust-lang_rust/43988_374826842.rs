rust
fn nested_visit_map<'this>(&'this mut self) -> NestedVisitorMap<'this, 'tcx> {
    // only visit the bodies of functions, not their definitions
    NestedVisitorMap::OnlyBodies(&this.hir)
}
