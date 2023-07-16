rust
#[derive(Clone, PartialEq)]
pub enum Categorization<'tcx> {
    // temporary val, argument is its scope
    Rvalue(ty::Region<'tcx>, ty::Region<'tcx>),
    //     ^^^^^^^^^^^^^^^^ this data is the new temporary lifetime, after the change
    //                       ^^^^^^^^^^^^^^^^ this data is the *old* lifetime, before the change
    ...
}
