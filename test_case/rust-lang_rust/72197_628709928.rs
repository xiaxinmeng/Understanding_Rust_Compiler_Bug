rust
> use std::fmt::Debug;
> 
> trait Trait {
>     type A: Debug;
> }
> 
> struct S1 {}
> 
> impl Trait for S1 {
>     type A = u64;
> }
> 
> #[derive(Debug)]
> struct S2<T: Trait> {
>     a: T::A,
> }
>
 
The generated Debug impl expanded is:
