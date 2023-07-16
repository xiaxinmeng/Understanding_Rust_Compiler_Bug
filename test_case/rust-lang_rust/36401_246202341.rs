 rust
#[derive(Debug)]
pub enum Event {
    Key(u8),
    Resize,
    Unknown(i16),
}
