rust
pub struct MyParser<T> {
    iter: T
}

impl<T: Iterator<Item = u32>> MyParser<T> {
    pub fn step(self) -> Option<Self> {
        let iter = self.iter.strip_prefix([1u32, 2, 3])?;
        // put more logic here...
        Some(Self { iter })
    }
}
