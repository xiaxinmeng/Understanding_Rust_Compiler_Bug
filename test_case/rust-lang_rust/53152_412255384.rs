
[01:14:45] failures:
[01:14:45] 
[01:14:45] ---- [debuginfo-gdb] debuginfo/drop-locations.rs stdout ----
[01:14:45] NOTE: compiletest thinks it is using GDB without native rust support
[01:14:45] NOTE: compiletest thinks it is using GDB version 7011001
[01:14:45] 
[01:14:45] error: line not found in debugger output: [...]#loc1[...]
[01:14:45] status: exit code: 0
[01:14:45] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/drop-locations/drop-locations.debugger.script"
[01:14:45] stdout:
[01:14:45] ------------------------------------------
[01:14:45] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:14:45] Copyright (C) 2016 Free Software Foundation, Inc.
[01:14:45] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:14:45] This is free software: you are free to change and redistribute it.
[01:14:45] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:14:45] and "show warranty" for details.
[01:14:45] This GDB was configured as "x86_64-linux-gnu".
[01:14:45] Type "show configuration" for configuration details.
[01:14:45] For bug reporting instructions, please see:
[01:14:45] <http://www.gnu.org/software/gdb/bugs/>.
[01:14:45] Find the GDB manual and other documentation resources online at:
[01:14:45] <http://www.gnu.org/software/gdb/documentation/>.
[01:14:45] For help, type "help".
[01:14:45] Type "apropos word" to search for commands related to "word".
[01:14:45] Breakpoint 1 at 0x11c6: file /checkout/src/test/debuginfo/drop-locations.rs, line 81.
[01:14:45] [Thread debugging using libthread_db enabled]
[01:14:45] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:14:45] 
[01:14:45] Breakpoint 1, drop_locations::foo::h567f523ca0f8927c () at /checkout/src/test/debuginfo/drop-locations.rs:81
[01:14:45] 81	        let s = String::from("s"); // #break
[01:14:45] 0x565556a8 in ?? ()
[01:14:45] #0  0x565556a8 in ?? ()
[01:14:45] 
[01:14:45] ------------------------------------------
[01:14:45] stderr:
[01:14:45] ------------------------------------------
[01:14:45] /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/drop-locations/drop-locations.debugger.script:11: Error in sourced command file:
[01:14:45] Cannot find bounds of current function
[01:14:45] 
[01:14:45] ------------------------------------------
[01:14:45] 
[01:14:45] thread '[debuginfo-gdb] debuginfo/drop-locations.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:14:45] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:14:45] 
[01:14:45] 
[01:14:45] failures:
[01:14:45]     [debuginfo-gdb] debuginfo/drop-locations.rs
[01:14:45] 
[01:14:45] test result: [31mFAILED(B[m. 85 passed; 1 failed; 23 ignored; 0 measured; 0 filtered out
