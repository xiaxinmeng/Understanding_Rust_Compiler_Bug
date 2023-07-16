rust
type DynTrait = dyn Any + 'static; // not generic over T.
type ImplTrait<T> = impl Any + 'static = <T as Trait>::Assoc; // generic over T.
