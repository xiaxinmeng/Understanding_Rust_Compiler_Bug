Rust
pub trait Foo {}
pub struct Bar<T> { x: T }
macro_rules! impl_foo {
    {$n:expr, $t:ident $($ts:ident)*} => {
        impl<T> Foo for Bar<[T; $n]> {}
        impl_foo!{($n - 1), $($ts)*}
    };
    {$n:expr,} => {
        impl<T> Foo for Bar<[T; $n]> {}
    };
}
impl_foo!{32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}
