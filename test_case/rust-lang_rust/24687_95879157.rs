
extern crate lzw;

fn main() {
    let mut v = vec![];
    let _ = lzw::Encoder::new(lzw::LsbWriter::new(&mut v), 8);
}
