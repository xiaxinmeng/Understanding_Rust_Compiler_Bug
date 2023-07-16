
pub struct AllocRefFake<'a, 'tcx> {
    _alloc: &'a Allocation<AllocId, (), Box<[u8]>>,
    _range: AllocRange,
    _tcx: TyCtxt<'tcx>,
    _alloc_id: AllocId,
}
