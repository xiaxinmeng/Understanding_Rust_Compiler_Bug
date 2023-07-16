rust
use std::io::Write;

fn main() {
    let mut out = Vec::new();
    for val in 0u8..=255 {
        out.write_all(&[val, val]).unwrap();
    }

    out.write_all(&[0]).unwrap();
}
