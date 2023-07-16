rust
type Foo<P1...Pn> = Bar<...>;

// Desugars to
trait FooTrait<X> {
    type Out;
}

impl<P1...Pn> FooTrait<Bar<...>> for (P1, ..., Pn) {
    type Out = Bar<...>;
}
