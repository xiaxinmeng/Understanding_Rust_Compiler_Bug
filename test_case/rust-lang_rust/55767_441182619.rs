plain
[00:03:24]       Memory: 8 GB
[00:03:24]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:24]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:24]       SMC Version (system): 2.8f0
[00:03:24]       Serial Number (system): VMeNq5AwwOkL
[00:03:24] 
[00:03:25] hw.ncpu: 4
[00:03:25] hw.byteorder: 1234
[00:03:25] hw.memsize: 8589934592
---
[01:52:52] test collections::hash::set::test_set::test_disjoint ... ok
[01:52:52] test collections::hash::set::test_set::test_drain ... ok
[01:52:52] test collections::hash::set::test_set::test_eq ... ok
[01:52:52] test collections::hash::set::test_set::test_extend_ref ... ok
[01:52:52] std-10a3216f761584ad(85168,0x70000edec000) malloc: *** mach_vm_map(size=11529215046068469760) failed (error code=3)
[01:52:52] *** set a breakpoint in malloc_error_break to debug
[01:52:52] test collections::hash::set::test_set::test_from_iter ... ok
[01:52:52] test collections::hash::set::test_set::test_intersection ... ok
[01:52:52] test collections::hash::bench::find_existing ... ok
---
[01:54:37] 
[01:54:37] ---- alloc.rs - alloc::System (line 106) stdout ----
[01:54:37] error: linking with `cc` failed: signal: 4
[01:54:37]   |
[01:54:37]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.10.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.11.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.12.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.13.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.14.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.15.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.6.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.7.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.8.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.rust_out.7rcbfp3g-cgu.9.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlIzUaj/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/asan/build/lib/darwin" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/libbacktrace/" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/tsan/build/lib/darwin" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/build/compiler_builtins-7c29b7957a7e0034/out" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/build/profiler_builtins-90878ef4780b35b8/out" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1-std/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps/libstd-ae69a760be8b7053.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-e4468e88cc678cc0.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libunwind-d4351380f83add85.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/liblibc-e14d637badce231e.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/liballoc-85796c6312c279e8.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libcore-879924ea8a001728.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-5d6e0e7618d0f2e8.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[01:54:38] 
[01:54:38] thread 'alloc.rs - alloc::System (line 106)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:54:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:54:38] 
---
[01:54:38] test result: FAILED. 947 passed; 1 failed; 26 ignored; 0 measured; 0 filtered out
[01:54:38] 
[01:54:38] 
[01:54:38] 
[01:54:37] error: test failed, to rerun pass '--doc'
[01:54:38] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "test" "--target" "x86_64-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/libstd/Cargo.toml" "-p" "std" "--"
[01:54:38] 
[01:54:38] 
[01:54:38] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:54:38] Build completed unsuccessfully in 0:58:28
[01:54:38] Build completed unsuccessfully in 0:58:28
[01:54:38] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0663ae92
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 23 08:48:04 GMT 2018
---
travis_fold:start:after_failure.2
travis_time:start:3b50a610
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1264
drwx------  22 travis  staff    748 Nov 23 08:18 .
-rw-------@  1 travis  staff  62246 Nov 23 08:18 a_2018-11-23-081838-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37481 Nov 23 08:18 a_2018-11-23-081838_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60388 Nov 23 08:18 a_2018-11-23-081828-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37237 Nov 23 08:18 a_2018-11-23-081828_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Nov 23 08:18 a_2018-11-23-081822_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Nov 23 08:18 a_2018-11-23-081818_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9876 Nov 23 08:18 a_2018-11-23-081810-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9795 Nov 23 08:18 a_2018-11-23-081810_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Nov 23 08:17 a_2018-11-23-081742_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63072 Nov 23 08:17 a_2018-11-23-081732_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64247 Nov 23 08:17 a_2018-11-23-081729-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65090 Nov 23 08:17 a_2018-11-23-081729-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63914 Nov 23 08:17 a_2018-11-23-081729_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11730 Nov 23 08:15 a_2018-11-23-081519_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Nov 23 08:14 a_2018-11-23-081434_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Nov 23 08:13 a_2018-11-23-081323_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10486 Nov 23 08:12 a_2018-11-23-081229-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10484 Nov 23 08:12 a_2018-11-23-081229_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 Nov 23 08:11 a_2018-11-23-081158_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37481 Nov 23 08:10 a_2018-11-23-081017_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0047da7e
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-11-23-081017_Traviss-Mac-1044.crash
Process:               a [57555]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [57548]
Responsible:           a [57555]
User ID:               501
Date/Time:             2018-11-23 08:10:15.746 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4700 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x000070000d3ffe50
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0x70000d3ffe50:
    __LINKEDIT             0000000111e76000-0000000111e91000 [  108K] r--/rwx SM=COW  /usr/lib/dyld
--> Stack Guard            000070000d3ff000-000070000d400000 [    4K] ---/rwx SM=NUL  
    Stack                  000070000d400000-000070000d602000 [ 2056K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff7190005a __semwait_signal + 10
1   libsystem_pthread.dylib        0x00007fff71a3f8ec _pthread_join + 626
2   a                              0x00000001077a5f07 stack_probes_lto::main::h7f9d4bbc5a99dd1a + 2983
3   a                              0x00000001077c1786 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc4b333a70156415c + 6
4   a                              0x00000001077a7908 main + 520
5   libdyld.dylib                  0x00007fff717b0115 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0x00007fff718ffe3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff71a3e150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff7185c312 abort + 127
3   a                              0x00000001077a92a9 std::sys::unix::abort_internal::h087bb90e7fc1a385 + 9
4   a                              0x00000001077a929b std::sys_common::util::abort::hd9e430199fc79353 + 91
5   a                              0x00000001077bb209 std::sys::unix::stack_overflow::imp::signal_handler::h3582f53926c561f0 + 649
6   libsystem_platform.dylib       0x00007fff71a31f5a _sigtramp + 26
7   ???                            000000000000000000 0 + 0
8   a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
9   a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
10  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
11  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
12  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
13  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
14  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
15  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
16  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
17  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
18  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
19  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
20  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
21  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
22  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
23  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
24  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
25  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
26  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
27  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
28  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
29  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
30  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
31  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
32  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
33  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
34  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
35  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
36  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
37  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
38  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
39  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
40  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
41  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
42  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
43  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
44  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
45  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
46  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
47  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
48  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
49  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
50  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
51  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
52  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
53  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
54  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
55  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
56  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
57  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
58  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
59  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
60  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
61  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
62  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
63  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
64  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
65  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
66  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
67  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
68  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
69  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
70  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
71  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
72  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
73  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
74  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
75  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
76  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
77  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
78  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
79  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
80  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
81  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
82  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
83  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
84  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
85  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
86  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
87  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
88  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
89  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
90  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
91  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
92  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
93  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
94  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
95  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
96  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
97  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
98  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
99  a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
100 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
101 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
102 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
103 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
104 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
105 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
106 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
107 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
108 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
109 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
110 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
111 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
112 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
113 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
114 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
115 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
116 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
117 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
118 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
119 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
120 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
121 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
122 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
123 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
124 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
125 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
126 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
127 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
128 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
129 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
130 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
131 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
132 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
133 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
134 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
135 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
136 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
137 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
138 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
139 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
140 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
141 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
142 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
143 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
144 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
145 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
146 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
147 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
148 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
149 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
150 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
151 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
152 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
153 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
154 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
155 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
156 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
157 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
158 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
159 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
160 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
161 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
162 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
163 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
164 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
165 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
166 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
167 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
168 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
169 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
170 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
171 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
172 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
173 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
174 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
175 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
176 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
177 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
178 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
179 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
180 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
181 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
182 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
183 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
184 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
185 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
186 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
187 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
188 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
189 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
190 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
191 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
192 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
193 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
194 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
195 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
196 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
197 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
198 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
199 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
200 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
201 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
202 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
203 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
204 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
205 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
206 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
207 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
208 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
209 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
210 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
211 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
212 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
213 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
214 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
215 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
216 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
217 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
218 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
219 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
220 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
221 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
222 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
223 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
224 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
225 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
226 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
227 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
228 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
229 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
230 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
231 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
232 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
233 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
234 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
235 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
236 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
237 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
238 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
239 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
240 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
241 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
242 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
243 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
244 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
245 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
246 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
247 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
248 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
249 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
250 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
251 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
252 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
253 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
254 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
255 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
256 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
257 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
258 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
259 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
260 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
261 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
262 a                              0x00000001077a64e2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
263 a                              0x00000001077c0de1 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h80ad9e0172dfc5cb + 129
264 a                              0x00000001077bb5f8 std::sys::unix::thread::Thread::new::thread_start::h3c1771492ad9a1a5 + 136
265 libsystem_pthread.dylib        0x00007fff71a3b6c1 _pthread_body + 340
266 libsystem_pthread.dylib        0x00007fff71a3b56d _pthread_start + 377
267 libsystem_pthread.dylib        0x00007fff71a3ac5d thread_start + 13
Thread 1 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x000070000d600000  rcx: 0x0000000107832968  rdx: 0x0000000000000000
  rdi: 0x0000000000001303  rsi: 0x0000000000000006  rbp: 0x00000001078329a0  rsp: 0x0000000107832968
   r8: 0x00000001077d07e0   r9: 0x0000000000000001  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000001303  r13: 0x00007f8fbec02bd0  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff718ffe3e  rfl: 0x0000000000000206  cr2: 0x00000001077a92b0
Logical CPU:     0
Error Code:      0x0200014e
Trap Number:     133
Binary Images:
       0x1077a4000 -        0x1077d4fff +a (0) <71897E4E-B7E4-3251-840D-22969F5AD7C6> /Users/USER/*/a
       0x111df3000 -        0x111e3d98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6f01a000 -     0x7fff6f04dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6f52c000 -     0x7fff6f52dff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6f7e2000 -     0x7fff6f838fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6f839000 -     0x7fff6f85dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff70baf000 -     0x7fff70fa03b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7106d000 -     0x7fff71089ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff71647000 -     0x7fff7164bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff7164c000 -     0x7fff71656ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff71657000 -     0x7fff7165efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7165f000 -     0x7fff71667ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff71668000 -     0x7fff716edfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff71775000 -     0x7fff717aeff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff717af000 -     0x7fff717ccff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff717cd000 -     0x7fff717cdffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff717db000 -     0x7fff717dbff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff717dc000 -     0x7fff717e0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff717e1000 -     0x7fff717e3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff717e4000 -     0x7fff717e5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff717e6000 -     0x7fff717fdfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff717fe000 -     0x7fff717fefff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff717ff000 -     0x7fff71888ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff71889000 -     0x7fff7188cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7188d000 -     0x7fff71890ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff71891000 -     0x7fff71892fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff71893000 -     0x7fff71899ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7189a000 -     0x7fff718e3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff718e4000 -     0x7fff71909ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7190a000 -     0x7fff71955fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff71956000 -     0x7fff71975fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff71976000 -     0x7fff71a1aff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff71a1b000 -     0x7fff71a25ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff71a26000 -     0x7fff71a2fff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff71a30000 -     0x7fff71a37ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff71a38000 -     0x7fff71a43fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff71a44000 -     0x7fff71a47ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff71a48000 -     0x7fff71a49ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff71a4a000 -     0x7fff71a51ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff71a52000 -     0x7fff71a65ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff71a67000 -     0x7fff71a6cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff71a6d000 -     0x7fff71a99ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2058
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
__TEXT                            9196K       43 
===========                     =======  ======= 
TOTAL                            276.2M      108 
TOTAL                            276.2M      108 
TOTAL, minus reserved VM space   276.1M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-11-23-081158_Traviss-Mac-1044.crash
Process:               a [59963]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [59962]
Responsible:           a [59963]
User ID:               501
Date/Time:             2018-11-23 08:11:57.796 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000000010c59778e abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x000000010c596b89 std::panicking::try::do_call::h211e714b11d774c9 (.llvm.15271872327916709951) + 9
2   libstd-ae69a760be8b7053.dylib  0x000000010c5e5def __rust_maybe_catch_panic + 31
3   a                              0x000000010c5979e1 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x000000010c595f56 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h98f4e1f0429f2c0d + 6
5   libstd-ae69a760be8b7053.dylib  0x000000010c5c9ce8 std::panicking::try::do_call::hf1ad3dd8ac46e133 + 24
6   libstd-ae69a760be8b7053.dylib  0x000000010c5e5def __rust_maybe_catch_panic + 31
7   libstd-ae69a760be8b7053.dylib  0x000000010c5ca73b std::rt::lang_start_internal::h4d4eaeb8b8c2eb51 + 379
8   a                              0x000000010c597cec main + 44
9   libdyld.dylib                  0x00007fff717b0115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007fd85b402bc0  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee3668318  rsi: 0x000000001fffffff  rbp: 0x00007ffee3668d70  rsp: 0x00007ffee3668d70
   r8: 0x0000000085b402c1   r9: 0x0000000000000004  r10: 0x0000000112c1a8d8  r11: 0x00007fff71a6796c
  r12: 0x0000000000000000  r13: 0x0000000000000000  r14: 0x00007ffee3668e90  r15: 0x00007ffee3668dd8
  rip: 0x000000010c59778e  rfl: 0x0000000000010206  cr2: 0x000000010c5e6000
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10c595000 -        0x10c598ff7 +a (0) <06812589-C8DC-35F3-B341-55C86DF49595> /Users/USER/*/a
       0x10c5a4000 -        0x10c63cff7 +libstd-ae69a760be8b7053.dylib (0) <E2AA3BCF-77F8-3AE2-9F49-3726FBFEEBE9> /Users/USER/*/libstd-ae69a760be8b7053.dylib
       0x112bc8000 -        0x112c1298f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6f01a000 -     0x7fff6f04dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6f52c000 -     0x7fff6f52dff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6f7e2000 -     0x7fff6f838fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6f839000 -     0x7fff6f85dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff70baf000 -     0x7fff70fa03b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7106d000 -     0x7fff71089ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff71647000 -     0x7fff7164bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff7164c000 -     0x7fff71656ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff71657000 -     0x7fff7165efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7165f000 -     0x7fff71667ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff71668000 -     0x7fff716edfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff71775000 -     0x7fff717aeff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff717af000 -     0x7fff717ccff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff717cd000 -     0x7fff717cdffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff717db000 -     0x7fff717dbff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff717dc000 -     0x7fff717e0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff717e1000 -     0x7fff717e3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff717e4000 -     0x7fff717e5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff717e6000 -     0x7fff717fdfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff717fe000 -     0x7fff717fefff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff717ff000 -     0x7fff71888ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff71889000 -     0x7fff7188cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7188d000 -     0x7fff71890ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff71891000 -     0x7fff71892fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff71893000 -     0x7fff71899ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7189a000 -     0x7fff718e3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff718e4000 -     0x7fff71909ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7190a000 -     0x7fff71955fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff71956000 -     0x7fff71975fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff71976000 -     0x7fff71a1aff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff71a1b000 -     0x7fff71a25ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff71a26000 -     0x7fff71a2fff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff71a30000 -     0x7fff71a37ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff71a38000 -     0x7fff71a43fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
