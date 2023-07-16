rust
fn next(&mut self) -> Option<Self::Item> {
    // Do not advance on the very first `.next()` call
    if !self.started {
        self.started = true;
    } else {
        self.last_value += 1;
    }

    return Some(self.last_value);
}
