
fn something(&mut self) {
    let rhs = self.queue.split_off(4);
    std::mem::swap(&mut rhs, self.queue);
}
