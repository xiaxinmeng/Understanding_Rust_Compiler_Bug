rust
pub fn promote_candidates<'tcx>(
    body: &mut Body<'tcx>, 
    tcx: TyCtxt<'tcx>, 
    temps: IndexVec<Local, TempState>, 
    candidates: Vec<Candidate>
)
