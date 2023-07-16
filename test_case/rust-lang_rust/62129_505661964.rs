plain
[00:03:38]       Memory: 8 GB
[00:03:38]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:38]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:38]       SMC Version (system): 2.8f0
[00:03:38]       Serial Number (system): VMEylqV+/qKj
[00:03:38] 
[00:03:38] hw.ncpu: 4
[00:03:38] hw.byteorder: 1234
[00:03:38] hw.memsize: 8589934592
---
[02:05:07] 
[02:05:07] ---- /Users/travis/build/rust-lang/rust/src/doc/rustdoc/src/documentation-tests.md - Documentation_tests::Attributes (line 311) stdout ----
[02:05:07] error: linking with `cc` failed: signal: 4
[02:05:07]   |
[02:05:07]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestcWfpd0/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestcWfpd0/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestcWfpd0/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestcWfpd0/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestcWfpd0/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestcWfpd0/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestcWfpd0/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-329ef83dbc1dd86d.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-ddf1f863d2eaf653.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace-a26391313b65d7dc.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-0ad3fa8cdd0b9704.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-4d45b60ed1186ef3.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-b5491b1ffbbc1406.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-4228aae7223b6103.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-99ea320c6f5b5eea.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-6595b4030f6ddb0e.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-d930cd0bdea4797b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-8bde4cd3b4c45eea.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-6450bce6cc789177.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-992f5c26c2d9c590.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-0f189b415a69de4e.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:05:07] 
[02:05:07] error: aborting due to previous error
[02:05:07] 
[02:05:07] Couldn't compile the test.
---
travis_fold:start:after_failure.2
travis_time:start:004a0e5c
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Jun 25 23:40 .
-rw-------@  1 travis  staff  13742 Jun 25 23:40 overflow_2019-06-25-234009_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Jun 25 23:39 foo_2019-06-25-233948_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1418 Jun 25 23:39 m4_2019-06-25-233914_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Jun 25 23:39 bar_2019-06-25-233904-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Jun 25 23:39 bar_2019-06-25-233904_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 Jun 25 23:39 b_2019-06-25-233903_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62244 Jun 25 22:51 a_2019-06-25-225137-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 Jun 25 22:51 a_2019-06-25-225137_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60388 Jun 25 22:51 a_2019-06-25-225127-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37410 Jun 25 22:51 a_2019-06-25-225127_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Jun 25 22:51 a_2019-06-25-225121_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Jun 25 22:51 a_2019-06-25-225116_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Jun 25 22:51 a_2019-06-25-225114_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 Jun 25 22:51 a_2019-06-25-225113_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10033 Jun 25 22:50 a_2019-06-25-225036_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63914 Jun 25 22:50 a_2019-06-25-225022-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65081 Jun 25 22:50 a_2019-06-25-225022-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63112 Jun 25 22:50 a_2019-06-25-225022-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64276 Jun 25 22:50 a_2019-06-25-225022_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11714 Jun 25 22:48 a_2019-06-25-224806_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Jun 25 22:47 a_2019-06-25-224719_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Jun 25 22:46 a_2019-06-25-224608_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Jun 25 22:45 a_2019-06-25-224548-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Jun 25 22:45 a_2019-06-25-224548-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 Jun 25 22:45 a_2019-06-25-224548_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:02060604
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-25-224548-1_Traviss-Mac-1044.crash
Process:               a [42411]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [42404]
Responsible:           a [42411]
User ID:               501
Date/Time:             2019-06-25 22:45:09.761 +0000
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
0   libstd-329ef83dbc1dd86d.dylib  0x0000000107f9a9de std::panicking::rust_panic_with_hook::h756d0bba11076be5 + 142
1   a                              0x0000000107f696c5 std::panicking::begin_panic::haff5b930812bcdc4 + 37
2   a                              0x0000000107f66d6c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x0000000107f66589 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x0000000107f66d43 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x0000000107f682aa backtrace::main::hc9a5bc8fc93ded64 + 5210 (backtrace.rs:119)
6   a                              0x0000000107f66276 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc80788536e97b081 + 6 (rt.rs:64)
7   libstd-329ef83dbc1dd86d.dylib  0x0000000107f9a458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
8   libstd-329ef83dbc1dd86d.dylib  0x0000000107faa03f __rust_maybe_catch_panic + 31
9   libstd-329ef83dbc1dd86d.dylib  0x0000000107f9af3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
10  a                              0x0000000107f68bf9 main + 41
11  libdyld.dylib                  0x00007fff62b82115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee7c9b268  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000107feae42  rbp: 0x00007ffee7c9b360  rsp: 0x00007ffee7c9b290
   r8: 0xffffffff00000100   r9: 0x000000010801eae0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000107fe9c38  r14: 0x0000000107f6b480  r15: 0x00007ffee7c9b370
  rip: 0x0000000107f9a9de  rfl: 0x0000000000010202  cr2: 0x00007fb832c560f0
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x107f62000 -        0x107f6aff7 +a (0) <DE32EEFB-76CF-3561-9EC6-A683DB68D532> /Users/USER/*/a
       0x107f77000 -        0x108016fff +libstd-329ef83dbc1dd86d.dylib (0) <371CFC9F-C01C-33C9-81CF-5F1B3231812E> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x115a14000 -        0x115a5e98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff603ec000 -     0x7fff6041ffff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff608fe000 -     0x7fff608ffff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff60bb4000 -     0x7fff60c0afff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff60c0b000 -     0x7fff60c2fff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff61f81000 -     0x7fff623723b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6243f000 -     0x7fff6245bffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff62a19000 -     0x7fff62a1dff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff62a1e000 -     0x7fff62a28ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff62a29000 -     0x7fff62a30fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff62a31000 -     0x7fff62a39ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff62a3a000 -     0x7fff62abffff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff62b47000 -     0x7fff62b80ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff62b81000 -     0x7fff62b9eff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff62b9f000 -     0x7fff62b9fffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff62bad000 -     0x7fff62badff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff62bae000 -     0x7fff62bb2ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff62bb3000 -     0x7fff62bb5ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff62bb6000 -     0x7fff62bb7ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff62bb8000 -     0x7fff62bcffff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff62bd0000 -     0x7fff62bd0fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff62bd1000 -     0x7fff62c5aff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff62c5b000 -     0x7fff62c5effb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff62c5f000 -     0x7fff62c62ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff62c63000 -     0x7fff62c64fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff62c65000 -     0x7fff62c6bff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff62c6c000 -     0x7fff62cb5ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff62cb6000 -     0x7fff62cdbff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff62cdc000 -     0x7fff62d27fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff62d28000 -     0x7fff62d47fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff62d48000 -     0x7fff62decff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff62ded000 -     0x7fff62df7ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff62df8000 -     0x7fff62e01ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff62e02000 -     0x7fff62e09ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff62e0a000 -     0x7fff62e15fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff62e16000 -     0x7fff62e19ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff62e1a000 -     0x7fff62e1bff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff62e1c000 -     0x7fff62e23ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff62e24000 -     0x7fff62e37ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff62e39000 -     0x7fff62e3eff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff62e3f000 -     0x7fff62e6bff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2064
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
__DATA                            4556K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9676K       44 
===========                     =======  ======= 
TOTAL                            276.6M      110 
TOTAL                            276.6M      110 
TOTAL, minus reserved VM space   276.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-25-224548-2_Traviss-Mac-1044.crash
Process:               a [42412]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [42404]
Responsible:           a [42412]
User ID:               501
Date/Time:             2019-06-25 22:45:09.785 +0000
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
0   libstd-329ef83dbc1dd86d.dylib  0x000000010bd219de std::panicking::rust_panic_with_hook::h756d0bba11076be5 + 142
1   a                              0x000000010bcf06c5 std::panicking::begin_panic::haff5b930812bcdc4 + 37
2   a                              0x000000010bcedd6c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x000000010bced589 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x000000010bcedd43 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x000000010bcef2aa backtrace::main::hc9a5bc8fc93ded64 + 5210 (backtrace.rs:119)
6   a                              0x000000010bced276 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc80788536e97b081 + 6 (rt.rs:64)
7   libstd-329ef83dbc1dd86d.dylib  0x000000010bd21458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
8   libstd-329ef83dbc1dd86d.dylib  0x000000010bd3103f __rust_maybe_catch_panic + 31
9   libstd-329ef83dbc1dd86d.dylib  0x000000010bd21f3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
10  a                              0x000000010bcefbf9 main + 41
11  libdyld.dylib                  0x00007fff62b82115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee3f14258  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010bd71e42  rbp: 0x00007ffee3f14350  rsp: 0x00007ffee3f14280
   r8: 0xffffffff00000100   r9: 0x000000010bda5ae0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010bd70c38  r14: 0x000000010bcf2480  r15: 0x00007ffee3f14360
  rip: 0x000000010bd219de  rfl: 0x0000000000010202  cr2: 0x000000010bd61c60
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10bce9000 -        0x10bcf1ff7 +a (0) <DE32EEFB-76CF-3561-9EC6-A683DB68D532> /Users/USER/*/a
       0x10bcfe000 -        0x10bd9dfff +libstd-329ef83dbc1dd86d.dylib (0) <371CFC9F-C01C-33C9-81CF-5F1B3231812E> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x10e831000 -        0x10e87b98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff603ec000 -     0x7fff6041ffff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff608fe000 -     0x7fff608ffff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff60bb4000 -     0x7fff60c0afff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff60c0b000 -     0x7fff60c2fff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff61f81000 -     0x7fff623723b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6243f000 -     0x7fff6245bffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff62a19000 -     0x7fff62a1dff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff62a1e000 -     0x7fff62a28ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff62a29000 -     0x7fff62a30fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff62a31000 -     0x7fff62a39ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff62a3a000 -     0x7fff62abffff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff62b47000 -     0x7fff62b80ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff62b81000 -     0x7fff62b9eff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff62b9f000 -     0x7fff62b9fffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff62bad000 -     0x7fff62badff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff62bae000 -     0x7fff62bb2ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff62bb3000 -     0x7fff62bb5ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff62bb6000 -     0x7fff62bb7ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff62bb8000 -     0x7fff62bcffff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff62bd0000 -     0x7fff62bd0fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff62bd1000 -     0x7fff62c5aff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff62c5b000 -     0x7fff62c5effb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff62c5f000 -     0x7fff62c62ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff62c63000 -     0x7fff62c64fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff62c65000 -     0x7fff62c6bff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff62c6c000 -     0x7fff62cb5ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff62cb6000 -     0x7fff62cdbff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff62cdc000 -     0x7fff62d27fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff62d28000 -     0x7fff62d47fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff62d48000 -     0x7fff62decff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff62ded000 -     0x7fff62df7ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff62df8000 -     0x7fff62e01ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff62e02000 -     0x7fff62e09ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff62e0a000 -     0x7fff62e15fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff62e16000 -     0x7fff62e19ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff62e1a000 -     0x7fff62e1bff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff62e1c000 -     0x7fff62e23ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff62e24000 -     0x7fff62e37ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff62e39000 -     0x7fff62e3eff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff62e3f000 -     0x7fff62e6bff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2064
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=27.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=27.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            19.5M        9 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4556K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9676K       44 
===========                     =======  ======= 
TOTAL                            286.6M      110 
TOTAL                            286.6M      110 
TOTAL, minus reserved VM space   286.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-25-224548_Traviss-Mac-1044.crash
Process:               a [41634]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41629]
Responsible:           a [41634]
User ID:               501
Date/Time:             2019-06-25 22:44:40.499 +0000
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
0   a                              0x00000001083667fe abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 30
1   a                              0x0000000108365bf9 std::panicking::try::do_call::h279475168e6f9cd6 (.llvm.15361624866771944484) + 9
2   libstd-329ef83dbc1dd86d.dylib  0x00000001083a503f __rust_maybe_catch_panic + 31
3   a                              0x0000000108366a51 abort_on_c_abi::main::he771bf881fc862e3 + 593
4   a                              0x00000001083650d6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hbc930dad328663fe + 6
5   libstd-329ef83dbc1dd86d.dylib  0x0000000108395458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
6   libstd-329ef83dbc1dd86d.dylib  0x00000001083a503f __rust_maybe_catch_panic + 31
7   libstd-329ef83dbc1dd86d.dylib  0x0000000108395f3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
8   a                              0x0000000108366d59 main + 41
9   libdyld.dylib                  0x00007fff62b82115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007f9c4cd003c0  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee7898ae8  rsi: 0x000000001fffffff  rbp: 0x00007ffee7899540  rsp: 0x00007ffee7899540
   r8: 0x00000000c4cd0041   r9: 0x0000000000000004  r10: 0x000000011066a8c0  r11: 0x00007fff62e3996c
  r12: 0x00000001086b8000  r13: 0x0000000000000000  r14: 0x00007ffee7899660  r15: 0x00007ffee78995a8
  rip: 0x00000001083667fe  rfl: 0x0000000000010206  cr2: 0x00000001083e4998
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x108364000 -        0x108367fff +a (0) <B9B7BC2A-8B83-375C-A5CD-F87771AAF509> /Users/USER/*/a
       0x108372000 -        0x108411fff +libstd-329ef83dbc1dd86d.dylib (0) <371CFC9F-C01C-33C9-81CF-5F1B3231812E> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x110618000 -        0x11066298f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff603ec000 -     0x7fff6041ffff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff608fe000 -     0x7fff608ffff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff60bb4000 -     0x7fff60c0afff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff60c0b000 -     0x7fff60c2fff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff61f81000 -     0x7fff623723b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6243f000 -     0x7fff6245bffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff62a19000 -     0x7fff62a1dff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff62a1e000 -     0x7fff62a28ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff62a29000 -     0x7fff62a30fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff62a31000 -     0x7fff62a39ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff62a3a000 -     0x7fff62abffff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff62b47000 -     0x7fff62b80ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff62b81000 -     0x7fff62b9eff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff62b9f000 -     0x7fff62b9fffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff62bad000 -     0x7fff62badff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff62bae000 -     0x7fff62bb2ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff62bb3000 -     0x7fff62bb5ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff62bb6000 -     0x7fff62bb7ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff62bb8000 -     0x7fff62bcffff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff62bd0000 -     0x7fff62bd0fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff62bd1000 -     0x7fff62c5aff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff62c5b000 -     0x7fff62c5effb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff62c5f000 -     0x7fff62c62ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff62c63000 -     0x7fff62c64fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff62c65000 -     0x7fff62c6bff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff62c6c000 -     0x7fff62cb5ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff62cb6000 -     0x7fff62cdbff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff62cdc000 -     0x7fff62d27fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff62d28000 -     0x7fff62d47fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff62d48000 -     0x7fff62decff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff62ded000 -     0x7fff62df7ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff62df8000 -     0x7fff62e01ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff62e02000 -     0x7fff62e09ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff62e0a000 -     0x7fff62e15fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff62e16000 -     0x7fff62e19ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff62e1a000 -     0x7fff62e1bff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff62e1c000 -     0x7fff62e23ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff62e24000 -     0x7fff62e37ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff62e39000 -     0x7fff62e3eff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff62e3f000 -     0x7fff62e6bff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2064
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
MALLOC guard page                   16K        4 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4556K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9656K       44 
===========                     =======  ======= 
TOTAL                            277.1M      108 
TOTAL                            277.1M      108 
TOTAL, minus reserved VM space   276.9M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-25-224608_Traviss-Mac-1044.crash
Process:               a [44136]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44135]
Responsible:           a [44136]
User ID:               501
Date/Time:             2019-06-25 22:46:07.795 +0000
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
0   libsystem_kernel.dylib         0x00007fff62cd1e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff62e10150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff62c2e312 abort + 127
3   libstd-329ef83dbc1dd86d.dylib  0x000000010e362579 std::sys::unix::abort_internal::hb0d87c939ffeb797 + 9
4   libstd-329ef83dbc1dd86d.dylib  0x000000010e352900 rust_oom + 32
5   libstd-329ef83dbc1dd86d.dylib  0x000000010e378539 alloc::alloc::handle_alloc_error::h4044521f3c8241d3 + 9
6   a                              0x000000010e32607f default_alloc_error_hook::main::h0fe124586986ad5d + 767
7   a                              0x000000010e326736 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h417afde3001e038b + 6
8   libstd-329ef83dbc1dd86d.dylib  0x000000010e353458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
9   libstd-329ef83dbc1dd86d.dylib  0x000000010e36303f __rust_maybe_catch_panic + 31
10  libstd-329ef83dbc1dd86d.dylib  0x000000010e353f3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
11  a                              0x000000010e3261d9 main + 41
12  libdyld.dylib                  0x00007fff62b82115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff9b8a8340  rcx: 0x00007ffee18d94a8  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee18d94e0  rsp: 0x00007ffee18d94a8
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff62cd1e3e  rfl: 0x0000000000000206  cr2: 0x00007fff9b886148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10e324000 -        0x10e326fff +a (0) <0A4F8E0E-5AEE-386A-AFF2-393BFC812577> /Users/USER/*/a
       0x10e330000 -        0x10e3cffff +libstd-329ef83dbc1dd86d.dylib (0) <371CFC9F-C01C-33C9-81CF-5F1B3231812E> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x115306000 -        0x11535098f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff603ec000 -     0x7fff6041ffff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff608fe000 -     0x7fff608ffff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff60bb4000 -     0x7fff60c0afff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff60c0b000 -     0x7fff60c2fff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff61f81000 -     0x7fff623723b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6243f000 -     0x7fff6245bffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff62a19000 -     0x7fff62a1dff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff62a1e000 -     0x7fff62a28ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff62a29000 -     0x7fff62a30fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff62a31000 -     0x7fff62a39ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff62a3a000 -     0x7fff62abffff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff62b47000 -     0x7fff62b80ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff62b81000 -     0x7fff62b9eff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff62b9f000 -     0x7fff62b9fffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff62bad000 -     0x7fff62badff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff62bae000 -     0x7fff62bb2ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff62bb3000 -     0x7fff62bb5ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff62bb6000 -     0x7fff62bb7ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff62bb8000 -     0x7fff62bcffff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff62bd0000 -     0x7fff62bd0fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff62bd1000 -     0x7fff62c5aff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff62c5b000 -     0x7fff62c5effb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff62c5f000 -     0x7fff62c62ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff62c63000 -     0x7fff62c64fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff62c65000 -     0x7fff62c6bff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff62c6c000 -     0x7fff62cb5ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff62cb6000 -     0x7fff62cdbff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff62cdc000 -     0x7fff62d27fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff62d28000 -     0x7fff62d47fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff62d48000 -     0x7fff62decff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff62ded000 -     0x7fff62df7ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff62df8000 -     0x7fff62e01ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff62e02000 -     0x7fff62e09ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff62e0a000 -     0x7fff62e15fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff62e16000 -     0x7fff62e19ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff62e1a000 -     0x7fff62e1bff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff62e1c000 -     0x7fff62e23ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff62e24000 -     0x7fff62e37ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff62e39000 -     0x7fff62e3eff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff62e3f000 -     0x7fff62e6bff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
