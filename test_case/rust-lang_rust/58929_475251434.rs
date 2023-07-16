plain
[00:02:59]       Memory: 8 GB
[00:02:59]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:02:59]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:59]       SMC Version (system): 2.8f0
[00:02:59]       Serial Number (system): VMCoZ2DKahCM
[00:02:59] 
[00:02:59] hw.ncpu: 4
[00:02:59] hw.byteorder: 1234
[00:02:59] hw.memsize: 8589934592
---
[02:11:01] 
[02:11:01] ---- /Users/travis/build/rust-lang/rust/src/doc/unstable-book/src/language-features/asm.md - asm::Assembly_template (line 75) stdout ----
[02:11:01] error: linking with `cc` failed: signal: 4
[02:11:01]   |
[02:11:01]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestaoiTWK/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestaoiTWK/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestaoiTWK/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestaoiTWK/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestaoiTWK/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestaoiTWK/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestaoiTWK/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-78c92a267c026dc1.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-9be3db07243d9141.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-4fc658290a5c6894.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-9704bcba7c1ba7f6.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-355afe853959c8e9.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-5ba6d123f0333f58.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-0ae8d24a073c548a.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-233abb06723a85b7.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-98b24fb5d296bb0b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-89c7b0a7a5795fcb.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:11:01] 
[02:11:01] error: aborting due to previous error
[02:11:01] 
[02:11:01] thread '/Users/travis/build/rust-lang/rust/src/doc/unstable-book/src/language-features/asm.md - asm::Assembly_template (line 75)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[02:11:01] 
[02:11:01] 
[02:11:01] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:11:01] Build completed unsuccessfully in 0:53:34
[02:11:01] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:098697bd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 21 14:27:30 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:11066997
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Mar 21 14:27 .
-rw-------@  1 travis  staff  13742 Mar 21 14:27 overflow_2019-03-21-142730_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Mar 21 14:27 foo_2019-03-21-142700_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1418 Mar 21 14:26 m4_2019-03-21-142629_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10660 Mar 21 14:26 b_2019-03-21-142617_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Mar 21 14:26 bar_2019-03-21-142616_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Mar 21 14:26 bar_2019-03-21-142617_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37481 Mar 21 13:49 a_2019-03-21-134956-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62244 Mar 21 13:49 a_2019-03-21-134956_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60389 Mar 21 13:49 a_2019-03-21-134946-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37229 Mar 21 13:49 a_2019-03-21-134946_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Mar 21 13:49 a_2019-03-21-134942_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Mar 21 13:49 a_2019-03-21-134936_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Mar 21 13:49 a_2019-03-21-134929_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9792 Mar 21 13:49 a_2019-03-21-134927_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Mar 21 13:48 a_2019-03-21-134849_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63047 Mar 21 13:48 a_2019-03-21-134840_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65082 Mar 21 13:48 a_2019-03-21-134835-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64264 Mar 21 13:48 a_2019-03-21-134835-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 Mar 21 13:48 a_2019-03-21-134835_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11714 Mar 21 13:46 a_2019-03-21-134627_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Mar 21 13:45 a_2019-03-21-134527_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Mar 21 13:44 a_2019-03-21-134407_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10213 Mar 21 13:43 a_2019-03-21-134332-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10489 Mar 21 13:43 a_2019-03-21-134332-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Mar 21 13:43 a_2019-03-21-134332_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:193e0288
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-21-134332-1_Traviss-Mac-1044.crash
Process:               a [41336]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41332]
Responsible:           a [41336]
User ID:               501
Date/Time:             2019-03-21 13:42:36.109 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5300 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00000001025bf8ae abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x00000001025beca9 std::panicking::try::do_call::hae45fd07e6fb139a (.llvm.3410359905292931506) + 9
2   libstd-78c92a267c026dc1.dylib  0x00000001025fa3ff __rust_maybe_catch_panic + 31
3   a                              0x00000001025bfb01 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x00000001025be0f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hcbce389c47385da3 + 6
5   libstd-78c92a267c026dc1.dylib  0x00000001025ea188 std::panicking::try::do_call::h151c5c490468c6ea + 24
6   libstd-78c92a267c026dc1.dylib  0x00000001025fa3ff __rust_maybe_catch_panic + 31
7   libstd-78c92a267c026dc1.dylib  0x00000001025eac6e std::rt::lang_start_internal::h6582bca3e7f27378 + 542
8   a                              0x00000001025bfe09 main + 41
9   libdyld.dylib                  0x00007fff50cab115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007fcfeb402c00  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffeed63fc88  rsi: 0x00000000ffffffe1  rbp: 0x00007ffeed6406e0  rsp: 0x00007ffeed6406e0
   r8: 0x00000000feb402c5   r9: 0x0000000000000004  r10: 0x000000010ab108c0  r11: 0x00007fff50f6296c
  r12: 0x0000000102902000  r13: 0x0000000000000000  r14: 0x00007ffeed640800  r15: 0x00007ffeed640748
  rip: 0x00000001025bf8ae  rfl: 0x0000000000010202  cr2: 0x0000000102630dec
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1025bd000 -        0x1025c0fff +a (0) <54A93332-1010-3050-B69F-3EBB1A669527> /Users/USER/*/a
       0x1025c7000 -        0x10265dfff +libstd-78c92a267c026dc1.dylib (0) <3F5E7511-C082-3322-BBAA-B26A736A4DA1> /Users/USER/*/libstd-78c92a267c026dc1.dylib
       0x10aabe000 -        0x10ab0898f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff4e515000 -     0x7fff4e548fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff4ea27000 -     0x7fff4ea28ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff4ecdd000 -     0x7fff4ed33fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff4ed34000 -     0x7fff4ed58ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff500aa000 -     0x7fff5049b3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff50568000 -     0x7fff50584ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff50b42000 -     0x7fff50b46ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff50b47000 -     0x7fff50b51ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff50b52000 -     0x7fff50b59fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff50b5a000 -     0x7fff50b62ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff50b63000 -     0x7fff50be8fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff50c70000 -     0x7fff50ca9ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff50caa000 -     0x7fff50cc7ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff50cc8000 -     0x7fff50cc8ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff50cd6000 -     0x7fff50cd6ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff50cd7000 -     0x7fff50cdbffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff50cdc000 -     0x7fff50cdeff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff50cdf000 -     0x7fff50ce0ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff50ce1000 -     0x7fff50cf8fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff50cf9000 -     0x7fff50cf9fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff50cfa000 -     0x7fff50d83ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff50d84000 -     0x7fff50d87ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff50d88000 -     0x7fff50d8bffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff50d8c000 -     0x7fff50d8dfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff50d8e000 -     0x7fff50d94ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff50d95000 -     0x7fff50ddeff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff50ddf000 -     0x7fff50e04ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff50e05000 -     0x7fff50e50fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff50e51000 -     0x7fff50e70fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff50e71000 -     0x7fff50f15ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff50f16000 -     0x7fff50f20ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff50f21000 -     0x7fff50f2aff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff50f2b000 -     0x7fff50f32ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff50f33000 -     0x7fff50f3efff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff50f3f000 -     0x7fff50f42ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff50f43000 -     0x7fff50f44ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff50f45000 -     0x7fff50f4cff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff50f4d000 -     0x7fff50f60ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff50f62000 -     0x7fff50f67ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff50f68000 -     0x7fff50f94ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2388
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
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4568K       44 
__LINKEDIT                       188.9M        5 
__TEXT                            9620K       44 
===========                     =======  ======= 
TOTAL                            276.0M      109 
TOTAL                            276.0M      109 
TOTAL, minus reserved VM space   275.9M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-21-134332-2_Traviss-Mac-1044.crash
Process:               a [42121]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        a [42114]
Responsible:           a [42121]
User ID:               501
Date/Time:             2019-03-21 13:43:06.264 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5300 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-78c92a267c026dc1.dylib  0x000000010d13d70e std::panicking::rust_panic_with_hook::h46175c45dd07c8cb + 142
1   a                              0x000000010d1088e5 std::panicking::begin_panic::h741d204172ce3884 + 37
2   a                              0x000000010d10640c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::h5ed13b737dc2b5ab + 28
3   a                              0x000000010d105cb9 core::ptr::real_drop_in_place::h433b5af917300811 + 9
4   a                              0x000000010d1063e3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x000000010d107559 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x000000010d105936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h457d5bd1fa0bccfb + 6 (rt.rs:64)
7   libstd-78c92a267c026dc1.dylib  0x000000010d13d188 std::panicking::try::do_call::h151c5c490468c6ea + 24
8   libstd-78c92a267c026dc1.dylib  0x000000010d14d3ff __rust_maybe_catch_panic + 31
9   libstd-78c92a267c026dc1.dylib  0x000000010d13dc6e std::rt::lang_start_internal::h6582bca3e7f27378 + 542
10  a                              0x000000010d107d99 main + 41
11  libdyld.dylib                  0x00007fff50cab115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee2afb438  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010d185272  rbp: 0x00007ffee2afb530  rsp: 0x00007ffee2afb460
   r8: 0x0000000000000100   r9: 0x000000010d1b8a10  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010d183ff8  r14: 0x000000010d10a460  r15: 0x00007ffee2afb540
  rip: 0x000000010d13d70e  rfl: 0x0000000000010206  cr2: 0x000000010d4148bf
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10d102000 -        0x10d109fff +a (0) <589E092F-23C7-3760-9A7E-3FF794EB6925> /Users/USER/*/a
       0x10d11a000 -        0x10d1b0fff +libstd-78c92a267c026dc1.dylib (0) <3F5E7511-C082-3322-BBAA-B26A736A4DA1> /Users/USER/*/libstd-78c92a267c026dc1.dylib
       0x11af4c000 -        0x11af9698f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff4e515000 -     0x7fff4e548fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff4ea27000 -     0x7fff4ea28ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff4ecdd000 -     0x7fff4ed33fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff4ed34000 -     0x7fff4ed58ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff500aa000 -     0x7fff5049b3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff50568000 -     0x7fff50584ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff50b42000 -     0x7fff50b46ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff50b47000 -     0x7fff50b51ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff50b52000 -     0x7fff50b59fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff50b5a000 -     0x7fff50b62ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff50b63000 -     0x7fff50be8fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff50c70000 -     0x7fff50ca9ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff50caa000 -     0x7fff50cc7ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff50cc8000 -     0x7fff50cc8ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff50cd6000 -     0x7fff50cd6ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff50cd7000 -     0x7fff50cdbffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff50cdc000 -     0x7fff50cdeff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff50cdf000 -     0x7fff50ce0ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff50ce1000 -     0x7fff50cf8fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff50cf9000 -     0x7fff50cf9fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff50cfa000 -     0x7fff50d83ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff50d84000 -     0x7fff50d87ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff50d88000 -     0x7fff50d8bffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff50d8c000 -     0x7fff50d8dfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff50d8e000 -     0x7fff50d94ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff50d95000 -     0x7fff50ddeff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff50ddf000 -     0x7fff50e04ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff50e05000 -     0x7fff50e50fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff50e51000 -     0x7fff50e70fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff50e71000 -     0x7fff50f15ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff50f16000 -     0x7fff50f20ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff50f21000 -     0x7fff50f2aff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff50f2b000 -     0x7fff50f32ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff50f33000 -     0x7fff50f3efff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff50f3f000 -     0x7fff50f42ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff50f43000 -     0x7fff50f44ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff50f45000 -     0x7fff50f4cff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff50f4d000 -     0x7fff50f60ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff50f62000 -     0x7fff50f67ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff50f68000 -     0x7fff50f94ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2388
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
__DATA                            4568K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9636K       44 
===========                     =======  ======= 
TOTAL                            286.5M      110 
TOTAL                            286.5M      110 
TOTAL, minus reserved VM space   286.3M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-21-134332_Traviss-Mac-1044.crash
Process:               a [42122]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [42114]
Responsible:           a [42122]
User ID:               501
Date/Time:             2019-03-21 13:43:06.282 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5300 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-78c92a267c026dc1.dylib  0x000000010882570e std::panicking::rust_panic_with_hook::h46175c45dd07c8cb + 142
1   a                              0x00000001087f38e5 std::panicking::begin_panic::h741d204172ce3884 + 37
2   a                              0x00000001087f140c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::h5ed13b737dc2b5ab + 28
3   a                              0x00000001087f0cb9 core::ptr::real_drop_in_place::h433b5af917300811 + 9
4   a                              0x00000001087f13e3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x00000001087f2559 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x00000001087f0936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h457d5bd1fa0bccfb + 6 (rt.rs:64)
7   libstd-78c92a267c026dc1.dylib  0x0000000108825188 std::panicking::try::do_call::h151c5c490468c6ea + 24
8   libstd-78c92a267c026dc1.dylib  0x00000001088353ff __rust_maybe_catch_panic + 31
9   libstd-78c92a267c026dc1.dylib  0x0000000108825c6e std::rt::lang_start_internal::h6582bca3e7f27378 + 542
10  a                              0x00000001087f2d99 main + 41
11  libdyld.dylib                  0x00007fff50cab115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee7410428  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010886d272  rbp: 0x00007ffee7410520  rsp: 0x00007ffee7410450
   r8: 0x0000000000000100   r9: 0x00000001088a0a10  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010886bff8  r14: 0x00000001087f5460  r15: 0x00007ffee7410530
  rip: 0x000000010882570e  rfl: 0x0000000000010202  cr2: 0x0000000108860c50
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1087ed000 -        0x1087f4fff +a (0) <589E092F-23C7-3760-9A7E-3FF794EB6925> /Users/USER/*/a
       0x108802000 -        0x108898fff +libstd-78c92a267c026dc1.dylib (0) <3F5E7511-C082-3322-BBAA-B26A736A4DA1> /Users/USER/*/libstd-78c92a267c026dc1.dylib
       0x10e630000 -        0x10e67a98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff4e515000 -     0x7fff4e548fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff4ea27000 -     0x7fff4ea28ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff4ecdd000 -     0x7fff4ed33fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff4ed34000 -     0x7fff4ed58ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff500aa000 -     0x7fff5049b3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff50568000 -     0x7fff50584ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff50b42000 -     0x7fff50b46ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff50b47000 -     0x7fff50b51ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff50b52000 -     0x7fff50b59fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff50b5a000 -     0x7fff50b62ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff50b63000 -     0x7fff50be8fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff50c70000 -     0x7fff50ca9ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff50caa000 -     0x7fff50cc7ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff50cc8000 -     0x7fff50cc8ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff50cd6000 -     0x7fff50cd6ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff50cd7000 -     0x7fff50cdbffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff50cdc000 -     0x7fff50cdeff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff50cdf000 -     0x7fff50ce0ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff50ce1000 -     0x7fff50cf8fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff50cf9000 -     0x7fff50cf9fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff50cfa000 -     0x7fff50d83ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff50d84000 -     0x7fff50d87ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff50d88000 -     0x7fff50d8bffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff50d8c000 -     0x7fff50d8dfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff50d8e000 -     0x7fff50d94ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff50d95000 -     0x7fff50ddeff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff50ddf000 -     0x7fff50e04ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff50e05000 -     0x7fff50e50fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff50e51000 -     0x7fff50e70fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff50e71000 -     0x7fff50f15ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff50f16000 -     0x7fff50f20ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff50f21000 -     0x7fff50f2aff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff50f2b000 -     0x7fff50f32ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff50f33000 -     0x7fff50f3efff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff50f3f000 -     0x7fff50f42ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff50f43000 -     0x7fff50f44ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff50f45000 -     0x7fff50f4cff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff50f4d000 -     0x7fff50f60ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff50f62000 -     0x7fff50f67ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff50f68000 -     0x7fff50f94ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2388
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=73.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9688K        9 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4568K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9636K       44 
===========                     =======  ======= 
TOTAL                            276.5M      109 
TOTAL                            276.5M      109 
TOTAL, minus reserved VM space   276.3M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-21-134407_Traviss-Mac-1044.crash
Process:               a [43825]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [43824]
Responsible:           a [43825]
User ID:               501
Date/Time:             2019-03-21 13:44:06.960 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff50dfae3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff50f39150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff50d57312 abort + 127
3   libstd-78c92a267c026dc1.dylib  0x0000000100c1f8e9 std::sys::unix::abort_internal::haf9f20ef8555f729 + 9
4   libstd-78c92a267c026dc1.dylib  0x0000000100c0f7a0 rust_oom + 32
5   libstd-78c92a267c026dc1.dylib  0x0000000100c30b89 alloc::alloc::handle_alloc_error::h9b042c7f61ba4c79 + 9
6   a                              0x0000000100be33ef default_alloc_error_hook::main::hbf2d06db626d002e + 767
7   a                              0x0000000100be3556 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h30bbd23b9ca13409 + 6
8   libstd-78c92a267c026dc1.dylib  0x0000000100c10188 std::panicking::try::do_call::h151c5c490468c6ea + 24
9   libstd-78c92a267c026dc1.dylib  0x0000000100c203ff __rust_maybe_catch_panic + 31
10  libstd-78c92a267c026dc1.dylib  0x0000000100c10c6e std::rt::lang_start_internal::h6582bca3e7f27378 + 542
11  a                              0x0000000100be3549 main + 41
12  libdyld.dylib                  0x00007fff50cab115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff899d1340  rcx: 0x00007ffeef01c648  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffeef01c680  rsp: 0x00007ffeef01c648
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff50dfae3e  rfl: 0x0000000000000206  cr2: 0x00007fff899af148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x100be1000 -        0x100be3fff +a (0) <959FCC5D-BDFF-35FA-A437-9A6EB07E797D> /Users/USER/*/a
       0x100bed000 -        0x100c83fff +libstd-78c92a267c026dc1.dylib (0) <3F5E7511-C082-3322-BBAA-B26A736A4DA1> /Users/USER/*/libstd-78c92a267c026dc1.dylib
       0x107755000 -        0x10779f98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff4e515000 -     0x7fff4e548fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff4ea27000 -     0x7fff4ea28ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff4ecdd000 -     0x7fff4ed33fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff4ed34000 -     0x7fff4ed58ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff500aa000 -     0x7fff5049b3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff50568000 -     0x7fff50584ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff50b42000 -     0x7fff50b46ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff50b47000 -     0x7fff50b51ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff50b52000 -     0x7fff50b59fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff50b5a000 -     0x7fff50b62ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff50b63000 -     0x7fff50be8fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff50c70000 -     0x7fff50ca9ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff50caa000 -     0x7fff50cc7ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff50cc8000 -     0x7fff50cc8ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff50cd6000 -     0x7fff50cd6ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff50cd7000 -     0x7fff50cdbffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff50cdc000 -     0x7fff50cdeff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff50cdf000 -     0x7fff50ce0ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff50ce1000 -     0x7fff50cf8fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff50cf9000 -     0x7fff50cf9fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff50cfa000 -     0x7fff50d83ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff50d84000 -     0x7fff50d87ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff50d88000 -     0x7fff50d8bffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff50d8c000 -     0x7fff50d8dfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff50d8e000 -     0x7fff50d94ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff50d95000 -     0x7fff50ddeff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff50ddf000 -     0x7fff50e04ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff50e05000 -     0x7fff50e50fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff50e51000 -     0x7fff50e70fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff50e71000 -     0x7fff50f15ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff50f16000 -     0x7fff50f20ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff50f21000 -     0x7fff50f2aff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff50f2b000 -     0x7fff50f32ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff50f33000 -     0x7fff50f3efff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff50f3f000 -     0x7fff50f42ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff50f43000 -     0x7fff50f44ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff50f45000 -     0x7fff50f4cff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff50f4d000 -     0x7fff50f60ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff50f62000 -     0x7fff50f67ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff50f68000 -     0x7fff50f94ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
