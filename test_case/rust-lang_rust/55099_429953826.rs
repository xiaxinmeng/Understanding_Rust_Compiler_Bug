rust
#![feature(existential_type)]

struct Foo<'a> {
    x: &'a mut (),
}

existential type F: Future<Item = (), Error = ()>;

impl<'a> Foo<'a> {
    fn foobar(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn reply<I>(&mut self, _: I) -> F {
        self.foobar().into_future().and_then(move |_| futures::future::ok(()))
    }
}
