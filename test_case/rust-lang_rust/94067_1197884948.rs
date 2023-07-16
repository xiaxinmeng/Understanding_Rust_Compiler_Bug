rust
use std::sync::Mutex;

#[derive(Default)]
pub struct Foo {
    x: Mutex<usize>,
}

async fn foo() {}

impl Foo {
    pub async fn bar(&self) {
        let mut guard = self.x.lock().unwrap();
        *guard += 1;
        drop(guard);
        foo().await;
    }
}

fn assert_send(_: impl Send) {}

pub fn check(foo: &Foo) {
    assert_send(foo.bar());
}
