
(gdb) break test.rs:3
Breakpoint 1 at 0x8924: file test.rs, line 3.
(gdb) r
Starting program: test
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, test::main () at test.rs:7
7	    let _ = bar();
