plain
[00:03:15]       Memory: 8 GB
[00:03:15]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:15]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:15]       SMC Version (system): 2.8f0
[00:03:15]       Serial Number (system): VMXMOq3+huIh
[00:03:15] 
[00:03:15] hw.ncpu: 4
[00:03:15] hw.byteorder: 1234
[00:03:15] hw.memsize: 8589934592
---
[02:01:13] 
[02:01:13] ---- /Users/travis/build/rust-lang/rust/src/doc/unstable-book/src/language-features/transparent-unions.md - The_tracking_issue_for_this_feature_is_ (line 36) stdout ----
[02:01:13] error: linking with `cc` failed: signal: 4
[02:01:13]   |
[02:01:13]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest5OtjBP/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest5OtjBP/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest5OtjBP/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest5OtjBP/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest5OtjBP/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest5OtjBP/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest5OtjBP/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-329ef83dbc1dd86d.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-ddf1f863d2eaf653.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace-a26391313b65d7dc.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-0ad3fa8cdd0b9704.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-4d45b60ed1186ef3.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-b5491b1ffbbc1406.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-4228aae7223b6103.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-99ea320c6f5b5eea.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-6595b4030f6ddb0e.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-d930cd0bdea4797b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-8bde4cd3b4c45eea.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-6450bce6cc789177.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-992f5c26c2d9c590.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-0f189b415a69de4e.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:01:13] 
[02:01:13] error: aborting due to previous error
[02:01:13] 
[02:01:13] Couldn't compile the test.
---
travis_fold:start:after_failure.2
travis_time:start:02e68195
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Jun 21 21:25 .
-rw-------@  1 travis  staff  13742 Jun 21 21:25 overflow_2019-06-21-212559_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Jun 21 21:25 foo_2019-06-21-212537_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1418 Jun 21 21:24 m4_2019-06-21-212459_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Jun 21 21:24 bar_2019-06-21-212449_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 Jun 21 21:24 b_2019-06-21-212447_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Jun 21 21:24 bar_2019-06-21-212448_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 Jun 21 20:38 a_2019-06-21-203847-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62244 Jun 21 20:38 a_2019-06-21-203847_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37410 Jun 21 20:38 a_2019-06-21-203839-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60386 Jun 21 20:38 a_2019-06-21-203839_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Jun 21 20:38 a_2019-06-21-203833_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Jun 21 20:38 a_2019-06-21-203828_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 Jun 21 20:38 a_2019-06-21-203825-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Jun 21 20:38 a_2019-06-21-203825_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10033 Jun 21 20:37 a_2019-06-21-203748_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63914 Jun 21 20:37 a_2019-06-21-203735-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64276 Jun 21 20:37 a_2019-06-21-203735-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63112 Jun 21 20:37 a_2019-06-21-203735-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65081 Jun 21 20:37 a_2019-06-21-203735_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11712 Jun 21 20:35 a_2019-06-21-203535_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Jun 21 20:34 a_2019-06-21-203443_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Jun 21 20:33 a_2019-06-21-203346-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 Jun 21 20:33 a_2019-06-21-203346-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Jun 21 20:33 a_2019-06-21-203346-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Jun 21 20:33 a_2019-06-21-203346_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:067ecc9f
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-21-203346-1_Traviss-Mac-1044.crash
Process:               a [42366]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [42361]
Responsible:           a [42366]
User ID:               501
Date/Time:             2019-06-21 20:32:34.294 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4200 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-329ef83dbc1dd86d.dylib  0x00000001090519de std::panicking::rust_panic_with_hook::h756d0bba11076be5 + 142
1   a                              0x00000001090216c5 std::panicking::begin_panic::haff5b930812bcdc4 + 37
2   a                              0x000000010901ed6c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x000000010901e589 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x000000010901ed43 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x00000001090202aa backtrace::main::hc9a5bc8fc93ded64 + 5210 (backtrace.rs:119)
6   a                              0x000000010901e276 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc80788536e97b081 + 6 (rt.rs:64)
7   libstd-329ef83dbc1dd86d.dylib  0x0000000109051458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
8   libstd-329ef83dbc1dd86d.dylib  0x000000010906103f __rust_maybe_catch_panic + 31
9   libstd-329ef83dbc1dd86d.dylib  0x0000000109051f3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
10  a                              0x0000000109020bf9 main + 41
11  libdyld.dylib                  0x00007fff5b29b115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee6be3398  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001090a1e42  rbp: 0x00007ffee6be3490  rsp: 0x00007ffee6be33c0
   r8: 0xffffffff00000100   r9: 0x00000001090d5ae0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001090a0c38  r14: 0x0000000109023480  r15: 0x00007ffee6be34a0
  rip: 0x00000001090519de  rfl: 0x0000000000010202  cr2: 0x0000000109091c60
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10901a000 -        0x109022ff7 +a (0) <DE32EEFB-76CF-3561-9EC6-A683DB68D532> /Users/USER/*/a
       0x10902e000 -        0x1090cdfff +libstd-329ef83dbc1dd86d.dylib (0) <4DD8F7E9-84AE-3D96-8DFF-7DBFF715188D> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x10b5e1000 -        0x10b62b98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff58b05000 -     0x7fff58b38fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff59017000 -     0x7fff59018ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff592cd000 -     0x7fff59323fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff59324000 -     0x7fff59348ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5a69a000 -     0x7fff5aa8b3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5ab58000 -     0x7fff5ab74ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5b132000 -     0x7fff5b136ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5b137000 -     0x7fff5b141ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5b142000 -     0x7fff5b149fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5b14a000 -     0x7fff5b152ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5b153000 -     0x7fff5b1d8fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5b260000 -     0x7fff5b299ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5b29a000 -     0x7fff5b2b7ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5b2b8000 -     0x7fff5b2b8ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5b2c6000 -     0x7fff5b2c6ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5b2c7000 -     0x7fff5b2cbffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5b2cc000 -     0x7fff5b2ceff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5b2cf000 -     0x7fff5b2d0ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5b2d1000 -     0x7fff5b2e8fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5b2e9000 -     0x7fff5b2e9fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5b2ea000 -     0x7fff5b373ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5b374000 -     0x7fff5b377ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5b378000 -     0x7fff5b37bffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5b37c000 -     0x7fff5b37dfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5b37e000 -     0x7fff5b384ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5b385000 -     0x7fff5b3ceff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5b3cf000 -     0x7fff5b3f4ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5b3f5000 -     0x7fff5b440fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5b441000 -     0x7fff5b460fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5b461000 -     0x7fff5b505ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5b506000 -     0x7fff5b510ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5b511000 -     0x7fff5b51aff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5b51b000 -     0x7fff5b522ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5b523000 -     0x7fff5b52efff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5b52f000 -     0x7fff5b532ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5b533000 -     0x7fff5b534ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5b535000 -     0x7fff5b53cff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5b53d000 -     0x7fff5b550ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5b552000 -     0x7fff5b557ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5b558000 -     0x7fff5b584ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2100
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
__TEXT                            9676K       44 
===========                     =======  ======= 
TOTAL                            276.5M      110 
TOTAL                            276.5M      110 
TOTAL, minus reserved VM space   276.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-21-203346-2_Traviss-Mac-1044.crash
Process:               a [41597]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41592]
Responsible:           a [41597]
User ID:               501
Date/Time:             2019-06-21 20:32:05.082 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4100 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000000010d42c7fe abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 30
1   a                              0x000000010d42bbf9 std::panicking::try::do_call::h279475168e6f9cd6 (.llvm.15361624866771944484) + 9
2   libstd-329ef83dbc1dd86d.dylib  0x000000010d46c03f __rust_maybe_catch_panic + 31
3   a                              0x000000010d42ca51 abort_on_c_abi::main::he771bf881fc862e3 + 593
4   a                              0x000000010d42b0d6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hbc930dad328663fe + 6
5   libstd-329ef83dbc1dd86d.dylib  0x000000010d45c458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
6   libstd-329ef83dbc1dd86d.dylib  0x000000010d46c03f __rust_maybe_catch_panic + 31
7   libstd-329ef83dbc1dd86d.dylib  0x000000010d45cf3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
8   a                              0x000000010d42cd59 main + 41
9   libdyld.dylib                  0x00007fff5b29b115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ff55de00290  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee27d2c28  rsi: 0x00000000ffffc3ff  rbp: 0x00007ffee27d3680  rsp: 0x00007ffee27d3680
   r8: 0x0000000055de002e   r9: 0x0000000000000004  r10: 0x000000010dcde8c0  r11: 0x00007fff5b55296c
  r12: 0x000000010d77e000  r13: 0x0000000000000000  r14: 0x00007ffee27d37a0  r15: 0x00007ffee27d36e8
  rip: 0x000000010d42c7fe  rfl: 0x0000000000010202  cr2: 0x000000010d4ab998
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10d42a000 -        0x10d42dfff +a (0) <B9B7BC2A-8B83-375C-A5CD-F87771AAF509> /Users/USER/*/a
       0x10d439000 -        0x10d4d8fff +libstd-329ef83dbc1dd86d.dylib (0) <4DD8F7E9-84AE-3D96-8DFF-7DBFF715188D> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x10dc8c000 -        0x10dcd698f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff58b05000 -     0x7fff58b38fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff59017000 -     0x7fff59018ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff592cd000 -     0x7fff59323fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff59324000 -     0x7fff59348ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5a69a000 -     0x7fff5aa8b3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5ab58000 -     0x7fff5ab74ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5b132000 -     0x7fff5b136ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5b137000 -     0x7fff5b141ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5b142000 -     0x7fff5b149fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5b14a000 -     0x7fff5b152ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5b153000 -     0x7fff5b1d8fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5b260000 -     0x7fff5b299ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5b29a000 -     0x7fff5b2b7ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5b2b8000 -     0x7fff5b2b8ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5b2c6000 -     0x7fff5b2c6ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5b2c7000 -     0x7fff5b2cbffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5b2cc000 -     0x7fff5b2ceff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5b2cf000 -     0x7fff5b2d0ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5b2d1000 -     0x7fff5b2e8fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5b2e9000 -     0x7fff5b2e9fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5b2ea000 -     0x7fff5b373ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5b374000 -     0x7fff5b377ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5b378000 -     0x7fff5b37bffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5b37c000 -     0x7fff5b37dfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5b37e000 -     0x7fff5b384ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5b385000 -     0x7fff5b3ceff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5b3cf000 -     0x7fff5b3f4ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5b3f5000 -     0x7fff5b440fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5b441000 -     0x7fff5b460fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5b461000 -     0x7fff5b505ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5b506000 -     0x7fff5b510ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5b511000 -     0x7fff5b51aff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5b51b000 -     0x7fff5b522ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5b523000 -     0x7fff5b52efff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5b52f000 -     0x7fff5b532ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5b533000 -     0x7fff5b534ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5b535000 -     0x7fff5b53cff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5b53d000 -     0x7fff5b550ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5b552000 -     0x7fff5b557ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5b558000 -     0x7fff5b584ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2100
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=20.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=20.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            12.0M        8 
MALLOC guard page                   16K        4 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4552K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9656K       44 
===========                     =======  ======= 
TOTAL                            279.1M      108 
TOTAL                            279.1M      108 
TOTAL, minus reserved VM space   278.9M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-21-203346-3_Traviss-Mac-1044.crash
Process:               a [42365]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [42361]
Responsible:           a [42365]
User ID:               501
Date/Time:             2019-06-21 20:32:34.232 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4200 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-329ef83dbc1dd86d.dylib  0x00000001075439de std::panicking::rust_panic_with_hook::h756d0bba11076be5 + 142
1   a                              0x00000001075156c5 std::panicking::begin_panic::haff5b930812bcdc4 + 37
2   a                              0x0000000107512d6c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x0000000107512589 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x0000000107512d43 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x00000001075142aa backtrace::main::hc9a5bc8fc93ded64 + 5210 (backtrace.rs:119)
6   a                              0x0000000107512276 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc80788536e97b081 + 6 (rt.rs:64)
7   libstd-329ef83dbc1dd86d.dylib  0x0000000107543458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
8   libstd-329ef83dbc1dd86d.dylib  0x000000010755303f __rust_maybe_catch_panic + 31
9   libstd-329ef83dbc1dd86d.dylib  0x0000000107543f3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
10  a                              0x0000000107514bf9 main + 41
11  libdyld.dylib                  0x00007fff5b29b115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee86ef3b8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000107593e42  rbp: 0x00007ffee86ef4b0  rsp: 0x00007ffee86ef3e0
   r8: 0xffffffff00000100   r9: 0x00000001075c7ae0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000107592c38  r14: 0x0000000107517480  r15: 0x00007ffee86ef4c0
  rip: 0x00000001075439de  rfl: 0x0000000000010202  cr2: 0x00007fa803fffdae
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10750e000 -        0x107516ff7 +a (0) <DE32EEFB-76CF-3561-9EC6-A683DB68D532> /Users/USER/*/a
       0x107520000 -        0x1075bffff +libstd-329ef83dbc1dd86d.dylib (0) <4DD8F7E9-84AE-3D96-8DFF-7DBFF715188D> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x1098dd000 -        0x10992798f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff58b05000 -     0x7fff58b38fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff59017000 -     0x7fff59018ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff592cd000 -     0x7fff59323fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff59324000 -     0x7fff59348ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5a69a000 -     0x7fff5aa8b3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5ab58000 -     0x7fff5ab74ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5b132000 -     0x7fff5b136ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5b137000 -     0x7fff5b141ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5b142000 -     0x7fff5b149fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5b14a000 -     0x7fff5b152ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5b153000 -     0x7fff5b1d8fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5b260000 -     0x7fff5b299ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5b29a000 -     0x7fff5b2b7ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5b2b8000 -     0x7fff5b2b8ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5b2c6000 -     0x7fff5b2c6ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5b2c7000 -     0x7fff5b2cbffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5b2cc000 -     0x7fff5b2ceff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5b2cf000 -     0x7fff5b2d0ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5b2d1000 -     0x7fff5b2e8fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5b2e9000 -     0x7fff5b2e9fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5b2ea000 -     0x7fff5b373ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5b374000 -     0x7fff5b377ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5b378000 -     0x7fff5b37bffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5b37c000 -     0x7fff5b37dfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5b37e000 -     0x7fff5b384ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5b385000 -     0x7fff5b3ceff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5b3cf000 -     0x7fff5b3f4ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5b3f5000 -     0x7fff5b440fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5b441000 -     0x7fff5b460fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5b461000 -     0x7fff5b505ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5b506000 -     0x7fff5b510ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5b511000 -     0x7fff5b51aff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5b51b000 -     0x7fff5b522ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5b523000 -     0x7fff5b52efff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5b52f000 -     0x7fff5b532ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5b533000 -     0x7fff5b534ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5b535000 -     0x7fff5b53cff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5b53d000 -     0x7fff5b550ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5b552000 -     0x7fff5b557ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5b558000 -     0x7fff5b584ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2100
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=35.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=35.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            27.5M        9 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4552K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9676K       44 
===========                     =======  ======= 
TOTAL                            294.5M      110 
TOTAL                            294.5M      110 
TOTAL, minus reserved VM space   294.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-21-203346_Traviss-Mac-1044.crash
Process:               a [44087]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44086]
Responsible:           a [44087]
User ID:               501
Date/Time:             2019-06-21 20:33:32.924 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4200 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff5b3eae3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff5b529150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff5b347312 abort + 127
3   libstd-329ef83dbc1dd86d.dylib  0x000000010678c579 std::sys::unix::abort_internal::hb0d87c939ffeb797 + 9
4   libstd-329ef83dbc1dd86d.dylib  0x000000010677c900 rust_oom + 32
5   libstd-329ef83dbc1dd86d.dylib  0x00000001067a2539 alloc::alloc::handle_alloc_error::h4044521f3c8241d3 + 9
6   a                              0x000000010675607f default_alloc_error_hook::main::h0fe124586986ad5d + 767
7   a                              0x0000000106756736 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h417afde3001e038b + 6
8   libstd-329ef83dbc1dd86d.dylib  0x000000010677d458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
9   libstd-329ef83dbc1dd86d.dylib  0x000000010678d03f __rust_maybe_catch_panic + 31
10  libstd-329ef83dbc1dd86d.dylib  0x000000010677df3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
11  a                              0x00000001067561d9 main + 41
12  libdyld.dylib                  0x00007fff5b29b115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff93fc1340  rcx: 0x00007ffee94a95e8  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee94a9620  rsp: 0x00007ffee94a95e8
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff5b3eae3e  rfl: 0x0000000000000206  cr2: 0x00007fff93f9f148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x106754000 -        0x106756fff +a (0) <0A4F8E0E-5AEE-386A-AFF2-393BFC812577> /Users/USER/*/a
       0x10675a000 -        0x1067f9fff +libstd-329ef83dbc1dd86d.dylib (0) <4DD8F7E9-84AE-3D96-8DFF-7DBFF715188D> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x10b7e8000 -        0x10b83298f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff58b05000 -     0x7fff58b38fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff59017000 -     0x7fff59018ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff592cd000 -     0x7fff59323fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff59324000 -     0x7fff59348ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5a69a000 -     0x7fff5aa8b3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5ab58000 -     0x7fff5ab74ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5b132000 -     0x7fff5b136ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5b137000 -     0x7fff5b141ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5b142000 -     0x7fff5b149fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5b14a000 -     0x7fff5b152ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5b153000 -     0x7fff5b1d8fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5b260000 -     0x7fff5b299ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5b29a000 -     0x7fff5b2b7ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5b2b8000 -     0x7fff5b2b8ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5b2c6000 -     0x7fff5b2c6ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5b2c7000 -     0x7fff5b2cbffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5b2cc000 -     0x7fff5b2ceff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5b2cf000 -     0x7fff5b2d0ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5b2d1000 -     0x7fff5b2e8fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5b2e9000 -     0x7fff5b2e9fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5b2ea000 -     0x7fff5b373ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5b374000 -     0x7fff5b377ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5b378000 -     0x7fff5b37bffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5b37c000 -     0x7fff5b37dfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5b37e000 -     0x7fff5b384ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5b385000 -     0x7fff5b3ceff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5b3cf000 -     0x7fff5b3f4ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5b3f5000 -     0x7fff5b440fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5b441000 -     0x7fff5b460fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5b461000 -     0x7fff5b505ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5b506000 -     0x7fff5b510ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5b511000 -     0x7fff5b51aff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5b51b000 -     0x7fff5b522ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5b523000 -     0x7fff5b52efff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5b52f000 -     0x7fff5b532ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5b533000 -     0x7fff5b534ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5b535000 -     0x7fff5b53cff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5b53d000 -     0x7fff5b550ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5b552000 -     0x7fff5b557ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5b558000 -     0x7fff5b584ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
