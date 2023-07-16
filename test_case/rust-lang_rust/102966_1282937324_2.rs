rust
trait Trait { type Assoc; }
struct Infinite<T>(<T as Trait>::Assoc);
impl<T> Trait for T { type Assoc = Infinite<T>; } // overflow evaluating the requirement `Infinite<T>: Sized`
