rust
pub fn gather_move_from_pat<'a, 'tcx>(bccx: &BorrowckCtxt<'a, 'tcx>,
                                      move_data: &MoveData<'tcx>,
                                      move_error_collector: &mut MoveErrorCollector<'tcx>,
                                      move_pat: &hir::Pat,
                                      cmt: mc::cmt<'tcx>) {
    let source = bccx.tcx.hir.get_pattern_source(move_pat);
    debug!("gather_move_from_pat: move_pat={:?} source={:?}", move_pat, source);
}
