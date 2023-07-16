 rust
extern mod native;
extern mod green;

fn spawn_native() {
    do native::task::spawn(TaskOpts::new()) {
        // this is forced to be a 1:1 task (this is a new OS thread)
    }
}

fn spawn_green_pool() {
    let mut pool = green::SchedPool::new(green::SchedConfig::new());
    do pool.spawn(TaskOpts::new()) {
        // this is an M:N task running in the above pool
        do spawn {
            // this is also an M:N task running in the above pool
        }
    }
    pool.shutdown();
}
