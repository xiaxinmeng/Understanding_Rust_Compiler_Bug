rust
enum LazyConstValue<'tcx, T> {
    Unevaluated(DefId, &'tcx Substs<'tcx>),
    Evaluated(T),
}
enum ConstValue<'tcx> {
    Scalar(Scalar),
    ScalarPair(Scalar, Scalar),
    Indirect(AllocId, &'tcx Allocation, Size),
}
