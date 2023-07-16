
use std::sync::mpsc::*;

fn action<T: Send, F: FnOnce(T) + Send>(v: T, f: F) {
    f(v);
}

pub fn main() {
    let (tx, rx) = channel();

    action("zomg", move |v| { tx.send(v); });
    rx.recv();
}
