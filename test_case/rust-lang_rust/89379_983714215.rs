rust
fn parse() -> Option<()> {
    let arr = array::try_from_fn(|_| self.iter.next())?;
    // ...
}
