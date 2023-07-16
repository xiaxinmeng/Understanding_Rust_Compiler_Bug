 Rust
trait Trait: Sized {
    fn foo(self) -> int;
}

struct Wrapped<D> {
    d: D
}

impl<'a, D> Trait for &'a Wrapped<D> where &'a D: Trait {
    fn foo(self) -> int {
        let r: &'a D = &self.d;
        r.foo()
    }
}

fn main() {}
