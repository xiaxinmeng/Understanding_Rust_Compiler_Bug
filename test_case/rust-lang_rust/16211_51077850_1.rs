
$ nm lol | ack begin_unwind
0000000000400970 t _ZN6unwind12begin_unwind20h7301209117934134866E
                 U _ZN6unwind18begin_unwind_inner20h8989957d4b709f80B8dE
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
(gdb) break _ZN6unwind12begin_unwind20h7301209117934134866E
Breakpoint 1 at 0x400970
(gdb) r
Starting program: /home/kemurphy/lol 
warning: Could not load shared library symbols for linux-vdso.so.1.
Do you need "set solib-search-path" or "set sysroot"?
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".

Breakpoint 1, 0x0000000000400970 in unwind::begin_unwind::h7301209117934134866 ()
