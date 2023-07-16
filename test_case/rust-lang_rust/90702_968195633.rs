rust
enum TyKind {
    Adt(Def, Substs),
    Str,
    Infer(InferTy),
    /// Placeholder used to silence errors
    Err,
}
