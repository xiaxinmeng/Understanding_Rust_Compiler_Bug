plain
[00:03:11]       Memory: 8 GB
[00:03:11]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:11]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:11]       SMC Version (system): 2.8f0
[00:03:11]       Serial Number (system): VMKHzktdn9VT
[00:03:11] 
[00:03:11] hw.ncpu: 4
[00:03:11] hw.byteorder: 1234
[00:03:11] hw.memsize: 8589934592
---
[01:37:37] stdout:
[01:37:37] ------------------------------------------
[01:37:37] 
[01:37:37] running 3 tests
[01:37:37] test src/test/rustdoc/test_option_check/test.rs - Foo (line 18) ... FAILED
[01:37:37] test src/test/rustdoc/test_option_check/test.rs - Bar (line 25) ... ok
[01:37:37] test src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) ... ok
[01:37:37] failures:
[01:37:37] 
[01:37:37] 
[01:37:37] ---- src/test/rustdoc/test_option_check/test.rs - Foo (line 18) stdout ----
[01:37:37] error: linking with `cc` failed: signal: 4
[01:37:37]   |
[01:37:37]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestCsoqDD/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestCsoqDD/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestCsoqDD/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestCsoqDD/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestCsoqDD/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestCsoqDD/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestCsoqDD/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestCsoqDD/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/rustdoc/test_option_check/test/auxiliary" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-db179440fd2b4b79.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-6e384115d75c6f23.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-98c1ac1d1718c807.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc_system-dc504310042a40f1.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-59a49f6bfd4ad943.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-cdc23716f7912f40.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-cb113e4fc381e5bc.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-f97056eb2c471a0f.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[01:37:37] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:37:37] 
[01:37:37] thread 'src/test/rustdoc/test_option_check/test.rs - Foo (line 18)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:37:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:37:37] 
[01:37:37] 
[01:37:37] 
[01:37:37] failures:
[01:37:37]     src/test/rustdoc/test_option_check/test.rs - Foo (line 18)
[01:37:37] make: *** [check] Error 1
[01:37:37] test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:37:37] 
[01:37:37] 
[01:37:37] ------------------------------------------
---
[01:37:37] test result: FAILED. 271 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
[01:37:37] 
[01:37:37] 
[01:37:37] 
[01:37:37] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/rustdoc" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/rustdoc" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "rustdoc" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:37:37] 
[01:37:37] 
[01:37:37] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:37:37] Build completed unsuccessfully in 0:44:05
---
travis_fold:start:after_failure.2
travis_time:start:051bc180
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1264
drwx------  22 travis  staff    748 Nov 10 19:41 .
-rw-------@  1 travis  staff  62246 Nov 10 19:41 a_2018-11-10-194132-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37481 Nov 10 19:41 a_2018-11-10-194132_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37237 Nov 10 19:41 a_2018-11-10-194126-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60386 Nov 10 19:41 a_2018-11-10-194126_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Nov 10 19:41 a_2018-11-10-194120_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Nov 10 19:41 a_2018-11-10-194116_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Nov 10 19:41 a_2018-11-10-194108-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9795 Nov 10 19:41 a_2018-11-10-194108_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Nov 10 19:40 a_2018-11-10-194040_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63074 Nov 10 19:40 a_2018-11-10-194030_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64247 Nov 10 19:40 a_2018-11-10-194027-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65090 Nov 10 19:40 a_2018-11-10-194027-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63914 Nov 10 19:40 a_2018-11-10-194027_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11730 Nov 10 19:38 a_2018-11-10-193825_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Nov 10 19:37 a_2018-11-10-193742_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Nov 10 19:36 a_2018-11-10-193631_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10486 Nov 10 19:35 a_2018-11-10-193537-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10486 Nov 10 19:35 a_2018-11-10-193537_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10213 Nov 10 19:35 a_2018-11-10-193507_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37481 Nov 10 19:33 a_2018-11-10-193329_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:098260c6
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-11-10-193329_Traviss-Mac-1044.crash
Process:               a [57353]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [57346]
Responsible:           a [57353]
User ID:               501
Date/Time:             2018-11-10 19:33:28.028 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4500 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x000070000899ae50
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0x70000899ae50:
    __LINKEDIT             0000000117069000-0000000117084000 [  108K] r--/rwx SM=COW  /usr/lib/dyld
--> Stack Guard            000070000899a000-000070000899b000 [    4K] ---/rwx SM=NUL  
    Stack                  000070000899b000-0000700008b9d000 [ 2056K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff7aaaa05a __semwait_signal + 10
1   libsystem_pthread.dylib        0x00007fff7abe98ec _pthread_join + 626
2   a                              0x000000010cb6d05c stack_probes_lto::main::h7f9d4bbc5a99dd1a + 2988
3   a                              0x000000010cb89646 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h9b7b48d5d3baa8e1 + 6
4   a                              0x000000010cb6ea58 main + 520
5   libdyld.dylib                  0x00007fff7a95a115 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0x00007fff7aaa9e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff7abe8150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff7aa06312 abort + 127
3   a                              0x000000010cb70219 std::sys::unix::abort_internal::h6d72903006c71aee + 9
4   a                              0x000000010cb7020b std::sys_common::util::abort::h3ef233da72e3e6b7 + 91
5   a                              0x000000010cb83079 std::sys::unix::stack_overflow::imp::signal_handler::h7a22a756b224b973 + 649
6   libsystem_platform.dylib       0x00007fff7abdbf5a _sigtramp + 26
7   ???                            000000000000000000 0 + 0
8   a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
9   a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
10  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
11  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
12  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
13  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
14  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
15  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
16  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
17  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
18  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
19  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
20  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
21  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
22  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
23  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
24  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
25  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
26  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
27  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
28  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
29  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
30  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
31  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
32  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
33  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
34  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
35  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
36  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
37  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
38  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
39  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
40  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
41  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
42  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
43  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
44  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
45  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
46  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
47  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
48  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
49  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
50  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
51  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
52  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
53  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
54  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
55  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
56  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
57  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
58  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
59  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
60  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
61  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
62  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
63  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
64  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
65  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
66  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
67  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
68  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
69  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
70  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
71  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
72  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
73  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
74  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
75  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
76  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
77  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
78  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
79  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
80  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
81  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
82  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
83  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
84  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
85  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
86  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
87  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
88  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
89  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
90  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
91  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
92  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
93  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
94  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
95  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
96  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
97  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
98  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
99  a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
100 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
101 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
102 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
103 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
104 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
105 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
106 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
107 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
108 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
109 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
110 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
111 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
112 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
113 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
114 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
115 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
116 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
117 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
118 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
119 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
120 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
121 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
122 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
123 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
124 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
125 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
126 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
127 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
128 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
129 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
130 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
131 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
132 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
133 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
134 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
135 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
136 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
137 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
138 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
139 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
140 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
141 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
142 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
143 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
144 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
145 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
146 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
147 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
148 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
149 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
150 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
151 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
152 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
153 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
154 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
155 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
156 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
157 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
158 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
159 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
160 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
161 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
162 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
163 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
164 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
165 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
166 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
167 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
168 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
169 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
170 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
171 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
172 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
173 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
174 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
175 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
176 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
177 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
178 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
179 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
180 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
181 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
182 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
183 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
184 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
185 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
186 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
187 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
188 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
189 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
190 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
191 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
192 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
193 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
194 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
195 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
196 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
197 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
198 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
199 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
200 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
201 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
202 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
203 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
204 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
205 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
206 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
207 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
208 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
209 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
210 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
211 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
212 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
213 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
214 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
215 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
216 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
217 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
218 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
219 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
220 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
221 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
222 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
223 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
224 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
225 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
226 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
227 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
228 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
229 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
230 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
231 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
232 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
233 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
234 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
235 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
236 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
237 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
238 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
239 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
240 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
241 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
242 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
243 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
244 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
245 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
246 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
247 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
248 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
249 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
250 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
251 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
252 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
253 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
254 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
255 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
256 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
257 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
258 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
259 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
260 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
261 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
262 a                              0x000000010cb6d632 stack_probes_lto::recurse::h907252696a8f0ddd + 34
263 a                              0x000000010cb88a41 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h25f233cf34beec00 + 129
264 a                              0x000000010cb83468 std::sys::unix::thread::Thread::new::thread_start::hfaf875d73af09eba + 136
265 libsystem_pthread.dylib        0x00007fff7abe56c1 _pthread_body + 340
266 libsystem_pthread.dylib        0x00007fff7abe556d _pthread_start + 377
267 libsystem_pthread.dylib        0x00007fff7abe4c5d thread_start + 13
Thread 1 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x0000700008b9b000  rcx: 0x000000010cbfb968  rdx: 0x0000000000000000
  rdi: 0x0000000000000a03  rsi: 0x0000000000000006  rbp: 0x000000010cbfb9a0  rsp: 0x000000010cbfb968
   r8: 0x000000010cb987b8   r9: 0x0000000000000001  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000a03  r13: 0x00007ff3b2c02bc0  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff7aaa9e3e  rfl: 0x0000000000000206  cr2: 0x00007fffb365e148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10cb6b000 -        0x10cb9cfff +a (0) <7C8EBB4E-7C27-36CB-9A21-1568F0FBFC11> /Users/USER/*/a
       0x116fe6000 -        0x11703098f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff781c4000 -     0x7fff781f7fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff786d6000 -     0x7fff786d7ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff7898c000 -     0x7fff789e2fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff789e3000 -     0x7fff78a07ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff79d59000 -     0x7fff7a14a3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7a217000 -     0x7fff7a233ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff7a7f1000 -     0x7fff7a7f5ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff7a7f6000 -     0x7fff7a800ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff7a801000 -     0x7fff7a808fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7a809000 -     0x7fff7a811ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff7a812000 -     0x7fff7a897fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff7a91f000 -     0x7fff7a958ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7a959000 -     0x7fff7a976ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff7a977000 -     0x7fff7a977ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff7a985000 -     0x7fff7a985ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff7a986000 -     0x7fff7a98affb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff7a98b000 -     0x7fff7a98dff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff7a98e000 -     0x7fff7a98fff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff7a990000 -     0x7fff7a9a7fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7a9a8000 -     0x7fff7a9a8fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7a9a9000 -     0x7fff7aa32ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7aa33000 -     0x7fff7aa36ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7aa37000 -     0x7fff7aa3affb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7aa3b000 -     0x7fff7aa3cfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7aa3d000 -     0x7fff7aa43ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7aa44000 -     0x7fff7aa8dff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7aa8e000 -     0x7fff7aab3ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7aab4000 -     0x7fff7aafffcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7ab00000 -     0x7fff7ab1ffff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7ab20000 -     0x7fff7abc4ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7abc5000 -     0x7fff7abcfffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7abd0000 -     0x7fff7abd9ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7abda000 -     0x7fff7abe1ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7abe2000 -     0x7fff7abedfff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff7abee000 -     0x7fff7abf1ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7abf2000 -     0x7fff7abf3ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7abf4000 -     0x7fff7abfbff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7abfc000 -     0x7fff7ac0fff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7ac11000 -     0x7fff7ac16ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff7ac17000 -     0x7fff7ac43ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by all processes on this machine:
  Calls made by all processes on this machine:
    task_for_pid: 2050
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=197.6M resident=0K(0%) swapped_out_or_unallocated=197.6M(100%)
Writable regions: Total=76.5M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.5M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                          8K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            2336K       43 
__LINKEDIT                       188.6M        4 
__TEXT                            9200K       43 
===========                     =======  ======= 
TOTAL                            276.2M      108 
TOTAL                            276.2M      108 
TOTAL, minus reserved VM space   276.1M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-11-10-193507_Traviss-Mac-1044.crash
Process:               a [59757]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [59756]
Responsible:           a [59757]
User ID:               501
Date/Time:             2018-11-10 19:35:07.013 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4600 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x0000000106e8778e abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x0000000106e86b89 std::panicking::try::do_call::h8156868856d70392 (.llvm.1928227664364819278) + 9
2   libstd-db179440fd2b4b79.dylib  0x0000000106ed612f __rust_maybe_catch_panic + 31
3   a                              0x0000000106e879e1 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x0000000106e85f56 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h61837ee806a26177 + 6
5   libstd-db179440fd2b4b79.dylib  0x0000000106eb9358 std::panicking::try::do_call::h1332ae7f5ab24668 + 24
6   libstd-db179440fd2b4b79.dylib  0x0000000106ed612f __rust_maybe_catch_panic + 31
7   libstd-db179440fd2b4b79.dylib  0x0000000106eb9ceb std::rt::lang_start_internal::haa743b4d5afa6457 + 379
8   a                              0x0000000106e87cec main + 44
9   libdyld.dylib                  0x00007fff7a95a115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffd98c02bc0  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee8d78238  rsi: 0x000000001fffffff  rbp: 0x00007ffee8d78c40  rsp: 0x00007ffee8d78c40
   r8: 0x00000000d98c02c1   r9: 0x0000000000000004  r10: 0x0000000114ef98d8  r11: 0x00007fff7ac1196c
  r12: 0x0000000000000000  r13: 0x0000000000000000  r14: 0x00007ffee8d78d60  r15: 0x00007ffee8d78ca8
  rip: 0x0000000106e8778e  rfl: 0x0000000000010206  cr2: 0x0000000106ed5f30
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x106e85000 -        0x106e88fff +a (0) <DD732837-E393-342F-A83D-194D5B9A64F6> /Users/USER/*/a
       0x106e94000 -        0x106f2cff7 +libstd-db179440fd2b4b79.dylib (0) <6C6EED6F-9542-3E98-A783-98E11E5D03C3> /Users/USER/*/libstd-db179440fd2b4b79.dylib
       0x114ea7000 -        0x114ef198f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff781c4000 -     0x7fff781f7fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff786d6000 -     0x7fff786d7ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff7898c000 -     0x7fff789e2fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff789e3000 -     0x7fff78a07ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff79d59000 -     0x7fff7a14a3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7a217000 -     0x7fff7a233ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff7a7f1000 -     0x7fff7a7f5ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff7a7f6000 -     0x7fff7a800ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff7a801000 -     0x7fff7a808fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7a809000 -     0x7fff7a811ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff7a812000 -     0x7fff7a897fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff7a91f000 -     0x7fff7a958ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7a959000 -     0x7fff7a976ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff7a977000 -     0x7fff7a977ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff7a985000 -     0x7fff7a985ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff7a986000 -     0x7fff7a98affb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff7a98b000 -     0x7fff7a98dff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff7a98e000 -     0x7fff7a98fff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff7a990000 -     0x7fff7a9a7fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7a9a8000 -     0x7fff7a9a8fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7a9a9000 -     0x7fff7aa32ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7aa33000 -     0x7fff7aa36ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7aa37000 -     0x7fff7aa3affb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7aa3b000 -     0x7fff7aa3cfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7aa3d000 -     0x7fff7aa43ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7aa44000 -     0x7fff7aa8dff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7aa8e000 -     0x7fff7aab3ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7aab4000 -     0x7fff7aafffcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7ab00000 -     0x7fff7ab1ffff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7ab20000 -     0x7fff7abc4ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7abc5000 -     0x7fff7abcfffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7abd0000 -     0x7fff7abd9ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7abda000 -     0x7fff7abe1ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7abe2000 -     0x7fff7abedfff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
