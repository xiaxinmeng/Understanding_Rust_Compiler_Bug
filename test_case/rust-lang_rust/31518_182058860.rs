 rust
pub trait Foo { type Bar; }

#[derive(Debug)]
struct Baz<F> where F: Foo { baz: F::Bar }

#[derive(Debug)]
pub struct Bang<F> where F: Foo { bang: Baz<F> }
