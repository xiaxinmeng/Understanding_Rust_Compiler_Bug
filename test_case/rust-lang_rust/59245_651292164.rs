rust
use std::cell::Cell;
use std::future::Future;

#[derive(Clone)]
struct NonSend {
    foo: Cell<()>,
}

impl NonSend {
    fn get_send(&self) -> Result<&(), ()> {
        Ok(&())
    }
}

async fn bar() {}

async fn foo() -> () {
    let ns = NonSend { foo: Cell::new(()) };
    loop {
        match ns.get_send() {
            Ok(r) => return *r,
            Err(_) => bar().await,
        }
    }
}

pub fn assert_send() -> impl Send + Future<Output = ()> {
    foo()
}
