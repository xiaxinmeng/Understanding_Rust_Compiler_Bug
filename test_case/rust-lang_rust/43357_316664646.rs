rust
fn f<T: ::std::ops::Neg>(x: T) -> T::Output {
    ::std::mem::transmute(x)
}
