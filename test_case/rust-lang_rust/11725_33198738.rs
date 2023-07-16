 rust
fn wait(&mut self) {
    self.arc.access_cond(|state, cond| {
        let id = state.generation_id;
        state.count += 1;
        if state.count < self.num_tasks {
            while state.generation_id == id && state.count < self.num_tasks {
                cond.wait();
            }
        } else {
            state.count = 0;
            state.generation_id += 1;
            cond.broadcast();
        }
    });
}
