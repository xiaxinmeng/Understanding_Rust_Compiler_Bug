plain
[00:03:21]       Memory: 8 GB
[00:03:21]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:21]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:21]       SMC Version (system): 2.8f0
[00:03:21]       Serial Number (system): VMvs0VuC9N2N
[00:03:21] 
[00:03:21] hw.ncpu: 4
[00:03:21] hw.byteorder: 1234
[00:03:21] hw.memsize: 8589934592
---
[02:09:32]   = note: #[warn(deprecated)] on by default
[02:09:32] 
[02:09:32] error: linking with `cc` failed: signal: 4
[02:09:32]   |
[02:09:32]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestMPJNVt/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestMPJNVt/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestMPJNVt/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestMPJNVt/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestMPJNVt/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestMPJNVt/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestMPJNVt/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-34734850493769af.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-a78108309d2b3c7b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-70da0e2448484869.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-9e554381d179e20a.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-651b9e51d633bf9b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-8921121bb4e873f4.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-3629f5938a98f297.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-37c5ecf7a0816003.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-6444c0be9f895cce.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-13624d4e52f9bfd1.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-929a815c3d2ded1a.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-e9e25c002504da63.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:09:32] 
[02:09:32] error: aborting due to previous error
[02:09:32] 
[02:09:32] thread '/Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/listing/warn-by-default.md - Warn_by_default_lints::deprecated (line 50)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:319:13
---
[02:09:32] 
[02:09:32] 
[02:09:32] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:09:32] Build completed unsuccessfully in 0:51:49
[02:09:32] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07bd1b4a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat May 11 03:54:12 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:018a7090
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 May 11 03:53 .
-rw-------@  1 travis  staff  13742 May 11 03:53 overflow_2019-05-11-035351_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 May 11 03:53 foo_2019-05-11-035330_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1417 May 11 03:52 m4_2019-05-11-035257_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 May 11 03:52 bar_2019-05-11-035247_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 May 11 03:52 bar_2019-05-11-035248_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10660 May 11 03:52 b_2019-05-11-035247_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37661 May 11 03:19 a_2019-05-11-031929-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62244 May 11 03:19 a_2019-05-11-031929_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37411 May 11 03:19 a_2019-05-11-031923-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60387 May 11 03:19 a_2019-05-11-031923_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 May 11 03:19 a_2019-05-11-031917_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 May 11 03:19 a_2019-05-11-031911_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 May 11 03:19 a_2019-05-11-031903_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9792 May 11 03:19 a_2019-05-11-031902_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 May 11 03:18 a_2019-05-11-031821_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63060 May 11 03:18 a_2019-05-11-031809_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 May 11 03:18 a_2019-05-11-031806_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64224 May 11 03:18 a_2019-05-11-031805-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65082 May 11 03:18 a_2019-05-11-031805_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11714 May 11 03:15 a_2019-05-11-031545_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 May 11 03:14 a_2019-05-11-031450_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 May 11 03:13 a_2019-05-11-031331_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 May 11 03:12 a_2019-05-11-031230-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 May 11 03:12 a_2019-05-11-031230_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10213 May 11 03:12 a_2019-05-11-031224_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:06e4b2e6
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-11-031224_Traviss-Mac-1044.crash
Process:               a [44399]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44396]
Responsible:           a [44399]
User ID:               501
Date/Time:             2019-05-11 03:11:56.493 +0000
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
0   a                              0x00000001007bf8ae abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 30
1   a                              0x00000001007beca9 std::panicking::try::do_call::ha971e0735812f8c6 (.llvm.3311831159045282148) + 9
2   libstd-34734850493769af.dylib  0x00000001007fd78f __rust_maybe_catch_panic + 31
3   a                              0x00000001007bfb01 abort_on_c_abi::main::he771bf881fc862e3 + 593
4   a                              0x00000001007be0f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h555e08ed9dceecfe + 6
5   libstd-34734850493769af.dylib  0x00000001007ed6d8 std::panicking::try::do_call::h8c832a84bb43d60d + 24
6   libstd-34734850493769af.dylib  0x00000001007fd78f __rust_maybe_catch_panic + 31
7   libstd-34734850493769af.dylib  0x00000001007ee1be std::rt::lang_start_internal::h12daf27a9b690671 + 542
8   a                              0x00000001007bfe09 main + 41
9   libdyld.dylib                  0x00007fff5b305115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007fb611c02bb0  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffeef43fc48  rsi: 0x000000000fffffff  rbp: 0x00007ffeef4406a0  rsp: 0x00007ffeef4406a0
   r8: 0x00000000611c02c0   r9: 0x0000000000000004  r10: 0x0000000102b188c0  r11: 0x00007fff5b5bc96c
  r12: 0x0000000100b01000  r13: 0x0000000000000000  r14: 0x00007ffeef4407c0  r15: 0x00007ffeef440708
  rip: 0x00000001007bf8ae  rfl: 0x0000000000010206  cr2: 0x0000000100834848
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1007bd000 -        0x1007c0fff +a (0) <33E23A92-F2BB-386A-BEFB-8EF801B39E19> /Users/USER/*/a
       0x1007c9000 -        0x100860fff +libstd-34734850493769af.dylib (0) <BE7DB041-1BCF-3378-9828-541687D4B726> /Users/USER/*/libstd-34734850493769af.dylib
       0x102ac6000 -        0x102b1098f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff58b6f000 -     0x7fff58ba2fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff59081000 -     0x7fff59082ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff59337000 -     0x7fff5938dfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5938e000 -     0x7fff593b2ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5a704000 -     0x7fff5aaf53b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5abc2000 -     0x7fff5abdeffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5b19c000 -     0x7fff5b1a0ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5b1a1000 -     0x7fff5b1abff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5b1ac000 -     0x7fff5b1b3fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5b1b4000 -     0x7fff5b1bcffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5b1bd000 -     0x7fff5b242fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5b2ca000 -     0x7fff5b303ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5b304000 -     0x7fff5b321ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5b322000 -     0x7fff5b322ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5b330000 -     0x7fff5b330ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5b331000 -     0x7fff5b335ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5b336000 -     0x7fff5b338ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5b339000 -     0x7fff5b33aff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5b33b000 -     0x7fff5b352fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5b353000 -     0x7fff5b353fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5b354000 -     0x7fff5b3ddff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5b3de000 -     0x7fff5b3e1ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5b3e2000 -     0x7fff5b3e5ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5b3e6000 -     0x7fff5b3e7fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5b3e8000 -     0x7fff5b3eeff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5b3ef000 -     0x7fff5b438ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5b439000 -     0x7fff5b45eff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5b45f000 -     0x7fff5b4aafcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5b4ab000 -     0x7fff5b4cafff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5b4cb000 -     0x7fff5b56fff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5b570000 -     0x7fff5b57affb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5b57b000 -     0x7fff5b584ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5b585000 -     0x7fff5b58cff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5b58d000 -     0x7fff5b598fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5b599000 -     0x7fff5b59cff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5b59d000 -     0x7fff5b59eff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5b59f000 -     0x7fff5b5a6ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5b5a7000 -     0x7fff5b5baff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5b5bc000 -     0x7fff5b5c1ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5b5c2000 -     0x7fff5b5eeff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2591
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
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
__DATA                            4556K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            276.0M      108 
TOTAL                            276.0M      108 
TOTAL, minus reserved VM space   275.9M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-11-031230-1_Traviss-Mac-1044.crash
Process:               a [45218]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45207]
Responsible:           a [45218]
User ID:               501
Date/Time:             2019-05-11 03:12:29.071 +0000
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
0   libstd-34734850493769af.dylib  0x000000010893cc5e std::panicking::rust_panic_with_hook::h51322746cd1bd29d + 142
1   a                              0x00000001089088e5 std::panicking::begin_panic::h1e77356346d0dd51 + 37
2   a                              0x000000010890640c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x0000000108905ef9 core::ptr::real_drop_in_place::he9e7b49e72e17b2e + 9
4   a                              0x00000001089063e3 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x0000000108907559 backtrace::main::hc9a5bc8fc93ded64 + 4201 (backtrace.rs:103)
6   a                              0x0000000108905936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha7fb8c74f530cd60 + 6 (rt.rs:64)
7   libstd-34734850493769af.dylib  0x000000010893c6d8 std::panicking::try::do_call::h8c832a84bb43d60d + 24
8   libstd-34734850493769af.dylib  0x000000010894c78f __rust_maybe_catch_panic + 31
9   libstd-34734850493769af.dylib  0x000000010893d1be std::rt::lang_start_internal::h12daf27a9b690671 + 542
10  a                              0x0000000108907d99 main + 41
11  libdyld.dylib                  0x00007fff5b305115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee72fb3e8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000108984cf5  rbp: 0x00007ffee72fb4e0  rsp: 0x00007ffee72fb410
   r8: 0xffffffff00000100   r9: 0x00000001089b79d0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000108983a58  r14: 0x000000010890a460  r15: 0x00007ffee72fb4f0
  rip: 0x000000010893cc5e  rfl: 0x0000000000010206  cr2: 0x000000010891cf50
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x108902000 -        0x108909fff +a (0) <EC755C11-4FA7-3FC7-A16B-E9A94AB340A7> /Users/USER/*/a
       0x108918000 -        0x1089affff +libstd-34734850493769af.dylib (0) <BE7DB041-1BCF-3378-9828-541687D4B726> /Users/USER/*/libstd-34734850493769af.dylib
       0x114fa6000 -        0x114ff098f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff58b6f000 -     0x7fff58ba2fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff59081000 -     0x7fff59082ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff59337000 -     0x7fff5938dfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5938e000 -     0x7fff593b2ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5a704000 -     0x7fff5aaf53b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5abc2000 -     0x7fff5abdeffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5b19c000 -     0x7fff5b1a0ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5b1a1000 -     0x7fff5b1abff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5b1ac000 -     0x7fff5b1b3fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5b1b4000 -     0x7fff5b1bcffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5b1bd000 -     0x7fff5b242fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5b2ca000 -     0x7fff5b303ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5b304000 -     0x7fff5b321ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5b322000 -     0x7fff5b322ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5b330000 -     0x7fff5b330ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5b331000 -     0x7fff5b335ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5b336000 -     0x7fff5b338ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5b339000 -     0x7fff5b33aff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5b33b000 -     0x7fff5b352fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5b353000 -     0x7fff5b353fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5b354000 -     0x7fff5b3ddff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5b3de000 -     0x7fff5b3e1ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5b3e2000 -     0x7fff5b3e5ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5b3e6000 -     0x7fff5b3e7fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5b3e8000 -     0x7fff5b3eeff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5b3ef000 -     0x7fff5b438ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5b439000 -     0x7fff5b45eff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5b45f000 -     0x7fff5b4aafcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5b4ab000 -     0x7fff5b4cafff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5b4cb000 -     0x7fff5b56fff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5b570000 -     0x7fff5b57affb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5b57b000 -     0x7fff5b584ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5b585000 -     0x7fff5b58cff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5b58d000 -     0x7fff5b598fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5b599000 -     0x7fff5b59cff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5b59d000 -     0x7fff5b59eff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5b59f000 -     0x7fff5b5a6ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5b5a7000 -     0x7fff5b5baff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5b5bc000 -     0x7fff5b5c1ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5b5c2000 -     0x7fff5b5eeff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2591
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=73.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9696K        9 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4556K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            276.5M      109 
TOTAL                            276.5M      109 
TOTAL, minus reserved VM space   276.3M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-11-031230_Traviss-Mac-1044.crash
Process:               a [45215]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45207]
Responsible:           a [45215]
User ID:               501
Date/Time:             2019-05-11 03:12:29.070 +0000
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
0   libstd-34734850493769af.dylib  0x000000010460fc5e std::panicking::rust_panic_with_hook::h51322746cd1bd29d + 142
1   a                              0x00000001045dc8e5 std::panicking::begin_panic::h1e77356346d0dd51 + 37
2   a                              0x00000001045da40c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x00000001045d9ef9 core::ptr::real_drop_in_place::he9e7b49e72e17b2e + 9
4   a                              0x00000001045da3e3 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x00000001045db559 backtrace::main::hc9a5bc8fc93ded64 + 4201 (backtrace.rs:103)
6   a                              0x00000001045d9936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha7fb8c74f530cd60 + 6 (rt.rs:64)
7   libstd-34734850493769af.dylib  0x000000010460f6d8 std::panicking::try::do_call::h8c832a84bb43d60d + 24
8   libstd-34734850493769af.dylib  0x000000010461f78f __rust_maybe_catch_panic + 31
9   libstd-34734850493769af.dylib  0x00000001046101be std::rt::lang_start_internal::h12daf27a9b690671 + 542
10  a                              0x00000001045dbd99 main + 41
11  libdyld.dylib                  0x00007fff5b305115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeeb6273f8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000104657cf5  rbp: 0x00007ffeeb6274f0  rsp: 0x00007ffeeb627420
   r8: 0xffffffff00000100   r9: 0x000000010468a9d0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000104656a58  r14: 0x00000001045de460  r15: 0x00007ffeeb627500
  rip: 0x000000010460fc5e  rfl: 0x0000000000010202  cr2: 0x00000001048e4619
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1045d6000 -        0x1045ddfff +a (0) <EC755C11-4FA7-3FC7-A16B-E9A94AB340A7> /Users/USER/*/a
       0x1045eb000 -        0x104682fff +libstd-34734850493769af.dylib (0) <BE7DB041-1BCF-3378-9828-541687D4B726> /Users/USER/*/libstd-34734850493769af.dylib
       0x11089a000 -        0x1108e498f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff58b6f000 -     0x7fff58ba2fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff59081000 -     0x7fff59082ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff59337000 -     0x7fff5938dfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5938e000 -     0x7fff593b2ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5a704000 -     0x7fff5aaf53b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5abc2000 -     0x7fff5abdeffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5b19c000 -     0x7fff5b1a0ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5b1a1000 -     0x7fff5b1abff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5b1ac000 -     0x7fff5b1b3fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5b1b4000 -     0x7fff5b1bcffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5b1bd000 -     0x7fff5b242fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5b2ca000 -     0x7fff5b303ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5b304000 -     0x7fff5b321ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5b322000 -     0x7fff5b322ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5b330000 -     0x7fff5b330ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5b331000 -     0x7fff5b335ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5b336000 -     0x7fff5b338ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5b339000 -     0x7fff5b33aff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5b33b000 -     0x7fff5b352fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5b353000 -     0x7fff5b353fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5b354000 -     0x7fff5b3ddff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5b3de000 -     0x7fff5b3e1ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5b3e2000 -     0x7fff5b3e5ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5b3e6000 -     0x7fff5b3e7fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5b3e8000 -     0x7fff5b3eeff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5b3ef000 -     0x7fff5b438ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5b439000 -     0x7fff5b45eff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5b45f000 -     0x7fff5b4aafcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5b4ab000 -     0x7fff5b4cafff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5b4cb000 -     0x7fff5b56fff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5b570000 -     0x7fff5b57affb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5b57b000 -     0x7fff5b584ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5b585000 -     0x7fff5b58cff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5b58d000 -     0x7fff5b598fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5b599000 -     0x7fff5b59cff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5b59d000 -     0x7fff5b59eff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5b59f000 -     0x7fff5b5a6ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5b5a7000 -     0x7fff5b5baff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5b5bc000 -     0x7fff5b5c1ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5b5c2000 -     0x7fff5b5eeff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2591
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=82.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=82.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.5M        9 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4556K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            285.5M      109 
TOTAL                            285.5M      109 
TOTAL, minus reserved VM space   285.3M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-11-031331_Traviss-Mac-1044.crash
Process:               a [46921]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [46919]
Responsible:           a [46921]
User ID:               501
Date/Time:             2019-05-11 03:13:30.972 +0000
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
0   libsystem_kernel.dylib         0x00007fff5b454e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff5b593150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff5b3b1312 abort + 127
3   libstd-34734850493769af.dylib  0x000000010488bc79 std::sys::unix::abort_internal::hbb724715ba3f53d5 + 9
4   libstd-34734850493769af.dylib  0x000000010487bcf0 rust_oom + 32
5   libstd-34734850493769af.dylib  0x000000010489cf89 alloc::alloc::handle_alloc_error::h6c35f3681b6a884b + 9
6   a                              0x0000000104852ecf default_alloc_error_hook::main::h0fe124586986ad5d + 767
7   a                              0x00000001048526d6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb7e7d73dd06ac8e7 + 6
8   libstd-34734850493769af.dylib  0x000000010487c6d8 std::panicking::try::do_call::h8c832a84bb43d60d + 24
9   libstd-34734850493769af.dylib  0x000000010488c78f __rust_maybe_catch_panic + 31
10  libstd-34734850493769af.dylib  0x000000010487d1be std::rt::lang_start_internal::h12daf27a9b690671 + 542
11  a                              0x0000000104853029 main + 41
12  libdyld.dylib                  0x00007fff5b305115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff9402b340  rcx: 0x00007ffeeb3ac608  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffeeb3ac640  rsp: 0x00007ffeeb3ac608
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff5b454e3e  rfl: 0x0000000000000206  cr2: 0x00007fff94009148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x104851000 -        0x104853fff +a (0) <30EE7E66-E22F-3BC4-93FD-A7FABBF05BF1> /Users/USER/*/a
       0x104858000 -        0x1048effff +libstd-34734850493769af.dylib (0) <BE7DB041-1BCF-3378-9828-541687D4B726> /Users/USER/*/libstd-34734850493769af.dylib
       0x11237b000 -        0x1123c598f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff58b6f000 -     0x7fff58ba2fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff59081000 -     0x7fff59082ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff59337000 -     0x7fff5938dfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5938e000 -     0x7fff593b2ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5a704000 -     0x7fff5aaf53b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5abc2000 -     0x7fff5abdeffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5b19c000 -     0x7fff5b1a0ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5b1a1000 -     0x7fff5b1abff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5b1ac000 -     0x7fff5b1b3fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5b1b4000 -     0x7fff5b1bcffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5b1bd000 -     0x7fff5b242fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5b2ca000 -     0x7fff5b303ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5b304000 -     0x7fff5b321ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5b322000 -     0x7fff5b322ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5b330000 -     0x7fff5b330ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5b331000 -     0x7fff5b335ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5b336000 -     0x7fff5b338ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5b339000 -     0x7fff5b33aff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5b33b000 -     0x7fff5b352fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5b353000 -     0x7fff5b353fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5b354000 -     0x7fff5b3ddff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5b3de000 -     0x7fff5b3e1ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5b3e2000 -     0x7fff5b3e5ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5b3e6000 -     0x7fff5b3e7fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5b3e8000 -     0x7fff5b3eeff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5b3ef000 -     0x7fff5b438ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5b439000 -     0x7fff5b45eff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5b45f000 -     0x7fff5b4aafcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5b4ab000 -     0x7fff5b4cafff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5b4cb000 -     0x7fff5b56fff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5b570000 -     0x7fff5b57affb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5b57b000 -     0x7fff5b584ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5b585000 -     0x7fff5b58cff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5b58d000 -     0x7fff5b598fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5b599000 -     0x7fff5b59cff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5b59d000 -     0x7fff5b59eff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5b59f000 -     0x7fff5b5a6ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5b5a7000 -     0x7fff5b5baff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5b5bc000 -     0x7fff5b5c1ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5b5c2000 -     0x7fff5b5eeff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
