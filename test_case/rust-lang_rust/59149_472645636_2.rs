
GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
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
Reading symbols from target/debug/experiment...done.
(gdb) b 21
Breakpoint 1 at 0x621f: file src/main.rs, line 21.
(gdb) r
Starting program: /home/ybli/Projects/tmp/experiment/target/debug/experiment 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, experiment::main::hb5ade26289f83d6e () at src/main.rs:21
21	    temp();
(gdb) p foo
$1 = generator = {B, 3}
(gdb) q
A debugging session is active.

	Inferior 1 [process 8255] will be killed.

Quit anyway? (y or n) y

