 rust
pub struct substs {
    pub self_ty: Option<ty::t>,
    pub tps: DefIdMap<t>,    // instead of Vec<t>
    pub regions: RegionSubsts,
}
pub enum RegionSubsts {
    ErasedRegions,
    NonerasedRegions(NodeMap<Region>)    // instead of OwnedSlice<Region>
}
