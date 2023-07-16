
foo: for<LTS> fn(ε, ...χ) method
χ does not refer to Self
(used to be `ε` is of the form &Self, &mut Self, Box<Self>)
Γ, Self: Sized ⊢ for<LTS> ε: CoerceUnsized<ε[Self ← dyn Self<Assoc1=Self::Assoc1,...>]>
    \-- this is in the trait param env, where we know that ε: Trait
------------
Γ ⊢ foo object-safe
