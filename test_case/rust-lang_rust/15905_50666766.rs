 rust
#![feature(overloaded_calls)]
use std::fmt;
use std::ops::Fn;

pub struct Con<V> {
    data: V
}

pub static Three: Con<&'static [u8]> = Con {
    data: &[3]
};

impl<T: fmt::Show, V: Vector<u8>> Fn<(T,), ConText<T>> for Con<V> {
    #[rust_call_abi_hack]
    fn call(&self, (show,): (T,)) -> ConText<T> {
        ConText {
            data: Vec::from_slice(self.data.as_slice()),
            subject: show
        }
    }
}

pub struct ConText<T> {
    data: Vec<u8>,
    subject: T
}

#[test]
fn test() {
    let f = Three("foo");
}
