
#0  0x000000395a87b4dd in malloc_consolidate () from /lib64/libc.so.6
#1  0x000000395a87bf08 in _int_free () from /lib64/libc.so.6
#2  0x00007ffff7d97b49 in rust_sched_loop::run_single_turn (this=0x405c00) at /run/media/jdm/ssd/rust/src/rt/rust_sched_loop.cpp:263
#3  0x00007ffff7d99235 in rust_sched_driver::start_main_loop (this=0x406e80) at /run/media/jdm/ssd/rust/src/rt/rust_sched_driver.cpp:50
#4  0x00007ffff7d92a8a in rust_thread_start (ptr=<optimized out>) at /run/media/jdm/ssd/rust/src/rt/sync/rust_thread.cpp:35
#5  0x000000395b007d14 in start_thread () from /lib64/libpthread.so.0
#6  0x000000395a8f168d in clone () from /lib64/libc.so.6
