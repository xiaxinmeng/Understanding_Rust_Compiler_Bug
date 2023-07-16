rust
> pub trait Unsatisfied {}
> 
> #[repr(transparent)]
> pub struct Foo<T: Unsatisfied>(T);
> 
> extern "C" {
>     pub fn foo() -> Foo<u32>;
> }
> 