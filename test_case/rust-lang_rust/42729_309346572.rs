rust
> struct Foo {
>     a: u32,
> }
> 
> let mut x = Foo { a: 1 };
> drop(x); // `x` is now uninitialized
> x.a = 2; // error, partial reinitialization of uninitialized structure `t`
> 