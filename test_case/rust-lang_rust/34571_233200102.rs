 rust
#[repr(u8)]
pub enum TupleSingle {
    Tuple(u8),
}

fn main() {
    let t = TupleSingle::Tuple(0);
}
