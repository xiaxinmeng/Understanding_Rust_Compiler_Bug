 rust
trait Foo<'r> { fn foo(&'r self); }
impl<'r> Foo<'r> for () { fn foo(&'r self) { fail!("placeholder"); } }

fn check<'a, F: Foo<'a>>(cont: &'a F) { cont.foo(); }

fn main() { check(()); }
