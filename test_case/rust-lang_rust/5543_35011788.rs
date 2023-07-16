 rust
use std::io;

fn bar(r:~Reader) { }

fn main() {
    let r : ~Reader = ~io::stdin();
    let _m = bar(r as ~Reader);
}
