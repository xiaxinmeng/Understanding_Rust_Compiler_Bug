
struct Foo { x: ~[int] }

impl Foo {
    fn foo(&self) -> typeof(self.x.iter()) { self.x.iter() }
}
