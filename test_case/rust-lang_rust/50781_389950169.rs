rust
trait Trait {}

trait X {
    fn foo(&self) where Self: Trait;
}

impl X for () {
    fn foo(&self) {}
}

impl Trait for dyn X {}

pub fn main() {
    <X as X>::foo(&()); // Segfault at opt-level 0, SIGILL otherwise.
}
