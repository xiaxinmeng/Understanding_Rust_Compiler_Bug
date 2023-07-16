 rust
{
    (); // block consists of an expression statement and an expression but still fails to release lock
    match self.cache.read() {
        Ok(ref data) if data.len() > to => Some(data[to].clone()),
        _ => None,
    }
}
