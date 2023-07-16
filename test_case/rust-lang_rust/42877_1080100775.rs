rust
struct Task<T: ?Sized> {
    completed: bool,
    fut: T,
}
impl<T> Task<T> {
    pub fn new(fut: T) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Task {
            completed: false,
            fut,
        }))
    }
}

impl Executor {
//...
    pub fn spawn_unpin(self: &Arc<Self>, fut: impl (Future<Output = ()>) + Send + Unpin + 'static) {
        let task = Task::new(fut);
        let weak_self = Arc::downgrade(self);
        self.async_pool.schedule(move || poll_task(task, weak_self));
    }
}
