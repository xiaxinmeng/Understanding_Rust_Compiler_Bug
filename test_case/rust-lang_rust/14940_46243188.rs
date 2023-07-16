 rust
use std::io::stdio;

fn main() {
    let mut out = stdio::stdout();
    out.write(['a' as u8, ..128 * 1024]).unwrap();
}
