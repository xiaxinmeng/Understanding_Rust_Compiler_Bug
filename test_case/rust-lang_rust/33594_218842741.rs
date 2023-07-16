 rust
extern crate futures;

use futures::*;

fn main() {
    let a: Box<Future<Item=i32, Error=()>> = loop {};
    a
        .map(|t| t)
        .map(|t| t)
        .map(|t| t)
        .map(|t| t)
        .map(|t| t)
        .map(|t| t)
        .map(|t| t)
        .map(|t| t)
        .map(|t| t)
        .map(|t| t)
        .map(|t| t)
        .map(|t| t)
        .map(|t| t)
        .map(|t| t);
}
