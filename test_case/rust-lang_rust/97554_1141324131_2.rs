diff
    // wrapping function
    fn check<T>(it: T)
    where
        T : for<'any> Trait<'any,
    +       Assoc = <T as TraitWithBoundOnAssoc<'any>>::Assoc,
        >,
        for<'any>
    -      <T as Trait<'any>>::Assoc : SomeBound
    +       T : TraitWithBoundOnAssoc<'any>
        ,
    {
        // original function which can be left untouched:
        fn check<T> …

        check::<T>(it) // <- turbofish to avoid the issue mentioned in this very issue!
    }
    