rust
pub enum ConstValue<'tcx> {
    // Used only for types with layout::abi::Scalar ABI and ZSTs which use PrimVal::Undef
    ByVal(PrimVal),
    // Used only for types with layout::abi::ScalarPair
    ByValPair(PrimVal, PrimVal),
    // Used only for the remaining cases
    ByRef(&'tcx Allocation),
}
