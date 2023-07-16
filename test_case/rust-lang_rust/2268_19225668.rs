
use std::str;

fn main() {
    let a = ~[65, 65, 65, 0, 65, 65];
    println(str::from_bytes(a));
}
