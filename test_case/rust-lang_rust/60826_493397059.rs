plain
travis_time:end:1cfa0d18:start=1558082053568313330,finish=1558082142100003668,duration=88531690338
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:54] 
[01:19:54] running 143 tests
[01:19:56] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:19:58] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:19:58] 
[01:19:58]  finished in 4.709
[01:19:58] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:00] 
[01:20:00] running 9 tests
[01:20:00] iiiiiiiii
[01:20:00] 
[01:20:00]  finished in 0.150
[01:20:00] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:16] 
[01:20:16] running 122 tests
[01:20:41] .iiiii...i.....i..i...i..i.iFi..i.iiF.i.i.....i..i....i..........iiii..........i..FiiF..i.......ii.i 100/122
[01:20:46] .i.i......iii.i.....ii
[01:20:46] 
[01:20:46] ---- [debuginfo-both] debuginfo/empty-string.rs stdout ----
[01:20:46] ---- [debuginfo-both] debuginfo/empty-string.rs stdout ----
[01:20:46] NOTE: compiletest thinks it is using GDB without native rust support
[01:20:46] NOTE: compiletest thinks it is using GDB version 7011001
[01:20:46] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:20:46] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:20:46] error: line not found in debugger output: $1 = ""
[01:20:46] status: exit code: 0
[01:20:46] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/empty-string/empty-string.debugger.script"
[01:20:46] ------------------------------------------
[01:20:46] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:20:46] Copyright (C) 2016 Free Software Foundation, Inc.
[01:20:46] Copyright (C) 2016 Free Software Foundation, Inc.
[01:20:46] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:20:46] This is free software: you are free to change and redistribute it.
[01:20:46] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:20:46] and "show warranty" for details.
[01:20:46] This GDB was configured as "x86_64-linux-gnu".
[01:20:46] Type "show configuration" for configuration details.
[01:20:46] For bug reporting instructions, please see:
[01:20:46] <http://www.gnu.org/software/gdb/bugs/>.
[01:20:46] Find the GDB manual and other documentation resources online at:
[01:20:46] <http://www.gnu.org/software/gdb/documentation/>.
[01:20:46] For help, type "help".
[01:20:46] Type "apropos word" to search for commands related to "word".
[01:20:46] Breakpoint 1 at 0xf0f: file /checkout/src/test/debuginfo/empty-string.rs, line 32.
[01:20:46] [Thread debugging using libthread_db enabled]
[01:20:46] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:20:46] 
[01:20:46] Breakpoint 1, empty_string::main::h4739d54fd70936a5 () at /checkout/src/test/debuginfo/empty-string.rs:32
[01:20:46] 32     zzz(); // #break
[01:20:46] $1 = Rust type: struct String
[01:20:46] Struct name: String
[01:20:46] ""
[01:20:46] $2 = Rust type: struct &str
[01:20:46] Struct name: &str
[01:20:46] ""
[01:20:46] A debugging session is active.
[01:20:46] 
[01:20:46]  Inferior 1 [process 32168] will be killed.
[01:20:46] 
[01:20:46] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:20:46] ------------------------------------------
[01:20:46] stderr:
[01:20:46] ------------------------------------------
[01:20:46] 
[01:20:46] 
[01:20:46] ------------------------------------------
[01:20:46] 
[01:20:46] 
[01:20:46] ---- [debuginfo-both] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
[01:20:46] NOTE: compiletest thinks it is using GDB without native rust support
[01:20:46] NOTE: compiletest thinks it is using GDB version 7011001
[01:20:46] 
[01:20:46] error: line not found in debugger output: $1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
[01:20:46] failed to decode compiler output as json: line: {the_first_field = Rust type: isize
[01:20:46] output: GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:20:46] Copyright (C) 2016 Free Software Foundation, Inc.
[01:20:46] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:20:46] This is free software: you are free to change and redistribute it.
[01:20:46] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:20:46] and "show warranty" for details.
[01:20:46] This GDB was configured as "x86_64-linux-gnu".
[01:20:46] Type "show configuration" for configuration details.
[01:20:46] For bug reporting instructions, please see:
[01:20:46] <http://www.gnu.org/software/gdb/bugs/>.
[01:20:46] Find the GDB manual and other documentation resources online at:
[01:20:46] <http://www.gnu.org/software/gdb/documentation/>.
[01:20:46] For help, type "help".
[01:20:46] Type "apropos word" to search for commands related to "word".
[01:20:46] Breakpoint 1 at 0xadf: file /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs, line 60.
[01:20:46] [Thread debugging using libthread_db enabled]
[01:20:46] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:20:46] 
[01:20:46] Breakpoint 1, gdb_pretty_struct_and_enums::main::hb77f1d849dda9a93 () at /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs:60
[01:20:46] 60     zzz(); // #break
[01:20:46] $1 = Rust type: struct RegularStruct
[01:20:46] Struct name: RegularStruct
[01:20:46] {the_first_field = Rust type: isize
[01:20:46] 101, the_second_field = Rust type: f64
[01:20:46] 102.5, the_third_field = Rust type: bool
[01:20:46] false}
[01:20:46] $2 = Rust type: struct EmptyStruct
[01:20:46] Struct name: EmptyStruct
[01:20:46] Rust type: struct EmptyStruct
[01:20:46] Struct name: EmptyStruct
[01:20:46] {<No data fields>}
[01:20:46] $3 = Rust type: enum class CStyleEnum
[01:20:46] Rust type: enum class CStyleEnum
[01:20:46] CStyleEnumVar1
[01:20:46] $4 = Rust type: enum class CStyleEnum
[01:20:46] Rust type: enum class CStyleEnum
[01:20:46] CStyleEnumVar2
[01:20:46] $5 = Rust type: enum class CStyleEnum
[01:20:46] Rust type: enum class CStyleEnum
[01:20:46] CStyleEnumVar3
[01:20:46] A debugging session is active.
[01:20:46] 
[01:20:46]  Inferior 1 [process 32241] will be killed.
[01:20:46] 
[01:20:46] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:20:46] thread 'main' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:87:21
[01:20:46] 
[01:20:46] ---- [debuginfo-both] debuginfo/pretty-huge-vec.rs stdout ----
[01:20:46] ---- [debuginfo-both] debuginfo/pretty-huge-vec.rs stdout ----
[01:20:46] NOTE: compiletest thinks it is using GDB without native rust support
[01:20:46] NOTE: compiletest thinks it is using GDB version 7011001
[01:20:46] 
[01:20:46] error: line not found in debugger output: $1 = Vec<u8>(len: 1000000000, cap: 1000000000) = {[...]...}
[01:20:46] failed to decode compiler output as json: line: {buf = Rust type: struct RawVec<u8, alloc::alloc::Global>
[01:20:46] output: GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:20:46] Copyright (C) 2016 Free Software Foundation, Inc.
[01:20:46] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:20:46] This is free software: you are free to change and redistribute it.
[01:20:46] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:20:46] and "show warranty" for details.
[01:20:46] This GDB was configured as "x86_64-linux-gnu".
[01:20:46] Type "show configuration" for configuration details.
[01:20:46] For bug reporting instructions, please see:
[01:20:46] <http://www.gnu.org/software/gdb/bugs/>.
[01:20:46] Find the GDB manual and other documentation resources online at:
[01:20:46] <http://www.gnu.org/software/gdb/documentation/>.
[01:20:46] For help, type "help".
[01:20:46] Type "apropos word" to search for commands related to "word".
[01:20:46] Breakpoint 1 at 0x17bc: file /checkout/src/test/debuginfo/pretty-huge-vec.rs, line 28.
[01:20:46] [Thread debugging using libthread_db enabled]
[01:20:46] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:20:46] 
[01:20:46] Breakpoint 1, pretty_huge_vec::main::h724d3ae1bb5250fc () at /checkout/src/test/debuginfo/pretty-huge-vec.rs:28
[01:20:46] 28     zzz(); // #break
[01:20:46] $1 = Rust type: struct Vec<u8>
[01:20:46] Struct name: Vec<u8>
[01:20:46] {buf = Rust type: struct RawVec<u8, alloc::alloc::Global>
[01:20:46] Struct name: RawVec<u8, alloc::alloc::Global>
[01:20:46] {ptr = Rust type: struct Unique<u8>
[01:20:46] Struct name: Unique<u8>
[01:20:46] {pointer = Rust type: u8 *
[01:20:46] 0x7fffbb511010 "", _marker = Rust type: struct PhantomData<u8>
[01:20:46] Struct name: PhantomData<u8>
[01:20:46] {<No data fields>}}, cap = Rust type: usize
[01:20:46] 1000000000, a = Rust type: struct Global
[01:20:46] Struct name: Global
[01:20:46] {<No data fields>}}, len = Rust type: usize
[01:20:46] 1000000000}
[01:20:46] $2 = Rust type: struct &[u8]
[01:20:46] Struct name: &[u8]
[01:20:46] {data_ptr = Rust type: u8 *
[01:20:46] 0x7fffbb511010 "", length = Rust type: usize
[01:20:46] 1000000000}
[01:20:46] A debugging session is active.
[01:20:46] 
[01:20:46]  Inferior 1 [process 473] will be killed.
[01:20:46] 
[01:20:46] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:20:46] thread 'main' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:87:21
[01:20:46] ---- [debuginfo-both] debuginfo/pretty-uninitialized-vec.rs stdout ----
[01:20:46] ---- [debuginfo-both] debuginfo/pretty-uninitialized-vec.rs stdout ----
[01:20:46] NOTE: compiletest thinks it is using GDB without native rust support
[01:20:46] NOTE: compiletest thinks it is using GDB version 7011001
[01:20:46] 
[01:20:46] error: line not found in debugger output: $1 = Vec<i32>(len: [...], cap: [...])[...]
[01:20:46] failed to decode compiler output as json: line: {buf = Rust type: struct RawVec<i32, alloc::alloc::Global>
[01:20:46] output: GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:20:46] Copyright (C) 2016 Free Software Foundation, Inc.
[01:20:46] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:20:46] This is free software: you are free to change and redistribute it.
[01:20:46] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:20:46] and "show warranty" for details.
[01:20:46] This GDB was configured as "x86_64-linux-gnu".
[01:20:46] Type "show configuration" for configuration details.
[01:20:46] For bug reporting instructions, please see:
[01:20:46] <http://www.gnu.org/software/gdb/bugs/>.
[01:20:46] Find the GDB manual and other documentation resources online at:
[01:20:46] <http://www.gnu.org/software/gdb/documentation/>.
[01:20:46] For help, type "help".
[01:20:46] Type "apropos word" to search for commands related to "word".
[01:20:46] Breakpoint 1 at 0x1ac0: file /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs, line 21.
[01:20:46] [Thread debugging using libthread_db enabled]
[01:20:46] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:20:46] 
[01:20:46] Breakpoint 1, pretty_uninitialized_vec::main::h8ad4cf0b9701f4b8 () at /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs:21
[01:20:46] 21     zzz(); // #break
[01:20:46] $1 = Rust type: struct Vec<i32>
[01:20:46] Struct name: Vec<i32>
[01:20:46] {buf = Rust type: struct RawVec<i32, alloc::alloc::Global>
[01:20:46] Struct name: RawVec<i32, alloc::alloc::Global>
[01:20:46] {ptr = Rust type: struct Unique<i32>
[01:20:46] Struct name: Unique<i32>
[01:20:46] {pointer = Rust type: i32 *
[01:20:46] 0x1, _marker = Rust type: struct PhantomData<i32>
[01:20:46] Struct name: PhantomData<i32>
[01:20:46] {<No data fields>}}, cap = Rust type: usize
[01:20:46] 140737479958528, a = Rust type: struct Global
[01:20:46] Struct name: Global
[01:20:46] {<No data fields>}}, len = Rust type: usize
[01:20:46] 140737488348544}
[01:20:46] A debugging session is active.
[01:20:46] 
[01:20:46]  Inferior 1 [process 502] will be killed.
[01:20:46] 
[01:20:46] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:20:46] thread 'main' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:87:21
[01:20:46] 
[01:20:46] failures:
[01:20:46]     [debuginfo-both] debuginfo/empty-string.rs
[01:20:46]     [debuginfo-both] debuginfo/gdb-pretty-struct-and-enums.rs
[01:20:46]     [debuginfo-both] debuginfo/gdb-pretty-struct-and-enums.rs
[01:20:46]     [debuginfo-both] debuginfo/pretty-huge-vec.rs
[01:20:46]     [debuginfo-both] debuginfo/pretty-uninitialized-vec.rs
[01:20:46] 
[01:20:46] test result: FAILED. 79 passed; 4 failed; 39 ignored; 0 measured; 0 filtered out
[01:20:46] 
[01:20:46] 
[01:20:46] 
[01:20:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-both" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:20:46] 
[01:20:46] 
[01:20:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:46] Build completed unsuccessfully in 0:12:48
[01:20:46] Build completed unsuccessfully in 0:12:48
[01:20:46] make: *** [check] Error 1
[01:20:46] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d8a6180
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 17 09:56:37 UTC 2019
---
travis_time:end:0d003f40:start=1558086999293320905,finish=1558086999360940441,duration=67619536
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:043ab47f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c6e59d4
$ dmesg | grep -i kill
