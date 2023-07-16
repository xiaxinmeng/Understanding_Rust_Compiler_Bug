plain
[00:03:00]       Memory: 8 GB
[00:03:00]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:00]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:00]       SMC Version (system): 2.8f0
[00:03:00]       Serial Number (system): VMs1wbiXV0Gn
[00:03:00] 
[00:03:00] hw.ncpu: 4
[00:03:00] hw.byteorder: 1234
[00:03:00] hw.memsize: 8589934592
---
[02:14:29] 
[02:14:29] ---- /Users/travis/build/rust-lang/rust/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals::By_value_trait_objects (line 104) stdout ----
[02:14:29] error: linking with `cc` failed: signal: 4
[02:14:29]   |
[02:14:29]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22bLoM/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22bLoM/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22bLoM/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22bLoM/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22bLoM/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22bLoM/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22bLoM/rust_out.rust_out.7rcbfp3g-cgu.6.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22bLoM/rust_out.rust_out.7rcbfp3g-cgu.7.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22bLoM/rust_out.rust_out.7rcbfp3g-cgu.8.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22bLoM/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22bLoM/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-28edbc69fd879800.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-b152b62305f45f50.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-6c9d408d4cf1d58a.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-460a4e8729661107.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-f24cc8363760ae6f.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-376cf09404a6e600.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-cff1600e2f9b6c28.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-233abb06723a85b7.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-98b24fb5d296bb0b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-51279115ab0d6b82.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:14:29] 
[02:14:29] error: aborting due to previous error
[02:14:29] 
[02:14:29] thread '/Users/travis/build/rust-lang/rust/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals::By_value_trait_objects (line 104)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[02:14:29] 
[02:14:29] 
[02:14:29] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:14:29] Build completed unsuccessfully in 0:55:51
[02:14:29] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:26a03f81
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr  2 13:04:45 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:2367af02
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Apr  2 13:04 .
-rw-------@  1 travis  staff  13742 Apr  2 13:04 overflow_2019-04-02-130432_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Apr  2 13:04 foo_2019-04-02-130407_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1418 Apr  2 13:03 m4_2019-04-02-130336_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Apr  2 13:03 bar_2019-04-02-130326_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Apr  2 13:03 bar_2019-04-02-130325_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 Apr  2 13:03 b_2019-04-02-130324_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62246 Apr  2 12:25 a_2019-04-02-122550_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37481 Apr  2 12:25 a_2019-04-02-122549_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60389 Apr  2 12:25 a_2019-04-02-122541-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37229 Apr  2 12:25 a_2019-04-02-122541_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Apr  2 12:25 a_2019-04-02-122535_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Apr  2 12:25 a_2019-04-02-122531_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Apr  2 12:25 a_2019-04-02-122524-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 Apr  2 12:25 a_2019-04-02-122524_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Apr  2 12:24 a_2019-04-02-122444_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63100 Apr  2 12:24 a_2019-04-02-122434_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64264 Apr  2 12:24 a_2019-04-02-122429-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 Apr  2 12:24 a_2019-04-02-122429-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65082 Apr  2 12:24 a_2019-04-02-122429_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11715 Apr  2 12:22 a_2019-04-02-122221_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Apr  2 12:21 a_2019-04-02-122120_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10213 Apr  2 12:20 a_2019-04-02-122013-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Apr  2 12:20 a_2019-04-02-122013-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Apr  2 12:20 a_2019-04-02-122013-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Apr  2 12:20 a_2019-04-02-122013_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:2dd527bb
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-02-122013-1_Traviss-Mac-1044.crash
Process:               a [44156]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44153]
Responsible:           a [44156]
User ID:               501
Date/Time:             2019-04-02 12:18:29.931 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000000010919b8ae abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x000000010919aca9 std::panicking::try::do_call::hc7b4d50989e643d9 (.llvm.1328971572518733427) + 9
2   libstd-28edbc69fd879800.dylib  0x00000001091dc7bf __rust_maybe_catch_panic + 31
3   a                              0x000000010919bb01 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x000000010919a0f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf4f268f28e9a6b3d + 6
5   libstd-28edbc69fd879800.dylib  0x00000001091cc578 std::panicking::try::do_call::he44de2c8a6962430 + 24
6   libstd-28edbc69fd879800.dylib  0x00000001091dc7bf __rust_maybe_catch_panic + 31
7   libstd-28edbc69fd879800.dylib  0x00000001091cd05e std::rt::lang_start_internal::h808ee93b9537043a + 542
8   a                              0x000000010919be09 main + 41
9   libdyld.dylib                  0x00007fff5da3d115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007fc84dd02b40  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee6a63c58  rsi: 0x00000000fe1fffff  rbp: 0x00007ffee6a646b0  rsp: 0x00007ffee6a646b0
   r8: 0x0000000084dd02b9   r9: 0x0000000000000004  r10: 0x000000010a58a8c0  r11: 0x00007fff5dcf496c
  r12: 0x00000001094f3000  r13: 0x0000000000000000  r14: 0x00007ffee6a647d0  r15: 0x00007ffee6a64718
  rip: 0x000000010919b8ae  rfl: 0x0000000000010206  cr2: 0x0000000109213b18
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x109199000 -        0x10919cff7 +a (0) <20AB01AA-BF77-3A4D-BFFF-B6261B463FB9> /Users/USER/*/a
       0x1091a8000 -        0x10923fff7 +libstd-28edbc69fd879800.dylib (0) <EEE6EA29-230C-3531-BB50-391386928C2F> /Users/USER/*/libstd-28edbc69fd879800.dylib
       0x10a538000 -        0x10a58298f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5b2a7000 -     0x7fff5b2dafff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5b7b9000 -     0x7fff5b7baff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5ba6f000 -     0x7fff5bac5fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5bac6000 -     0x7fff5baeaff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5ce3c000 -     0x7fff5d22d3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5d2fa000 -     0x7fff5d316ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5d8d4000 -     0x7fff5d8d8ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5d8d9000 -     0x7fff5d8e3ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5d8e4000 -     0x7fff5d8ebfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5d8ec000 -     0x7fff5d8f4ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5d8f5000 -     0x7fff5d97afff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5da02000 -     0x7fff5da3bff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5da3c000 -     0x7fff5da59ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5da5a000 -     0x7fff5da5affb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5da68000 -     0x7fff5da68ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5da69000 -     0x7fff5da6dffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5da6e000 -     0x7fff5da70ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5da71000 -     0x7fff5da72ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5da73000 -     0x7fff5da8afff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5da8b000 -     0x7fff5da8bfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5da8c000 -     0x7fff5db15ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5db16000 -     0x7fff5db19ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5db1a000 -     0x7fff5db1dffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5db1e000 -     0x7fff5db1ffff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5db20000 -     0x7fff5db26ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5db27000 -     0x7fff5db70ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5db71000 -     0x7fff5db96ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5db97000 -     0x7fff5dbe2fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5dbe3000 -     0x7fff5dc02fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5dc03000 -     0x7fff5dca7ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5dca8000 -     0x7fff5dcb2ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5dcb3000 -     0x7fff5dcbcff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5dcbd000 -     0x7fff5dcc4ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5dcc5000 -     0x7fff5dcd0fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5dcd1000 -     0x7fff5dcd4ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5dcd5000 -     0x7fff5dcd6ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5dcd7000 -     0x7fff5dcdeff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5dcdf000 -     0x7fff5dcf2ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5dcf4000 -     0x7fff5dcf9ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5dcfa000 -     0x7fff5dd26ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2598
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=74.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=74.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4636K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            277.1M      108 
TOTAL                            277.1M      108 
TOTAL, minus reserved VM space   277.0M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-02-122013-2_Traviss-Mac-1044.crash
Process:               a [46650]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [46649]
Responsible:           a [46650]
User ID:               501
Date/Time:             2019-04-02 12:19:58.208 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff5db8ce3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff5dccb150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff5dae9312 abort + 127
3   libstd-28edbc69fd879800.dylib  0x0000000108b48ca9 std::sys::unix::abort_internal::h8137cbab466f581f + 9
4   libstd-28edbc69fd879800.dylib  0x0000000108b38b90 rust_oom + 32
5   libstd-28edbc69fd879800.dylib  0x0000000108b59f69 alloc::alloc::handle_alloc_error::h0c82d64670f3f29b + 9
6   a                              0x0000000108b0b41f default_alloc_error_hook::main::hbf2d06db626d002e + 767
7   a                              0x0000000108b0a696 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd8082d12249549da + 6
8   libstd-28edbc69fd879800.dylib  0x0000000108b39578 std::panicking::try::do_call::he44de2c8a6962430 + 24
9   libstd-28edbc69fd879800.dylib  0x0000000108b497bf __rust_maybe_catch_panic + 31
10  libstd-28edbc69fd879800.dylib  0x0000000108b3a05e std::rt::lang_start_internal::h808ee93b9537043a + 542
11  a                              0x0000000108b0b579 main + 41
12  libdyld.dylib                  0x00007fff5da3d115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff96763340  rcx: 0x00007ffee70f4628  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee70f4660  rsp: 0x00007ffee70f4628
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff5db8ce3e  rfl: 0x0000000000000206  cr2: 0x00007fff96741148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x108b09000 -        0x108b0bff7 +a (0) <41E90DB3-381D-3F00-963F-675AF220E7E6> /Users/USER/*/a
       0x108b15000 -        0x108bacff7 +libstd-28edbc69fd879800.dylib (0) <EEE6EA29-230C-3531-BB50-391386928C2F> /Users/USER/*/libstd-28edbc69fd879800.dylib
       0x114397000 -        0x1143e198f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5b2a7000 -     0x7fff5b2dafff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5b7b9000 -     0x7fff5b7baff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5ba6f000 -     0x7fff5bac5fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5bac6000 -     0x7fff5baeaff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5ce3c000 -     0x7fff5d22d3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5d2fa000 -     0x7fff5d316ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5d8d4000 -     0x7fff5d8d8ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5d8d9000 -     0x7fff5d8e3ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5d8e4000 -     0x7fff5d8ebfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5d8ec000 -     0x7fff5d8f4ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5d8f5000 -     0x7fff5d97afff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5da02000 -     0x7fff5da3bff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5da3c000 -     0x7fff5da59ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5da5a000 -     0x7fff5da5affb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5da68000 -     0x7fff5da68ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5da69000 -     0x7fff5da6dffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5da6e000 -     0x7fff5da70ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5da71000 -     0x7fff5da72ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5da73000 -     0x7fff5da8afff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5da8b000 -     0x7fff5da8bfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5da8c000 -     0x7fff5db15ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5db16000 -     0x7fff5db19ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5db1a000 -     0x7fff5db1dffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5db1e000 -     0x7fff5db1ffff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5db20000 -     0x7fff5db26ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5db27000 -     0x7fff5db70ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5db71000 -     0x7fff5db96ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5db97000 -     0x7fff5dbe2fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5dbe3000 -     0x7fff5dc02fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5dc03000 -     0x7fff5dca7ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5dca8000 -     0x7fff5dcb2ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5dcb3000 -     0x7fff5dcbcff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5dcbd000 -     0x7fff5dcc4ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5dcc5000 -     0x7fff5dcd0fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5dcd1000 -     0x7fff5dcd4ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5dcd5000 -     0x7fff5dcd6ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5dcd7000 -     0x7fff5dcdeff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5dcdf000 -     0x7fff5dcf2ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5dcf4000 -     0x7fff5dcf9ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5dcfa000 -     0x7fff5dd26ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2598
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.3M resident=0K(0%) swapped_out_or_unallocated=198.3M(100%)
Writable regions: Total=73.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9260K        8 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4636K       44 
__LINKEDIT                       188.9M        5 
__TEXT                            9620K       44 
===========                     =======  ======= 
TOTAL                            276.1M      108 
TOTAL                            276.1M      108 
TOTAL, minus reserved VM space   275.9M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-02-122013-3_Traviss-Mac-1044.crash
Process:               a [44976]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44972]
Responsible:           a [44976]
User ID:               501
Date/Time:             2019-04-02 12:18:59.607 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-28edbc69fd879800.dylib  0x000000010657cafe std::panicking::rust_panic_with_hook::h18eece72e0e76c42 + 142
1   a                              0x00000001065478e5 std::panicking::begin_panic::hc3f6f6ee8ad55670 + 37
2   a                              0x000000010654540c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x0000000106544969 core::ptr::real_drop_in_place::h08174337b74568a1 + 9
4   a                              0x00000001065453e3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x0000000106546559 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x0000000106544936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf6a1ce72644edbef + 6 (rt.rs:64)
7   libstd-28edbc69fd879800.dylib  0x000000010657c578 std::panicking::try::do_call::he44de2c8a6962430 + 24
8   libstd-28edbc69fd879800.dylib  0x000000010658c7bf __rust_maybe_catch_panic + 31
9   libstd-28edbc69fd879800.dylib  0x000000010657d05e std::rt::lang_start_internal::h808ee93b9537043a + 542
10  a                              0x0000000106546d99 main + 41
11  libdyld.dylib                  0x00007fff5da3d115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee96bc418  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001065c4fc2  rbp: 0x00007ffee96bc510  rsp: 0x00007ffee96bc440
   r8: 0xffffffff00000100   r9: 0x00000001065f7a80  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001065c3d28  r14: 0x0000000106549460  r15: 0x00007ffee96bc520
  rip: 0x000000010657cafe  rfl: 0x0000000000010206  cr2: 0x000000010686544c
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x106541000 -        0x106548fff +a (0) <59C5D946-D10E-3D29-BE2F-861F81C75EC9> /Users/USER/*/a
       0x106558000 -        0x1065efff7 +libstd-28edbc69fd879800.dylib (0) <EEE6EA29-230C-3531-BB50-391386928C2F> /Users/USER/*/libstd-28edbc69fd879800.dylib
       0x107af7000 -        0x107b4198f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5b2a7000 -     0x7fff5b2dafff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5b7b9000 -     0x7fff5b7baff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5ba6f000 -     0x7fff5bac5fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5bac6000 -     0x7fff5baeaff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5ce3c000 -     0x7fff5d22d3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5d2fa000 -     0x7fff5d316ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5d8d4000 -     0x7fff5d8d8ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5d8d9000 -     0x7fff5d8e3ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5d8e4000 -     0x7fff5d8ebfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5d8ec000 -     0x7fff5d8f4ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5d8f5000 -     0x7fff5d97afff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5da02000 -     0x7fff5da3bff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5da3c000 -     0x7fff5da59ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5da5a000 -     0x7fff5da5affb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5da68000 -     0x7fff5da68ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5da69000 -     0x7fff5da6dffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5da6e000 -     0x7fff5da70ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5da71000 -     0x7fff5da72ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5da73000 -     0x7fff5da8afff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5da8b000 -     0x7fff5da8bfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5da8c000 -     0x7fff5db15ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5db16000 -     0x7fff5db19ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5db1a000 -     0x7fff5db1dffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5db1e000 -     0x7fff5db1ffff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5db20000 -     0x7fff5db26ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5db27000 -     0x7fff5db70ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5db71000 -     0x7fff5db96ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5db97000 -     0x7fff5dbe2fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5dbe3000 -     0x7fff5dc02fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5dc03000 -     0x7fff5dca7ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5dca8000 -     0x7fff5dcb2ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5dcb3000 -     0x7fff5dcbcff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5dcbd000 -     0x7fff5dcc4ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5dcc5000 -     0x7fff5dcd0fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5dcd1000 -     0x7fff5dcd4ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5dcd5000 -     0x7fff5dcd6ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5dcd7000 -     0x7fff5dcdeff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5dcdf000 -     0x7fff5dcf2ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5dcf4000 -     0x7fff5dcf9ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5dcfa000 -     0x7fff5dd26ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2598
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=92.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=92.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            28.5M       10 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4636K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            295.5M      110 
TOTAL                            295.5M      110 
TOTAL, minus reserved VM space   295.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-02-122013_Traviss-Mac-1044.crash
Process:               a [44977]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44972]
Responsible:           a [44977]
User ID:               501
Date/Time:             2019-04-02 12:18:59.608 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-28edbc69fd879800.dylib  0x0000000106d0dafe std::panicking::rust_panic_with_hook::h18eece72e0e76c42 + 142
1   a                              0x0000000106cdd8e5 std::panicking::begin_panic::hc3f6f6ee8ad55670 + 37
2   a                              0x0000000106cdb40c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x0000000106cda969 core::ptr::real_drop_in_place::h08174337b74568a1 + 9
4   a                              0x0000000106cdb3e3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x0000000106cdc559 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x0000000106cda936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf6a1ce72644edbef + 6 (rt.rs:64)
7   libstd-28edbc69fd879800.dylib  0x0000000106d0d578 std::panicking::try::do_call::he44de2c8a6962430 + 24
8   libstd-28edbc69fd879800.dylib  0x0000000106d1d7bf __rust_maybe_catch_panic + 31
9   libstd-28edbc69fd879800.dylib  0x0000000106d0e05e std::rt::lang_start_internal::h808ee93b9537043a + 542
10  a                              0x0000000106cdcd99 main + 41
11  libdyld.dylib                  0x00007fff5da3d115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee8f263f8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000106d55fc2  rbp: 0x00007ffee8f264f0  rsp: 0x00007ffee8f26420
   r8: 0xffffffff00000100   r9: 0x0000000106d88a80  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000106d54d28  r14: 0x0000000106cdf460  r15: 0x00007ffee8f26500
  rip: 0x0000000106d0dafe  rfl: 0x0000000000010202  cr2: 0x00007fc97c54811e
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x106cd7000 -        0x106cdefff +a (0) <59C5D946-D10E-3D29-BE2F-861F81C75EC9> /Users/USER/*/a
       0x106ce9000 -        0x106d80ff7 +libstd-28edbc69fd879800.dylib (0) <EEE6EA29-230C-3531-BB50-391386928C2F> /Users/USER/*/libstd-28edbc69fd879800.dylib
       0x114e55000 -        0x114e9f98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5b2a7000 -     0x7fff5b2dafff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5b7b9000 -     0x7fff5b7baff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5ba6f000 -     0x7fff5bac5fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5bac6000 -     0x7fff5baeaff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5ce3c000 -     0x7fff5d22d3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5d2fa000 -     0x7fff5d316ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5d8d4000 -     0x7fff5d8d8ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5d8d9000 -     0x7fff5d8e3ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5d8e4000 -     0x7fff5d8ebfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5d8ec000 -     0x7fff5d8f4ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5d8f5000 -     0x7fff5d97afff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5da02000 -     0x7fff5da3bff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5da3c000 -     0x7fff5da59ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5da5a000 -     0x7fff5da5affb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5da68000 -     0x7fff5da68ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5da69000 -     0x7fff5da6dffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5da6e000 -     0x7fff5da70ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5da71000 -     0x7fff5da72ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5da73000 -     0x7fff5da8afff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5da8b000 -     0x7fff5da8bfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5da8c000 -     0x7fff5db15ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5db16000 -     0x7fff5db19ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5db1a000 -     0x7fff5db1dffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5db1e000 -     0x7fff5db1ffff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5db20000 -     0x7fff5db26ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5db27000 -     0x7fff5db70ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5db71000 -     0x7fff5db96ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5db97000 -     0x7fff5dbe2fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5dbe3000 -     0x7fff5dc02fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5dc03000 -     0x7fff5dca7ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5dca8000 -     0x7fff5dcb2ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5dcb3000 -     0x7fff5dcbcff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5dcbd000 -     0x7fff5dcc4ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5dcc5000 -     0x7fff5dcd0fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5dcd1000 -     0x7fff5dcd4ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5dcd5000 -     0x7fff5dcd6ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5dcd7000 -     0x7fff5dcdeff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5dcdf000 -     0x7fff5dcf2ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5dcf4000 -     0x7fff5dcf9ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5dcfa000 -     0x7fff5dd26ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
