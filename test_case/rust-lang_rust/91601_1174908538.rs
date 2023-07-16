rust
#![feature(type_alias_impl_trait)]

trait Callable {
    type Out;
    fn call() -> Self::Out;
}

type Any = impl std::any::Any;

impl<'a> Callable for () {
    type Out = Any;
    fn call() -> Self::Out {}
}
