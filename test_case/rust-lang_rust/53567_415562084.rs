plain
[00:03:45]       Memory: 8 GB
[00:03:45]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:45]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:45]       SMC Version (system): 2.8f0
[00:03:45]       Serial Number (system): VMy7v2NEpocu
[00:03:45] 
[00:03:45] hw.ncpu: 4
[00:03:45] hw.byteorder: 1234
[00:03:45] hw.memsize: 8589934592
---
[01:11:31] ---- [run-pass] run-pass/issue-52705/main.rs stdout ----
[01:11:31] 
[01:11:31] error: compilation failed!
[01:11:31] status: exit code: 1
[01:11:31] command: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/travis/build/rust-lang/rust/src/test/run-pass/issue-52705/main.rs" "--target=x86_64-apple-darwin" "-C" "prefer-dynamic" "-o" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass/issue-52705/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--edition=2018" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass/issue-52705/main/auxiliary"
[01:11:31] ------------------------------------------
[01:11:31] 
[01:11:31] ------------------------------------------
[01:11:31] stderr:
[01:11:31] stderr:
[01:11:31] ------------------------------------------
[01:11:31] error: couldn't load codegen backend "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/codegen-backends/librustc_codegen_llvm-llvm.dylib": "dlopen(/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/codegen-backends/librustc_codegen_llvm-llvm.dylib, 10): Library not loaded: /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libPng.dylib\n  Referenced from: /System/Library/Frameworks/ImageIO.framework/Versions/A/ImageIO\n  Reason: Incompatible library version: ImageIO requires version 1.0.0 or later, but libPng.dylib provides version 0.0.0"
[01:11:31] 
[01:11:31] ------------------------------------------
[01:11:31] 
[01:11:31] thread '[run-pass] run-pass/issue-52705/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
---
[01:11:31] 
[01:11:31] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:11:31] 
[01:11:31] 
[01:11:31] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/run-pass" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "run-pass" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "7.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:31] 
[01:11:31] 
[01:11:31] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:11:31] Build completed unsuccessfully in 0:19:21
[01:11:31] Build completed unsuccessfully in 0:19:21
[01:11:31] make: *** [check] Error 1
travis_time:end:15e8afd6:start=1535052022748200000,finish=1535056314068856000,duration=4291320656000

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0bc14404
---
travis_fold:start:after_failure.2
travis_time:start:0d938638
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1184
drwx------  21 travis  staff    714 Aug 23 20:30 .
-rw-------@  1 travis  staff  37510 Aug 23 20:30 a_2018-08-23-203030-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62273 Aug 23 20:30 a_2018-08-23-203030_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60415 Aug 23 20:30 a_2018-08-23-203025-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37409 Aug 23 20:30 a_2018-08-23-203025_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10170 Aug 23 20:30 a_2018-08-23-203017_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9901 Aug 23 20:30 a_2018-08-23-203012_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9906 Aug 23 20:30 a_2018-08-23-203002-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9868 Aug 23 20:30 a_2018-08-23-203002_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9965 Aug 23 20:29 a_2018-08-23-202934_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63186 Aug 23 20:29 a_2018-08-23-202925_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65147 Aug 23 20:29 a_2018-08-23-202922-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64334 Aug 23 20:29 a_2018-08-23-202922_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63943 Aug 23 20:29 a_2018-08-23-202921_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11783 Aug 23 20:26 a_2018-08-23-202658_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9925 Aug 23 20:26 a_2018-08-23-202601_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10332 Aug 23 20:24 a_2018-08-23-202447_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10512 Aug 23 20:24 a_2018-08-23-202415-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10242 Aug 23 20:24 a_2018-08-23-202415-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10514 Aug 23 20:24 a_2018-08-23-202415_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:19622c31
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-23-202415-1_Traviss-Mac-1044.crash
Process:               a [39122]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        a [39114]
Responsible:           a [39122]
User ID:               501
Date/Time:             2018-08-23 20:23:57.104 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 3900 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-ad701aaa9383bcfc.dylib  0x000000010b17752b std::panicking::rust_panic_with_hook::h5c8747e4f3f671f4 + 139
1   a                              0x000000010b1347d8 std::panicking::begin_panic::hbd11d4c5bdf8705f + 40
2   a                              0x000000010b13225c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hee8ed3175b25d327 + 28
3   a                              0x000000010b131e09 core::ptr::drop_in_place::hb47aeb4441f77ce0 + 9
4   a                              0x000000010b132233 backtrace::double::h3a79da7ae181846f + 35
5   a                              0x000000010b1333ee backtrace::main::h9751c5dfa7559d82 + 4254 (backtrace.rs:113)
6   a                              0x000000010b131736 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h9a8fb9f3814170e8 + 6 (rt.rs:74)
7   libstd-ad701aaa9383bcfc.dylib  0x000000010b176ff8 std::panicking::try::do_call::h6ef2de9af150b120 (.llvm.8771432455608051652) + 24
8   libstd-ad701aaa9383bcfc.dylib  0x000000010b1880cf __rust_maybe_catch_panic + 31
9   libstd-ad701aaa9383bcfc.dylib  0x000000010b15da7d std::rt::lang_start_internal::h2f337f6fc9b8f4b8 + 237
10  a                              0x000000010b133c7c main + 44
11  libdyld.dylib                  0x00007fff6d0a6115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee4ad1103  rbx: 0x0000000000000002  rcx: 0x00007fff6d1f79d2  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010b1e6445  rbp: 0x00007ffee4ad1280  rsp: 0x00007ffee4ad1200
   r8: 0x000000010b1e6040   r9: 0x000000010b22bf20  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010b1e6040  r14: 0x000000010b136460  r15: 0x00007ffee4ad1290
  rip: 0x000000010b17752b  rfl: 0x0000000000010202  cr2: 0x000000010b45ba9f
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10b12d000 -        0x10b135fff +a (0) <339F3072-AF61-30AF-B1D6-849E9B68C93F> /Users/USER/*/a
       0x10b143000 -        0x10b221ff7 +libstd-ad701aaa9383bcfc.dylib (0) <D8531DD3-C6A3-3B7C-BE30-0C763EB5871D> /Users/USER/*/libstd-ad701aaa9383bcfc.dylib
       0x10f306000 -        0x10f35098f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6a910000 -     0x7fff6a943fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6ae22000 -     0x7fff6ae23ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6b0d8000 -     0x7fff6b12efff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6b12f000 -     0x7fff6b153ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6c4a5000 -     0x7fff6c8963b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6c963000 -     0x7fff6c97fffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6cf3d000 -     0x7fff6cf41ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6cf42000 -     0x7fff6cf4cff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6cf4d000 -     0x7fff6cf54fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6cf55000 -     0x7fff6cf5dffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6cf5e000 -     0x7fff6cfe3fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6d06b000 -     0x7fff6d0a4ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6d0a5000 -     0x7fff6d0c2ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6d0c3000 -     0x7fff6d0c3ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6d0d1000 -     0x7fff6d0d1ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6d0d2000 -     0x7fff6d0d6ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6d0d7000 -     0x7fff6d0d9ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6d0da000 -     0x7fff6d0dbff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6d0dc000 -     0x7fff6d0f3fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6d0f4000 -     0x7fff6d0f4fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6d0f5000 -     0x7fff6d17eff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6d17f000 -     0x7fff6d182ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6d183000 -     0x7fff6d186ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6d187000 -     0x7fff6d188fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6d189000 -     0x7fff6d18fff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6d190000 -     0x7fff6d1d9ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6d1da000 -     0x7fff6d1ffff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6d200000 -     0x7fff6d24bfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6d24c000 -     0x7fff6d26bfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6d26c000 -     0x7fff6d310ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6d311000 -     0x7fff6d31bffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6d31c000 -     0x7fff6d325ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6d326000 -     0x7fff6d32dff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6d32e000 -     0x7fff6d339fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6d33a000 -     0x7fff6d33dff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6d33e000 -     0x7fff6d33fff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6d340000 -     0x7fff6d347ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6d348000 -     0x7fff6d35bff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6d35d000 -     0x7fff6d362ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6d363000 -     0x7fff6d38fff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2547
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.8M resident=0K(0%) swapped_out_or_unallocated=198.8M(100%)
Writable regions: Total=76.9M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.9M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9268K       10 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4340K       45 
__LINKEDIT                       189.1M        5 
__TEXT                            9928K       44 
===========                     =======  ======= 
TOTAL                            280.2M      113 
TOTAL                            280.2M      113 
TOTAL, minus reserved VM space   280.1M      113 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-23-202415-2_Traviss-Mac-1044.crash
Process:               a [38439]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [38438]
Responsible:           a [38439]
User ID:               501
Date/Time:             2018-08-23 20:23:32.774 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 3900 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x0000000101f3173e abort_on_c_abi::panic_in_ffi::h305168285df224eb + 30
1   a                              0x0000000101f30b19 std::panicking::try::do_call::h9929387673d5db4e (.llvm.18291960263502774623) + 9
2   libstd-ad701aaa9383bcfc.dylib  0x0000000101f820cf __rust_maybe_catch_panic + 31
3   a                              0x0000000101f31991 abort_on_c_abi::main::h82c0574e6cdd7b93 + 593
4   a                              0x0000000101f2fd96 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h195a004642ff1eb3 + 6
5   libstd-ad701aaa9383bcfc.dylib  0x0000000101f70ff8 std::panicking::try::do_call::h6ef2de9af150b120 (.llvm.8771432455608051652) + 24
6   libstd-ad701aaa9383bcfc.dylib  0x0000000101f820cf __rust_maybe_catch_panic + 31
7   libstd-ad701aaa9383bcfc.dylib  0x0000000101f57a7d std::rt::lang_start_internal::h2f337f6fc9b8f4b8 + 237
8   a                              0x0000000101f31c9c main + 44
9   libdyld.dylib                  0x00007fff6d0a6115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x000000010261c050  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffeedccea28  rsi: 0xfffffffffffffce8  rbp: 0x00007ffeedccf430  rsp: 0x00007ffeedccf430
   r8: 0x0000000000000000   r9: 0x0000000102025f20  r10: 0x000000010e8f08d0  r11: 0x00007fff6d35d96c
  r12: 0x0000000000000000  r13: 0x0000000000000000  r14: 0x00007ffeedccf550  r15: 0x00007ffeedccf498
  rip: 0x0000000101f3173e  rfl: 0x0000000000010202  cr2: 0x0000000101fdd458
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x101f2f000 -        0x101f32ff7 +a (0) <30A1AF57-6322-315F-BA4A-BF3F385325E2> /Users/USER/*/a
       0x101f3d000 -        0x10201bff7 +libstd-ad701aaa9383bcfc.dylib (0) <D8531DD3-C6A3-3B7C-BE30-0C763EB5871D> /Users/USER/*/libstd-ad701aaa9383bcfc.dylib
       0x10e89e000 -        0x10e8e898f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6a910000 -     0x7fff6a943fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6ae22000 -     0x7fff6ae23ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6b0d8000 -     0x7fff6b12efff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6b12f000 -     0x7fff6b153ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6c4a5000 -     0x7fff6c8963b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6c963000 -     0x7fff6c97fffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6cf3d000 -     0x7fff6cf41ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6cf42000 -     0x7fff6cf4cff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6cf4d000 -     0x7fff6cf54fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6cf55000 -     0x7fff6cf5dffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6cf5e000 -     0x7fff6cfe3fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6d06b000 -     0x7fff6d0a4ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6d0a5000 -     0x7fff6d0c2ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6d0c3000 -     0x7fff6d0c3ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6d0d1000 -     0x7fff6d0d1ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6d0d2000 -     0x7fff6d0d6ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6d0d7000 -     0x7fff6d0d9ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6d0da000 -     0x7fff6d0dbff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6d0dc000 -     0x7fff6d0f3fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6d0f4000 -     0x7fff6d0f4fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6d0f5000 -     0x7fff6d17eff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6d17f000 -     0x7fff6d182ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6d183000 -     0x7fff6d186ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6d187000 -     0x7fff6d188fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6d189000 -     0x7fff6d18fff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6d190000 -     0x7fff6d1d9ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6d1da000 -     0x7fff6d1ffff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6d200000 -     0x7fff6d24bfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6d24c000 -     0x7fff6d26bfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6d26c000 -     0x7fff6d310ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6d311000 -     0x7fff6d31bffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6d31c000 -     0x7fff6d325ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6d326000 -     0x7fff6d32dff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6d32e000 -     0x7fff6d339fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6d33a000 -     0x7fff6d33dff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6d33e000 -     0x7fff6d33fff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6d340000 -     0x7fff6d347ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6d348000 -     0x7fff6d35bff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6d35d000 -     0x7fff6d362ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6d363000 -     0x7fff6d38fff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2547
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.7M resident=0K(0%) swapped_out_or_unallocated=198.7M(100%)
Writable regions: Total=85.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=85.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            19.1M       10 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4340K       45 
__LINKEDIT                       189.1M        5 
__TEXT                            9908K       44 
===========                     =======  ======= 
TOTAL                            290.2M      113 
TOTAL                            290.2M      113 
TOTAL, minus reserved VM space   290.1M      113 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-23-202415_Traviss-Mac-1044.crash
Process:               a [39123]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [39114]
Responsible:           a [39123]
User ID:               501
Date/Time:             2018-08-23 20:23:57.171 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 3900 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-ad701aaa9383bcfc.dylib  0x000000010bd8952b std::panicking::rust_panic_with_hook::h5c8747e4f3f671f4 + 139
1   a                              0x000000010bd497d8 std::panicking::begin_panic::hbd11d4c5bdf8705f + 40
2   a                              0x000000010bd4725c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hee8ed3175b25d327 + 28
3   a                              0x000000010bd46e09 core::ptr::drop_in_place::hb47aeb4441f77ce0 + 9
4   a                              0x000000010bd47233 backtrace::double::h3a79da7ae181846f + 35
5   a                              0x000000010bd483ee backtrace::main::h9751c5dfa7559d82 + 4254 (backtrace.rs:113)
6   a                              0x000000010bd46736 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h9a8fb9f3814170e8 + 6 (rt.rs:74)
7   libstd-ad701aaa9383bcfc.dylib  0x000000010bd88ff8 std::panicking::try::do_call::h6ef2de9af150b120 (.llvm.8771432455608051652) + 24
8   libstd-ad701aaa9383bcfc.dylib  0x000000010bd9a0cf __rust_maybe_catch_panic + 31
9   libstd-ad701aaa9383bcfc.dylib  0x000000010bd6fa7d std::rt::lang_start_internal::h2f337f6fc9b8f4b8 + 237
10  a                              0x000000010bd48c7c main + 44
11  libdyld.dylib                  0x00007fff6d0a6115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee3ebc103  rbx: 0x0000000000000002  rcx: 0x00007fff6d1f79d2  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010bdf8445  rbp: 0x00007ffee3ebc270  rsp: 0x00007ffee3ebc1f0
   r8: 0x000000010bdf8040   r9: 0x000000010be3df20  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010bdf8040  r14: 0x000000010bd4b460  r15: 0x00007ffee3ebc280
  rip: 0x000000010bd8952b  rfl: 0x0000000000010202  cr2: 0x000000010e29e000
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10bd42000 -        0x10bd4afff +a (0) <339F3072-AF61-30AF-B1D6-849E9B68C93F> /Users/USER/*/a
       0x10bd55000 -        0x10be33ff7 +libstd-ad701aaa9383bcfc.dylib (0) <D8531DD3-C6A3-3B7C-BE30-0C763EB5871D> /Users/USER/*/libstd-ad701aaa9383bcfc.dylib
       0x11bb33000 -        0x11bb7d98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6a910000 -     0x7fff6a943fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6ae22000 -     0x7fff6ae23ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6b0d8000 -     0x7fff6b12efff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6b12f000 -     0x7fff6b153ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6c4a5000 -     0x7fff6c8963b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6c963000 -     0x7fff6c97fffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6cf3d000 -     0x7fff6cf41ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6cf42000 -     0x7fff6cf4cff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6cf4d000 -     0x7fff6cf54fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6cf55000 -     0x7fff6cf5dffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6cf5e000 -     0x7fff6cfe3fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6d06b000 -     0x7fff6d0a4ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6d0a5000 -     0x7fff6d0c2ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6d0c3000 -     0x7fff6d0c3ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6d0d1000 -     0x7fff6d0d1ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6d0d2000 -     0x7fff6d0d6ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6d0d7000 -     0x7fff6d0d9ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6d0da000 -     0x7fff6d0dbff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6d0dc000 -     0x7fff6d0f3fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6d0f4000 -     0x7fff6d0f4fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6d0f5000 -     0x7fff6d17eff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6d17f000 -     0x7fff6d182ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6d183000 -     0x7fff6d186ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6d187000 -     0x7fff6d188fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6d189000 -     0x7fff6d18fff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6d190000 -     0x7fff6d1d9ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6d1da000 -     0x7fff6d1ffff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6d200000 -     0x7fff6d24bfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6d24c000 -     0x7fff6d26bfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6d26c000 -     0x7fff6d310ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6d311000 -     0x7fff6d31bffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6d31c000 -     0x7fff6d325ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6d326000 -     0x7fff6d32dff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6d32e000 -     0x7fff6d339fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6d33a000 -     0x7fff6d33dff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6d33e000 -     0x7fff6d33fff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6d340000 -     0x7fff6d347ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6d348000 -     0x7fff6d35bff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6d35d000 -     0x7fff6d362ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6d363000 -     0x7fff6d38fff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2547
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.8M resident=0K(0%) swapped_out_or_unallocated=198.8M(100%)
Writable regions: Total=76.9M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.9M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9268K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4340K       45 
__LINKEDIT                       189.1M        5 
__TEXT                            9928K       44 
===========                     =======  ======= 
TOTAL                            280.2M      114 
TOTAL                            280.2M      114 
TOTAL, minus reserved VM space   280.1M      114 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-23-202447_Traviss-Mac-1044.crash
Process:               a [40514]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [40513]
Responsible:           a [40514]
User ID:               501
Date/Time:             2018-08-23 20:24:47.287 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff6d1f5e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff6d334150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff6d152312 abort + 127
3   libstd-ad701aaa9383bcfc.dylib  0x0000000108ade5b9 std::sys::unix::abort_internal::ha9fe51dbbe3c616e + 9
4   libstd-ad701aaa9383bcfc.dylib  0x0000000108ad1500 rust_oom + 32
5   libstd-ad701aaa9383bcfc.dylib  0x0000000108b3b829 alloc::alloc::handle_alloc_error::h2c54d38bf1a61b50 + 9
6   a                              0x0000000108abc26d default_alloc_error_hook::main::hbf3cf79eecbb97ff + 797
7   a                              0x0000000108abb296 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h21a6cf08109b57bc + 6
8   libstd-ad701aaa9383bcfc.dylib  0x0000000108af4ff8 std::panicking::try::do_call::h6ef2de9af150b120 (.llvm.8771432455608051652) + 24
9   libstd-ad701aaa9383bcfc.dylib  0x0000000108b060cf __rust_maybe_catch_panic + 31
10  libstd-ad701aaa9383bcfc.dylib  0x0000000108adba7d std::rt::lang_start_internal::h2f337f6fc9b8f4b8 + 237
11  a                              0x0000000108abc3dc main + 44
12  libdyld.dylib                  0x00007fff6d0a6115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fffa5dcc340  rcx: 0x00007ffee7144398  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee71443d0  rsp: 0x00007ffee7144398
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff6d1f5e3e  rfl: 0x0000000000000206  cr2: 0x00007fffa5daa148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x108aba000 -        0x108abcff7 +a (0) <673E4032-ABC8-3C48-B6C1-498A8E678AFE> /Users/USER/*/a
       0x108ac1000 -        0x108b9fff7 +libstd-ad701aaa9383bcfc.dylib (0) <D8531DD3-C6A3-3B7C-BE30-0C763EB5871D> /Users/USER/*/libstd-ad701aaa9383bcfc.dylib
       0x111b9a000 -        0x111be498f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6a910000 -     0x7fff6a943fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6ae22000 -     0x7fff6ae23ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6b0d8000 -     0x7fff6b12efff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6b12f000 -     0x7fff6b153ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6c4a5000 -     0x7fff6c8963b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6c963000 -     0x7fff6c97fffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6cf3d000 -     0x7fff6cf41ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6cf42000 -     0x7fff6cf4cff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6cf4d000 -     0x7fff6cf54fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6cf55000 -     0x7fff6cf5dffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6cf5e000 -     0x7fff6cfe3fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6d06b000 -     0x7fff6d0a4ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6d0a5000 -     0x7fff6d0c2ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6d0c3000 -     0x7fff6d0c3ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6d0d1000 -     0x7fff6d0d1ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6d0d2000 -     0x7fff6d0d6ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6d0d7000 -     0x7fff6d0d9ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6d0da000 -     0x7fff6d0dbff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6d0dc000 -     0x7fff6d0f3fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6d0f4000 -     0x7fff6d0f4fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6d0f5000 -     0x7fff6d17eff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6d17f000 -     0x7fff6d182ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6d183000 -     0x7fff6d186ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6d187000 -     0x7fff6d188fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6d189000 -     0x7fff6d18fff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6d190000 -     0x7fff6d1d9ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6d1da000 -     0x7fff6d1ffff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6d200000 -     0x7fff6d24bfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6d24c000 -     0x7fff6d26bfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6d26c000 -     0x7fff6d310ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6d311000 -     0x7fff6d31bffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6d31c000 -     0x7fff6d325ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6d326000 -     0x7fff6d32dff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6d32e000 -     0x7fff6d339fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6d33a000 -     0x7fff6d33dff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6d33e000 -     0x7fff6d33fff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6d340000 -     0x7fff6d347ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6d348000 -     0x7fff6d35bff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6d35d000 -     0x7fff6d362ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6d363000 -     0x7fff6d38fff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2547
    thread_create: 0
VM Region Summary:
VM Region Summary:
