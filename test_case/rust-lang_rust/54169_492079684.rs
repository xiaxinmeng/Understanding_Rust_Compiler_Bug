rust
#![feature(existential_type, async_await)]

use std::future::Future;

pub trait MyTrait<'a> {
    type ResultFut: Future<Output = ()> + 'a;

    fn do_sth(&'a mut self, a: &'a str) -> Self::ResultFut;
}

struct MyImpl {
}

impl<'a> MyTrait<'a> for MyImpl {
    existential type ResultFut<'a>: Future<Output = ()> + 'a;

    fn do_sth(&'a mut self, a: &'a str) -> Self::ResultFut {
        async {
            Ok(())
        }
    }
}
