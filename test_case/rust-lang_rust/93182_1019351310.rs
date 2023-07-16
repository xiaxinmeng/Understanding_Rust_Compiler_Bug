rust
> trait Foo<const N: usize> {}
> struct Bar<const N: usize = { 2 + 3 }> where (): Foo<N>;
> 