
struct WorkList {
    sleeper: AtomicOption<SchedHandle>,
    work_queues: [(WorkQueue, WorkQueueClaim), .. MAX_SCHEDULERS]
}
