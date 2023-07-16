
#![feature(nll)]

fn main() {
    vec![42].iter().map(|_| ()).count();
}