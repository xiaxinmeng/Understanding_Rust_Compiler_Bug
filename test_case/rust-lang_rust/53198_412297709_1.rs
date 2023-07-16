Rust
#![feature(nll)]

fn main() { doit(0, 0); }

fn doit(i: usize, j: usize) {
    let mut x = [Vec::new(), Vec::new()];
    x[i].push(x[j].len());
}
