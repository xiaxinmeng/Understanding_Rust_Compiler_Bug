
phantom /tmp $ gdb test                                                                                                                                                                                                                       
GNU gdb (GDB) 7.6.1
Copyright (C) 2013 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-unknown-linux-gnu".
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>...
Reading symbols from /tmp/test...(no debugging symbols found)...done.
(gdb) r
Starting program: /tmp/test 
warning: Could not load shared library symbols for linux-vdso.so.1.
Do you need "set solib-search-path" or "set sysroot"?
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib64/libthread_db.so.1".
[New Thread 0x7ffff7fbd700 (LWP 6749)]
[New Thread 0x7ffff6204700 (LWP 6750)]
Called
[New Thread 0x7ffff6103700 (LWP 6751)]

Program received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7ffff7fbd700 (LWP 6749)]
0x00007ffff658d90e in memset () from /usr/lib64/libc.so.6
(gdb) backtrace 
#0  0x00007ffff658d90e in memset () from /usr/lib64/libc.so.6
#1  0x0000000000400f02 in main::_d7d397690bb6180::v0.0 ()
#2  0x0000000000400f4e in _rust_main ()
#3  0x00007ffff779ade0 in rt::task::__extensions__::build_start_wrapper::anon::anon::expr_fn_27304 () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#4  0x00007ffff77992d7 in rt::task::__extensions__::run::anon::expr_fn_27232 () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#5  0x00007ffff7403754 in rust_try (f=<optimized out>, fptr=<optimized out>, env=<optimized out>) at /home/flaper87/workspace/personal/rust/src/rt/rust_builtin.cpp:523
#6  0x00007ffff779920c in rt::task::Unwinder::try::_199ab8d6eb226980Qdan::v0.8$x2dpre () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#7  0x00007ffff7799081 in rt::task::Task::run::_199ab8d6eb226980xqam::v0.8$x2dpre () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#8  0x00007ffff779a9cc in rt::task::__extensions__::build_start_wrapper::anon::expr_fn_27289 () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#9  0x0000000000000000 in ?? ()
(gdb
