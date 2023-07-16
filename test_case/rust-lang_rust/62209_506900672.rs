plain
[00:03:26]       Memory: 8 GB
[00:03:26]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:26]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:26]       SMC Version (system): 2.8f0
[00:03:26]       Serial Number (system): VMfMeR+b9dlq
[00:03:26] 
[00:03:26] hw.ncpu: 4
[00:03:26] hw.byteorder: 1234
[00:03:26] hw.memsize: 8589934592
---
[02:13:51] 
[02:13:51] ---- /Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/listing/allowed-by-default.md - Allowed_by_default_lints::anonymous_parameters (line 32) stdout ----
[02:13:51] error: linking with `cc` failed: signal: 4
[02:13:51]   |
[02:13:51]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest890X6U/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest890X6U/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest890X6U/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest890X6U/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest890X6U/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest890X6U/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest890X6U/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-d690bb63a1263fc0.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-56af4824651d38f6.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-aac8defe80f64147.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-e3949140845e6adf.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-9dc2a2b70d7ea2f5.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-c85649cb4f4a29f8.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-d8a71f17e88bc401.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-37c5ecf7a0816003.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-f6b9bb6623768736.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-13624d4e52f9bfd1.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-929a815c3d2ded1a.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-8aebdf4552891151.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:13:51] 
[02:13:51] error: aborting due to previous error
[02:13:51] 
[02:13:51] thread '/Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/listing/allowed-by-default.md - Allowed_by_default_lints::anonymous_parameters (line 32)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:319:13
---
[02:13:51] 
[02:13:51] 
[02:13:51] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:13:51] Build completed unsuccessfully in 0:54:36
[02:13:51] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1caacdbf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jun 28 23:01:05 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:00df8ab6
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Jun 28 23:00 .
-rw-------@  1 travis  staff  13742 Jun 28 23:00 overflow_2019-06-28-230049_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Jun 28 23:00 foo_2019-06-28-230028_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1418 Jun 28 22:59 m4_2019-06-28-225954_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Jun 28 22:59 bar_2019-06-28-225945-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Jun 28 22:59 bar_2019-06-28-225945_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 Jun 28 22:59 b_2019-06-28-225944_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62246 Jun 28 22:23 a_2019-06-28-222327-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 Jun 28 22:23 a_2019-06-28-222327_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60388 Jun 28 22:23 a_2019-06-28-222319-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37410 Jun 28 22:23 a_2019-06-28-222319_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Jun 28 22:23 a_2019-06-28-222314_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Jun 28 22:23 a_2019-06-28-222309_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Jun 28 22:23 a_2019-06-28-222308-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 Jun 28 22:23 a_2019-06-28-222308_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Jun 28 22:22 a_2019-06-28-222229_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63057 Jun 28 22:22 a_2019-06-28-222219_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65068 Jun 28 22:22 a_2019-06-28-222216-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63914 Jun 28 22:22 a_2019-06-28-222216-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64223 Jun 28 22:22 a_2019-06-28-222216_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11858 Jun 28 22:20 a_2019-06-28-222002_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Jun 28 22:19 a_2019-06-28-221913_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Jun 28 22:17 a_2019-06-28-221759_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Jun 28 22:16 a_2019-06-28-221659-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10489 Jun 28 22:16 a_2019-06-28-221659_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 Jun 28 22:16 a_2019-06-28-221656_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:037cba8a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-28-221656_Traviss-Mac-1044.crash
Process:               a [44574]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44573]
Responsible:           a [44574]
User ID:               501
Date/Time:             2019-06-28 22:16:26.552 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00000001037628ae abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 30
1   a                              0x0000000103761ca9 std::panicking::try::do_call::hdfd124d2d64c5453 (.llvm.14445406436959211721) + 9
2   libstd-d690bb63a1263fc0.dylib  0x000000010379c94f __rust_maybe_catch_panic + 31
3   a                              0x0000000103762b01 abort_on_c_abi::main::he771bf881fc862e3 + 593
4   a                              0x00000001037610f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h6fd9cbe071830d96 + 6
5   libstd-d690bb63a1263fc0.dylib  0x000000010378c998 std::panicking::try::do_call::hfeca15b955301ef9 + 24
6   libstd-d690bb63a1263fc0.dylib  0x000000010379c94f __rust_maybe_catch_panic + 31
7   libstd-d690bb63a1263fc0.dylib  0x000000010378d47e std::rt::lang_start_internal::h681d2dabb4bb573e + 542
8   a                              0x0000000103762e09 main + 41
9   libdyld.dylib                  0x00007fff6b21a115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007f8911d00010  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffeec49cb38  rsi: 0x00000000ffffffc3  rbp: 0x00007ffeec49d590  rsp: 0x00007ffeec49d590
   r8: 0x00000000911d0006   r9: 0x0000000000000004  r10: 0x000000010769f8c0  r11: 0x00007fff6b4d196c
  r12: 0x0000000103a9f000  r13: 0x0000000000000000  r14: 0x00007ffeec49d6b0  r15: 0x00007ffeec49d5f8
  rip: 0x00000001037628ae  rfl: 0x0000000000010202  cr2: 0x00000001037d3a38
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x103760000 -        0x103763fff +a (0) <23D32AF5-42A7-3B04-9E8B-DFFE6A11795C> /Users/USER/*/a
       0x103768000 -        0x1037ffff7 +libstd-d690bb63a1263fc0.dylib (0) <5264166E-D511-3CFF-B6DD-48883E015FCB> /Users/USER/*/libstd-d690bb63a1263fc0.dylib
       0x10764d000 -        0x10769798f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff68a84000 -     0x7fff68ab7fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff68f96000 -     0x7fff68f97ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6924c000 -     0x7fff692a2fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff692a3000 -     0x7fff692c7ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6a619000 -     0x7fff6aa0a3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6aad7000 -     0x7fff6aaf3ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6b0b1000 -     0x7fff6b0b5ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6b0b6000 -     0x7fff6b0c0ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6b0c1000 -     0x7fff6b0c8fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6b0c9000 -     0x7fff6b0d1ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6b0d2000 -     0x7fff6b157fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6b1df000 -     0x7fff6b218ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6b219000 -     0x7fff6b236ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6b237000 -     0x7fff6b237ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6b245000 -     0x7fff6b245ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6b246000 -     0x7fff6b24affb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6b24b000 -     0x7fff6b24dff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6b24e000 -     0x7fff6b24fff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6b250000 -     0x7fff6b267fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6b268000 -     0x7fff6b268fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6b269000 -     0x7fff6b2f2ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6b2f3000 -     0x7fff6b2f6ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6b2f7000 -     0x7fff6b2faffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6b2fb000 -     0x7fff6b2fcfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6b2fd000 -     0x7fff6b303ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6b304000 -     0x7fff6b34dff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6b34e000 -     0x7fff6b373ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6b374000 -     0x7fff6b3bffcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6b3c0000 -     0x7fff6b3dffff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6b3e0000 -     0x7fff6b484ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6b485000 -     0x7fff6b48fffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6b490000 -     0x7fff6b499ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6b49a000 -     0x7fff6b4a1ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6b4a2000 -     0x7fff6b4adfff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6b4ae000 -     0x7fff6b4b1ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6b4b2000 -     0x7fff6b4b3ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6b4b4000 -     0x7fff6b4bbff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6b4bc000 -     0x7fff6b4cfff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6b4d1000 -     0x7fff6b4d6ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6b4d7000 -     0x7fff6b503ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2593
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
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4536K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            277.0M      109 
TOTAL                            277.0M      109 
TOTAL, minus reserved VM space   276.9M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-28-221659-1_Traviss-Mac-1044.crash
Process:               a [45380]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45375]
Responsible:           a [45380]
User ID:               501
Date/Time:             2019-06-28 22:16:57.741 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-d690bb63a1263fc0.dylib  0x00000001098aaf1e std::panicking::rust_panic_with_hook::he7bcf59185f223c3 + 142
1   a                              0x000000010987b8d5 std::panicking::begin_panic::h4ffa484020c768d5 + 37
2   a                              0x00000001098793fc _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x0000000109878ee9 core::ptr::real_drop_in_place::he9e7b49e72e17b2e + 9
4   a                              0x00000001098793d3 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x000000010987a549 backtrace::main::hc9a5bc8fc93ded64 + 4201 (backtrace.rs:104)
6   a                              0x0000000109878906 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf2aa91774941491f + 6 (rt.rs:64)
7   libstd-d690bb63a1263fc0.dylib  0x00000001098aa998 std::panicking::try::do_call::hfeca15b955301ef9 + 24
8   libstd-d690bb63a1263fc0.dylib  0x00000001098ba94f __rust_maybe_catch_panic + 31
9   libstd-d690bb63a1263fc0.dylib  0x00000001098ab47e std::rt::lang_start_internal::h681d2dabb4bb573e + 542
10  a                              0x000000010987ad89 main + 41
11  libdyld.dylib                  0x00007fff6b21a115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee63882d8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001098f2e25  rbp: 0x00007ffee63883d0  rsp: 0x00007ffee6388300
   r8: 0x0000000000000100   r9: 0x0000000109925980  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001098f1c38  r14: 0x000000010987d460  r15: 0x00007ffee63883e0
  rip: 0x00000001098aaf1e  rfl: 0x0000000000010206  cr2: 0x000000011026a6f8
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x109875000 -        0x10987cfff +a (0) <6E91A64F-2483-330D-890A-91ED52C0E22D> /Users/USER/*/a
       0x109886000 -        0x10991dff7 +libstd-d690bb63a1263fc0.dylib (0) <5264166E-D511-3CFF-B6DD-48883E015FCB> /Users/USER/*/libstd-d690bb63a1263fc0.dylib
       0x10e35f000 -        0x10e3a998f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff68a84000 -     0x7fff68ab7fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff68f96000 -     0x7fff68f97ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6924c000 -     0x7fff692a2fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff692a3000 -     0x7fff692c7ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6a619000 -     0x7fff6aa0a3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6aad7000 -     0x7fff6aaf3ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6b0b1000 -     0x7fff6b0b5ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6b0b6000 -     0x7fff6b0c0ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6b0c1000 -     0x7fff6b0c8fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6b0c9000 -     0x7fff6b0d1ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6b0d2000 -     0x7fff6b157fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6b1df000 -     0x7fff6b218ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6b219000 -     0x7fff6b236ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6b237000 -     0x7fff6b237ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6b245000 -     0x7fff6b245ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6b246000 -     0x7fff6b24affb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6b24b000 -     0x7fff6b24dff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6b24e000 -     0x7fff6b24fff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6b250000 -     0x7fff6b267fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6b268000 -     0x7fff6b268fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6b269000 -     0x7fff6b2f2ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6b2f3000 -     0x7fff6b2f6ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6b2f7000 -     0x7fff6b2faffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6b2fb000 -     0x7fff6b2fcfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6b2fd000 -     0x7fff6b303ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6b304000 -     0x7fff6b34dff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6b34e000 -     0x7fff6b373ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6b374000 -     0x7fff6b3bffcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6b3c0000 -     0x7fff6b3dffff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6b3e0000 -     0x7fff6b484ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6b485000 -     0x7fff6b48fffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6b490000 -     0x7fff6b499ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6b49a000 -     0x7fff6b4a1ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6b4a2000 -     0x7fff6b4adfff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6b4ae000 -     0x7fff6b4b1ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6b4b2000 -     0x7fff6b4b3ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6b4b4000 -     0x7fff6b4bbff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6b4bc000 -     0x7fff6b4cfff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6b4d1000 -     0x7fff6b4d6ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6b4d7000 -     0x7fff6b503ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2593
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=73.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9704K        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4536K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            276.4M      110 
TOTAL                            276.4M      110 
TOTAL, minus reserved VM space   276.3M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-28-221659_Traviss-Mac-1044.crash
Process:               a [45379]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        a [45375]
Responsible:           a [45379]
User ID:               501
Date/Time:             2019-06-28 22:16:57.691 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-d690bb63a1263fc0.dylib  0x000000010b89ff1e std::panicking::rust_panic_with_hook::he7bcf59185f223c3 + 142
1   a                              0x000000010b86b8d5 std::panicking::begin_panic::h4ffa484020c768d5 + 37
2   a                              0x000000010b8693fc _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x000000010b868ee9 core::ptr::real_drop_in_place::he9e7b49e72e17b2e + 9
4   a                              0x000000010b8693d3 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x000000010b86a549 backtrace::main::hc9a5bc8fc93ded64 + 4201 (backtrace.rs:104)
6   a                              0x000000010b868906 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf2aa91774941491f + 6 (rt.rs:64)
7   libstd-d690bb63a1263fc0.dylib  0x000000010b89f998 std::panicking::try::do_call::hfeca15b955301ef9 + 24
8   libstd-d690bb63a1263fc0.dylib  0x000000010b8af94f __rust_maybe_catch_panic + 31
9   libstd-d690bb63a1263fc0.dylib  0x000000010b8a047e std::rt::lang_start_internal::h681d2dabb4bb573e + 542
10  a                              0x000000010b86ad89 main + 41
11  libdyld.dylib                  0x00007fff6b21a115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee43982e8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010b8e7e25  rbp: 0x00007ffee43983e0  rsp: 0x00007ffee4398310
   r8: 0x0000000000000100   r9: 0x000000010b91a980  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010b8e6c38  r14: 0x000000010b86d460  r15: 0x00007ffee43983f0
  rip: 0x000000010b89ff1e  rfl: 0x0000000000010206  cr2: 0x000000010bb6f5a9
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10b865000 -        0x10b86cfff +a (0) <6E91A64F-2483-330D-890A-91ED52C0E22D> /Users/USER/*/a
       0x10b87b000 -        0x10b912ff7 +libstd-d690bb63a1263fc0.dylib (0) <5264166E-D511-3CFF-B6DD-48883E015FCB> /Users/USER/*/libstd-d690bb63a1263fc0.dylib
       0x10cd59000 -        0x10cda398f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff68a84000 -     0x7fff68ab7fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff68f96000 -     0x7fff68f97ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6924c000 -     0x7fff692a2fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff692a3000 -     0x7fff692c7ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6a619000 -     0x7fff6aa0a3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6aad7000 -     0x7fff6aaf3ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6b0b1000 -     0x7fff6b0b5ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6b0b6000 -     0x7fff6b0c0ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6b0c1000 -     0x7fff6b0c8fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6b0c9000 -     0x7fff6b0d1ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6b0d2000 -     0x7fff6b157fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6b1df000 -     0x7fff6b218ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6b219000 -     0x7fff6b236ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6b237000 -     0x7fff6b237ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6b245000 -     0x7fff6b245ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6b246000 -     0x7fff6b24affb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6b24b000 -     0x7fff6b24dff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6b24e000 -     0x7fff6b24fff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6b250000 -     0x7fff6b267fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6b268000 -     0x7fff6b268fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6b269000 -     0x7fff6b2f2ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6b2f3000 -     0x7fff6b2f6ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6b2f7000 -     0x7fff6b2faffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6b2fb000 -     0x7fff6b2fcfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6b2fd000 -     0x7fff6b303ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6b304000 -     0x7fff6b34dff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6b34e000 -     0x7fff6b373ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6b374000 -     0x7fff6b3bffcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6b3c0000 -     0x7fff6b3dffff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6b3e0000 -     0x7fff6b484ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6b485000 -     0x7fff6b48fffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6b490000 -     0x7fff6b499ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6b49a000 -     0x7fff6b4a1ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6b4a2000 -     0x7fff6b4adfff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6b4ae000 -     0x7fff6b4b1ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6b4b2000 -     0x7fff6b4b3ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6b4b4000 -     0x7fff6b4bbff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6b4bc000 -     0x7fff6b4cfff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6b4d1000 -     0x7fff6b4d6ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6b4d7000 -     0x7fff6b503ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2593
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=73.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9704K        9 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4536K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            276.4M      109 
TOTAL                            276.4M      109 
TOTAL, minus reserved VM space   276.3M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-28-221759_Traviss-Mac-1044.crash
Process:               a [47089]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [47087]
Responsible:           a [47089]
User ID:               501
Date/Time:             2019-06-28 22:17:57.786 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5600 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff6b369e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff6b4a8150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff6b2c6312 abort + 127
3   libstd-d690bb63a1263fc0.dylib  0x000000010c564e39 std::sys::unix::abort_internal::h55e58f7caec3696a + 9
4   libstd-d690bb63a1263fc0.dylib  0x000000010c554fb0 rust_oom + 32
5   libstd-d690bb63a1263fc0.dylib  0x000000010c5761c9 alloc::alloc::handle_alloc_error::hf7a19cd5bd7db43f + 9
6   a                              0x000000010c52740f default_alloc_error_hook::main::h0fe124586986ad5d + 767
7   a                              0x000000010c526656 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb59e91e03edff360 + 6
8   libstd-d690bb63a1263fc0.dylib  0x000000010c555998 std::panicking::try::do_call::hfeca15b955301ef9 + 24
9   libstd-d690bb63a1263fc0.dylib  0x000000010c56594f __rust_maybe_catch_panic + 31
10  libstd-d690bb63a1263fc0.dylib  0x000000010c55647e std::rt::lang_start_internal::h681d2dabb4bb573e + 542
11  a                              0x000000010c527569 main + 41
12  libdyld.dylib                  0x00007fff6b21a115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fffa3f40340  rcx: 0x00007ffee36d84f8  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee36d8530  rsp: 0x00007ffee36d84f8
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff6b369e3e  rfl: 0x0000000000000206  cr2: 0x00007fffa3f1e148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10c525000 -        0x10c527ff7 +a (0) <CB137E37-C44B-3F13-ADDC-F4B305981DBB> /Users/USER/*/a
       0x10c531000 -        0x10c5c8ff7 +libstd-d690bb63a1263fc0.dylib (0) <5264166E-D511-3CFF-B6DD-48883E015FCB> /Users/USER/*/libstd-d690bb63a1263fc0.dylib
       0x11b6fc000 -        0x11b74698f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff68a84000 -     0x7fff68ab7fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff68f96000 -     0x7fff68f97ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6924c000 -     0x7fff692a2fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff692a3000 -     0x7fff692c7ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6a619000 -     0x7fff6aa0a3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6aad7000 -     0x7fff6aaf3ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6b0b1000 -     0x7fff6b0b5ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6b0b6000 -     0x7fff6b0c0ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6b0c1000 -     0x7fff6b0c8fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6b0c9000 -     0x7fff6b0d1ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6b0d2000 -     0x7fff6b157fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6b1df000 -     0x7fff6b218ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6b219000 -     0x7fff6b236ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6b237000 -     0x7fff6b237ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6b245000 -     0x7fff6b245ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6b246000 -     0x7fff6b24affb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6b24b000 -     0x7fff6b24dff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6b24e000 -     0x7fff6b24fff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6b250000 -     0x7fff6b267fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6b268000 -     0x7fff6b268fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6b269000 -     0x7fff6b2f2ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6b2f3000 -     0x7fff6b2f6ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6b2f7000 -     0x7fff6b2faffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6b2fb000 -     0x7fff6b2fcfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6b2fd000 -     0x7fff6b303ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6b304000 -     0x7fff6b34dff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6b34e000 -     0x7fff6b373ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6b374000 -     0x7fff6b3bffcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6b3c0000 -     0x7fff6b3dffff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6b3e0000 -     0x7fff6b484ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6b485000 -     0x7fff6b48fffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6b490000 -     0x7fff6b499ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6b49a000 -     0x7fff6b4a1ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6b4a2000 -     0x7fff6b4adfff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6b4ae000 -     0x7fff6b4b1ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6b4b2000 -     0x7fff6b4b3ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6b4b4000 -     0x7fff6b4bbff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6b4bc000 -     0x7fff6b4cfff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6b4d1000 -     0x7fff6b4d6ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6b4d7000 -     0x7fff6b503ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
