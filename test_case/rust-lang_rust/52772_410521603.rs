
[root@s7netserver dev]# gdb -quiet --args rustc --version
Reading symbols from rustc...(no debugging symbols found)...done.
(gdb) break rustc::main::h8593b8c0c617293e
Breakpoint 1 at 0x9f0
(gdb) run
Starting program: /usr/local/bin/rustc --version
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/libthread_db.so.1".

Breakpoint 1, 0x800009f0 in rustc::main::h8593b8c0c617293e ()
(gdb) continue
Continuing.

Program terminated with signal SIGSEGV, Segmentation fault.
The program no longer exists.
(gdb) 
