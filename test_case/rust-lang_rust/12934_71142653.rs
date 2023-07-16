
pub fn mk_subr<'a, 'tcx>(cx: &InferCtxt<'a, 'tcx>,
                         origin: SubregionOrigin<'tcx>,
                         a: ty::Region,
                         b: ty::Region) {

