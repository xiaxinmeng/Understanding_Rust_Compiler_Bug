rust
#![feature(nll)]
#![allow(unused_variables)]

fn main() {
    let mut v = Vec::new();
    let x = good(v.iter().cloned());
    let y = bad(v.iter().cloned());
    for _ in y { }
    v.push(3);
}

fn good<I: Iterator<Item = usize>>(x: I) -> std::vec::IntoIter<usize> {
    x.collect::<Vec<usize>>().into_iter()
}

fn bad<I: Iterator<Item = usize>>(x: I) -> impl Iterator<Item = usize> {
    x.collect::<Vec<usize>>().into_iter()
}
