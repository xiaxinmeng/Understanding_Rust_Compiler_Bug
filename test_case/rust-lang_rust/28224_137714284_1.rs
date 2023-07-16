
(gdb) break _ZN4heap15exchange_malloc20he8623aa1084055ffJfaE
Breakpoint 1 at 0x4a90: file ../src/liballoc/heap.rs, line 123.
(gdb) ignore 1 1000
Will ignore next 1000 crossings of breakpoint 1.
(gdb) run
Starting program: /home/andrew/foo 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[Inferior 1 (process 1890) exited normally]
(gdb) info breakpoints
Num     Type           Disp Enb Address            What
1       breakpoint     keep y   0x0000555555558a90 in foo::heap::exchange_malloc at ../src/liballoc/heap.rs:123
    breakpoint already hit 100 times
    ignore next 900 hits
