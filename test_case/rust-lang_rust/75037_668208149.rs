rust
    pub fn const_from_str(tcx: TyCtxt<'tcx>, val: &str, span: Span) -> Operand<'tcx> {
        let tcx = tcx;
        let allocation = Allocation::from_byte_aligned_bytes(val.as_bytes());
        let allocation = tcx.intern_const_alloc(allocation);
