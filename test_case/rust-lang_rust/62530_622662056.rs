rust
> struct T;
> trait Trait {}
> fn foo<X: Trait>(t: X) {}
> impl<'a> Trait for &'a T {}
> fn main() {
>     let t: &mut T = &mut T;
>     foo(t); //~ ERROR failed to find an implementation of trait Trait for &mut T
> }
> 