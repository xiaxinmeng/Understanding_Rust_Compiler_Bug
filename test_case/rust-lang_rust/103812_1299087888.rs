rs
static BYTES: &[u8] = include_bytes!("out");

fn main() {
    println!("{}", BYTES.len());
}
