rust
#![feature(nll)]

use std::io::{stdin, BufRead};

fn foo() -> usize {
    let stdin = stdin();
    let stdinlock = stdin.lock();
    stdinlock
        .lines()
        .count()
}

fn main() {}
