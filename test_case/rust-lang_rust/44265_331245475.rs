rust
pub enum TraitItemKind {
    // Generics aren't supported here yet
    Const(P<Ty>, Option<P<Expr>>),
    // `Generics` is already a field in `MethodSig`
    Method(MethodSig, Option<P<Block>>),
    // Added `Generics` here:
    Type(Generics, TyParamBounds, Option<P<Ty>>),
    Macro(Mac),
}
