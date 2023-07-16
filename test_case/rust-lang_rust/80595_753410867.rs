rust
/// An inline representation of `Option<char>`.
pub struct Char(u32);

impl PartialEq<char> for Char {
    fn eq(&self, other: &char) -> bool {
        self.0 == *other as u32
    }
}
