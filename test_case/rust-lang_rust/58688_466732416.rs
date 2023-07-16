plain
[01:06:10] test [debuginfo-both] debuginfo/vec.rs ... ignored
[01:06:10] 
[01:06:10] failures:
[01:06:10] 
[01:06:10] ---- [debuginfo-both] debuginfo/pretty-std.rs stdout ----
[01:06:10] NOTE: compiletest thinks it is using GDB without native rust support
[01:06:10] NOTE: compiletest thinks it is using GDB version 7011001
[01:06:10] 
[01:06:10] error: line not found in debugger output: $5 = Some = {8}
[01:06:10] status: exit code: 0
[01:06:10] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std/pretty-std.debugger.script"
[01:06:10] ------------------------------------------
[01:06:10] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:06:10] Copyright (C) 2016 Free Software Foundation, Inc.
[01:06:10] Copyright (C) 2016 Free Software Foundation, Inc.
[01:06:10] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:06:10] This is free software: you are free to change and redistribute it.
[01:06:10] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:06:10] and "show warranty" for details.
[01:06:10] This GDB was configured as "x86_64-linux-gnu".
[01:06:10] Type "show configuration" for configuration details.
[01:06:10] For bug reporting instructions, please see:
[01:06:10] <http://www.gnu.org/software/gdb/bugs/>.
[01:06:10] Find the GDB manual and other documentation resources online at:
[01:06:10] <http://www.gnu.org/software/gdb/documentation/>.
[01:06:10] For help, type "help".
[01:06:10] Type "apropos word" to search for commands related to "word".
[01:06:10] Breakpoint 1 at 0x401b56: file /checkout/src/test/debuginfo/pretty-std.rs, line 103.
[01:06:10] 
[01:06:10] Breakpoint 1, pretty_std::main::hd868472729f88994 () at /checkout/src/test/debuginfo/pretty-std.rs:103
[01:06:10] 103     zzz(); // #break
[01:06:10] $1 = &[i32](len: 4) = {0, 1, 2, 3}
[01:06:10] $2 = Vec<u64>(len: 4, cap: 4) = {4, 5, 6, 7}
[01:06:10] $3 = "IAMA string slice!"
[01:06:10] $4 = "IAMA string!"
[01:06:10] $5 = Option<i16>
[01:06:10] $6 = Option<i64>
[01:06:10] $7 = "IAMA OS string"
[01:06:10] $8 = Option<alloc::string::String>
[01:06:10] $9 = Option<alloc::string::String>
[01:06:10] $10 = ""
[01:06:10] A debugging session is active.
[01:06:10] 
[01:06:10]  Inferior 1 [process 16634] will be killed.
[01:06:10] 
[01:06:10] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:06:10] ------------------------------------------
[01:06:10] stderr:
[01:06:10] ------------------------------------------
[01:06:10] 
---
[01:06:10] test result: FAILED. 74 passed; 1 failed; 44 ignored; 0 measured; 0 filtered out
[01:06:10] 
[01:06:10] 
[01:06:10] 
[01:06:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--mode" "debuginfo-both" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-x86_64/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:10] 
[01:06:10] 
[01:06:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target x86_64-unknown-linux-musl
[01:06:10] Build completed unsuccessfully in 1:02:46
---
travis_time:end:0bc44bcf:start=1550981753556639947,finish=1550981753584075953,duration=27436006
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14baa087
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0250215f
travis_time:start:0250215f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01935978
$ dmesg | grep -i kill
