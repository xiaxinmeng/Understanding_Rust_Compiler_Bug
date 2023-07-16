rust
impl<'tcx> MirPass<'tcx> for ElaborateDrops {
    fn run_pass(&self, tcx: TyCtxt<'tcx>, src: MirSource<'tcx>, body: &mut BodyAndCache<'tcx>) {
        debug!("elaborate_drops({:?} @ {:?})", src, body.span);

        let def_id = src.def_id();
        let param_env = tcx.param_env(src.def_id()).with_reveal_all();
        let move_data = match MoveData::gather_moves(body, tcx, param_env) {
            Ok(move_data) => move_data,
            Err((move_data, _)) => {
                tcx.sess.delay_span_bug(
                    body.span,
                    "No `move_errors` should be allowed in MIR borrowck",
                );
                move_data
            }
        };
