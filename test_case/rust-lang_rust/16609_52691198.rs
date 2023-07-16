
struct Foo {
    x: Option<Rc<()>>,
}

impl Foo {
    fn foo(&mut self) {
        self.x = match self.x.take() {
            None => { Some(Rc::new(())) },
            Some(bar) => { Some(bar) },
        }
    }
}
