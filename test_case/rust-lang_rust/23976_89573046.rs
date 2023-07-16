

failures:

---- [debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs stdout ----
    NOTE: compiletest thinks it is using GDB version 7.7

error: line not found in debugger output: $2 = {<No data fields>}
status: exit code: 0
command: gdb -quiet -batch -nx -command=x86_64-unknown-linux-gnu/test/debuginfo-gdb/gdb-pretty-struct-and-enums-pre-gdb-7-7.debugger.script
stdout:
------------------------------------------
GNU gdb (Ubuntu 7.7-0ubuntu3.1) 7.7
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
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x8b9: file /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs, line 68.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, gdb_pretty_struct_and_enums_pre_gdb_7_7::main () at /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs:68
68      zzz(); // #break
$1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
$2 = EmptyStruct
$3 = CStyleEnumVar1
$4 = CStyleEnumVar2
$5 = CStyleEnumVar3
A debugging session is active.

    Inferior 1 [process 22074] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]

------------------------------------------
stderr:
------------------------------------------

------------------------------------------

thread '[debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs' panicked at 'explicit panic', /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/compiletest/runtest.rs:1483



failures:
    [debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs

test result: FAILED. 98 passed; 1 failed; 5 ignored; 0 measured

