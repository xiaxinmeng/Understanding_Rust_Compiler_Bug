rust
trait Bar<T> {}
trait Caz {
    type A;
    type B: Bar<Self::A>;
}
fn test<T: Caz<A = ()>>() {}
