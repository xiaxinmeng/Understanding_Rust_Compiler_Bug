
==24424== Invalid write of size 8
==24424==    at 0x554F76F: ev_run (ev.c:2198)
==24424==    by 0x5541EB1: uv_run (core.c:212)
==24424==    by 0x554100C: ??? (in /home/gareth/projects/dretch-rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustc/x86_64-unknown-linux-gnu/lib/librustrt.so)
==24424==  Address 0x6ddfb68 is 2,744 bytes inside a block of size 3,632 free'd
==24424==    at 0x4C282E0: free (vg_replace_malloc.c:366)
==24424==    by 0x552D3E6: rust_task::cleanup_after_turn() (rust_task.cpp:561)
==24424==    by 0x5529ED4: rust_sched_loop::activate(rust_task*) (rust_sched_loop.cpp:47)
==24424==    by 0x552A020: rust_sched_loop::run_single_turn() (rust_sched_loop.cpp:230)
==24424==    by 0x552B704: rust_sched_driver::start_main_loop() (rust_sched_driver.cpp:28)
==24424==    by 0x5524BB9: rust_thread_start(void*) (rust_thread.cpp:25)
==24424==    by 0x63BFEFB: start_thread (pthread_create.c:304)
==24424==    by 0x5A6A59C: clone (clone.S:112)
==24424== 
==24424== Invalid write of size 4
==24424==    at 0x554F701: ev_run (ev.c:1189)
==24424==    by 0x5541EB1: uv_run (core.c:212)
==24424==    by 0x554100C: ??? (in /home/gareth/projects/dretch-rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustc/x86_64-unknown-linux-gnu/lib/librustrt.so)
==24424==  Address 0x6ddfb48 is 2,712 bytes inside a block of size 3,632 free'd
==24424==    at 0x4C282E0: free (vg_replace_malloc.c:366)
==24424==    by 0x552D3E6: rust_task::cleanup_after_turn() (rust_task.cpp:561)
==24424==    by 0x5529ED4: rust_sched_loop::activate(rust_task*) (rust_sched_loop.cpp:47)
==24424==    by 0x552A020: rust_sched_loop::run_single_turn() (rust_sched_loop.cpp:230)
==24424==    by 0x552B704: rust_sched_driver::start_main_loop() (rust_sched_driver.cpp:28)
==24424==    by 0x5524BB9: rust_thread_start(void*) (rust_thread.cpp:25)
==24424==    by 0x63BFEFB: start_thread (pthread_create.c:304)
==24424==    by 0x5A6A59C: clone (clone.S:112)
==24424== 
==24424== Invalid read of size 4
==24424==    at 0x554BB93: ev_feed_event (ev.c:900)
==24424==    by 0x554F851: ev_run (ev.c:924)
==24424==    by 0x5541EB1: uv_run (core.c:212)
==24424==    by 0x554100C: ??? (in /home/gareth/projects/dretch-rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustc/x86_64-unknown-linux-gnu/lib/librustrt.so)
==24424==  Address 0x6ddfb50 is 2,720 bytes inside a block of size 3,632 free'd
==24424==    at 0x4C282E0: free (vg_replace_malloc.c:366)
==24424==    by 0x552D3E6: rust_task::cleanup_after_turn() (rust_task.cpp:561)
==24424==    by 0x5529ED4: rust_sched_loop::activate(rust_task*) (rust_sched_loop.cpp:47)
==24424==    by 0x552A020: rust_sched_loop::run_single_turn() (rust_sched_loop.cpp:230)
==24424==    by 0x552B704: rust_sched_driver::start_main_loop() (rust_sched_driver.cpp:28)
==24424==    by 0x5524BB9: rust_thread_start(void*) (rust_thread.cpp:25)
==24424==    by 0x63BFEFB: start_thread (pthread_create.c:304)
==24424==    by 0x5A6A59C: clone (clone.S:112)
==24424== 
==24424== Invalid read of size 4
==24424==    at 0x554BB97: ev_feed_event (ev.c:902)
==24424==    by 0x554F851: ev_run (ev.c:924)
==24424==    by 0x5541EB1: uv_run (core.c:212)
==24424==    by 0x554100C: ??? (in /home/gareth/projects/dretch-rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustc/x86_64-unknown-linux-gnu/lib/librustrt.so)
==24424==  Address 0x6ddfb4c is 2,716 bytes inside a block of size 3,632 free'd
==24424==    at 0x4C282E0: free (vg_replace_malloc.c:366)
==24424==    by 0x552D3E6: rust_task::cleanup_after_turn() (rust_task.cpp:561)
==24424==    by 0x5529ED4: rust_sched_loop::activate(rust_task*) (rust_sched_loop.cpp:47)
==24424==    by 0x552A020: rust_sched_loop::run_single_turn() (rust_sched_loop.cpp:230)
==24424==    by 0x552B704: rust_sched_driver::start_main_loop() (rust_sched_driver.cpp:28)
==24424==    by 0x5524BB9: rust_thread_start(void*) (rust_thread.cpp:25)
==24424==    by 0x63BFEFB: start_thread (pthread_create.c:304)
==24424==    by 0x5A6A59C: clone (clone.S:112)
==24424== 
==24424== Invalid read of size 4
==24424==    at 0x554BBAD: ev_feed_event (ev.c:906)
==24424==    by 0x554F851: ev_run (ev.c:924)
==24424==    by 0x5541EB1: uv_run (core.c:212)
==24424==    by 0x554100C: ??? (in /home/gareth/projects/dretch-rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustc/x86_64-unknown-linux-gnu/lib/librustrt.so)
==24424==  Address 0x227ab8bc is not stack'd, malloc'd or (recently) free'd
==24424== 
==24424== 
==24424== Process terminating with default action of signal 11 (SIGSEGV)
==24424==  Access not within mapped region at address 0x227AB8BC
==24424==    at 0x554BBAD: ev_feed_event (ev.c:906)
==24424==    by 0x554F851: ev_run (ev.c:924)
==24424==    by 0x5541EB1: uv_run (core.c:212)
==24424==    by 0x554100C: ??? (in /home/gareth/projects/dretch-rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustc/x86_64-unknown-linux-gnu/lib/librustrt.so)
==24424==  If you believe this happened as a result of a stack
==24424==  overflow in your program's main thread (unlikely but
==24424==  possible), you can try to increase the size of the
==24424==  main thread stack using the --main-stacksize= flag.
==24424==  The main thread stack size used in this run was 16777216.
make: *** [check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-std-dummy] Killed
