
use std::env;
use std::collections::HashSet;

fn main() {
    let args = env::args().collect::<HashSet<_>>();

    println!("{:?}", args.is_empty());
}
