 rust
use std::io;

fn main() {
    let ref mut w = &mut io::stdout() as &mut Writer;
    w.write([]);
}
