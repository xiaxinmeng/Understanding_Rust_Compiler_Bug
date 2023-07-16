rust
    impl<'lt, T : ?Sized> TraitWithBoundOnAssoc<'lt> for T
    where
        T : Trait<'lt>,
        <T as Trait<'lt>>::Assoc : SomeBound,
    {
        type Assoc = <T as Trait<'lt>>::Assoc;
    }
    