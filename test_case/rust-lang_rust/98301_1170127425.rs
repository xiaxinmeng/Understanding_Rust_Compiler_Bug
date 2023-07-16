plain
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...F.....ii..i.i..i.......i........i...............
failures:

---- [debuginfo-gdb] src/test/debuginfo/numeric-types.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 9002000

error: line not found in debugger output: [...]$1 = 11 '\v'
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/numeric-types.gdb/numeric-types.debugger.script"
GNU gdb (Ubuntu 9.2-0ubuntu1~20.04.1) 9.2
Copyright (C) 2020 Free Software Foundation, Inc.
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x1a00: file /checkout/src/test/debuginfo/numeric-types.rs, line 288.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
Breakpoint 1, numeric_types::main () at /checkout/src/test/debuginfo/numeric-types.rs:288
Breakpoint 1, numeric_types::main () at /checkout/src/test/debuginfo/numeric-types.rs:288
288     zzz(); // #break
$2 = 22
$3 = 33
$4 = 44
$5 = 55
---
$11 = 111
$12 = 122
A debugging session is active.

 Inferior 1 [process 125260] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
stderr: none



