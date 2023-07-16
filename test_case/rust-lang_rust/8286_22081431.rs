
GNU gdb (GDB) Fedora (7.3.50.20110722-16.fc16)
Copyright (C) 2011 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-redhat-linux-gnu".
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>...
Reading symbols from /home/panzi/software/rust/bin/rustc...(no debugging symbols found)...done.
(gdb) run
Starting program: /home/panzi/software/rust/bin/rustc main.rs
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".
[New Thread 0x7ffff605b700 (LWP 17992)]
[New Thread 0x7ffff6042700 (LWP 17993)]
[New Thread 0x7ffff4053700 (LWP 17994)]
rust: task failed at 'option::get none', /home/panzi/software/rust-0.7/src/libstd/option.rs:331
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug
note: try running with RUST_LOG=rustc=1,::rt::backtrace to get further details and report the results to github.com/mozilla/rust/issues
rust: task failed at 'explicit failure', /home/panzi/software/rust-0.7/src/librustc/rustc.rs:354
rust: domain main @0x606d90 root task failed
[Thread 0x7ffff605b700 (LWP 17992) exited]
[Thread 0x7ffff4053700 (LWP 17994) exited]
[Thread 0x7ffff6042700 (LWP 17993) exited]
[Inferior 1 (process 17989) exited with code 0145]
(gdb) backtrace
No stack.
(gdb) quit
