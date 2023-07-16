rust
use std::future::Future;
use std::pin::Pin;

struct Foo {
    x: String
}

trait FooServer {
   fn foo(&self) -> Pin<Box<dyn Future<Output=()> + 'static>>;
}

impl FooServer for Foo {
    fn foo(&self) -> Pin<Box<dyn Future<Output=()> + 'static>> {
        // do stuff here that accesses &self.
        println!("{}", self.x);
        Box::pin(async {
            // no longer have access to &self.
            ()
        })
    }
}

pub fn main() {
    let foo  = Foo { x: "hi".into() };
    let fut : Pin<Box<dyn Future<Output=()> + 'static>> = foo.foo();
}
