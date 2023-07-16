rust
fn something(&mut self) {
    self.queue = self.queue.split_off(4);
}
