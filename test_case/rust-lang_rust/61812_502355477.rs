rust
> trait A {
>     const B: u8;
> }
> 
> trait Foo<T> {
>     const BAR: u8 = T::B;
> }
> 