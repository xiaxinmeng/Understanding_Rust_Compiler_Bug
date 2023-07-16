plain
[00:03:13]       Memory: 8 GB
[00:03:13]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:13]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:13]       SMC Version (system): 2.8f0
[00:03:13]       Serial Number (system): VMOVIVDU9e2M
[00:03:13] 
[00:03:13] hw.ncpu: 4
[00:03:13] hw.byteorder: 1234
[00:03:13] hw.memsize: 8589934592
---
[01:59:01] 
[01:59:01] ---- /Users/travis/build/rust-lang/rust/src/doc/unstable-book/src/language-features/unsized-locals.md - The_tracking_issue_for_this_feature_is__::By_value_trait_objects (line 104) stdout ----
[01:59:01] error: linking with `cc` failed: signal: 4
[01:59:01]   |
[01:59:01]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest37prRd/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest37prRd/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest37prRd/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest37prRd/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest37prRd/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest37prRd/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest37prRd/rust_out.rust_out.7rcbfp3g-cgu.6.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest37prRd/rust_out.rust_out.7rcbfp3g-cgu.7.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest37prRd/rust_out.rust_out.7rcbfp3g-cgu.8.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest37prRd/rust_out.rust_out.7rcbfp3g-cgu.9.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest37prRd/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest37prRd/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-329ef83dbc1dd86d.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-ddf1f863d2eaf653.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace-a26391313b65d7dc.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-0ad3fa8cdd0b9704.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-4d45b60ed1186ef3.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-b5491b1ffbbc1406.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-4228aae7223b6103.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-99ea320c6f5b5eea.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-6595b4030f6ddb0e.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-d930cd0bdea4797b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-8bde4cd3b4c45eea.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-6450bce6cc789177.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-992f5c26c2d9c590.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-0f189b415a69de4e.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[01:59:01] 
[01:59:01] error: aborting due to previous error
[01:59:01] 
[01:59:01] Couldn't compile the test.
---
travis_fold:start:after_failure.2
travis_time:start:02fcd6c3
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Jun 19 06:43 .
-rw-------@  1 travis  staff  13742 Jun 19 06:43 overflow_2019-06-19-064342_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Jun 19 06:43 foo_2019-06-19-064320_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1418 Jun 19 06:42 m4_2019-06-19-064245_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 Jun 19 06:42 b_2019-06-19-064235_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Jun 19 06:42 bar_2019-06-19-064235-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Jun 19 06:42 bar_2019-06-19-064235_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62246 Jun 19 05:58 a_2019-06-19-055840-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 Jun 19 05:58 a_2019-06-19-055840_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60389 Jun 19 05:58 a_2019-06-19-055829-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37411 Jun 19 05:58 a_2019-06-19-055829_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Jun 19 05:58 a_2019-06-19-055823_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Jun 19 05:58 a_2019-06-19-055820_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Jun 19 05:58 a_2019-06-19-055816_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 Jun 19 05:58 a_2019-06-19-055815_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Jun 19 05:57 a_2019-06-19-055740_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64224 Jun 19 05:57 a_2019-06-19-055728-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65090 Jun 19 05:57 a_2019-06-19-055728-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63060 Jun 19 05:57 a_2019-06-19-055728-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 Jun 19 05:57 a_2019-06-19-055728_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11712 Jun 19 05:55 a_2019-06-19-055527_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Jun 19 05:54 a_2019-06-19-055439_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Jun 19 05:53 a_2019-06-19-055337-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10489 Jun 19 05:53 a_2019-06-19-055337-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 Jun 19 05:53 a_2019-06-19-055337-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Jun 19 05:53 a_2019-06-19-055337_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0ac13281
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-19-055337-1_Traviss-Mac-1044.crash
Process:               a [42345]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [42334]
Responsible:           a [42345]
User ID:               501
Date/Time:             2019-06-19 05:52:33.407 +0000
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
0   libstd-329ef83dbc1dd86d.dylib  0x000000010892c75e std::panicking::rust_panic_with_hook::h756d0bba11076be5 + 142
1   a                              0x00000001088fc6c5 std::panicking::begin_panic::haff5b930812bcdc4 + 37
2   a                              0x00000001088f9d6c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x00000001088f9589 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x00000001088f9d43 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x00000001088fb2aa backtrace::main::hc9a5bc8fc93ded64 + 5210 (backtrace.rs:119)
6   a                              0x00000001088f9276 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc80788536e97b081 + 6 (rt.rs:64)
7   libstd-329ef83dbc1dd86d.dylib  0x000000010892c1d8 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
8   libstd-329ef83dbc1dd86d.dylib  0x000000010893c0af __rust_maybe_catch_panic + 31
9   libstd-329ef83dbc1dd86d.dylib  0x000000010892ccbe std::rt::lang_start_internal::hb2916a8a31799db8 + 542
10  a                              0x00000001088fbbf9 main + 41
11  libdyld.dylib                  0x00007fff5e060115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee73080e8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010897ce82  rbp: 0x00007ffee73081e0  rsp: 0x00007ffee7308110
   r8: 0xffffffff00000100   r9: 0x00000001089b0ae0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010897bc78  r14: 0x00000001088fe480  r15: 0x00007ffee73081f0
  rip: 0x000000010892c75e  rfl: 0x0000000000010206  cr2: 0x0000000100df5000
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1088f5000 -        0x1088fdff7 +a (0) <C9E8C9DB-F13E-3333-8266-89D1976649E2> /Users/USER/*/a
       0x108909000 -        0x1089a8fff +libstd-329ef83dbc1dd86d.dylib (0) <473334B9-7978-3FCB-AAC2-299F4D113E14> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x10f65d000 -        0x10f6a798f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5b8ca000 -     0x7fff5b8fdfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5bddc000 -     0x7fff5bdddff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5c092000 -     0x7fff5c0e8fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5c0e9000 -     0x7fff5c10dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5d45f000 -     0x7fff5d8503b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5d91d000 -     0x7fff5d939ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5def7000 -     0x7fff5defbff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5defc000 -     0x7fff5df06ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5df07000 -     0x7fff5df0efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5df0f000 -     0x7fff5df17ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5df18000 -     0x7fff5df9dfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5e025000 -     0x7fff5e05eff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5e05f000 -     0x7fff5e07cff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5e07d000 -     0x7fff5e07dffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5e08b000 -     0x7fff5e08bff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5e08c000 -     0x7fff5e090ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5e091000 -     0x7fff5e093ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5e094000 -     0x7fff5e095ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5e096000 -     0x7fff5e0adfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5e0ae000 -     0x7fff5e0aefff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5e0af000 -     0x7fff5e138ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5e139000 -     0x7fff5e13cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5e13d000 -     0x7fff5e140ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5e141000 -     0x7fff5e142fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5e143000 -     0x7fff5e149ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5e14a000 -     0x7fff5e193ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5e194000 -     0x7fff5e1b9ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5e1ba000 -     0x7fff5e205fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5e206000 -     0x7fff5e225fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5e226000 -     0x7fff5e2caff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5e2cb000 -     0x7fff5e2d5ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5e2d6000 -     0x7fff5e2dfff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5e2e0000 -     0x7fff5e2e7ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5e2e8000 -     0x7fff5e2f3fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5e2f4000 -     0x7fff5e2f7ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5e2f8000 -     0x7fff5e2f9ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5e2fa000 -     0x7fff5e301ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5e302000 -     0x7fff5e315ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5e317000 -     0x7fff5e31cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5e31d000 -     0x7fff5e349ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2109
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=26.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=26.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.5M        9 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4552K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9676K       44 
===========                     =======  ======= 
TOTAL                            285.5M      110 
TOTAL                            285.5M      110 
TOTAL, minus reserved VM space   285.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-19-055337-2_Traviss-Mac-1044.crash
Process:               a [42342]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        a [42334]
Responsible:           a [42342]
User ID:               501
Date/Time:             2019-06-19 05:52:33.168 +0000
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
0   libstd-329ef83dbc1dd86d.dylib  0x0000000101dcd75e std::panicking::rust_panic_with_hook::h756d0bba11076be5 + 142
1   a                              0x0000000101d9a6c5 std::panicking::begin_panic::haff5b930812bcdc4 + 37
2   a                              0x0000000101d97d6c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x0000000101d97589 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x0000000101d97d43 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x0000000101d992aa backtrace::main::hc9a5bc8fc93ded64 + 5210 (backtrace.rs:119)
6   a                              0x0000000101d97276 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc80788536e97b081 + 6 (rt.rs:64)
7   libstd-329ef83dbc1dd86d.dylib  0x0000000101dcd1d8 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
8   libstd-329ef83dbc1dd86d.dylib  0x0000000101ddd0af __rust_maybe_catch_panic + 31
9   libstd-329ef83dbc1dd86d.dylib  0x0000000101dcdcbe std::rt::lang_start_internal::hb2916a8a31799db8 + 542
10  a                              0x0000000101d99bf9 main + 41
11  libdyld.dylib                  0x00007fff5e060115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeede6a108  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000101e1de82  rbp: 0x00007ffeede6a200  rsp: 0x00007ffeede6a130
   r8: 0xffffffff00000100   r9: 0x0000000101e51ae0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000101e1cc78  r14: 0x0000000101d9c480  r15: 0x00007ffeede6a210
  rip: 0x0000000101dcd75e  rfl: 0x0000000000010202  cr2: 0x00007fa3205481ee
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x101d93000 -        0x101d9bff7 +a (0) <C9E8C9DB-F13E-3333-8266-89D1976649E2> /Users/USER/*/a
       0x101daa000 -        0x101e49fff +libstd-329ef83dbc1dd86d.dylib (0) <473334B9-7978-3FCB-AAC2-299F4D113E14> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x10a191000 -        0x10a1db98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5b8ca000 -     0x7fff5b8fdfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5bddc000 -     0x7fff5bdddff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5c092000 -     0x7fff5c0e8fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5c0e9000 -     0x7fff5c10dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5d45f000 -     0x7fff5d8503b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5d91d000 -     0x7fff5d939ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5def7000 -     0x7fff5defbff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5defc000 -     0x7fff5df06ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5df07000 -     0x7fff5df0efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5df0f000 -     0x7fff5df17ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5df18000 -     0x7fff5df9dfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5e025000 -     0x7fff5e05eff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5e05f000 -     0x7fff5e07cff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5e07d000 -     0x7fff5e07dffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5e08b000 -     0x7fff5e08bff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5e08c000 -     0x7fff5e090ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5e091000 -     0x7fff5e093ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5e094000 -     0x7fff5e095ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5e096000 -     0x7fff5e0adfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5e0ae000 -     0x7fff5e0aefff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5e0af000 -     0x7fff5e138ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5e139000 -     0x7fff5e13cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5e13d000 -     0x7fff5e140ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5e141000 -     0x7fff5e142fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5e143000 -     0x7fff5e149ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5e14a000 -     0x7fff5e193ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5e194000 -     0x7fff5e1b9ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5e1ba000 -     0x7fff5e205fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5e206000 -     0x7fff5e225fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5e226000 -     0x7fff5e2caff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5e2cb000 -     0x7fff5e2d5ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5e2d6000 -     0x7fff5e2dfff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5e2e0000 -     0x7fff5e2e7ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5e2e8000 -     0x7fff5e2f3fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5e2f4000 -     0x7fff5e2f7ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5e2f8000 -     0x7fff5e2f9ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5e2fa000 -     0x7fff5e301ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5e302000 -     0x7fff5e315ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5e317000 -     0x7fff5e31cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5e31d000 -     0x7fff5e349ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2109
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=26.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=26.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.5M        9 
MALLOC guard page                   16K        4 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4552K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9676K       44 
===========                     =======  ======= 
TOTAL                            285.5M      109 
TOTAL                            285.5M      109 
TOTAL, minus reserved VM space   285.4M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-19-055337-3_Traviss-Mac-1044.crash
Process:               a [41567]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41562]
Responsible:           a [41567]
User ID:               501
Date/Time:             2019-06-19 05:52:04.882 +0000
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
0   a                              0x0000000104dec7fe abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 30
1   a                              0x0000000104debbf9 std::panicking::try::do_call::h279475168e6f9cd6 (.llvm.15361624866771944484) + 9
2   libstd-329ef83dbc1dd86d.dylib  0x0000000104e290af __rust_maybe_catch_panic + 31
3   a                              0x0000000104deca51 abort_on_c_abi::main::he771bf881fc862e3 + 593
4   a                              0x0000000104deb0d6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hbc930dad328663fe + 6
5   libstd-329ef83dbc1dd86d.dylib  0x0000000104e191d8 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
6   libstd-329ef83dbc1dd86d.dylib  0x0000000104e290af __rust_maybe_catch_panic + 31
7   libstd-329ef83dbc1dd86d.dylib  0x0000000104e19cbe std::rt::lang_start_internal::hb2916a8a31799db8 + 542
8   a                              0x0000000104decd59 main + 41
9   libdyld.dylib                  0x00007fff5e060115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007f8443402b90  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffeeae12978  rsi: 0x00000000c3ffffff  rbp: 0x00007ffeeae133d0  rsp: 0x00007ffeeae133d0
   r8: 0x00000000443402be   r9: 0x0000000000000004  r10: 0x00000001097128c0  r11: 0x00007fff5e31796c
  r12: 0x000000010513c000  r13: 0x0000000000000000  r14: 0x00007ffeeae134f0  r15: 0x00007ffeeae13438
  rip: 0x0000000104dec7fe  rfl: 0x0000000000010206  cr2: 0x0000000104e68a8c
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x104dea000 -        0x104dedfff +a (0) <C4F92C48-5115-39E4-B886-FFE9D9948CC3> /Users/USER/*/a
       0x104df6000 -        0x104e95fff +libstd-329ef83dbc1dd86d.dylib (0) <473334B9-7978-3FCB-AAC2-299F4D113E14> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x1096c0000 -        0x10970a98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5b8ca000 -     0x7fff5b8fdfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5bddc000 -     0x7fff5bdddff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5c092000 -     0x7fff5c0e8fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5c0e9000 -     0x7fff5c10dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5d45f000 -     0x7fff5d8503b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5d91d000 -     0x7fff5d939ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5def7000 -     0x7fff5defbff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5defc000 -     0x7fff5df06ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5df07000 -     0x7fff5df0efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5df0f000 -     0x7fff5df17ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5df18000 -     0x7fff5df9dfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5e025000 -     0x7fff5e05eff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5e05f000 -     0x7fff5e07cff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5e07d000 -     0x7fff5e07dffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5e08b000 -     0x7fff5e08bff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5e08c000 -     0x7fff5e090ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5e091000 -     0x7fff5e093ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5e094000 -     0x7fff5e095ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5e096000 -     0x7fff5e0adfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5e0ae000 -     0x7fff5e0aefff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5e0af000 -     0x7fff5e138ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5e139000 -     0x7fff5e13cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5e13d000 -     0x7fff5e140ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5e141000 -     0x7fff5e142fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5e143000 -     0x7fff5e149ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5e14a000 -     0x7fff5e193ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5e194000 -     0x7fff5e1b9ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5e1ba000 -     0x7fff5e205fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5e206000 -     0x7fff5e225fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5e226000 -     0x7fff5e2caff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5e2cb000 -     0x7fff5e2d5ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5e2d6000 -     0x7fff5e2dfff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5e2e0000 -     0x7fff5e2e7ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5e2e8000 -     0x7fff5e2f3fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5e2f4000 -     0x7fff5e2f7ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5e2f8000 -     0x7fff5e2f9ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5e2fa000 -     0x7fff5e301ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5e302000 -     0x7fff5e315ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5e317000 -     0x7fff5e31cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5e31d000 -     0x7fff5e349ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2109
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=17.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9260K        8 
MALLOC guard page                   16K        4 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4552K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9656K       44 
===========                     =======  ======= 
TOTAL                            276.1M      108 
TOTAL                            276.1M      108 
TOTAL, minus reserved VM space   275.9M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-19-055337_Traviss-Mac-1044.crash
Process:               a [44060]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44059]
Responsible:           a [44060]
User ID:               501
Date/Time:             2019-06-19 05:53:28.510 +0000
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
0   libsystem_kernel.dylib         0x00007fff5e1afe3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff5e2ee150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff5e10c312 abort + 127
3   libstd-329ef83dbc1dd86d.dylib  0x000000010a3d95e9 std::sys::unix::abort_internal::hb0d87c939ffeb797 + 9
4   libstd-329ef83dbc1dd86d.dylib  0x000000010a3c96c0 rust_oom + 32
5   libstd-329ef83dbc1dd86d.dylib  0x000000010a3ef5a9 alloc::alloc::handle_alloc_error::h4044521f3c8241d3 + 9
6   a                              0x000000010a39c07f default_alloc_error_hook::main::h0fe124586986ad5d + 767
7   a                              0x000000010a39c736 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h417afde3001e038b + 6
8   libstd-329ef83dbc1dd86d.dylib  0x000000010a3ca1d8 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
9   libstd-329ef83dbc1dd86d.dylib  0x000000010a3da0af __rust_maybe_catch_panic + 31
10  libstd-329ef83dbc1dd86d.dylib  0x000000010a3cacbe std::rt::lang_start_internal::hb2916a8a31799db8 + 542
11  a                              0x000000010a39c1d9 main + 41
12  libdyld.dylib                  0x00007fff5e060115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff96d86340  rcx: 0x00007ffee5863348  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee5863380  rsp: 0x00007ffee5863348
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff5e1afe3e  rfl: 0x0000000000000206  cr2: 0x00007fff96d64148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10a39a000 -        0x10a39cfff +a (0) <0A4F8E0E-5AEE-386A-AFF2-393BFC812577> /Users/USER/*/a
       0x10a3a7000 -        0x10a446fff +libstd-329ef83dbc1dd86d.dylib (0) <473334B9-7978-3FCB-AAC2-299F4D113E14> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x1113a2000 -        0x1113ec98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5b8ca000 -     0x7fff5b8fdfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5bddc000 -     0x7fff5bdddff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5c092000 -     0x7fff5c0e8fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5c0e9000 -     0x7fff5c10dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5d45f000 -     0x7fff5d8503b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5d91d000 -     0x7fff5d939ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5def7000 -     0x7fff5defbff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5defc000 -     0x7fff5df06ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5df07000 -     0x7fff5df0efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5df0f000 -     0x7fff5df17ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5df18000 -     0x7fff5df9dfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5e025000 -     0x7fff5e05eff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5e05f000 -     0x7fff5e07cff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5e07d000 -     0x7fff5e07dffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5e08b000 -     0x7fff5e08bff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5e08c000 -     0x7fff5e090ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5e091000 -     0x7fff5e093ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5e094000 -     0x7fff5e095ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5e096000 -     0x7fff5e0adfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5e0ae000 -     0x7fff5e0aefff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5e0af000 -     0x7fff5e138ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5e139000 -     0x7fff5e13cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5e13d000 -     0x7fff5e140ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5e141000 -     0x7fff5e142fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5e143000 -     0x7fff5e149ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5e14a000 -     0x7fff5e193ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5e194000 -     0x7fff5e1b9ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5e1ba000 -     0x7fff5e205fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5e206000 -     0x7fff5e225fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5e226000 -     0x7fff5e2caff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5e2cb000 -     0x7fff5e2d5ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5e2d6000 -     0x7fff5e2dfff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5e2e0000 -     0x7fff5e2e7ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5e2e8000 -     0x7fff5e2f3fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5e2f4000 -     0x7fff5e2f7ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5e2f8000 -     0x7fff5e2f9ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5e2fa000 -     0x7fff5e301ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5e302000 -     0x7fff5e315ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5e317000 -     0x7fff5e31cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5e31d000 -     0x7fff5e349ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
