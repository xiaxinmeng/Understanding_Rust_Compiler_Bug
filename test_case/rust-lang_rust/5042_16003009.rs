
struct KernelData {
    task_count: Atomic<int>,
    schedulers: [AtomicRef<SchedulerHandle>>, .. MAX_SCHEDS],
    etc.
}
