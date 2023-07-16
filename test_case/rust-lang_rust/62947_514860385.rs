rust
#[derive(Clone, Debug)]
pub struct Chars<'a> {
    iter: slice::Iter<'a, u8>
}
