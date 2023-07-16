
pub struct AllocRef<'a, 'tcx, Prov: Provenance, Extra, Bytes: AllocBytes = Box<[u8]>> {
    alloc: &'a Allocation<Prov, Extra, Bytes>,
    range: AllocRange,
    tcx: TyCtxt<'tcx>,
    alloc_id: AllocId,
}
