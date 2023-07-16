rust
#![feature(slice_patterns)]
#![allow(warnings)]

struct X;

fn main() {
    let arr = [X, X, X, X, X, X];
    let [_, x @ .. ,   _, _] = arr;
    let [_, _, _, _, y @ ..] = arr;
}
