rust
#![feature(nll)]

extern crate itertools;
use itertools::Itertools;

fn main() {
    vec![0]
        .into_iter()
        .group_by(|_| 0)
        .into_iter()
        .map(|_| 0);
}
