rust
fn foo<T: Copy>(t: T) -> Foo<T, T> { (t, t) }
fn bar<T, U>(t: T, u: U) -> Foo<T, U> { (t, u) }
fn mep<U, T>(u: U, t: T) -> Foo<T, U> { (t, u) }
