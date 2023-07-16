plain
[00:48:59] ....................................................................................................
[00:49:02] ................................................................i...................................
[00:49:05] ....................................................................................................
[00:49:08] ....................................................................................................
[00:49:11] .............iiiiiiiii..............................................................................
[00:49:16] ....................................................................................................
[00:49:20] .................................................................................................i..
[00:49:22] ....................................................................................................
[00:49:25] .........................................................i.i..ii....................................
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:10] 
[01:02:10] running 110 tests
[01:02:21] iiii.......i..i........i..i.i.............i..........iiii...........i.F..i..........ii.i.i.......ii.
[01:02:22] failures:
[01:02:22] 
[01:02:22] ---- [debuginfo-gdb] debuginfo/nil-enum.rs stdout ----
[01:02:22] ---- [debuginfo-gdb] debuginfo/nil-enum.rs stdout ----
[01:02:22] NOTE: compiletest thinks it is using GDB without native rust support
[01:02:22] NOTE: compiletest thinks it is using GDB version 7011001
[01:02:22] 
[01:02:22] error: line not found in debugger output: $1 = {<No data fields>}
[01:02:22] status: exit code: 0
[01:02:22] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/nil-enum/nil-enum.debugger.script"
[01:02:22] ------------------------------------------
[01:02:22] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:02:22] Copyright (C) 2016 Free Software Foundation, Inc.
[01:02:22] Copyright (C) 2016 Free Software Foundation, Inc.
[01:02:22] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:02:22] This is free software: you are free to change and redistribute it.
[01:02:22] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:02:22] and "show warranty" for details.
[01:02:22] This GDB was configured as "x86_64-linux-gnu".
[01:02:22] Type "show configuration" for configuration details.
[01:02:22] For bug reporting instructions, please see:
[01:02:22] <http://www.gnu.org/software/gdb/bugs/>.
[01:02:22] Find the GDB manual and other documentation resources online at:
[01:02:22] <http://www.gnu.org/software/gdb/documentation/>.
[01:02:22] For help, type "help".
[01:02:22] Type "apropos word" to search for commands related to "word".
travis_time:end:0a8db558:start=1538226795790682033,finish=1538230538538590360,duration=3742747908327

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0da2fca0
---
travis_time:end:0bc9fab7:start=1538230540501494873,finish=1538230540669280206,duration=167785333
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05b2855e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:056d3e55
$ dmesg | grep -i kill
