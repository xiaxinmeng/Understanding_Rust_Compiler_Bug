rust
trait Bar<T> {}
trait Caz {
    type A;
    type B: Bar<Self::A>;
}

fn test<T, U>() where T: Caz, U: Caz<A = T::A> {}
