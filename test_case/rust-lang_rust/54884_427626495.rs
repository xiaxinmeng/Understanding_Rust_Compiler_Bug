rust
struct Fear{}

impl Fear {
    #[must_use]
    fn bravery() -> u8 { 1 }
}

fn main() {
    Fear::bravery(); // no warning
}
