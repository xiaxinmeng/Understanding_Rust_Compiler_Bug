 rust
use std::iter::Iterator;

fn map<T, U, I: Iterator<Item = T>>(fun: &(Fn(&T) -> U), mut iter: I) -> Vec<U> {
    let mut acc = vec![];
    for elt in iter {
        acc.push(fun(&elt));
    }
    acc
}

fn main() {
    let x = map(&|&x| x + 2, vec![1u32,2,3].iter());
    println!("{:?}", x);
}
