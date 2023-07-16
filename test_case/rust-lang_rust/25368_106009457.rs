 Rust
struct Foo<T>(Option<T>);

fn spawn<T: Send>(_: T) {}

struct Tx<T: Send>(Option<T>);

impl<T: Send> Tx<T> {
    fn send(&self, _: T) {}
}

fn channel<T: Send>() -> Tx<T> { Tx(None) }

fn main() {
    let tx = channel();

    spawn(move || {
        tx.send(Foo(None));
    });
}
