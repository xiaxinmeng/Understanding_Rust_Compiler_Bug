 rust
struct Foo {
    x: Option<Rc<()>>,
}

fn foo() -> Rc<()> { fail!() }

impl Foo {
    fn foo(&mut self) {
        match self.x {
            None => self.x = foo(),
            x => self.x = x,
        }
    }
}
