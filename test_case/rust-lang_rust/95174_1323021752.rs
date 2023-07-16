rust
#[repr(u8)]
pub enum Facing {
    Up,
    Down,
}

pub struct Card<const FACING: u8> {
    // would normally have a "suit" and "number" field
}

pub fn foo(card: Card<{Facing::Up as u8}>) {
    let _: Card<{Facing::Down as u8}> = card;
}
