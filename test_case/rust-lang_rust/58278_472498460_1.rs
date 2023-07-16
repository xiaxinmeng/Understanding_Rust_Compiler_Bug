rust
const fn bar<T: const Foo>(t: &T) { t.foo(); }
fn main() { bar(&B) } // Nope.
