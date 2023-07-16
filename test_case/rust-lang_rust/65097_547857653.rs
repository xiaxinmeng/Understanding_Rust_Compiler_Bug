rust
fn drop(&mut self) {
    if self.inner().strong.fetch_sub(1, Release) != 1 {
        return;
    }
    atomic::fence(Acquire);
    /* drop contents of Arc */
}
