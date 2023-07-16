plain
test [debuginfo-gdb] src/test/debuginfo/cross-crate-type-uniquing.rs ... ok

failures:

---- [debuginfo-gdb] src/test/debuginfo/no_mangle-info.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001
error: line not found in debugger output: $2 = 42
status: exit status: 0
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/debuginfo/no_mangle-info.gdb/no_mangle-info.debugger.script"
GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
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
Breakpoint 1 at 0xbbd: file src/test/debuginfo/no_mangle-info.rs, line 38.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".


Breakpoint 1, no_mangle_info::main::hb3b9c5efc50504a9 () at src/test/debuginfo/no_mangle-info.rs:38
38     println!("OTHER TEST: {}", namespace::OTHER_TEST); // #break
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/debuginfo/no_mangle-info.gdb/no_mangle-info.debugger.script:11: Error in sourced command file:
No type "namespace" within class or namespace "no_mangle_info".



failures:
failures:
    [debuginfo-gdb] src/test/debuginfo/no_mangle-info.rs

test result: FAILED. 85 passed; 1 failed; 54 ignored; 0 measured; 0 filtered out; finished in 4.48s

Build completed unsuccessfully in 0:18:18
make: *** [check] Error 1
Makefile:42: recipe for target 'check' failed
