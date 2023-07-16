
do group_arc.with |tg| {
    for each_task(tg) |task| {
        rustrt::rust_task_kill_other(task);
    }
}
