
1163                 let task = ~do Coroutine::new_root(&mut sched.stack_pool) {
1164                     unsafe { *task_count_ptr = *task_count_ptr + 1; }
1165                 };
