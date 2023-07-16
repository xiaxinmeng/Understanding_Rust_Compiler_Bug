
fn is_char_boundary(&self, index: uint) -> bool {
        if index == self.len() { return true; }
        if index > self.len() { return false; }
        let b = self.as_bytes()[index];
        return b < 128u8 || b >= 192u8;
}
