 rust
pub trait Foo { type Bar; }

#[derive(Debug)]
enum Baz<F> where F: Foo { Baz(F::Bar) }

#[derive(Debug)]
pub enum Bang<F> where F: Foo { Bang(Baz<F>) }
