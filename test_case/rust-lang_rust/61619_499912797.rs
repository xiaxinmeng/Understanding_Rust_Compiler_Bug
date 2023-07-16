plain
[00:03:24]       Memory: 8 GB
[00:03:24]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:24]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:24]       SMC Version (system): 2.8f0
[00:03:24]       Serial Number (system): VM6TrsClgfRn
[00:03:24] 
[00:03:24] hw.ncpu: 4
[00:03:24] hw.byteorder: 1234
[00:03:24] hw.memsize: 8589934592
---
[02:08:48] 
[02:08:48] ---- /Users/travis/build/rust-lang/rust/src/doc/rustdoc/src/documentation-tests.md - Documentation_tests::Attributes (line 311) stdout ----
[02:08:48] error: linking with `cc` failed: signal: 4
[02:08:48]   |
[02:08:48]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestwVi1h8/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestwVi1h8/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestwVi1h8/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestwVi1h8/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestwVi1h8/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestwVi1h8/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestwVi1h8/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-0aaed73800111db0.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-85f203700598fc0f.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace-a26391313b65d7dc.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-0ad3fa8cdd0b9704.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-4d45b60ed1186ef3.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-6595b4030f6ddb0e.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-b5491b1ffbbc1406.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-4228aae7223b6103.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-bc2e34e619def26f.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-d930cd0bdea4797b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-8bde4cd3b4c45eea.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-6450bce6cc789177.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-992f5c26c2d9c590.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-0f189b415a69de4e.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:08:48] 
[02:08:48] error: aborting due to previous error
[02:08:48] 
[02:08:48] Couldn't compile the test.
---
travis_fold:start:after_failure.2
travis_time:start:016bdb63
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Jun  7 14:42 .
-rw-------@  1 travis  staff  13742 Jun  7 14:42 overflow_2019-06-07-144212_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Jun  7 14:41 foo_2019-06-07-144149_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1417 Jun  7 14:41 m4_2019-06-07-144113_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Jun  7 14:41 bar_2019-06-07-144102_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10660 Jun  7 14:41 b_2019-06-07-144101_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Jun  7 14:41 bar_2019-06-07-144101_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 Jun  7 13:51 a_2019-06-07-135102-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62246 Jun  7 13:51 a_2019-06-07-135102_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37396 Jun  7 13:50 a_2019-06-07-135054-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60389 Jun  7 13:50 a_2019-06-07-135054_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Jun  7 13:50 a_2019-06-07-135048_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Jun  7 13:50 a_2019-06-07-135041_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Jun  7 13:50 a_2019-06-07-135038-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 Jun  7 13:50 a_2019-06-07-135038_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Jun  7 13:50 a_2019-06-07-135001_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63060 Jun  7 13:49 a_2019-06-07-134948-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65082 Jun  7 13:49 a_2019-06-07-134947-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64224 Jun  7 13:49 a_2019-06-07-134947-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 Jun  7 13:49 a_2019-06-07-134947_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11712 Jun  7 13:47 a_2019-06-07-134733_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Jun  7 13:46 a_2019-06-07-134638_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Jun  7 13:45 a_2019-06-07-134524_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Jun  7 13:44 a_2019-06-07-134459-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 Jun  7 13:44 a_2019-06-07-134458-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Jun  7 13:44 a_2019-06-07-134458_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0446dfc1
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-07-134458-1_Traviss-Mac-1044.crash
Process:               a [41509]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41505]
Responsible:           a [41509]
User ID:               501
Date/Time:             2019-06-07 13:43:52.486 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4300 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000000010af507fe abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 30
1   a                              0x000000010af4fbf9 std::panicking::try::do_call::h0432200c321e0448 (.llvm.11845119723594457445) + 9
2   libstd-0aaed73800111db0.dylib  0x000000010af8ad5f __rust_maybe_catch_panic + 31
3   a                              0x000000010af50a51 abort_on_c_abi::main::he771bf881fc862e3 + 593
4   a                              0x000000010af4f0d6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h9a1b2aeb3cc2900b + 6
5   libstd-0aaed73800111db0.dylib  0x000000010af7ae58 std::panicking::try::do_call::ha441bad49ba6bcd5 + 24
6   libstd-0aaed73800111db0.dylib  0x000000010af8ad5f __rust_maybe_catch_panic + 31
7   libstd-0aaed73800111db0.dylib  0x000000010af7b93e std::rt::lang_start_internal::h4d4b16bc6c2ece3c + 542
8   a                              0x000000010af50d59 main + 41
9   libdyld.dylib                  0x00007fff69705115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007fc2e4c02640  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee4caebf8  rsi: 0x00000000fffffe1f  rbp: 0x00007ffee4caf650  rsp: 0x00007ffee4caf650
   r8: 0x000000002e4c0269   r9: 0x0000000000000004  r10: 0x00000001121248c0  r11: 0x00007fff699bc96c
  r12: 0x000000010b296000  r13: 0x0000000000000000  r14: 0x00007ffee4caf770  r15: 0x00007ffee4caf6b8
  rip: 0x000000010af507fe  rfl: 0x0000000000010202  cr2: 0x000000010afc6ac0
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10af4e000 -        0x10af51fff +a (0) <C5A5997A-A270-3F73-8C25-D2FFAABC25EC> /Users/USER/*/a
       0x10af58000 -        0x10aff3ff7 +libstd-0aaed73800111db0.dylib (0) <AC479981-3942-3D2F-91AE-D5A3BED47881> /Users/USER/*/libstd-0aaed73800111db0.dylib
       0x1120d2000 -        0x11211c98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff66f6f000 -     0x7fff66fa2fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff67481000 -     0x7fff67482ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff67737000 -     0x7fff6778dfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6778e000 -     0x7fff677b2ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff68b04000 -     0x7fff68ef53b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff68fc2000 -     0x7fff68fdeffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6959c000 -     0x7fff695a0ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff695a1000 -     0x7fff695abff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff695ac000 -     0x7fff695b3fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff695b4000 -     0x7fff695bcffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff695bd000 -     0x7fff69642fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff696ca000 -     0x7fff69703ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff69704000 -     0x7fff69721ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff69722000 -     0x7fff69722ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff69730000 -     0x7fff69730ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff69731000 -     0x7fff69735ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff69736000 -     0x7fff69738ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff69739000 -     0x7fff6973aff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6973b000 -     0x7fff69752fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff69753000 -     0x7fff69753fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff69754000 -     0x7fff697ddff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff697de000 -     0x7fff697e1ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff697e2000 -     0x7fff697e5ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff697e6000 -     0x7fff697e7fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff697e8000 -     0x7fff697eeff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff697ef000 -     0x7fff69838ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff69839000 -     0x7fff6985eff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6985f000 -     0x7fff698aafcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff698ab000 -     0x7fff698cafff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff698cb000 -     0x7fff6996fff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff69970000 -     0x7fff6997affb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6997b000 -     0x7fff69984ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff69985000 -     0x7fff6998cff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6998d000 -     0x7fff69998fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff69999000 -     0x7fff6999cff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6999d000 -     0x7fff6999eff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6999f000 -     0x7fff699a6ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff699a7000 -     0x7fff699baff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff699bc000 -     0x7fff699c1ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff699c2000 -     0x7fff699eeff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2094
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=18.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=18.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4552K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            277.0M      109 
TOTAL                            277.0M      109 
TOTAL, minus reserved VM space   276.9M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-07-134458_Traviss-Mac-1044.crash
Process:               a [42296]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [42285]
Responsible:           a [42296]
User ID:               501
Date/Time:             2019-06-07 13:44:22.076 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-0aaed73800111db0.dylib  0x000000010402a3de std::panicking::rust_panic_with_hook::h235ff3c6fa83ad55 + 142
1   a                              0x0000000103ff96d5 std::panicking::begin_panic::h2a34f37f3f4ed5c9 + 37
2   a                              0x0000000103ff6d9c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x0000000103ff6659 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x0000000103ff6d73 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x0000000103ff82ba backtrace::main::hc9a5bc8fc93ded64 + 5178 (backtrace.rs:119)
6   a                              0x0000000103ff62a6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h5eea6c96c9a4d1d5 + 6 (rt.rs:64)
7   libstd-0aaed73800111db0.dylib  0x0000000104029e58 std::panicking::try::do_call::ha441bad49ba6bcd5 + 24
8   libstd-0aaed73800111db0.dylib  0x0000000104039d5f __rust_maybe_catch_panic + 31
9   libstd-0aaed73800111db0.dylib  0x000000010402a93e std::rt::lang_start_internal::h4d4b16bc6c2ece3c + 542
10  a                              0x0000000103ff8c09 main + 41
11  libdyld.dylib                  0x00007fff69705115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeebc0b368  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000104076ea5  rbp: 0x00007ffeebc0b460  rsp: 0x00007ffeebc0b390
   r8: 0xffffffff00000100   r9: 0x00000001040aaa80  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000104075cf8  r14: 0x0000000103ffb480  r15: 0x00007ffeebc0b470
  rip: 0x000000010402a3de  rfl: 0x0000000000010202  cr2: 0x000000010407b001
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x103ff2000 -        0x103ffafff +a (0) <D1BDE4A5-809C-33EC-95DF-F41D37F7943D> /Users/USER/*/a
       0x104007000 -        0x1040a2ff7 +libstd-0aaed73800111db0.dylib (0) <AC479981-3942-3D2F-91AE-D5A3BED47881> /Users/USER/*/libstd-0aaed73800111db0.dylib
       0x10b576000 -        0x10b5c098f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff66f6f000 -     0x7fff66fa2fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff67481000 -     0x7fff67482ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff67737000 -     0x7fff6778dfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6778e000 -     0x7fff677b2ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff68b04000 -     0x7fff68ef53b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff68fc2000 -     0x7fff68fdeffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6959c000 -     0x7fff695a0ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff695a1000 -     0x7fff695abff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff695ac000 -     0x7fff695b3fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff695b4000 -     0x7fff695bcffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff695bd000 -     0x7fff69642fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff696ca000 -     0x7fff69703ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff69704000 -     0x7fff69721ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff69722000 -     0x7fff69722ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff69730000 -     0x7fff69730ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff69731000 -     0x7fff69735ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff69736000 -     0x7fff69738ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff69739000 -     0x7fff6973aff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6973b000 -     0x7fff69752fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff69753000 -     0x7fff69753fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff69754000 -     0x7fff697ddff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff697de000 -     0x7fff697e1ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff697e2000 -     0x7fff697e5ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff697e6000 -     0x7fff697e7fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff697e8000 -     0x7fff697eeff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff697ef000 -     0x7fff69838ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff69839000 -     0x7fff6985eff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6985f000 -     0x7fff698aafcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff698ab000 -     0x7fff698cafff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff698cb000 -     0x7fff6996fff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff69970000 -     0x7fff6997affb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6997b000 -     0x7fff69984ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff69985000 -     0x7fff6998cff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6998d000 -     0x7fff69998fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff69999000 -     0x7fff6999cff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6999d000 -     0x7fff6999eff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6999f000 -     0x7fff699a6ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff699a7000 -     0x7fff699baff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff699bc000 -     0x7fff699c1ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff699c2000 -     0x7fff699eeff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2094
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=17.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9724K        9 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4552K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9660K       44 
===========                     =======  ======= 
TOTAL                            276.5M      110 
TOTAL                            276.5M      110 
TOTAL, minus reserved VM space   276.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-07-134459-1_Traviss-Mac-1044.crash
Process:               a [42293]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [42285]
Responsible:           a [42293]
User ID:               501
Date/Time:             2019-06-07 13:44:22.032 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-0aaed73800111db0.dylib  0x000000010ee1e3de std::panicking::rust_panic_with_hook::h235ff3c6fa83ad55 + 142
1   a                              0x000000010edef6d5 std::panicking::begin_panic::h2a34f37f3f4ed5c9 + 37
2   a                              0x000000010edecd9c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x000000010edec659 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x000000010edecd73 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x000000010edee2ba backtrace::main::hc9a5bc8fc93ded64 + 5178 (backtrace.rs:119)
6   a                              0x000000010edec2a6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h5eea6c96c9a4d1d5 + 6 (rt.rs:64)
7   libstd-0aaed73800111db0.dylib  0x000000010ee1de58 std::panicking::try::do_call::ha441bad49ba6bcd5 + 24
8   libstd-0aaed73800111db0.dylib  0x000000010ee2dd5f __rust_maybe_catch_panic + 31
9   libstd-0aaed73800111db0.dylib  0x000000010ee1e93e std::rt::lang_start_internal::h4d4b16bc6c2ece3c + 542
10  a                              0x000000010edeec09 main + 41
11  libdyld.dylib                  0x00007fff69705115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee0e15388  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010ee6aea5  rbp: 0x00007ffee0e15480  rsp: 0x00007ffee0e153b0
   r8: 0xffffffff00000100   r9: 0x000000010ee9ea80  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010ee69cf8  r14: 0x000000010edf1480  r15: 0x00007ffee0e15490
  rip: 0x000000010ee1e3de  rfl: 0x0000000000010206  cr2: 0x00007f8378c560d0
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10ede8000 -        0x10edf0fff +a (0) <D1BDE4A5-809C-33EC-95DF-F41D37F7943D> /Users/USER/*/a
       0x10edfb000 -        0x10ee96ff7 +libstd-0aaed73800111db0.dylib (0) <AC479981-3942-3D2F-91AE-D5A3BED47881> /Users/USER/*/libstd-0aaed73800111db0.dylib
       0x11c8ea000 -        0x11c93498f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff66f6f000 -     0x7fff66fa2fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff67481000 -     0x7fff67482ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff67737000 -     0x7fff6778dfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6778e000 -     0x7fff677b2ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff68b04000 -     0x7fff68ef53b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff68fc2000 -     0x7fff68fdeffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6959c000 -     0x7fff695a0ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff695a1000 -     0x7fff695abff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff695ac000 -     0x7fff695b3fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff695b4000 -     0x7fff695bcffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff695bd000 -     0x7fff69642fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff696ca000 -     0x7fff69703ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff69704000 -     0x7fff69721ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff69722000 -     0x7fff69722ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff69730000 -     0x7fff69730ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff69731000 -     0x7fff69735ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff69736000 -     0x7fff69738ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff69739000 -     0x7fff6973aff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6973b000 -     0x7fff69752fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff69753000 -     0x7fff69753fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff69754000 -     0x7fff697ddff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff697de000 -     0x7fff697e1ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff697e2000 -     0x7fff697e5ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff697e6000 -     0x7fff697e7fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff697e8000 -     0x7fff697eeff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff697ef000 -     0x7fff69838ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff69839000 -     0x7fff6985eff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6985f000 -     0x7fff698aafcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff698ab000 -     0x7fff698cafff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff698cb000 -     0x7fff6996fff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff69970000 -     0x7fff6997affb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6997b000 -     0x7fff69984ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff69985000 -     0x7fff6998cff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6998d000 -     0x7fff69998fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff69999000 -     0x7fff6999cff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6999d000 -     0x7fff6999eff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6999f000 -     0x7fff699a6ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff699a7000 -     0x7fff699baff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff699bc000 -     0x7fff699c1ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff699c2000 -     0x7fff699eeff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2094
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=17.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9724K        9 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4552K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9660K       44 
===========                     =======  ======= 
TOTAL                            276.5M      110 
TOTAL                            276.5M      110 
TOTAL, minus reserved VM space   276.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-07-134524_Traviss-Mac-1044.crash
Process:               a [44010]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44008]
Responsible:           a [44010]
User ID:               501
Date/Time:             2019-06-07 13:45:22.959 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff69854e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff69993150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff697b1312 abort + 127
3   libstd-0aaed73800111db0.dylib  0x000000010ede3299 std::sys::unix::abort_internal::h907dc65c14d13bdf + 9
4   libstd-0aaed73800111db0.dylib  0x000000010edd3340 rust_oom + 32
5   libstd-0aaed73800111db0.dylib  0x000000010edf9269 alloc::alloc::handle_alloc_error::h4044521f3c8241d3 + 9
6   a                              0x000000010eda80af default_alloc_error_hook::main::h0fe124586986ad5d + 767
7   a                              0x000000010eda78b6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h36f6756909322aa0 + 6
8   libstd-0aaed73800111db0.dylib  0x000000010edd3e58 std::panicking::try::do_call::ha441bad49ba6bcd5 + 24
9   libstd-0aaed73800111db0.dylib  0x000000010ede3d5f __rust_maybe_catch_panic + 31
10  libstd-0aaed73800111db0.dylib  0x000000010edd493e std::rt::lang_start_internal::h4d4b16bc6c2ece3c + 542
11  a                              0x000000010eda8209 main + 41
12  libdyld.dylib                  0x00007fff69705115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fffa242b340  rcx: 0x00007ffee0e575c8  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee0e57600  rsp: 0x00007ffee0e575c8
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff69854e3e  rfl: 0x0000000000000206  cr2: 0x00007fffa2409148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10eda6000 -        0x10eda8ff7 +a (0) <C29D4880-9BD1-33D7-9887-88FA5E911E89> /Users/USER/*/a
       0x10edb1000 -        0x10ee4cff7 +libstd-0aaed73800111db0.dylib (0) <AC479981-3942-3D2F-91AE-D5A3BED47881> /Users/USER/*/libstd-0aaed73800111db0.dylib
       0x11ab51000 -        0x11ab9b98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff66f6f000 -     0x7fff66fa2fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff67481000 -     0x7fff67482ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff67737000 -     0x7fff6778dfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6778e000 -     0x7fff677b2ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff68b04000 -     0x7fff68ef53b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff68fc2000 -     0x7fff68fdeffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6959c000 -     0x7fff695a0ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff695a1000 -     0x7fff695abff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff695ac000 -     0x7fff695b3fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff695b4000 -     0x7fff695bcffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff695bd000 -     0x7fff69642fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff696ca000 -     0x7fff69703ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff69704000 -     0x7fff69721ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff69722000 -     0x7fff69722ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff69730000 -     0x7fff69730ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff69731000 -     0x7fff69735ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff69736000 -     0x7fff69738ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff69739000 -     0x7fff6973aff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6973b000 -     0x7fff69752fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff69753000 -     0x7fff69753fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff69754000 -     0x7fff697ddff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff697de000 -     0x7fff697e1ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff697e2000 -     0x7fff697e5ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff697e6000 -     0x7fff697e7fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff697e8000 -     0x7fff697eeff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff697ef000 -     0x7fff69838ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff69839000 -     0x7fff6985eff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6985f000 -     0x7fff698aafcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff698ab000 -     0x7fff698cafff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff698cb000 -     0x7fff6996fff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff69970000 -     0x7fff6997affb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6997b000 -     0x7fff69984ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff69985000 -     0x7fff6998cff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6998d000 -     0x7fff69998fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff69999000 -     0x7fff6999cff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6999d000 -     0x7fff6999eff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6999f000 -     0x7fff699a6ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff699a7000 -     0x7fff699baff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff699bc000 -     0x7fff699c1ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff699c2000 -     0x7fff699eeff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
