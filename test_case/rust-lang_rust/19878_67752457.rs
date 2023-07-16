 rust
use std::thunk::Invoke;

fn consume_closure_boxed(b: Box<Invoke>) {
    b.invoke(());
}

fn consume_closure<F: FnOnce(())>(f: F) {
    consume_closure_boxed(box f);
}

fn main() {
    let x = 42i;
    consume_closure(|()| {
        println!("{}", x);
    });
}
