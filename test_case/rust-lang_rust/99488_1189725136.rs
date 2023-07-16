plain
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..........ii..i.i..i.......i........i..........F....
failures:

---- [debuginfo-gdb] src/test/debuginfo/vec-slices.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 9002000

error: gdb failed to execute
status: exit status: 1
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/vec-slices.gdb/vec-slices.debugger.script"
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
Breakpoint 1 at 0x149b: file /checkout/src/test/debuginfo/vec-slices.rs, line 126.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, vec_slices::main () at /checkout/src/test/debuginfo/vec-slices.rs:126
126     let mut_slice: &mut [i64] = &mut [1, 2, 3, 4, 5];
$2 = 1
$3 = [1]
$4 = 4
$5 = [2, 3, 4, 5]
$5 = [2, 3, 4, 5]
$6 = 2
$7 = [3, 4]
$8 = 2
$9 = (6, 7)
$10 = (8, 9)
$11 = 2
$12 = vec_slices::AStruct {x: 10, y: 11, z: 12}
$13 = vec_slices::AStruct {x: 13, y: 14, z: 15}
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/vec-slices.gdb/vec-slices.debugger.script:24: Error in sourced command file:
No symbol 'mut_slice' in current context



failures:
