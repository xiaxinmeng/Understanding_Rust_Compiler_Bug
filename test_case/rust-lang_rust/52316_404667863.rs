rust
struct MyOptimizedTaskQueue {
    local_queue: RefCell<VecDeque<Arc<MyTask>>>,
    sync_queue: crossbeam::sync::SegQueue<Arc<MyTask>>,
}

unsafe impl Send for MyOptimizedTaskQueue {}
unsafe impl Sync for MyOptimizedTaskQueue {}

struct MyTask {
    task: RefCell<TaskObj>,
    queue: Arc<MyOptimizedTaskQueue>,
}

unsafe impl Send for MyTask {}
unsafe impl Sync for MyTask {}

impl Wake for MyOptimizedTaskQueue {
    fn wake(arc_self: &Arc<Self>) {
        arc_self.queue.sync_queue.push(arc_self.clone());
    }

    // HERE is the key bit that `LocalWaker` allows-- because we know we're on the same thread
    // from which the `LocalWaker` was created, we know that we can use the `local_queue` `RefCell`
    // without any synchronization.
    unsafe fn wake_local(arc_self: &Arc<Self>) {
        arc_self.queue.local_queue.push(arc_self.clone());
    }
}
