 rust
{
    let t = match self.cache.read() {
        Ok(ref data) if data.len() > to => Some(data[to].clone()),
        _ => None,
    };
    t
}
