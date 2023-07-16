rust
enum MPlaceTy {
    Allocated(MemPlace, TyAndLayout),
    Constant(Constant, TyAndLayout),
}
