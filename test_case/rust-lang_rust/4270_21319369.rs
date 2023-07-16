 rust
use std::io::WriterUtil;

struct NotAWriter {x: int}

impl NotAWriter {
    fn write_be_u32(y: u32) {}
}

fn main() {
    let x = &(NotAWriter {x: 1});
    x.write_be_u32(0);
}
