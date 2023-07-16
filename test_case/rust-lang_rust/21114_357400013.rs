rust
use std::io::{BufRead, stdin};

fn foo() -> usize {
    let stdin = stdin();
    let stdinlock = stdin.lock();
    stdinlock
        .lines()
        .count()
}

fn main() {}
