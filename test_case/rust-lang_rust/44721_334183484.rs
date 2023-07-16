rust
#[derive(Copy, Clone, Debug)]
enum TypeLoweringContext {
    FnParameter(DefId), // the DefId here is the def-id of the function to which this is a parameter
    FnReturnType(DefId), // here too
    Other
}

impl TypeLoweringContext {
    fn impl_trait_treatment(self) -> ImplTraitTreatment {
        use self::TypeLoweringContext::*;
        use self::ImplTraitTreatment::*;

        match self {
            FnParameter(_) => Universal,
            FnReturnType(_) => Existential,
            Other => Disallowed,
        }
    }
}

enum ImplTraitTreatment {
    /// Treat `impl Trait` as shorthand for a new universal generic parameter.
    /// Example: `fn foo(x: impl Debug)`, where `impl Debug` is conceptually
    /// equivalent to a fresh universal parameter like `fn foo<T: Debug>(x: T)`.
    Universal,
    
    /// Treat `impl Trait` as shorthand for a new universal existential parameter.
    /// Example: `fn foo() -> impl Debug`, where `impl Debug` is conceptually
    /// equivalent to a fresh existential parameter like `abstract type T; fn foo() -> T`.
    Existential,

    /// `impl Trait` is not accepted in this position.
    Disallowed,
}
