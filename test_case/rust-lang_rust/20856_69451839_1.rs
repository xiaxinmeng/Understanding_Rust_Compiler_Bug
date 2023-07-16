 rust
impl PartialEq for Key {
    fn eq(&self, other: &Key) -> bool {
        if discriminant(self) != discriminant(other) {
            return false;
        }
         match (*self, *other) {
                (Key::Sleep(n), Key::Sleep(m)) => n == m,
                _ => true,
         }
    }
}
