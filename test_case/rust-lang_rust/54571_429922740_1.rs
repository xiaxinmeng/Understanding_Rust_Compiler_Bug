rust
pub enum UserTypeAnnotation<'tcx> {
    Ty(CanonicalTy<'tcx>),
    FnDef(DefId, CanonicalUserSubsts<'tcx>),
    AdtDef(&'tcx AdtDef, CanonicalUserSubsts<'tcx>),
    ConstantDef(DefId, CanonicalUserSubsts<'tcx>),
}
