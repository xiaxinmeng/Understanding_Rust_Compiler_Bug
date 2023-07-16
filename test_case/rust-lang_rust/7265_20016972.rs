
unsafe {
    let sched = Local::unsafe_borrow::<Scheduler>();
    ...stuff...
    Context::swap(...ctx1..., ...ctx2...);

    // We could be executing in a different thread now 
    let sched = Local::unsafe_borrow::<Scheduler>();
    (*sched).run_cleanup_job();
}
