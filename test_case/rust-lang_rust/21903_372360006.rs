rust
type Assoc<T> = <T as Trait>::Assoc;
type SomethingElse<T> = Vec<Assoc<T>>;
