 rust
#![feature(unsafe_destructor)]

extern crate serialize;

use std::io::IoError;
use serialize::{Encoder, Encodable};
use serialize::json;

struct Foo<T> {
    v: T,
}

#[unsafe_destructor]
impl<'a, T: Encodable<json::Encoder<'a>, IoError>> Drop for Foo<T> {
    fn drop(&mut self) {
        json::encode(&self.v);
    }
}

fn main() {
    let _ = Foo { v: 10i };
}
