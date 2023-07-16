
fn visit_lifetime_def(&mut self, lifetime_def: &hir::LifetimeDef) {
    let def_id = self.tcx.hir.as_local_def_id(lifetime_def.id);
    ...
}
