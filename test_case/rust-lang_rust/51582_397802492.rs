rust
#[repr(i8)]
pub enum Enum {
    VariantA,
    VariantB,
}

fn make_b() -> Enum { Enum::VariantB }

fn main() {
    assert_eq!(1, make_b() as i8);
}
