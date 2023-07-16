 rust
impl PartialEq for Key {
    fn eq(&self, other: &Key) -> bool {
        discriminant(self) == discriminant(other)
    }
}

pub fn discriminant(k: &Key) -> usize {
    match *k {
        Key::Unknown                 => 0,
        Key::Backspace               => 1,
        Key::Tab                     => 2,
        Key::Return                  => 3,
        Key::Escape                  => 4,
        // and so on
    }
}
