rust
    impl<'a, T: Type> Copied<'a, Pointer<T>>
    where
        [(); <NonNullPointer<T>>::SIZE]:,
        [(); <Pointer<T>>::SIZE]:,
        [(); T::SIZE]:,
    {
