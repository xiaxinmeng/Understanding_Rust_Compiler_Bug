rust
#![feature(async_await)]

struct Foo {
}

impl Foo {
    pub async fn do_foo<F: Fn()>(&self, _f: F) {
    }
}

async fn x() {
    let foo = Foo{};
    foo.do_foo(||{ }).await;
}

fn main() {
    let _fut = x();
}
