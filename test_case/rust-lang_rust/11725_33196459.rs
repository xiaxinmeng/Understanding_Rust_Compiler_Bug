 rust
pub fn enter(&self) {
    self.arc.access_cond(|state, cond| {
        state.count += 1;
        if state.count < self.num_tasks {
            cond.wait();
        } else {
            state.count = 0;
            cond.broadcast();
        }
    });
}
