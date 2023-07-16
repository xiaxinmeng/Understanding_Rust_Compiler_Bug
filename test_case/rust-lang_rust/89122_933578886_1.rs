rust
trait Foo {
    type Bar<T: A>: B
        where T: C
        ensures T: Foo<Self::Bar<T>>;
}
