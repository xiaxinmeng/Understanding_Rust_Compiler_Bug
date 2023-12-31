rust
// build-pass (FIXME(62277): could be check-pass?)
// edition:2018

#![feature(async_await)]

struct Xyz {
    a: u64,
}

trait Foo {}

impl Xyz {
    async fn do_sth<'a>(
        &'a self, foo: &dyn Foo
    ) -> &dyn Foo
    {
        foo
    }
}

fn main() {}
