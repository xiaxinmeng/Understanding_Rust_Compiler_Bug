 rust
pub trait AsciiExt {
    type Owned;
    fn is_ascii(&self) -> bool;
    fn to_ascii_uppercase(&self) -> Owned;
    fn to_ascii_lowercase(&self) -> Owned;
    fn make_ascii_uppercase(&mut self);
    fn make_ascii_lowercase(&mut self);
    fn eq_ignore_ascii_case(&self, other: &Self) -> bool;
}
