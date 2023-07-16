rust
fn drop(&mut self) {
    if self.inner().strong.fetch_sub(1, AcqRel) != 1 {
        return;
    }
    /* drop contents of Arc */
}
