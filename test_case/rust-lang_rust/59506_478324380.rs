plain
[00:02:50]       Memory: 8 GB
[00:02:50]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:02:50]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:50]       SMC Version (system): 2.8f0
[00:02:50]       Serial Number (system): VMAx/iRXvvGe
[00:02:50] 
[00:02:50] hw.ncpu: 4
[00:02:50] hw.byteorder: 1234
[00:02:50] hw.memsize: 8589934592
---
[01:33:49] ---- [codegen] codegen/instrument-mcount.rs stdout ----
[01:33:49] 
[01:33:49] error: compilation failed!
[01:33:49] status: exit code: 101
[01:33:49] command: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/travis/build/rust-lang/rust/src/test/codegen/instrument-mcount.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "-C" "prefer-dynamic" "--out-dir" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/codegen/instrument-mcount" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-Z" "instrument-mcount" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/codegen/instrument-mcount/auxiliary" "--emit=llvm-ir"
[01:33:49] ------------------------------------------
[01:33:49] 
[01:33:49] ------------------------------------------
[01:33:49] stderr:
[01:33:49] stderr:
[01:33:49] ------------------------------------------
[01:33:49] thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: NulError(0, [0, 49, 109, 99, 111, 117, 110, 116])', src/libcore/result.rs:997:5
[01:33:49] 
[01:33:49] error: internal compiler error: unexpected panic
[01:33:49] 
[01:33:49] note: the compiler unexpectedly panicked. this is a bug.
[01:33:49] note: the compiler unexpectedly panicked. this is a bug.
[01:33:49] 
[01:33:49] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:33:49] 
[01:33:49] note: rustc 1.35.0-dev running on x86_64-apple-darwin
[01:33:49] 
[01:33:49] note: compiler flags: -Z threads=1 -Z unstable-options -Z instrument-mcount -C prefer-dynamic -C rpath
[01:33:49] 
[01:33:49] ------------------------------------------
[01:33:49] 
[01:33:49] thread '[codegen] codegen/instrument-mcount.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
---
[01:33:49] 
[01:33:49] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:33:49] 
[01:33:49] 
[01:33:49] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/codegen" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/codegen" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "codegen" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0-rust-1.35.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:33:49] 
[01:33:49] 
[01:33:49] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:33:49] Build completed unsuccessfully in 0:19:27
[01:33:49] Build completed unsuccessfully in 0:19:27
[01:33:49] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02db0d96
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 31 09:03:09 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:0270cbf0
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1184
drwx------  21 travis  staff    714 Mar 31 09:00 .
-rw-------@  1 travis  staff  62244 Mar 31 09:00 a_2019-03-31-090007-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37481 Mar 31 09:00 a_2019-03-31-090007_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60387 Mar 31 08:59 a_2019-03-31-085959-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37229 Mar 31 08:59 a_2019-03-31-085959_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Mar 31 08:59 a_2019-03-31-085954_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Mar 31 08:59 a_2019-03-31-085949_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Mar 31 08:59 a_2019-03-31-085941-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 Mar 31 08:59 a_2019-03-31-085941_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Mar 31 08:59 a_2019-03-31-085906_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63100 Mar 31 08:58 a_2019-03-31-085855_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 Mar 31 08:58 a_2019-03-31-085851-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65082 Mar 31 08:58 a_2019-03-31-085851-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64264 Mar 31 08:58 a_2019-03-31-085851_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11715 Mar 31 08:56 a_2019-03-31-085646_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Mar 31 08:55 a_2019-03-31-085547_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Mar 31 08:54 a_2019-03-31-085429_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10213 Mar 31 08:54 a_2019-03-31-085410-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10493 Mar 31 08:54 a_2019-03-31-085411-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10489 Mar 31 08:54 a_2019-03-31-085410_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:05aec2ad
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-31-085410-1_Traviss-Mac-1044.crash
Process:               a [44147]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44144]
Responsible:           a [44147]
User ID:               501
Date/Time:             2019-03-31 08:52:59.224 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5100 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x0000000109b468ae abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x0000000109b45ca9 std::panicking::try::do_call::hc7b4d50989e643d9 (.llvm.1328971572518733427) + 9
2   libstd-28edbc69fd879800.dylib  0x0000000109b867bf __rust_maybe_catch_panic + 31
3   a                              0x0000000109b46b01 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x0000000109b450f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf4f268f28e9a6b3d + 6
5   libstd-28edbc69fd879800.dylib  0x0000000109b76578 std::panicking::try::do_call::he44de2c8a6962430 + 24
6   libstd-28edbc69fd879800.dylib  0x0000000109b867bf __rust_maybe_catch_panic + 31
7   libstd-28edbc69fd879800.dylib  0x0000000109b7705e std::rt::lang_start_internal::h808ee93b9537043a + 542
8   a                              0x0000000109b46e09 main + 41
9   libdyld.dylib                  0x00007fff7b904115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007fdef7d00010  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee60b8c18  rsi: 0x00000000ffffffc3  rbp: 0x00007ffee60b9670  rsp: 0x00007ffee60b9670
   r8: 0x00000000ef7d0006   r9: 0x0000000000000004  r10: 0x000000010faa58c0  r11: 0x00007fff7bbbb96c
  r12: 0x0000000109e9d000  r13: 0x0000000000000000  r14: 0x00007ffee60b9790  r15: 0x00007ffee60b96d8
  rip: 0x0000000109b468ae  rfl: 0x0000000000010206  cr2: 0x0000000109bbdb18
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x109b44000 -        0x109b47ff7 +a (0) <20AB01AA-BF77-3A4D-BFFF-B6261B463FB9> /Users/USER/*/a
       0x109b52000 -        0x109be9ff7 +libstd-28edbc69fd879800.dylib (0) <C8EDCC36-CD0C-39F3-87E7-27A2D78C38DD> /Users/USER/*/libstd-28edbc69fd879800.dylib
       0x10fa53000 -        0x10fa9d98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7916e000 -     0x7fff791a1fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff79680000 -     0x7fff79681ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff79936000 -     0x7fff7998cfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff7998d000 -     0x7fff799b1ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff7ad03000 -     0x7fff7b0f43b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7b1c1000 -     0x7fff7b1ddffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff7b79b000 -     0x7fff7b79fff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff7b7a0000 -     0x7fff7b7aaff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff7b7ab000 -     0x7fff7b7b2fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7b7b3000 -     0x7fff7b7bbffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff7b7bc000 -     0x7fff7b841fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff7b8c9000 -     0x7fff7b902ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7b903000 -     0x7fff7b920ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff7b921000 -     0x7fff7b921ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff7b92f000 -     0x7fff7b92fff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff7b930000 -     0x7fff7b934ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff7b935000 -     0x7fff7b937ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff7b938000 -     0x7fff7b939ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff7b93a000 -     0x7fff7b951fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7b952000 -     0x7fff7b952fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7b953000 -     0x7fff7b9dcff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7b9dd000 -     0x7fff7b9e0ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7b9e1000 -     0x7fff7b9e4ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7b9e5000 -     0x7fff7b9e6fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7b9e7000 -     0x7fff7b9edff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7b9ee000 -     0x7fff7ba37ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7ba38000 -     0x7fff7ba5dff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7ba5e000 -     0x7fff7baa9fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7baaa000 -     0x7fff7bac9fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7baca000 -     0x7fff7bb6eff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7bb6f000 -     0x7fff7bb79ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7bb7a000 -     0x7fff7bb83ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7bb84000 -     0x7fff7bb8bff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7bb8c000 -     0x7fff7bb97fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff7bb98000 -     0x7fff7bb9bff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7bb9c000 -     0x7fff7bb9dff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7bb9e000 -     0x7fff7bba5ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7bba6000 -     0x7fff7bbb9ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7bbbb000 -     0x7fff7bbc0ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff7bbc1000 -     0x7fff7bbedff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2368
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-31-085410_Traviss-Mac-1044.crash
Process:               a [44959]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        a [44955]
Responsible:           a [44959]
User ID:               501
Date/Time:             2019-03-31 08:53:27.646 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5200 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-28edbc69fd879800.dylib  0x000000010a986afe std::panicking::rust_panic_with_hook::h18eece72e0e76c42 + 142
1   a                              0x000000010a9518e5 std::panicking::begin_panic::hc3f6f6ee8ad55670 + 37
2   a                              0x000000010a94f40c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x000000010a94e969 core::ptr::real_drop_in_place::h08174337b74568a1 + 9
4   a                              0x000000010a94f3e3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x000000010a950559 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x000000010a94e936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf6a1ce72644edbef + 6 (rt.rs:64)
7   libstd-28edbc69fd879800.dylib  0x000000010a986578 std::panicking::try::do_call::he44de2c8a6962430 + 24
8   libstd-28edbc69fd879800.dylib  0x000000010a9967bf __rust_maybe_catch_panic + 31
9   libstd-28edbc69fd879800.dylib  0x000000010a98705e std::rt::lang_start_internal::h808ee93b9537043a + 542
10  a                              0x000000010a950d99 main + 41
11  libdyld.dylib                  0x00007fff7b904115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee52b23d8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010a9cefc2  rbp: 0x00007ffee52b24d0  rsp: 0x00007ffee52b2400
   r8: 0xffffffff00000100   r9: 0x000000010aa01a80  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010a9cdd28  r14: 0x000000010a953460  r15: 0x00007ffee52b24e0
  rip: 0x000000010a986afe  rfl: 0x0000000000010206  cr2: 0x000000010ac6f44c
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10a94b000 -        0x10a952fff +a (0) <46B436BC-6EBF-3549-8E78-C71D322D386A> /Users/USER/*/a
       0x10a962000 -        0x10a9f9ff7 +libstd-28edbc69fd879800.dylib (0) <C8EDCC36-CD0C-39F3-87E7-27A2D78C38DD> /Users/USER/*/libstd-28edbc69fd879800.dylib
       0x11a6c3000 -        0x11a70d98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7916e000 -     0x7fff791a1fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff79680000 -     0x7fff79681ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff79936000 -     0x7fff7998cfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff7998d000 -     0x7fff799b1ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff7ad03000 -     0x7fff7b0f43b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7b1c1000 -     0x7fff7b1ddffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff7b79b000 -     0x7fff7b79fff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff7b7a0000 -     0x7fff7b7aaff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff7b7ab000 -     0x7fff7b7b2fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7b7b3000 -     0x7fff7b7bbffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff7b7bc000 -     0x7fff7b841fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff7b8c9000 -     0x7fff7b902ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7b903000 -     0x7fff7b920ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff7b921000 -     0x7fff7b921ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff7b92f000 -     0x7fff7b92fff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff7b930000 -     0x7fff7b934ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff7b935000 -     0x7fff7b937ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff7b938000 -     0x7fff7b939ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff7b93a000 -     0x7fff7b951fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7b952000 -     0x7fff7b952fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7b953000 -     0x7fff7b9dcff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7b9dd000 -     0x7fff7b9e0ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7b9e1000 -     0x7fff7b9e4ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7b9e5000 -     0x7fff7b9e6fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7b9e7000 -     0x7fff7b9edff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7b9ee000 -     0x7fff7ba37ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7ba38000 -     0x7fff7ba5dff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7ba5e000 -     0x7fff7baa9fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7baaa000 -     0x7fff7bac9fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7baca000 -     0x7fff7bb6eff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7bb6f000 -     0x7fff7bb79ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7bb7a000 -     0x7fff7bb83ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7bb84000 -     0x7fff7bb8bff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7bb8c000 -     0x7fff7bb97fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff7bb98000 -     0x7fff7bb9bff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7bb9c000 -     0x7fff7bb9dff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7bb9e000 -     0x7fff7bba5ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7bba6000 -     0x7fff7bbb9ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7bbbb000 -     0x7fff7bbc0ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff7bbc1000 -     0x7fff7bbedff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2368
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=83.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=83.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            19.5M       10 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4636K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            286.5M      110 
TOTAL                            286.5M      110 
TOTAL, minus reserved VM space   286.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-31-085411-1_Traviss-Mac-1044.crash
Process:               a [44960]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44955]
Responsible:           a [44960]
User ID:               501
Date/Time:             2019-03-31 08:53:27.689 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5200 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-28edbc69fd879800.dylib  0x00000001081a0afe std::panicking::rust_panic_with_hook::h18eece72e0e76c42 + 142
1   a                              0x00000001081708e5 std::panicking::begin_panic::hc3f6f6ee8ad55670 + 37
2   a                              0x000000010816e40c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x000000010816d969 core::ptr::real_drop_in_place::h08174337b74568a1 + 9
4   a                              0x000000010816e3e3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x000000010816f559 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x000000010816d936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf6a1ce72644edbef + 6 (rt.rs:64)
7   libstd-28edbc69fd879800.dylib  0x00000001081a0578 std::panicking::try::do_call::he44de2c8a6962430 + 24
8   libstd-28edbc69fd879800.dylib  0x00000001081b07bf __rust_maybe_catch_panic + 31
9   libstd-28edbc69fd879800.dylib  0x00000001081a105e std::rt::lang_start_internal::h808ee93b9537043a + 542
10  a                              0x000000010816fd99 main + 41
11  libdyld.dylib                  0x00007fff7b904115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee7a933b8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001081e8fc2  rbp: 0x00007ffee7a934b0  rsp: 0x00007ffee7a933e0
   r8: 0xffffffff00000100   r9: 0x000000010821ba80  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001081e7d28  r14: 0x0000000108172460  r15: 0x00007ffee7a934c0
  rip: 0x00000001081a0afe  rfl: 0x0000000000010202  cr2: 0x00007f9636748180
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10816a000 -        0x108171fff +a (0) <46B436BC-6EBF-3549-8E78-C71D322D386A> /Users/USER/*/a
       0x10817c000 -        0x108213ff7 +libstd-28edbc69fd879800.dylib (0) <C8EDCC36-CD0C-39F3-87E7-27A2D78C38DD> /Users/USER/*/libstd-28edbc69fd879800.dylib
       0x115bf4000 -        0x115c3e98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7916e000 -     0x7fff791a1fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff79680000 -     0x7fff79681ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff79936000 -     0x7fff7998cfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff7998d000 -     0x7fff799b1ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff7ad03000 -     0x7fff7b0f43b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7b1c1000 -     0x7fff7b1ddffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff7b79b000 -     0x7fff7b79fff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff7b7a0000 -     0x7fff7b7aaff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff7b7ab000 -     0x7fff7b7b2fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7b7b3000 -     0x7fff7b7bbffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff7b7bc000 -     0x7fff7b841fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff7b8c9000 -     0x7fff7b902ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7b903000 -     0x7fff7b920ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff7b921000 -     0x7fff7b921ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff7b92f000 -     0x7fff7b92fff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff7b930000 -     0x7fff7b934ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff7b935000 -     0x7fff7b937ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff7b938000 -     0x7fff7b939ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff7b93a000 -     0x7fff7b951fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7b952000 -     0x7fff7b952fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7b953000 -     0x7fff7b9dcff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7b9dd000 -     0x7fff7b9e0ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7b9e1000 -     0x7fff7b9e4ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7b9e5000 -     0x7fff7b9e6fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7b9e7000 -     0x7fff7b9edff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7b9ee000 -     0x7fff7ba37ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7ba38000 -     0x7fff7ba5dff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7ba5e000 -     0x7fff7baa9fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7baaa000 -     0x7fff7bac9fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7baca000 -     0x7fff7bb6eff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7bb6f000 -     0x7fff7bb79ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7bb7a000 -     0x7fff7bb83ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7bb84000 -     0x7fff7bb8bff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7bb8c000 -     0x7fff7bb97fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff7bb98000 -     0x7fff7bb9bff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7bb9c000 -     0x7fff7bb9dff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7bb9e000 -     0x7fff7bba5ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7bba6000 -     0x7fff7bbb9ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7bbbb000 -     0x7fff7bbc0ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff7bbc1000 -     0x7fff7bbedff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2368
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=100.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=100.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            36.5M        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4636K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            303.5M      110 
TOTAL                            303.5M      110 
TOTAL, minus reserved VM space   303.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-31-085429_Traviss-Mac-1044.crash
Process:               a [46640]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [46638]
Responsible:           a [46640]
User ID:               501
Date/Time:             2019-03-31 08:54:27.153 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5200 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff7ba53e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff7bb92150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff7b9b0312 abort + 127
3   libstd-28edbc69fd879800.dylib  0x000000010efd8ca9 std::sys::unix::abort_internal::h8137cbab466f581f + 9
4   libstd-28edbc69fd879800.dylib  0x000000010efc8b90 rust_oom + 32
5   libstd-28edbc69fd879800.dylib  0x000000010efe9f69 alloc::alloc::handle_alloc_error::h0c82d64670f3f29b + 9
6   a                              0x000000010efa141f default_alloc_error_hook::main::hbf2d06db626d002e + 767
7   a                              0x000000010efa0696 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd8082d12249549da + 6
8   libstd-28edbc69fd879800.dylib  0x000000010efc9578 std::panicking::try::do_call::he44de2c8a6962430 + 24
9   libstd-28edbc69fd879800.dylib  0x000000010efd97bf __rust_maybe_catch_panic + 31
10  libstd-28edbc69fd879800.dylib  0x000000010efca05e std::rt::lang_start_internal::h808ee93b9537043a + 542
11  a                              0x000000010efa1579 main + 41
12  libdyld.dylib                  0x00007fff7b904115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fffb462a340  rcx: 0x00007ffee0c5e5e8  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee0c5e620  rsp: 0x00007ffee0c5e5e8
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff7ba53e3e  rfl: 0x0000000000000206  cr2: 0x00007fffb4608148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10ef9f000 -        0x10efa1ff7 +a (0) <41E90DB3-381D-3F00-963F-675AF220E7E6> /Users/USER/*/a
       0x10efa5000 -        0x10f03cff7 +libstd-28edbc69fd879800.dylib (0) <C8EDCC36-CD0C-39F3-87E7-27A2D78C38DD> /Users/USER/*/libstd-28edbc69fd879800.dylib
       0x113759000 -        0x1137a398f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7916e000 -     0x7fff791a1fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff79680000 -     0x7fff79681ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff79936000 -     0x7fff7998cfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff7998d000 -     0x7fff799b1ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff7ad03000 -     0x7fff7b0f43b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7b1c1000 -     0x7fff7b1ddffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff7b79b000 -     0x7fff7b79fff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff7b7a0000 -     0x7fff7b7aaff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff7b7ab000 -     0x7fff7b7b2fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7b7b3000 -     0x7fff7b7bbffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff7b7bc000 -     0x7fff7b841fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff7b8c9000 -     0x7fff7b902ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7b903000 -     0x7fff7b920ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff7b921000 -     0x7fff7b921ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff7b92f000 -     0x7fff7b92fff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff7b930000 -     0x7fff7b934ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff7b935000 -     0x7fff7b937ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff7b938000 -     0x7fff7b939ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff7b93a000 -     0x7fff7b951fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7b952000 -     0x7fff7b952fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7b953000 -     0x7fff7b9dcff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7b9dd000 -     0x7fff7b9e0ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7b9e1000 -     0x7fff7b9e4ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7b9e5000 -     0x7fff7b9e6fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7b9e7000 -     0x7fff7b9edff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7b9ee000 -     0x7fff7ba37ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7ba38000 -     0x7fff7ba5dff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7ba5e000 -     0x7fff7baa9fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7baaa000 -     0x7fff7bac9fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7baca000 -     0x7fff7bb6eff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7bb6f000 -     0x7fff7bb79ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7bb7a000 -     0x7fff7bb83ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7bb84000 -     0x7fff7bb8bff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7bb8c000 -     0x7fff7bb97fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff7bb98000 -     0x7fff7bb9bff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7bb9c000 -     0x7fff7bb9dff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7bb9e000 -     0x7fff7bba5ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7bba6000 -     0x7fff7bbb9ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7bbbb000 -     0x7fff7bbc0ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff7bbc1000 -     0x7fff7bbedff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2368
    thread_create: 0
VM Region Summary:
VM Region Summary:
