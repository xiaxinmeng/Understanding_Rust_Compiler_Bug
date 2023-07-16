rust
const fn foo(x: &dyn Trait);
const fn foo(x: impl Trait);
const fn foo<T: Trait>(x: T);
