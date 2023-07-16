rust
    trait TraitWithBoundOnAssoc<'lt> {
        type Assoc : ?Sized + SomeBound;
    }
    