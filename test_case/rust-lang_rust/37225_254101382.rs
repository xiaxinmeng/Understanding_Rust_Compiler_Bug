
---- [debuginfo-gdb] debuginfo-gdb/macro-stepping.rs stdout ----
    NOTE: compiletest thinks it is using GDB version 7.11

error: line not found in debugger output: [...]#loc6[...]
status: exit code: 0
command: gdb -quiet -batch -nx -command=aarch64-unknown-linux-gnu/test/debuginfo-gdb/macro-stepping.debugger.script
stdout:
------------------------------------------
GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.04) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "aarch64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x19dc: file /home/emv/src/rust-lang/rust/src/test/debuginfo/macro-stepping.rs, line 87.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/aarch64-linux-gnu/libthread_db.so.1".

Breakpoint 1, macro_stepping::main () at /home/emv/src/rust-lang/rust/src/test/debuginfo/macro-stepping.rs:87
87      zzz(); // #break
89      foo!(); // #loc1
#0  macro_stepping::main () at /home/emv/src/rust-lang/rust/src/test/debuginfo/macro-stepping.rs:89
89      foo!(); // #loc1
91      foo2!(); // #loc2
#0  macro_stepping::main () at /home/emv/src/rust-lang/rust/src/test/debuginfo/macro-stepping.rs:91
91      foo2!(); // #loc2
93      let x = vec![42]; // #loc3
#0  macro_stepping::main () at /home/emv/src/rust-lang/rust/src/test/debuginfo/macro-stepping.rs:93
93      let x = vec![42]; // #loc3
93      let x = vec![42]; // #loc3
#0  macro_stepping::main () at /home/emv/src/rust-lang/rust/src/test/debuginfo/macro-stepping.rs:93
93      let x = vec![42]; // #loc3
95      new_scope!(); // #loc4
#0  macro_stepping::main () at /home/emv/src/rust-lang/rust/src/test/debuginfo/macro-stepping.rs:95
95      new_scope!(); // #loc4
97      println!("Hello {}", // #loc5
#0  macro_stepping::main () at /home/emv/src/rust-lang/rust/src/test/debuginfo/macro-stepping.rs:97
97      println!("Hello {}", // #loc5
A debugging session is active.

    Inferior 1 [process 7811] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]

------------------------------------------
stderr:
------------------------------------------

------------------------------------------

thread '[debuginfo-gdb] debuginfo-gdb/macro-stepping.rs' panicked at 'explicit panic', /home/emv/src/rust-lang/rust/src/tools/compiletest/src/runtest.rs:2372
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
[debuginfo-gdb] debuginfo-gdb/macro-stepping.rs
