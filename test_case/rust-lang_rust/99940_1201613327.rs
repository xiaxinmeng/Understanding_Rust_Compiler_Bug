rust
// dep
pub trait Assoc {
    type Ty;
}
impl Assoc for () {
    type Ty = ();
}

// local crate
trait Trait {}

impl Trait for <() as dep::Assoc>::Ty {} // err

trait Bar {}
impl<T: Bar> Trait for T {} // this errors, but we know that `(): Bar` does not hold.
