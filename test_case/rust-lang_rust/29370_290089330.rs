rust
use std::process;

fn main() {
    println!("aborting");

    process::abort();

    // execution never gets here
}
