
---- [debuginfo-gdb] debuginfo-gdb/simple-struct.rs stdout ----
	NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 7011090

error: line not found in debugger output: $1 = simple_struct::NoPadding16 {x: 1000, y: -1001}
status: exit code: 0
command: /usr/bin/gdb -quiet -batch -nx -command=/home/ncameron/rust2/build/x86_64-unknown-linux-gnu/test/debuginfo/simple-struct.debugger.script
stdout:
------------------------------------------
GNU gdb (Ubuntu 7.11.90.20161005-0ubuntu1) 7.11.90.20161005-git
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
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x92e: file /home/ncameron/rust2/src/test/debuginfo/simple-struct.rs, line 233.
$1 = {x = 1000, y = -1001}
$2 = {x = 1, y = 2, z = 3}
$3 = {x = 4, y = 5, z = 6}
$4 = {a = 7, b = 8, c = 9, d = 10}
$5 = {x = 11, y = 12}
$6 = {x = 13, y = 14}
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, simple_struct::main () at /home/ncameron/rust2/src/test/debuginfo/simple-struct.rs:233
233	    zzz(); // #break
$7 = simple_struct::NoPadding16 {x: 10000, y: -10001}
$8 = simple_struct::NoPadding32 {x: -10002, y: -10003.5, z: 10004}
$9 = simple_struct::NoPadding64 {x: -10005.5, y: 10006, z: 10007}
$10 = simple_struct::NoPadding163264 {a: -10008, b: 10009, c: 10010, d: 10011}
$11 = simple_struct::InternalPadding {x: 10012, y: -10013}
$12 = simple_struct::PaddingAtEnd {x: -10014, y: 10015}
$13 = simple_struct::NoPadding16 {x: 100, y: -101}
$14 = simple_struct::NoPadding32 {x: -15, y: -16, z: 17}
$15 = simple_struct::NoPadding64 {x: -18, y: 19, z: 20}
$16 = simple_struct::NoPadding163264 {a: -21, b: 22, c: 23, d: 24}
$17 = simple_struct::InternalPadding {x: 25, y: -26}
$18 = simple_struct::PaddingAtEnd {x: -27, y: 28}
[Inferior 1 (process 13879) exited normally]

------------------------------------------
