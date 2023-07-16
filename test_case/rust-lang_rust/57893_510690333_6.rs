rust
fn transmute<T, U>(x: T) -> U {
    foo::<dyn Object<U, Output = T>, U>(x)
    //                           ^
    // Output is normalized to `T`
}
