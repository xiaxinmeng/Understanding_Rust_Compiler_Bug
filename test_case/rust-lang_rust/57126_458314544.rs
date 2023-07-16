rust
fn copied<'a, T>(self) -> Cloned<Self> where
    Self: Iterator<Item = &'a T>,
    T: 'a + Copy, 
