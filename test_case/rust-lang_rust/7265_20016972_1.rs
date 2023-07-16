
fn borrow_scheduler_with_ctx_swap(blk: fn(Scheduler,
                                          swap: fn(Scheduler, Context, Context) -> Scheduler)
                                         -> U) -> U
