
[edef@alarm ~]$ gdb rustc
(gdb) break main   
Breakpoint 1 at 0x890
(gdb) run   
Starting program: /usr/local/bin/rustc 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".

Program received signal SIGSEGV, Segmentation fault.
0x00000000 in ?? ()
(gdb) 
