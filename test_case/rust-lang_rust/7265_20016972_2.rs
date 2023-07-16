
do borrow_scheduler_with_ctx_swap |sched, swap_fn| {
    ... stuff ...
    let reborrowed_sched = swap_fn(sched, ...ctx1..., ...ctx2...);
    (*reborrowed_sched).run_cleanup_job(); // swap() could perhaps do this for you
}
