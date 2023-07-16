rust
pub enum Vtable<'tcx, N> {
    ...
    VtableAlias(VtableAliasData),
    ...
}

pub struct VtableAliasData<N> {
    pub alias_def_id: DefId,
    pub substs: &'tcx Substs<'tcx>,
    pub nested: Vec<N>,
}
