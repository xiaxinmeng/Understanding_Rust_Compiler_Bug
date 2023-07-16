rust
use futures::stream;
use futures::stream::StreamExt;
use std::future;
struct Bar {}
impl Bar {
    async fn get_foo<'a>(&self) -> Vec<Foo<'a>> {
        let results = vec![()];
        let foo: Vec<Foo<'a>> = stream::iter(results)
            .filter_map(|_| async move { Some(Foo::new()) })
            .filter(|m| future::ready(true))
            .collect()
            .await;
        foo
    }
}
struct Foo<'a> {
    _use_a: &'a (),
}
impl<'a> Foo<'a> {
    fn new() -> Foo<'a> {
        Foo { _use_a: &() }
    }
}
fn demand_is_send<T>(t: T)
where
    T: Send,
{
}
fn main() {
    let bar = Bar {};
    demand_is_send(async move { bar.get_foo().await })
}
