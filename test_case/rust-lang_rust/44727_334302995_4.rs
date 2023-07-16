rust
TyImplTrait {
    // If `-> impl Iterator<Item = &'a T>` becomes `-> Foo<'a, T>`, then these vectors
    // store the `'a` and `T`
    lifetime_parameters: Vec<Lifetime>, // `['a]` in the above example
    type_parameters: Vec<DefId>, // `[D]` where `D` is the def-id of `T`, in the above example
    exist_ty: ExistTy,
}

struct ExistTy {
    // In above example, this would include `'a` and `T`, each with fresh def-ids
    generics: hir::Generics,

    // Paths in here are "rewritten" to refer to the `T` in our generics above
    bounds: hir::TyParamBounds,
}
