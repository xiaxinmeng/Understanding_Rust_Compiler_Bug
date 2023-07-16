
fn aa<T: A B>(x: T) { with_a(x); with_b(x); }
fn with_a<T: A>(x: T) { x.foo() }
fn with_b<T: A>(x: T) { x.foo() }
