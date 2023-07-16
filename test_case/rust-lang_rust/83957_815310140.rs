rust
#[derive(PartialEq)]
pub enum MyItem<'a> {
    Empty,
    Thing(&'a [u8]),
}
