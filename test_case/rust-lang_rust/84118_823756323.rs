
(gdb) run
Starting program: /home/joshua/.local/lib/cargo/target/debug/hello-world 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Program received signal SIGBUS, Bus error.
0x00007ffff7e3e334 in __GI___clock_nanosleep (clock_id=<optimized out>, 
    clock_id@entry=0, flags=flags@entry=0, req=0x7fffffffdec0, rem=0x7fffffffdec0)
    at ../sysdeps/unix/sysv/linux/clock_nanosleep.c:78
78	../sysdeps/unix/sysv/linux/clock_nanosleep.c: No such file or directory.
(gdb) step
std::sys::unix::stack_overflow::imp::signal_handler ()
    at library/std/src/sys/unix/stack_overflow.rs:93
93	library/std/src/sys/unix/stack_overflow.rs: No such file or directory.
