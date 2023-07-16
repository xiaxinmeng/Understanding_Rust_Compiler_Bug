
$ gdb hello 
GNU gdb (Debian 7.7.1+dfsg-5) 7.7.1
Copyright (C) 2014 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from hello...done.
warning: Missing auto-load scripts referenced in section .debug_gdb_scripts
of file /home/steve/tmp/hello
Use `info auto-load python-scripts [REGEXP]' to list them.
(gdb) break _ZN4main20h5165c54df1b2632dfaaE
Breakpoint 1 at 0x4834: file hello.rs, line 5.
(gdb) s
The program is not being run.
(gdb) run
Starting program: /home/steve/tmp/hello 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, hello::main () at hello.rs:5
5       let mut sum = 0;
(gdb) s
6       for i in 0..10 { // `s<CR>` lands on this line twice
(gdb) s
hello::iter::I.IntoIterator::into_iter (self=...) at ../src/libcore/iter.rs:2638
2638    ../src/libcore/iter.rs: No such file or directory.
(gdb) s
hello::iter::ops::Range<A>.Iterator::next (self=0x7fffffffddb0)
    at ../src/libcore/iter.rs:4470
4470    in ../src/libcore/iter.rs
(gdb) s
hello::cmp::impls::i32.PartialOrd::lt (self=0x7fffffffddb0, other=0x7fffffffddb4)
    at ../src/libcore/cmp.rs:470
470 ../src/libcore/cmp.rs: No such file or directory.
(gdb) s
hello::iter::ops::Range<A>.Iterator::next (self=0x7fffffffddb0)
    at ../src/libcore/iter.rs:4471
4471    ../src/libcore/iter.rs: No such file or directory.

