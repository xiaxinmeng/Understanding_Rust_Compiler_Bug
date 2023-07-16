 rust
// `type(type)` is the kind * -> *
fn foo<type(type) F: Functor, X, Y>(x: F(X), f: X -> Y) -> F(Y) { x.map(f) }
