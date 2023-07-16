plain
[00:03:22]       Memory: 8 GB
[00:03:22]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:22]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:22]       SMC Version (system): 2.8f0
[00:03:22]       Serial Number (system): VMqiflzg1SV4
[00:03:22] 
[00:03:22] hw.ncpu: 4
[00:03:22] hw.byteorder: 1234
[00:03:22] hw.memsize: 8589934592
---
[01:36:51] stdout:
[01:36:51] ------------------------------------------
[01:36:51] 
[01:36:51] running 3 tests
[01:36:51] test src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) ... FAILED
[01:36:51] test src/test/rustdoc/test_option_check/test.rs - Bar (line 25) ... ok
[01:36:51] test src/test/rustdoc/test_option_check/test.rs - Foo (line 18) ... ok
[01:36:51] failures:
[01:36:51] 
[01:36:51] 
[01:36:51] ---- src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) stdout ----
[01:36:51] error: linking with `cc` failed: signal: 4
[01:36:51]   |
[01:36:51]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnQL5fq/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnQL5fq/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnQL5fq/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnQL5fq/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnQL5fq/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnQL5fq/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnQL5fq/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnQL5fq/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/rustdoc/test_option_check/test/auxiliary" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-f7dd09cf573f28a7.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-8dbebee0d2595683.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc_jemalloc-3b871a6380e0d65e.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-4171cdd997d1e3e4.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc_system-d336a120b1902c73.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-745c32f2fe064e3d.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-1f42cebe2f2f5d37.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-3e5cc53f3835b20a.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-3048cc32a441c3bf.rlib" "-lSystem" "-lresolv" "-lpthread" "-lc" "-lm"
[01:36:51] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[01:36:51] 
[01:36:51] 
[01:36:51] thread 'src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:36:51] 
[01:36:51] 
[01:36:51] failures:
[01:36:51] failures:
[01:36:51]     src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16)
[01:36:51] test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:36:51] 
[01:36:51] 
[01:36:51] 
[01:36:51] make: *** [check] Error 1
[01:36:51] stderr:
[01:36:51] ------------------------------------------
[01:36:51] make: INTERNAL: Exiting with 1 jobserver tokens available; should be 4!
[01:36:51] 
---
[01:36:51] test result: FAILED. 266 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
[01:36:51] 
[01:36:51] 
[01:36:51] 
[01:36:51] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/rustdoc" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/rustdoc" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "rustdoc" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:36:51] 
[01:36:51] 
[01:36:51] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:36:51] Build completed unsuccessfully in 0:41:44
---
travis_fold:start:after_failure.2
travis_time:start:11520993
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1264
drwx------  22 travis  staff    748 Oct 17 22:58 .
-rw-------@  1 travis  staff  37510 Oct 17 22:58 a_2018-10-17-225846-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62275 Oct 17 22:58 a_2018-10-17-225846_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37409 Oct 17 22:58 a_2018-10-17-225842-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60417 Oct 17 22:58 a_2018-10-17-225842_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10170 Oct 17 22:58 a_2018-10-17-225837_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9901 Oct 17 22:58 a_2018-10-17-225833_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9906 Oct 17 22:58 a_2018-10-17-225825-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9845 Oct 17 22:58 a_2018-10-17-225825_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Oct 17 22:57 a_2018-10-17-225758_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63186 Oct 17 22:57 a_2018-10-17-225749_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65147 Oct 17 22:57 a_2018-10-17-225747-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63943 Oct 17 22:57 a_2018-10-17-225747-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64334 Oct 17 22:57 a_2018-10-17-225747_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11757 Oct 17 22:55 a_2018-10-17-225545_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9925 Oct 17 22:55 a_2018-10-17-225501_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10332 Oct 17 22:53 a_2018-10-17-225351_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10514 Oct 17 22:52 a_2018-10-17-225258-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10512 Oct 17 22:52 a_2018-10-17-225258_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10241 Oct 17 22:52 a_2018-10-17-225230_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37510 Oct 17 22:50 a_2018-10-17-225052_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:18f27f6d
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-10-17-225052_Traviss-Mac-1044.crash
Process:               a [51573]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [51569]
Responsible:           a [51573]
User ID:               501
Date/Time:             2018-10-17 22:50:51.315 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4500 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x0000700000c76e30
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0x700000c76e30:
    __LINKEDIT             000000010c161000-000000010c17c000 [  108K] r--/rwx SM=COW  /usr/lib/dyld
--> Stack Guard            0000700000c76000-0000700000c77000 [    4K] ---/rwx SM=NUL  
    Stack                  0000700000c77000-0000700000e79000 [ 2056K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff6222505a __semwait_signal + 10
1   libsystem_pthread.dylib        0x00007fff623648ec _pthread_join + 626
2   a                              0x0000000100f933b9 stack_probes_lto::main::h46e0341d7865a006 + 2841
3   a                              0x0000000100fb0376 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h5ec44decb225b5bd + 6
4   a                              0x0000000100f9499f main + 511
5   libdyld.dylib                  0x00007fff620d5115 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0x00007fff62224e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff62363150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff62181312 abort + 127
3   a                              0x0000000100fa0359 std::sys::unix::abort_internal::h920319b3609835ff + 9
4   a                              0x0000000100fa4cab std::sys_common::util::abort::h53eadd5a3111ea4a + 91
5   a                              0x0000000100f96ea6 std::sys::unix::stack_overflow::imp::signal_handler::hbb993e795b77e561 (.llvm.10413841796049484272) + 678
6   libsystem_platform.dylib       0x00007fff62356f5a _sigtramp + 26
7   ???                            000000000000000000 0 + 0
8   a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
9   a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
10  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
11  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
12  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
13  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
14  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
15  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
16  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
17  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
18  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
19  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
20  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
21  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
22  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
23  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
24  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
25  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
26  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
27  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
28  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
29  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
30  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
31  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
32  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
33  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
34  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
35  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
36  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
37  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
38  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
39  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
40  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
41  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
42  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
43  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
44  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
45  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
46  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
47  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
48  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
49  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
50  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
51  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
52  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
53  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
54  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
55  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
56  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
57  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
58  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
59  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
60  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
61  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
62  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
63  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
64  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
65  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
66  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
67  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
68  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
69  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
70  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
71  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
72  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
73  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
74  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
75  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
76  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
77  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
78  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
79  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
80  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
81  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
82  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
83  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
84  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
85  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
86  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
87  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
88  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
89  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
90  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
91  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
92  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
93  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
94  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
95  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
96  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
97  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
98  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
99  a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
100 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
101 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
102 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
103 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
104 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
105 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
106 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
107 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
108 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
109 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
110 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
111 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
112 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
113 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
114 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
115 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
116 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
117 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
118 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
119 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
120 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
121 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
122 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
123 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
124 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
125 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
126 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
127 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
128 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
129 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
130 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
131 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
132 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
133 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
134 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
135 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
136 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
137 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
138 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
139 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
140 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
141 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
142 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
143 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
144 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
145 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
146 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
147 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
148 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
149 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
150 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
151 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
152 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
153 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
154 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
155 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
156 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
157 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
158 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
159 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
160 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
161 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
162 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
163 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
164 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
165 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
166 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
167 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
168 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
169 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
170 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
171 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
172 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
173 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
174 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
175 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
176 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
177 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
178 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
179 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
180 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
181 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
182 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
183 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
184 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
185 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
186 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
187 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
188 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
189 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
190 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
191 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
192 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
193 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
194 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
195 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
196 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
197 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
198 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
199 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
200 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
201 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
202 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
203 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
204 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
205 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
206 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
207 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
208 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
209 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
210 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
211 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
212 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
213 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
214 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
215 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
216 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
217 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
218 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
219 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
220 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
221 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
222 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
223 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
224 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
225 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
226 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
227 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
228 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
229 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
230 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
231 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
232 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
233 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
234 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
235 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
236 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
237 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
238 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
239 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
240 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
241 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
242 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
243 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
244 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
245 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
246 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
247 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
248 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
249 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
250 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
251 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
252 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
253 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
254 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
255 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
256 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
257 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
258 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
259 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
260 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
261 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
262 a                              0x0000000100f93982 stack_probes_lto::recurse::h6895313952b89569 + 34
263 a                              0x0000000100faf7ad _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h281802e05c31f773 + 141
264 a                              0x0000000100f96f78 std::sys::unix::thread::Thread::new::thread_start::h90b1618c8e63e2d6 + 136
265 libsystem_pthread.dylib        0x00007fff623606c1 _pthread_body + 340
266 libsystem_pthread.dylib        0x00007fff6236056d _pthread_start + 377
267 libsystem_pthread.dylib        0x00007fff6235fc5d thread_start + 13
Thread 1 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x0000700000e77000  rcx: 0x0000000101065968  rdx: 0x0000000000000000
  rdi: 0x0000000000001303  rsi: 0x0000000000000006  rbp: 0x00000001010659a0  rsp: 0x0000000101065968
   r8: 0x0000000100fe5b30   r9: 0x0000000000000001  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000001303  r13: 0x0000000101418060  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff62224e3e  rfl: 0x0000000000000206  cr2: 0x0000000100f93000
Logical CPU:     0
Error Code:      0x0200014e
Trap Number:     133
Binary Images:
       0x100f91000 -        0x100ff2fcf +a (0) <7CCC1E01-29FE-3B0E-B135-BC7F7A34ACA8> /Users/USER/*/a
       0x10c0de000 -        0x10c12898f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5f93f000 -     0x7fff5f972fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5fe51000 -     0x7fff5fe52ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff60107000 -     0x7fff6015dfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6015e000 -     0x7fff60182ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff614d4000 -     0x7fff618c53b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff61992000 -     0x7fff619aeffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff61f6c000 -     0x7fff61f70ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff61f71000 -     0x7fff61f7bff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff61f7c000 -     0x7fff61f83fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff61f84000 -     0x7fff61f8cffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff61f8d000 -     0x7fff62012fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6209a000 -     0x7fff620d3ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff620d4000 -     0x7fff620f1ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff620f2000 -     0x7fff620f2ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff62100000 -     0x7fff62100ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff62101000 -     0x7fff62105ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff62106000 -     0x7fff62108ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff62109000 -     0x7fff6210aff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6210b000 -     0x7fff62122fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff62123000 -     0x7fff62123fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff62124000 -     0x7fff621adff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff621ae000 -     0x7fff621b1ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff621b2000 -     0x7fff621b5ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff621b6000 -     0x7fff621b7fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff621b8000 -     0x7fff621beff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff621bf000 -     0x7fff62208ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff62209000 -     0x7fff6222eff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6222f000 -     0x7fff6227afcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6227b000 -     0x7fff6229afff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6229b000 -     0x7fff6233fff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff62340000 -     0x7fff6234affb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6234b000 -     0x7fff62354ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff62355000 -     0x7fff6235cff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6235d000 -     0x7fff62368fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff62369000 -     0x7fff6236cff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6236d000 -     0x7fff6236eff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6236f000 -     0x7fff62376ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff62377000 -     0x7fff6238aff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6238c000 -     0x7fff62391ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff62392000 -     0x7fff623beff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2046
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=197.8M resident=0K(0%) swapped_out_or_unallocated=197.8M(100%)
Writable regions: Total=78.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=78.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                          8K        3 
VM_ALLOCATE                       4228K        5 
VM_ALLOCATE                       4228K        5 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            2344K       43 
__LINKEDIT                       188.7M        4 
__TEXT                            9392K       43 
===========                     =======  ======= 
TOTAL                            280.5M      111 
TOTAL                            280.5M      111 
TOTAL, minus reserved VM space   280.3M      111 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-10-17-225230_Traviss-Mac-1044.crash
Process:               a [53997]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [53995]
Responsible:           a [53997]
User ID:               501
Date/Time:             2018-10-17 22:52:30.172 +0000
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
0   a                              0x000000010995386e abort_on_c_abi::panic_in_ffi::h305168285df224eb + 30
1   a                              0x0000000109952c69 std::panicking::try::do_call::hfd34e42b75e37246 (.llvm.5894081457290078391) + 9
2   libstd-f7dd09cf573f28a7.dylib  0x00000001099a44bf __rust_maybe_catch_panic + 31
3   a                              0x0000000109953ac1 abort_on_c_abi::main::h82c0574e6cdd7b93 + 593
4   a                              0x0000000109952026 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h2003daba2b9b5296 + 6
5   libstd-f7dd09cf573f28a7.dylib  0x0000000109996a88 std::panicking::try::do_call::h6a4dd829f738a148 (.llvm.5698316011968908660) + 24
6   libstd-f7dd09cf573f28a7.dylib  0x00000001099a44bf __rust_maybe_catch_panic + 31
7   libstd-f7dd09cf573f28a7.dylib  0x00000001099754ad std::rt::lang_start_internal::hdda43f5656e8d027 + 237
8   a                              0x0000000109953dcc main + 44
9   libdyld.dylib                  0x00007fff620d5115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x000000010a01c050  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee62ac618  rsi: 0xfffffffffffffce8  rbp: 0x00007ffee62ad020  rsp: 0x00007ffee62ad020
   r8: 0xffffffff00000000   r9: 0x0000000109a47f00  r10: 0x000000010e08b8d8  r11: 0x00007fff6238c96c
  r12: 0x0000000000000000  r13: 0x0000000000000000  r14: 0x00007ffee62ad140  r15: 0x00007ffee62ad088
  rip: 0x000000010995386e  rfl: 0x0000000000010202  cr2: 0x0000000109a1c1d4
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x109951000 -        0x109954fff +a (0) <B1D3A0D4-2F53-3B7F-94AB-DBE82B2CA48E> /Users/USER/*/a
       0x10995f000 -        0x109a3dfd7 +libstd-f7dd09cf573f28a7.dylib (0) <A18F2486-2D57-3EFD-8DDA-026EA5022DA6> /Users/USER/*/libstd-f7dd09cf573f28a7.dylib
       0x10e039000 -        0x10e08398f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5f93f000 -     0x7fff5f972fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5fe51000 -     0x7fff5fe52ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff60107000 -     0x7fff6015dfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6015e000 -     0x7fff60182ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff614d4000 -     0x7fff618c53b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff61992000 -     0x7fff619aeffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff61f6c000 -     0x7fff61f70ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff61f71000 -     0x7fff61f7bff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff61f7c000 -     0x7fff61f83fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff61f84000 -     0x7fff61f8cffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff61f8d000 -     0x7fff62012fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6209a000 -     0x7fff620d3ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff620d4000 -     0x7fff620f1ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff620f2000 -     0x7fff620f2ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff62100000 -     0x7fff62100ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff62101000 -     0x7fff62105ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff62106000 -     0x7fff62108ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff62109000 -     0x7fff6210aff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6210b000 -     0x7fff62122fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff62123000 -     0x7fff62123fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff62124000 -     0x7fff621adff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff621ae000 -     0x7fff621b1ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff621b2000 -     0x7fff621b5ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff621b6000 -     0x7fff621b7fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff621b8000 -     0x7fff621beff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff621bf000 -     0x7fff62208ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff62209000 -     0x7fff6222eff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6222f000 -     0x7fff6227afcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6227b000 -     0x7fff6229afff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6229b000 -     0x7fff6233fff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff62340000 -     0x7fff6234affb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6234b000 -     0x7fff62354ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff62355000 -     0x7fff6235cff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6235d000 -     0x7fff62368fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
---
travis_time:end:18f27f6d:start=1539818049541756000,finish=1539818058322310000,duration=8780554000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f22fd22
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0121ecac
travis_time:start:0121ecac
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0389fbd4
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
