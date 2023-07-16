rust
fn foo<T: ?Sized, U>(x: <T as Object<U>>::Output) -> U {
    //                  ^^^^^^^^^^^^^^^^^^^^^^^^^
    // Resolves to `U`, so `foo` thinks it has an argument of
    // type `U` -- therefore, it can be returned.
    x
}
