
1139     fn test_simple_scheduling() {
1140         do run_in_bare_thread {
1141             let mut task_ran = false;
1142             let task_ran_ptr: *mut bool = &mut task_ran;
1143 
1144             let mut sched = ~new_test_uv_sched();
1145             let task = ~do Coroutine::new_root(&mut sched.stack_pool) {
1146                 unsafe { *task_ran_ptr = true; }
1147             };
1148             sched.enqueue_task(task);
1149             sched.run();
1150             assert!(task_ran);
1151         }
1152     }
