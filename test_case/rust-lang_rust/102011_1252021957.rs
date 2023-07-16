rust
pub enum ErrorHandled<'tcx> {
    Reported(ErrorGuaranteed),
    Linted,
    TooGeneric(Ty<'tcx>),
}
