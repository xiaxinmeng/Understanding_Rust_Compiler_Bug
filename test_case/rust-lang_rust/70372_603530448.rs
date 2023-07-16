rust
fn check_arms<'p, 'tcx>(
    cx: &mut MatchCheckCtxt<'p, 'tcx>,
    arms: &[(&'p super::Pat<'tcx>, HirId, bool)],
) -> Matrix<'p, 'tcx> {
    let mut seen = Matrix::empty();
    let mut catchall = None;
    for (arm_index, (pat, id, has_guard)) in arms.iter().copied().enumerate() {
        let v = PatStack::from_pattern(pat);
        match is_useful(cx, &seen, &v, LeaveOutWitness, id, true) {
            NotUseful => { ... } // Elided, this branch should not be taken in this case.
            Useful(unreachable_subpatterns) => {
                for pat in unreachable_subpatterns {
                    unreachable_pattern(cx.tcx, pat.span, id, None);
                }
            }
            UsefulWithWitness(_) => bug!(),
        }
        if !has_guard {
            seen.push(v);
            if catchall.is_none() && pat_is_catchall(pat) {
                catchall = Some(pat.span);
            }
        }
    }
    seen
}
