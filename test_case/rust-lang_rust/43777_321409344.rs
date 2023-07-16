rust
trait Base {
    fn base() {}
}

trait Partial: Base {}
trait Complete: Partial {}

impl<T: Complete> Partial for T {}

struct TypeB;
impl Complete for TypeB {}

fn main() {
    ice::<TypeB>();
}

fn ice<P: Partial>() {
    P::base();
}
