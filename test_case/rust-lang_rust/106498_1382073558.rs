rust
struct Foo<const A: usize> where 
    [(); Bar<A>::T]:,
 {}
