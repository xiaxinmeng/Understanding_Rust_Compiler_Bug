rust
use futures::stream::{self, *};
use futures::future;
pub trait Whelk: Send {}
fn needs_send<T: Send>(_: T) {}
fn main() {
    let f = async move {
        let mut results = stream::empty().map(move |r: Box<dyn Whelk>| {
            future::ready(())
        })
        .buffered(5);
        future::ready(()).await
    };
    needs_send(f);
}
