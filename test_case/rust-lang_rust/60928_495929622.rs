plain
[00:03:21]       Memory: 8 GB
[00:03:21]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:21]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:21]       SMC Version (system): 2.8f0
[00:03:21]       Serial Number (system): VMoImQQ+nRQu
[00:03:21] 
[00:03:21] hw.ncpu: 4
[00:03:21] hw.byteorder: 1234
[00:03:21] hw.memsize: 8589934592
---
[02:08:42]   |
[02:08:42] 6 |     5.0 => {},
[02:08:42]   |     ^^^
[02:08:42]   |
[02:08:42]   = note: #[warn(illegal_floating_point_literal_pattern)] on by default
[02:08:42]   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[02:08:42] 
[02:08:42] warning: floating-point types cannot be used in patterns
[02:08:42]  --> /Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/listing/warn-by-default.md:79:5
[02:08:42]   |
---
[02:08:42]   = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>
[02:08:42] 
[02:08:42] error: linking with `cc` failed: signal: 4
[02:08:42]   |
[02:08:42]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest3y3fZ3/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest3y3fZ3/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest3y3fZ3/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest3y3fZ3/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest3y3fZ3/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest3y3fZ3/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest3y3fZ3/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-27347a76819fc551.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-6e0bbbf4ad4d6624.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-1d1ab7ff67a05a37.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-d2cde691f13b7682.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-6214c18b57987aa0.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-3a30c974a3950bba.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-1e528aa314767a87.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-37c5ecf7a0816003.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-636bdc03b519d720.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-13624d4e52f9bfd1.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-929a815c3d2ded1a.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-402967aff5bd2938.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:08:42] 
[02:08:42] error: aborting due to previous error
[02:08:42] 
[02:08:42] thread '/Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/listing/warn-by-default.md - Warn_by_default_lints::illegal_floating_point_literal_pattern (line 75)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
---
[02:08:42] 
[02:08:42] 
[02:08:42] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:08:42] Build completed unsuccessfully in 0:51:53
[02:08:42] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d8c8106
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat May 25 16:06:04 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:1a5a1c1a
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 May 25 16:05 .
-rw-------@  1 travis  staff  13742 May 25 16:05 overflow_2019-05-25-160546_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 May 25 16:05 foo_2019-05-25-160525_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1417 May 25 16:04 m4_2019-05-25-160453_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1431 May 25 16:04 bar_2019-05-25-160443-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 May 25 16:04 bar_2019-05-25-160443_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 May 25 16:04 b_2019-05-25-160442_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 May 25 15:31 a_2019-05-25-153115-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62246 May 25 15:31 a_2019-05-25-153115_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37411 May 25 15:31 a_2019-05-25-153102-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60389 May 25 15:31 a_2019-05-25-153102_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 May 25 15:30 a_2019-05-25-153059_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 May 25 15:30 a_2019-05-25-153050_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 May 25 15:30 a_2019-05-25-153048-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 May 25 15:30 a_2019-05-25-153048_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 May 25 15:30 a_2019-05-25-153009_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63111 May 25 15:29 a_2019-05-25-152958_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65082 May 25 15:29 a_2019-05-25-152954-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 May 25 15:29 a_2019-05-25-152954-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64224 May 25 15:29 a_2019-05-25-152954_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11713 May 25 15:27 a_2019-05-25-152749_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 May 25 15:26 a_2019-05-25-152652_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 May 25 15:25 a_2019-05-25-152553-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10489 May 25 15:25 a_2019-05-25-152553-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10213 May 25 15:25 a_2019-05-25-152553-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 May 25 15:25 a_2019-05-25-152553_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:24494248
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-152553-1_Traviss-Mac-1044.crash
Process:               a [45360]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45355]
Responsible:           a [45360]
User ID:               501
Date/Time:             2019-05-25 15:24:32.877 +0000
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
0   libstd-27347a76819fc551.dylib  0x000000010e402cde std::panicking::rust_panic_with_hook::h7c6313598a9b0dc3 + 142
1   a                              0x000000010e0f08e5 std::panicking::begin_panic::h5572456cacbcfa44 + 37
2   a                              0x000000010e0ee40c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x000000010e0edef9 core::ptr::real_drop_in_place::he9e7b49e72e17b2e + 9
4   a                              0x000000010e0ee3e3 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x000000010e0ef559 backtrace::main::hc9a5bc8fc93ded64 + 4201 (backtrace.rs:104)
6   a                              0x000000010e0ed936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h00b0f061c0c74972 + 6 (rt.rs:64)
7   libstd-27347a76819fc551.dylib  0x000000010e402758 std::panicking::try::do_call::hf57ca92cee898b14 + 24
8   libstd-27347a76819fc551.dylib  0x000000010e4129ef __rust_maybe_catch_panic + 31
9   libstd-27347a76819fc551.dylib  0x000000010e40323e std::rt::lang_start_internal::h60877bbdab26e9ca + 542
10  a                              0x000000010e0efd99 main + 41
11  libdyld.dylib                  0x00007fff6cfc7115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee1b13368  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010e44ae75  rbp: 0x00007ffee1b13460  rsp: 0x00007ffee1b13390
   r8: 0x0000000000000100   r9: 0x000000010e47d980  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010e449c88  r14: 0x000000010e0f2460  r15: 0x00007ffee1b13470
  rip: 0x000000010e402cde  rfl: 0x0000000000010202  cr2: 0x000000010feb4029
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10e0ea000 -        0x10e0f1fff +a (0) <644AC2F5-4276-3A43-B086-34CD9A58793B> /Users/USER/*/a
       0x10e340000 -        0x10e38a98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
       0x10e3de000 -        0x10e475ff7 +libstd-27347a76819fc551.dylib (0) <F3CF3E30-9F12-34C2-86CC-3E0437C9F1C3> /Users/USER/*/libstd-27347a76819fc551.dylib
    0x7fff6a831000 -     0x7fff6a864fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6ad43000 -     0x7fff6ad44ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6aff9000 -     0x7fff6b04ffff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6b050000 -     0x7fff6b074ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6c3c6000 -     0x7fff6c7b73b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6c884000 -     0x7fff6c8a0ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6ce5e000 -     0x7fff6ce62ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6ce63000 -     0x7fff6ce6dff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6ce6e000 -     0x7fff6ce75fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6ce76000 -     0x7fff6ce7effb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6ce7f000 -     0x7fff6cf04fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6cf8c000 -     0x7fff6cfc5ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6cfc6000 -     0x7fff6cfe3ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6cfe4000 -     0x7fff6cfe4ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6cff2000 -     0x7fff6cff2ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6cff3000 -     0x7fff6cff7ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6cff8000 -     0x7fff6cffaff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6cffb000 -     0x7fff6cffcff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6cffd000 -     0x7fff6d014fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6d015000 -     0x7fff6d015fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6d016000 -     0x7fff6d09fff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6d0a0000 -     0x7fff6d0a3ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6d0a4000 -     0x7fff6d0a7ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6d0a8000 -     0x7fff6d0a9fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6d0aa000 -     0x7fff6d0b0ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6d0b1000 -     0x7fff6d0faff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6d0fb000 -     0x7fff6d120ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6d121000 -     0x7fff6d16cfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6d16d000 -     0x7fff6d18cfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6d18d000 -     0x7fff6d231ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6d232000 -     0x7fff6d23cffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6d23d000 -     0x7fff6d246ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6d247000 -     0x7fff6d24eff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6d24f000 -     0x7fff6d25afff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6d25b000 -     0x7fff6d25eff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6d25f000 -     0x7fff6d260ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6d261000 -     0x7fff6d268ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6d269000 -     0x7fff6d27cff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6d27e000 -     0x7fff6d283ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6d284000 -     0x7fff6d2b0ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2590
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-152553-2_Traviss-Mac-1044.crash
Process:               a [45359]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        a [45355]
Responsible:           a [45359]
User ID:               501
Date/Time:             2019-05-25 15:24:32.832 +0000
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
0   libstd-27347a76819fc551.dylib  0x000000010f521cde std::panicking::rust_panic_with_hook::h7c6313598a9b0dc3 + 142
1   a                              0x000000010f4f18e5 std::panicking::begin_panic::h5572456cacbcfa44 + 37
2   a                              0x000000010f4ef40c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x000000010f4eeef9 core::ptr::real_drop_in_place::he9e7b49e72e17b2e + 9
4   a                              0x000000010f4ef3e3 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x000000010f4f0559 backtrace::main::hc9a5bc8fc93ded64 + 4201 (backtrace.rs:104)
6   a                              0x000000010f4ee936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h00b0f061c0c74972 + 6 (rt.rs:64)
7   libstd-27347a76819fc551.dylib  0x000000010f521758 std::panicking::try::do_call::hf57ca92cee898b14 + 24
8   libstd-27347a76819fc551.dylib  0x000000010f5319ef __rust_maybe_catch_panic + 31
9   libstd-27347a76819fc551.dylib  0x000000010f52223e std::rt::lang_start_internal::h60877bbdab26e9ca + 542
10  a                              0x000000010f4f0d99 main + 41
11  libdyld.dylib                  0x00007fff6cfc7115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee0712388  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010f569e75  rbp: 0x00007ffee0712480  rsp: 0x00007ffee07123b0
   r8: 0x0000000000000100   r9: 0x000000010f59c980  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010f568c88  r14: 0x000000010f4f3460  r15: 0x00007ffee0712490
  rip: 0x000000010f521cde  rfl: 0x0000000000010206  cr2: 0x000000010f7f15b9
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10f4eb000 -        0x10f4f2fff +a (0) <644AC2F5-4276-3A43-B086-34CD9A58793B> /Users/USER/*/a
       0x10f4fd000 -        0x10f594ff7 +libstd-27347a76819fc551.dylib (0) <F3CF3E30-9F12-34C2-86CC-3E0437C9F1C3> /Users/USER/*/libstd-27347a76819fc551.dylib
       0x11f315000 -        0x11f35f98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6a831000 -     0x7fff6a864fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6ad43000 -     0x7fff6ad44ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6aff9000 -     0x7fff6b04ffff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6b050000 -     0x7fff6b074ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6c3c6000 -     0x7fff6c7b73b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6c884000 -     0x7fff6c8a0ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6ce5e000 -     0x7fff6ce62ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6ce63000 -     0x7fff6ce6dff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6ce6e000 -     0x7fff6ce75fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6ce76000 -     0x7fff6ce7effb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6ce7f000 -     0x7fff6cf04fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6cf8c000 -     0x7fff6cfc5ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6cfc6000 -     0x7fff6cfe3ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6cfe4000 -     0x7fff6cfe4ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6cff2000 -     0x7fff6cff2ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6cff3000 -     0x7fff6cff7ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6cff8000 -     0x7fff6cffaff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6cffb000 -     0x7fff6cffcff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6cffd000 -     0x7fff6d014fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6d015000 -     0x7fff6d015fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6d016000 -     0x7fff6d09fff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6d0a0000 -     0x7fff6d0a3ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6d0a4000 -     0x7fff6d0a7ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6d0a8000 -     0x7fff6d0a9fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6d0aa000 -     0x7fff6d0b0ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6d0b1000 -     0x7fff6d0faff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6d0fb000 -     0x7fff6d120ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6d121000 -     0x7fff6d16cfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6d16d000 -     0x7fff6d18cfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6d18d000 -     0x7fff6d231ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6d232000 -     0x7fff6d23cffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6d23d000 -     0x7fff6d246ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6d247000 -     0x7fff6d24eff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6d24f000 -     0x7fff6d25afff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6d25b000 -     0x7fff6d25eff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6d25f000 -     0x7fff6d260ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6d261000 -     0x7fff6d268ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6d269000 -     0x7fff6d27cff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6d27e000 -     0x7fff6d283ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6d284000 -     0x7fff6d2b0ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2590
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
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4536K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            285.4M      110 
TOTAL                            285.4M      110 
TOTAL, minus reserved VM space   285.3M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-152553-3_Traviss-Mac-1044.crash
Process:               a [44584]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44580]
Responsible:           a [44584]
User ID:               501
Date/Time:             2019-05-25 15:24:05.292 +0000
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
0   a                              0x00000001053718ae abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 30
1   a                              0x0000000105370ca9 std::panicking::try::do_call::hdacb85ea041c28a1 (.llvm.6251442833206298123) + 9
2   libstd-27347a76819fc551.dylib  0x00000001053b09ef __rust_maybe_catch_panic + 31
3   a                              0x0000000105371b01 abort_on_c_abi::main::he771bf881fc862e3 + 593
4   a                              0x00000001053700f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h596d4e0b24ed245f + 6
5   libstd-27347a76819fc551.dylib  0x00000001053a0758 std::panicking::try::do_call::hf57ca92cee898b14 + 24
6   libstd-27347a76819fc551.dylib  0x00000001053b09ef __rust_maybe_catch_panic + 31
7   libstd-27347a76819fc551.dylib  0x00000001053a123e std::rt::lang_start_internal::h60877bbdab26e9ca + 542
8   a                              0x0000000105371e09 main + 41
9   libdyld.dylib                  0x00007fff6cfc7115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007f9513402b90  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffeea88dbc8  rsi: 0x00000000c3ffffff  rbp: 0x00007ffeea88e620  rsp: 0x00007ffeea88e620
   r8: 0x00000000513402be   r9: 0x0000000000000004  r10: 0x00000001060198c0  r11: 0x00007fff6d27e96c
  r12: 0x00000001056ae000  r13: 0x0000000000000000  r14: 0x00007ffeea88e740  r15: 0x00007ffeea88e688
  rip: 0x00000001053718ae  rfl: 0x0000000000010202  cr2: 0x00000001053e7a74
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10536f000 -        0x105372ff7 +a (0) <79069807-50AD-372F-8685-DBE65D27E82D> /Users/USER/*/a
       0x10537c000 -        0x105413ff7 +libstd-27347a76819fc551.dylib (0) <F3CF3E30-9F12-34C2-86CC-3E0437C9F1C3> /Users/USER/*/libstd-27347a76819fc551.dylib
       0x105fc7000 -        0x10601198f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6a831000 -     0x7fff6a864fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6ad43000 -     0x7fff6ad44ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6aff9000 -     0x7fff6b04ffff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6b050000 -     0x7fff6b074ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6c3c6000 -     0x7fff6c7b73b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6c884000 -     0x7fff6c8a0ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6ce5e000 -     0x7fff6ce62ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6ce63000 -     0x7fff6ce6dff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6ce6e000 -     0x7fff6ce75fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6ce76000 -     0x7fff6ce7effb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6ce7f000 -     0x7fff6cf04fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6cf8c000 -     0x7fff6cfc5ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6cfc6000 -     0x7fff6cfe3ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6cfe4000 -     0x7fff6cfe4ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6cff2000 -     0x7fff6cff2ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6cff3000 -     0x7fff6cff7ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6cff8000 -     0x7fff6cffaff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6cffb000 -     0x7fff6cffcff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6cffd000 -     0x7fff6d014fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6d015000 -     0x7fff6d015fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6d016000 -     0x7fff6d09fff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6d0a0000 -     0x7fff6d0a3ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6d0a4000 -     0x7fff6d0a7ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6d0a8000 -     0x7fff6d0a9fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6d0aa000 -     0x7fff6d0b0ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6d0b1000 -     0x7fff6d0faff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6d0fb000 -     0x7fff6d120ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6d121000 -     0x7fff6d16cfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6d16d000 -     0x7fff6d18cfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6d18d000 -     0x7fff6d231ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6d232000 -     0x7fff6d23cffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6d23d000 -     0x7fff6d246ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6d247000 -     0x7fff6d24eff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6d24f000 -     0x7fff6d25afff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6d25b000 -     0x7fff6d25eff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6d25f000 -     0x7fff6d260ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6d261000 -     0x7fff6d268ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6d269000 -     0x7fff6d27cff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6d27e000 -     0x7fff6d283ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6d284000 -     0x7fff6d2b0ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2590
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
__DATA                            4536K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            276.0M      108 
TOTAL                            276.0M      108 
TOTAL, minus reserved VM space   275.9M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-152553_Traviss-Mac-1044.crash
Process:               a [47078]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [47077]
Responsible:           a [47078]
User ID:               501
Date/Time:             2019-05-25 15:25:34.702 +0000
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
0   libsystem_kernel.dylib         0x00007fff6d116e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff6d255150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff6d073312 abort + 127
3   libstd-27347a76819fc551.dylib  0x000000010d7deed9 std::sys::unix::abort_internal::h04584ee5534c2bae + 9
4   libstd-27347a76819fc551.dylib  0x000000010d7ced70 rust_oom + 32
5   libstd-27347a76819fc551.dylib  0x000000010d7f01e9 alloc::alloc::handle_alloc_error::h6fd7e6b2858c3c1b + 9
6   a                              0x000000010d7a440f default_alloc_error_hook::main::h0fe124586986ad5d + 767
7   a                              0x000000010d7a3ba6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h456c5f09ccbafab7 + 6
8   libstd-27347a76819fc551.dylib  0x000000010d7cf758 std::panicking::try::do_call::hf57ca92cee898b14 + 24
9   libstd-27347a76819fc551.dylib  0x000000010d7df9ef __rust_maybe_catch_panic + 31
10  libstd-27347a76819fc551.dylib  0x000000010d7d023e std::rt::lang_start_internal::h60877bbdab26e9ca + 542
11  a                              0x000000010d7a4569 main + 41
12  libdyld.dylib                  0x00007fff6cfc7115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fffa5ced340  rcx: 0x00007ffee245b588  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee245b5c0  rsp: 0x00007ffee245b588
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff6d116e3e  rfl: 0x0000000000000206  cr2: 0x00007fffa5ccb148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10d7a2000 -        0x10d7a4ff7 +a (0) <76333FAE-50A8-3073-98BC-02662F3AE746> /Users/USER/*/a
       0x10d7ab000 -        0x10d842ff7 +libstd-27347a76819fc551.dylib (0) <F3CF3E30-9F12-34C2-86CC-3E0437C9F1C3> /Users/USER/*/libstd-27347a76819fc551.dylib
       0x115f67000 -        0x115fb198f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6a831000 -     0x7fff6a864fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6ad43000 -     0x7fff6ad44ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6aff9000 -     0x7fff6b04ffff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6b050000 -     0x7fff6b074ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6c3c6000 -     0x7fff6c7b73b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6c884000 -     0x7fff6c8a0ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6ce5e000 -     0x7fff6ce62ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6ce63000 -     0x7fff6ce6dff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6ce6e000 -     0x7fff6ce75fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6ce76000 -     0x7fff6ce7effb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6ce7f000 -     0x7fff6cf04fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6cf8c000 -     0x7fff6cfc5ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6cfc6000 -     0x7fff6cfe3ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6cfe4000 -     0x7fff6cfe4ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6cff2000 -     0x7fff6cff2ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6cff3000 -     0x7fff6cff7ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6cff8000 -     0x7fff6cffaff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6cffb000 -     0x7fff6cffcff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6cffd000 -     0x7fff6d014fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6d015000 -     0x7fff6d015fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6d016000 -     0x7fff6d09fff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6d0a0000 -     0x7fff6d0a3ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6d0a4000 -     0x7fff6d0a7ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6d0a8000 -     0x7fff6d0a9fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6d0aa000 -     0x7fff6d0b0ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6d0b1000 -     0x7fff6d0faff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6d0fb000 -     0x7fff6d120ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6d121000 -     0x7fff6d16cfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6d16d000 -     0x7fff6d18cfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6d18d000 -     0x7fff6d231ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6d232000 -     0x7fff6d23cffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6d23d000 -     0x7fff6d246ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6d247000 -     0x7fff6d24eff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6d24f000 -     0x7fff6d25afff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6d25b000 -     0x7fff6d25eff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6d25f000 -     0x7fff6d260ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6d261000 -     0x7fff6d268ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6d269000 -     0x7fff6d27cff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6d27e000 -     0x7fff6d283ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6d284000 -     0x7fff6d2b0ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
