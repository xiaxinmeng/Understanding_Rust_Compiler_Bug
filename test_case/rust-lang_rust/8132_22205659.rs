
let mut environment = ...;
let ok_to_run_task = AtomicBool::new(false);
do sched.deschedule_running_task_and_then |sched, task| {
    sched.schedule_blocked_task(task);
    ... use environment ... // A
    ok_to_run_task.store(true, Release); // always the last thing that runs in the closure
}
while !ok_to_run_task.load(Acquire) { } // always the first thing to run when task resumes
... use environment ... // B

// victory, a happens-before relationship established between A and B
