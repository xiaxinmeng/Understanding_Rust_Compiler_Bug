

failures:

---- [debuginfo-gdb] debuginfo-gdb/pretty-huge-vec.rs stdout ----
    NOTE: compiletest thinks it is using GDB version 7.11

error: line not found in debugger output: $1 = Vec<u8>(len: 1000000000, cap: 1000000000) = {[...]...}
status: exit code: 0
command: gdb -quiet -batch -nx -command=x86_64-unknown-linux-gnu/test/debuginfo-gdb/pretty-huge-vec.debugger.script
stdout:
------------------------------------------
GNU gdb (GDB) 7.11
Copyright (C) 2016 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-pc-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0xcac: file /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/tmp/distcheck/rustc-nightly/src/test/debuginfo/pretty-huge-vec.rs, line 38.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".

Breakpoint 1, pretty_huge_vec::main () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/tmp/distcheck/rustc-nightly/src/test/debuginfo/pretty-huge-vec.rs:38
38      zzz(); // #break
$1 = Vec<u8>(len: 1000000000, cap: 1000000000)
$2 = &[u8](len: 1000000000)
A debugging session is active.

    Inferior 1 [process 25786] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]

------------------------------------------
stderr:
------------------------------------------
Python Exception <type 'exceptions.MemoryError'> : 
Python Exception <type 'exceptions.MemoryError'> : 

------------------------------------------

thread '[debuginfo-gdb] debuginfo-gdb/pretty-huge-vec.rs' panicked at 'explicit panic', /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/tmp/distcheck/rustc-nightly/src/tools/compiletest/src/runtest.rs:2243
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [debuginfo-gdb] debuginfo-gdb/pretty-huge-vec.rs

test result: FAILED. 100 passed; 1 failed; 3 ignored; 0 measured
