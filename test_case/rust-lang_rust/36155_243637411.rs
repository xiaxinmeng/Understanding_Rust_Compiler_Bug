 rust
impl Spawn<BoxFuture<(), ()>> {
    pub fn execute(self, exec: Arc<Executor>) {
        // ...
    }
}
