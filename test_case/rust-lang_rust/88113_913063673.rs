rust
> trait Foo<T> {}
> 
> impl<T> dyn Foo<T> {
>     fn hi(_x: T)  {}
> }
> 
> fn main() {
>     Foo::hi(123); // bare_trait_objects
> }
> 