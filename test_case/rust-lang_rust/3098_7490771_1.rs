
let mut group = none;
do group_arc.with |tg| { *tg <-> group; }
for each_task(group) |task| {
    rustrt::rust_task_kill_other(task);
    rustrt::rust_task_deref(task);
}
