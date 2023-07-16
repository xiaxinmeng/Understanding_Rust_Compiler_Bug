rust
fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)
where
    FromA: Default + Extend<A>,
    FromB: Default + Extend<B>,
    Self: Iterator<Item = (A, B)>, 
