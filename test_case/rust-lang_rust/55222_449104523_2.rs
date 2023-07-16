rust
let x: Foo<X1...Xn>;

// Desugars to

let x: <(X1, ..., Xn) as FooTrait<_>>::Out;
