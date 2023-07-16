rust
use core::future::Future;

struct Foo;
impl Foo {
    fn foo(&mut self) {}
}

async fn bar<T>(x: impl FnMut() -> T)
where
    T: Future<Output = ()>,
{
}
fn main() {
    let mut x = Foo;
    bar(move || async {
        x.foo();
    });
}
