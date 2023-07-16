rust
trait Object<U> {
    type Output;
}

impl<T: ?Sized, U> Object<U> for T {
    type Output = U;
}

fn foo<T: ?Sized, U>(x: <T as Object<U>>::Output) -> U {
    x
}

fn transmute<T, U>(x: T) -> U {
    foo::<dyn Object<U, Output = T>, U>(x)
}
