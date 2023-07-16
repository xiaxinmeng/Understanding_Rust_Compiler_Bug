 rust
trait Trait {}
trait TraitPlusUnwindSafe: Trait + UnwindSafe {}
impl<T: Trait + UnwindSafe> TraitPlusUnwindSafe for T {}

Box<TraitPlusUnwindSafe>
