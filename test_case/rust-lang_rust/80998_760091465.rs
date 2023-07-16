
#![feature(type_alias_impl_trait)]

use std::future::Future;

pub struct Task<F: Future>(F);
impl<F: Future> Task<F> {
    fn new() -> Self {
        todo!()
    }
    fn spawn(&self, _: impl FnOnce() -> F) {
        todo!()
    }
}

fn main() {
    async fn cb() {
        let a = Foo;
    }

    type F = impl Future;
    static POOL: Task<F> = Task::new();
    Task::spawn(&POOL, || cb());
}
