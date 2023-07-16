
use std::io;

fn main() {
    for line in io::stdin().lines() {
        match line {
            Ok(line) => io::print(line),
            Err(err) => fail!("IO error: {}", err),
        }
    }
}
