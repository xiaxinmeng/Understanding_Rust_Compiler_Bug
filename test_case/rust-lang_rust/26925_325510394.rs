rust
trait Trait { type Assoc; }
impl Trait for A where B: Trait { type Assoc = B::Assoc; }
impl Trait for B where A: Trait { type Assoc = A::Assoc; }
