rust
struct Foo<T>(T);

impl<T> TraitA for Foo<T> {
    type Assoc = SomeType;
}

impl<T> TraitB<LocalType> for <Foo<T> as Trait>::Assoc {}
