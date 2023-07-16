 rust
struct Utf8Char {
    bytes: [u8; 4],
    position: usize,
}

impl Deref for Utf8Char {
    type Target = [u8];
    fn deref(&self) -> &[u8] { &self.bytes[self.position..] }
}

impl Iterator for Utf8Char {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.position < self.bytes.len() {
            let byte = self.bytes[self.position];
            self.position += 1;
            Some(byte)
        } else {
            None
        }
    }
}
