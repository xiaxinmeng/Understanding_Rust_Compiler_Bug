 rust
struct A<Foo = Bar>; // gated
let _: A = ... ; // not gated
let _: A<MyType> = ... ; // gated
