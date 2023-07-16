rust
mod m {
    pub struct S(#[rustfmt::skip] pub u8);
}

fn main() {
    m::S(0);
}
