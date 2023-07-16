
> % RUST_LOG=::rt gdb ./f
GNU gdb (GDB) 7.0.1-debian
Copyright (C) 2009 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>...
Reading symbols from /users/laden/r/f...(no debugging symbols found)...done.
(gdb) r
Starting program: /users/laden/r/f
[Thread debugging using libthread_db enabled]
rust: new dom 0x602460
rust: created task thread: 0x602450, id: 0, live_threads: 1
rust: new dom 0x6039d0
rust: created task thread: 0x6039c0, id: 0, live_threads: 1
[New Thread 0x7ffff7ff7700 (LWP 13723)]
rust: pumping scheduler
rust: scheduler 0 resuming ...
rust: worker 0, number_of_live_tasks = 0
rust: all tasks are blocked, scheduler id 0 yielding ...
rust: blocking scheduler
[New Thread 0x7ffff7fde700 (LWP 13724)]
rust: Creating task main, on thread 0.
rust: New non-weak tasks 1
rust: new task 0x605080
rust: sizeof(task) = 704 (0x2c0)
rust: creating new stack for task 605080
rust: calculating new stack size for 0x605080
rust: min: 768 current: 0 requested: 768
rust: next stack size: 768
rust: new stk 0x6053a0
rust: stk end 0x605ed0
rust: created task: 0x605080, spawner: (none), name: main
rust: starting task from fn 0x400ab0 with env 0x0 and arg 0x0
rust: task main 0x6039d0 state change 'newborn' -> 'running' while in 'newborn'
rust: pumping scheduler
rust: scheduler 0 resuming ...
rust: worker 0, number_of_live_tasks = 0
rust: all tasks are blocked, scheduler id 0 yielding ...
rust: blocking scheduler
rust: pumping scheduler
rust: scheduler 0 resuming ...
rust: worker 0, number_of_live_tasks = 1
rust: activating task main 0x605080, state: running
rust: Running task 0x605080 on worker 0
rust: descheduling...
rust: > UPCALL upcall_s_malloc - task: main 0x605080 retpc: x7ffff76f9fc1
rust: upcall malloc(0x7ffff7c7fc10)
rust: @malloc()=0x605ee0 with td 0x7ffff7c7fc10, size 36==32+4, align 4, prev (nil), next (nil)

rust: upcall malloc(0x7ffff7c7fc10) = box 0x605ee0 with body 0x605f00
Hello world!
rust: > UPCALL upcall_s_free - task: main 0x605080 retpc: x7ffff76f9fc1
rust: upcall free(0x605ee0, is_gc=140737344792672)
rust: @free(0x605ee0) with td 0x7ffff7c7fc10, prev (nil), next (nil)

rust: creating new stack for task 605080
rust: calculating new stack size for 0x605080
rust: min: 768 current: 768 requested: 392
rust: next stack size: 1536
rust: new stk 0x605ee0
rust: stk end 0x606d10
rust: task main 0x6039d0 state change 'running' -> 'dead' while in 'running'
rust: freeing stk segment 0x605ee0
rust: task has returned
rust: returned from task main @0x605080 in state 'dead', worker id=0lx
rust: freeing stk segment 0x6053a0
rust: ~rust_task main @0x605080, refcnt=0
rust: New non-weak tasks 0
rust: Allowing main scheduler to exit
rust: Requesting exit for thread 0
rust: Allowing osmain scheduler to exit
rust: Requesting exit for thread 0
rust: pumping scheduler
rust: scheduler 0 resuming ...
rust: finished main-loop 0
rust: pumping scheduler
rust: scheduler 0 resuming ...
rust: finished main-loop 0

Program received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7ffff7fde700 (LWP 13724)]
buffered_vfprintf (s=0x7ffff7039860, format=0x7ffff7716857 "rust: %s\n", args=0x7ffff7fdc430) at vfprintf.c:2284
2284    vfprintf.c: No such file or directory.
        in vfprintf.c
(gdb) bt
#0  buffered_vfprintf (s=0x7ffff7039860, format=0x7ffff7716857 "rust: %s\n", args=0x7ffff7fdc430) at vfprintf.c:2284
#1  0x00007ffff6d1eece in _IO_vfprintf_internal (s=0x7ffff7039860, format=0x7ffff7716857 "rust: %s\n", ap=0x7ffff7fdc430) at vfprintf.c:1309
#2  0x00007ffff6d299b8 in __fprintf (stream=0x7ffff7fdbc30, format=0x7ffff7716857 "rust: %s\n") at fprintf.c:33
#3  0x00007ffff76ed9d8 in rust_log::trace_ln (this=<value optimized out>, prefix=0x7ffff7fdcd40 "", message=<value optimized out>) at /scratch/laden/rust/src/rt/rust_log.cpp:126
#4  0x00007ffff76eda58 in rust_log::trace_ln (this=0x6020f8, task=<value optimized out>, level=<value optimized out>, message=0x7ffff7fdd560 "Deleting scheduler 1")
    at /scratch/laden/rust/src/rt/rust_log.cpp:169
#5  0x00007ffff76f4516 in rust_kernel::log (this=0x6020f0, level=4, fmt=<value optimized out>) at /scratch/laden/rust/src/rt/rust_kernel.cpp:62
#6  0x00007ffff76f4e39 in rust_kernel::wait_for_schedulers (this=0x6020f0) at /scratch/laden/rust/src/rt/rust_kernel.cpp:182
#7  0x00007ffff76e0f9a in rust_thread_start (ptr=0x7ffff7fdbc30) at /scratch/laden/rust/src/rt/sync/rust_thread.cpp:35
#8  0x00007ffff68b18ca in start_thread (arg=<value optimized out>) at pthread_create.c:300
#9  0x00007ffff6dacb6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:112
#10 0x0000000000000000 in ?? ()
