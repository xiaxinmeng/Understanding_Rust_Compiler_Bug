plain
 finished in 14.082 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 141 tests
iiiiii...i...iii.......i..i........F....F..i............iiiii....ii.........i......i.... 88/141
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [debuginfo-gdb] src/test/debuginfo/empty-string.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/empty-string.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12000090

error: line not found in debugger output: $1 = ""
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/empty-string.gdb/empty-string.debugger.script"
GNU gdb (Ubuntu 12.0.90-0ubuntu1) 12.0.90
Copyright (C) 2022 Free Software Foundation, Inc.
Copyright (C) 2022 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x1b21: file /checkout/src/test/debuginfo/empty-string.rs, line 33.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, empty_string::main () at /checkout/src/test/debuginfo/empty-string.rs:33
33     zzz(); // #break
$1 = alloc::string::String<alloc::alloc::Global> {vec: Vec(size=0)}
$2 = ""
A debugging session is active.

 Inferior 1 [process 126824] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
stderr: none


---- [debuginfo-gdb] src/test/debuginfo/embedded-visualizer.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/embedded-visualizer.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12000090

error: line not found in debugger output: $4 = "Person A" is 10 years old.
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/embedded-visualizer.gdb/embedded-visualizer.debugger.script"
GNU gdb (Ubuntu 12.0.90-0ubuntu1) 12.0.90
Copyright (C) 2022 Free Software Foundation, Inc.
Copyright (C) 2022 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x1ec9: file /checkout/src/test/debuginfo/embedded-visualizer.rs, line 107.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, embedded_visualizer::main () at /checkout/src/test/debuginfo/embedded-visualizer.rs:107
107     zzz(); // #break
Loaded  Script                                                                 
Yes     gdb_load_rust_pretty_printers.py                                       
 full name: /checkout/src/etc/gdb_load_rust_pretty_printers.py
Yes     pretty-printer-embedded_visualizer-0                                   
Yes     pretty-printer-embedded_visualizer-1                                   
Yes     pretty-printer-embedded_visualizer-2                                   
$1 = (0, 0)
$2 = (5, 8)
$3 = ((0, 0), (5, 8))
$4 = alloc::string::String<alloc::alloc::Global> {vec: Vec(size=8) = {80, 101, 114, 115, 111, 110, 32, 65}} is 10 years old.
A debugging session is active.

 Inferior 1 [process 126918] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
stderr: none



