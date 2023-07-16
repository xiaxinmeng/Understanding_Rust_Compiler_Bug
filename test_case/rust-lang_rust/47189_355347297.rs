
pub enum TypeVariants<'tcx> {
    ...
    TyClosure(DefId, ClosureSubsts<'tcx>),
    TyGenerator(DefId, ClosureSubsts<'tcx>, GeneratorInterior<'tcx>),
}
