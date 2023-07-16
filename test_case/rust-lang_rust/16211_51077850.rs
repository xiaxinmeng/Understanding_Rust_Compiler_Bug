
$ gdb lol
GNU gdb (GDB) 7.7.1
Copyright (C) 2014 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-unknown-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from lol...done.
(gdb) breka rust_fail
Undefined command: "breka".  Try "help".
(gdb) break rust_fail
Function "rust_fail" not defined.
Make breakpoint pending on future shared library load? (y or [n]) y
Breakpoint 1 (rust_fail) pending.
(gdb) r
Starting program: /home/kemurphy/lol 
warning: Could not load shared library symbols for linux-vdso.so.1.
Do you need "set solib-search-path" or "set sysroot"?
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".
task '<main>' failed at 'explicit failure', lol.rs:2
[Inferior 1 (process 23728) exited with code 0145]
