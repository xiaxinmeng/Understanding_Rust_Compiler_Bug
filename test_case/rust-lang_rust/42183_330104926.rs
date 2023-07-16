rust
fn foo(x: &dyn Trait)
fn foo(x: impl Trait)

impl Trait1 for dyn Trait2 { ... }
impl<T:Trait2> Trait1 for T { ... }
