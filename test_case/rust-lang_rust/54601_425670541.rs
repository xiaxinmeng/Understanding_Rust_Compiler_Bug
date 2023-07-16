plain
[00:03:47]       Memory: 8 GB
[00:03:47]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:47]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:47]       SMC Version (system): 2.8f0
[00:03:47]       Serial Number (system): VMZkmQr15K3H
[00:03:47] 
[00:03:47] hw.ncpu: 4
[00:03:47] hw.byteorder: 1234
[00:03:47] hw.memsize: 8589934592
---
[01:50:58] 
[01:50:58] ---- hex.rs - hex::[u8]::to_hex (line 32) stdout ----
[01:50:58] error: linking with `cc` failed: signal: 4
[01:50:58]   |
[01:50:58]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.10.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.11.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.12.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.13.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.14.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.15.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.6.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.7.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.8.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out.rust_out.7rcbfp3g-cgu.9.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest62qrVD/rust_out" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1-rustc/x86_64-apple-darwin/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1-rustc/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1-rustc/x86_64-apple-darwin/release/deps/libserialize-aa8e39d68e9f9c9f.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libsmallvec-408b3187042ab988.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libunreachable-e321618d3dd0e649.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libvoid-c42e3633cc40cbdb.rlib" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib" "-lstd-f7dd09cf573f28a7" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-3048cc32a441c3bf.rlib" "-lSystem" "-lresolv" "-lpthread" "-lc" "-lm"
[01:50:58] 
[01:50:58] thread 'hex.rs - hex::[u8]::to_hex (line 32)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:50:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:50:58] 
---
[01:50:58] test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:50:58] 
[01:50:58] 
[01:50:58] 
[01:50:58] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "test" "--target" "x86_64-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/rustc/Cargo.toml" "-p" "serialize" "--"
[01:50:58] 
[01:50:58] 
[01:50:58] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:50:58] Build completed unsuccessfully in 0:57:04
[01:50:58] Build completed unsuccessfully in 0:57:04
[01:50:58] error: test failed, to rerun pass '--doc'
[01:50:58] make: *** [check] Error 1
travis_time:end:069f5d30:start=1538243126587185000,finish=1538249784658840000,duration=6658071655000

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b21e013
---
travis_fold:start:after_failure.2
travis_time:start:0063bcb8
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1264
drwx------  22 travis  staff    748 Sep 29 19:06 .
-rw-------@  1 travis  staff  37510 Sep 29 19:06 a_2018-09-29-190620-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62275 Sep 29 19:06 a_2018-09-29-190620_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60415 Sep 29 19:06 a_2018-09-29-190615-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37410 Sep 29 19:06 a_2018-09-29-190615_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10171 Sep 29 19:06 a_2018-09-29-190610_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9902 Sep 29 19:06 a_2018-09-29-190605_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9905 Sep 29 19:05 a_2018-09-29-190558_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9871 Sep 29 19:05 a_2018-09-29-190557_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10060 Sep 29 19:05 a_2018-09-29-190530_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63131 Sep 29 19:05 a_2018-09-29-190522_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64335 Sep 29 19:05 a_2018-09-29-190519-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63943 Sep 29 19:05 a_2018-09-29-190519-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65148 Sep 29 19:05 a_2018-09-29-190519_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11785 Sep 29 19:03 a_2018-09-29-190303_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9926 Sep 29 19:02 a_2018-09-29-190206_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10333 Sep 29 19:00 a_2018-09-29-190055_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10515 Sep 29 19:00 a_2018-09-29-190002-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10513 Sep 29 19:00 a_2018-09-29-190002_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10243 Sep 29 18:59 a_2018-09-29-185933_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37510 Sep 29 18:57 a_2018-09-29-185751_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1035f9a4
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-09-29-185751_Traviss-Mac-1044.crash
Process:               a [50534]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [50527]
Responsible:           a [50534]
User ID:               501
Date/Time:             2018-09-29 18:57:49.968 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4400 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x0000700009aebe40
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0x700009aebe40:
    __LINKEDIT             0000000117ccf000-0000000117cea000 [  108K] r--/rwx SM=COW  /usr/lib/dyld
--> Stack Guard            0000700009aeb000-0000700009aec000 [    4K] ---/rwx SM=NUL  
    Stack                  0000700009aec000-0000700009cee000 [ 2056K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff591bb05a __semwait_signal + 10
1   libsystem_pthread.dylib        0x00007fff592fa8ec _pthread_join + 626
2   a                              0x000000010dbaaad2 stack_probes_lto::main::h46e0341d7865a006 + 2850
3   a                              0x000000010dbc74c6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb97ab777cebf3d0d + 6
4   a                              0x000000010dbac07f main + 511
5   libdyld.dylib                  0x00007fff5906b115 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0x00007fff591bae3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff592f9150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff59117312 abort + 127
3   a                              0x000000010dbb9999 std::sys::unix::abort_internal::h920319b3609835ff + 9
4   a                              0x000000010dbbe05b std::sys_common::util::abort::h53eadd5a3111ea4a + 91
5   a                              0x000000010dbad036 std::sys::unix::stack_overflow::imp::signal_handler::hbb993e795b77e561 (.llvm.13105859290220886925) + 678
6   libsystem_platform.dylib       0x00007fff592ecf5a _sigtramp + 26
7   ???                            000000000000000000 0 + 0
8   a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
9   a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
10  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
11  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
12  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
13  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
14  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
15  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
16  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
17  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
18  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
19  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
20  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
21  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
22  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
23  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
24  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
25  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
26  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
27  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
28  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
29  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
30  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
31  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
32  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
33  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
34  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
35  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
36  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
37  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
38  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
39  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
40  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
41  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
42  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
43  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
44  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
45  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
46  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
47  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
48  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
49  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
50  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
51  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
52  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
53  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
54  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
55  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
56  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
57  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
58  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
59  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
60  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
61  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
62  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
63  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
64  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
65  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
66  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
67  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
68  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
69  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
70  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
71  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
72  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
73  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
74  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
75  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
76  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
77  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
78  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
79  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
80  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
81  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
82  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
83  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
84  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
85  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
86  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
87  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
88  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
89  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
90  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
91  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
92  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
93  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
94  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
95  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
96  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
97  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
98  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
99  a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
100 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
101 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
102 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
103 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
104 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
105 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
106 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
107 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
108 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
109 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
110 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
111 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
112 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
113 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
114 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
115 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
116 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
117 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
118 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
119 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
120 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
121 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
122 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
123 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
124 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
125 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
126 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
127 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
128 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
129 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
130 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
131 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
132 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
133 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
134 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
135 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
136 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
137 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
138 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
139 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
140 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
141 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
142 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
143 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
144 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
145 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
146 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
147 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
148 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
149 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
150 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
151 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
152 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
153 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
154 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
155 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
156 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
157 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
158 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
159 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
160 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
161 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
162 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
163 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
164 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
165 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
166 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
167 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
168 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
169 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
170 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
171 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
172 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
173 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
174 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
175 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
176 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
177 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
178 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
179 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
180 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
181 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
182 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
183 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
184 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
185 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
186 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
187 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
188 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
189 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
190 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
191 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
192 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
193 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
194 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
195 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
196 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
197 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
198 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
199 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
200 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
201 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
202 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
203 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
204 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
205 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
206 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
207 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
208 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
209 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
210 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
211 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
212 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
213 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
214 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
215 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
216 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
217 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
218 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
219 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
220 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
221 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
222 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
223 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
224 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
225 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
226 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
227 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
228 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
229 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
230 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
231 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
232 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
233 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
234 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
235 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
236 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
237 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
238 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
239 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
240 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
241 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
242 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
243 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
244 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
245 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
246 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
247 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
248 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
249 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
250 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
251 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
252 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
253 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
254 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
255 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
256 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
257 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
258 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
259 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
260 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
261 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
262 a                              0x000000010dbab0a2 stack_probes_lto::recurse::h6895313952b89569 + 34
263 a                              0x000000010dbc697d _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hdfbc3a3fa11720dc + 141
264 a                              0x000000010dbad106 std::sys::unix::thread::Thread::new::thread_start::h90b1618c8e63e2d6 + 134
265 libsystem_pthread.dylib        0x00007fff592f66c1 _pthread_body + 340
266 libsystem_pthread.dylib        0x00007fff592f656d _pthread_start + 377
267 libsystem_pthread.dylib        0x00007fff592f5c5d thread_start + 13
Thread 1 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x0000700009cec000  rcx: 0x000000010dc7c968  rdx: 0x0000000000000000
  rdi: 0x0000000000000a03  rsi: 0x0000000000000006  rbp: 0x000000010dc7c9a0  rsp: 0x000000010dc7c968
   r8: 0x000000010dbfcc70   r9: 0x0000000000000001  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000a03  r13: 0x000000010e018060  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff591bae3e  rfl: 0x0000000000000206  cr2: 0x00007fff91d6f148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10dba9000 -        0x10dc09fe7 +a (0) <42B27133-3333-3B61-9EC0-F68F300A2305> /Users/USER/*/a
       0x117c4c000 -        0x117c9698f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff568d5000 -     0x7fff56908fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff56de7000 -     0x7fff56de8ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5709d000 -     0x7fff570f3fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff570f4000 -     0x7fff57118ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5846a000 -     0x7fff5885b3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff58928000 -     0x7fff58944ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff58f02000 -     0x7fff58f06ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff58f07000 -     0x7fff58f11ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff58f12000 -     0x7fff58f19fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff58f1a000 -     0x7fff58f22ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff58f23000 -     0x7fff58fa8fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff59030000 -     0x7fff59069ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5906a000 -     0x7fff59087ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff59088000 -     0x7fff59088ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff59096000 -     0x7fff59096ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff59097000 -     0x7fff5909bffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5909c000 -     0x7fff5909eff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5909f000 -     0x7fff590a0ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff590a1000 -     0x7fff590b8fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff590b9000 -     0x7fff590b9fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff590ba000 -     0x7fff59143ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff59144000 -     0x7fff59147ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff59148000 -     0x7fff5914bffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5914c000 -     0x7fff5914dfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5914e000 -     0x7fff59154ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff59155000 -     0x7fff5919eff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5919f000 -     0x7fff591c4ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff591c5000 -     0x7fff59210fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff59211000 -     0x7fff59230fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff59231000 -     0x7fff592d5ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff592d6000 -     0x7fff592e0ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff592e1000 -     0x7fff592eaff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff592eb000 -     0x7fff592f2ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff592f3000 -     0x7fff592fefff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff592ff000 -     0x7fff59302ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff59303000 -     0x7fff59304ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff59305000 -     0x7fff5930cff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5930d000 -     0x7fff59320ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff59322000 -     0x7fff59327ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff59328000 -     0x7fff59354ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2308
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
__TEXT                            9388K       43 
===========                     =======  ======= 
TOTAL                            280.4M      111 
TOTAL                            280.4M      111 
TOTAL, minus reserved VM space   280.3M      111 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-09-29-185933_Traviss-Mac-1044.crash
Process:               a [52921]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [52920]
Responsible:           a [52921]
User ID:               501
Date/Time:             2018-09-29 18:59:33.362 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000000010ae9984e abort_on_c_abi::panic_in_ffi::h305168285df224eb + 30
1   a                              0x000000010ae98c49 std::panicking::try::do_call::h0d3695bae5474f86 (.llvm.14533246755493964085) + 9
2   libstd-f7dd09cf573f28a7.dylib  0x000000010aee78ef __rust_maybe_catch_panic + 31
3   a                              0x000000010ae99aa1 abort_on_c_abi::main::h82c0574e6cdd7b93 + 593
4   a                              0x000000010ae97fd6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hfb5d86c96c6bab8c + 6
5   libstd-f7dd09cf573f28a7.dylib  0x000000010aed9f18 std::panicking::try::do_call::h4d61a54973e308d7 (.llvm.15302646972053011970) + 24
6   libstd-f7dd09cf573f28a7.dylib  0x000000010aee78ef __rust_maybe_catch_panic + 31
7   libstd-f7dd09cf573f28a7.dylib  0x000000010aebdc5d std::rt::lang_start_internal::hdda43f5656e8d027 + 237
8   a                              0x000000010ae99dac main + 44
9   libdyld.dylib                  0x00007fff5906b115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x000000010b61c050  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee4d66658  rsi: 0xfffffffffffffce8  rbp: 0x00007ffee4d67060  rsp: 0x00007ffee4d67060
   r8: 0xffffffff00000000   r9: 0x000000010af8af20  r10: 0x000000010d8b68d8  r11: 0x00007fff5932296c
  r12: 0x0000000000000000  r13: 0x0000000000000000  r14: 0x00007ffee4d67180  r15: 0x00007ffee4d670c8
  rip: 0x000000010ae9984e  rfl: 0x0000000000010206  cr2: 0x000000010af4228c
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10ae97000 -        0x10ae9aff7 +a (0) <1231C031-042C-36EF-9942-4F681D5D8282> /Users/USER/*/a
       0x10aea2000 -        0x10af80fc7 +libstd-f7dd09cf573f28a7.dylib (0) <EF8C1740-542C-3FC0-9102-4A7731C05EBE> /Users/USER/*/libstd-f7dd09cf573f28a7.dylib
       0x10d864000 -        0x10d8ae98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff568d5000 -     0x7fff56908fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff56de7000 -     0x7fff56de8ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5709d000 -     0x7fff570f3fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff570f4000 -     0x7fff57118ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5846a000 -     0x7fff5885b3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff58928000 -     0x7fff58944ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff58f02000 -     0x7fff58f06ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff58f07000 -     0x7fff58f11ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff58f12000 -     0x7fff58f19fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff58f1a000 -     0x7fff58f22ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff58f23000 -     0x7fff58fa8fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff59030000 -     0x7fff59069ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5906a000 -     0x7fff59087ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff59088000 -     0x7fff59088ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff59096000 -     0x7fff59096ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff59097000 -     0x7fff5909bffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5909c000 -     0x7fff5909eff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5909f000 -     0x7fff590a0ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff590a1000 -     0x7fff590b8fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff590b9000 -     0x7fff590b9fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff590ba000 -     0x7fff59143ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff59144000 -     0x7fff59147ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff59148000 -     0x7fff5914bffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5914c000 -     0x7fff5914dfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5914e000 -     0x7fff59154ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff59155000 -     0x7fff5919eff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5919f000 -     0x7fff591c4ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff591c5000 -     0x7fff59210fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff59211000 -     0x7fff59230fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff59231000 -     0x7fff592d5ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff592d6000 -     0x7fff592e0ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff592e1000 -     0x7fff592eaff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff592eb000 -     0x7fff592f2ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff592f3000 -     0x7fff592fefff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
---
travis_time:end:1035f9a4:start=1538249793086200000,finish=1538249802824625000,duration=9738425000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2958a347
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d7e9950
travis_time:start:0d7e9950
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:148dde10
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
