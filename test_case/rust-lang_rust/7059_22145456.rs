
trait Trait1 { ... }
trait Trait2 { ... }
impl<T: Trait1> Trait2 for T { ... }
impl<T: !Trait1> Trait2 for T { ... } // these cannot possibly overlap, so our hypothetical compiler accepts it
