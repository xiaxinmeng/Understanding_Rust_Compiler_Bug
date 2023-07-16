 rust
use std::io;

fn main() {
    for line in io::stdin().lines().fail_on_error() {
        println!("{}", line);
    }
}
