rust
fn foo<T: ?Sized + Object<U>, U>(x: <T as Object<U>>::Output) -> U {
    x
}
