 rust
pub struct Foo;
struct Bar;

pub type MaybeFoo = Option<Foo>; // <-- cool
pub type MaybeBar = Option<Bar>; // <-- compiles today, but i think it shouldnt
