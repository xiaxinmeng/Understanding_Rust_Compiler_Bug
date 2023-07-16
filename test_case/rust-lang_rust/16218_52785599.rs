 rust
trait Foo<T> { }

trait SerializeState {
    fn f<'a, T: Foo<&'a int>>(&self, x: T);
}

struct GatherTokens;

impl SerializeState for GatherTokens {
    fn f<T: Foo<int>>(&self, _x: T) { }
}

fn main() { }
