`
running 268 tests
.iiiiiF....F..i..i.....i..i.ii.....i...i...............i...........iiiii....ii.......ii......i...... 100/268
.........ii..i.i.........i.......ii....i..i.....i........i..i.ii.....i...ii..i..i..ii.i............. 200/268
i.i.iii...ii.......ii......i...ii........i.....i..i......ii.i.......
failures:

---- [debuginfo-gdb] debuginfo/borrowed-basic.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 11002000

error: line not found in debugger output: $3 = 97
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-basic.gdb/borrowed-basic.debugger.script"
--- stdout -------------------------------
GNU gdb (GDB) 11.2
Copyright (C) 2022 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-pc-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x1423: file /home/matthias/vcs/github/rust/src/test/debuginfo/borrowed-basic.rs, line 161.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".

Breakpoint 1, borrowed_basic::main () at /home/matthias/vcs/github/rust/src/test/debuginfo/borrowed-basic.rs:161
161	    zzz(); // #break
$1 = true
$2 = -1
$3 = 97 'a'
$4 = 68
$5 = -16
$6 = -32
$7 = -64
$8 = 1
$9 = 100
$10 = 16
$11 = 32
$12 = 64
$13 = 2.5
$14 = 3.5
A debugging session is active.

	Inferior 1 [process 109076] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
------------------------------------------
stderr: none


---- [debuginfo-gdb] debuginfo/borrowed-unique-basic.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 11002000

error: line not found in debugger output: $3 = 97
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-unique-basic.gdb/borrowed-unique-basic.debugger.script"
--- stdout -------------------------------
GNU gdb (GDB) 11.2
Copyright (C) 2022 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-pc-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x37aa: file /home/matthias/vcs/github/rust/src/test/debuginfo/borrowed-unique-basic.rs, line 164.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".

Breakpoint 1, borrowed_unique_basic::main () at /home/matthias/vcs/github/rust/src/test/debuginfo/borrowed-unique-basic.rs:164
164	    zzz(); // #break
$1 = true
$2 = -1
$3 = 97 'a'
$4 = 68
$5 = -16
$6 = -32
$7 = -64
$8 = 1
$9 = 100
$10 = 16
$11 = 32
$12 = 64
$13 = 2.5
$14 = 3.5
A debugging session is active.

	Inferior 1 [process 109238] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
------------------------------------------
stderr: none



failures:
    [debuginfo-gdb] debuginfo/borrowed-basic.rs
    [debuginfo-gdb] debuginfo/borrowed-unique-basic.rs

test result: FAILED. 202 passed; 2 failed; 64 ignored; 0 measured; 0 filtered out; finished in 111.36s
