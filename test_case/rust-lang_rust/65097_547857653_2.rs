rust
fn drop(&mut self) {
    if self.inner().strong.fetch_sub(1, Release) != 1 {
        return;
    }
    let _ = self.inner().strong.load(Acquire);
    /* drop contents of Arc */
}
