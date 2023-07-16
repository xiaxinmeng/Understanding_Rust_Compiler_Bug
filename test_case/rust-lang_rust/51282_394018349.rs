rust
fn bar(&self) -> impl Iterator<Item=u8> + '_ {
    self.v.iter().cloned()
}
