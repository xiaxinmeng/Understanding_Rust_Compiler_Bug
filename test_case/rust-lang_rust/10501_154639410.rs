
pub type Foo = fn(&i32) -> ();

#[derive(Copy, Clone)]
enum Baz {
    Bar(Foo)
}

fn main() {}
