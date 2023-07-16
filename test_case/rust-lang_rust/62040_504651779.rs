plain
[00:03:22]       Memory: 8 GB
[00:03:22]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:22]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:22]       SMC Version (system): 2.8f0
[00:03:22]       Serial Number (system): VM1RXkIyzfQ5
[00:03:22] 
[00:03:22] hw.ncpu: 4
[00:03:22] hw.byteorder: 1234
[00:03:22] hw.memsize: 8589934592
---
[02:05:52] 
[02:05:52] ---- /Users/travis/build/rust-lang/rust/src/doc/unstable-book/src/language-features/transparent-unions.md - The_tracking_issue_for_this_feature_is_ (line 15) stdout ----
[02:05:52] error: linking with `cc` failed: signal: 4
[02:05:52]   |
[02:05:52]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest4eyneW/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest4eyneW/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest4eyneW/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest4eyneW/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest4eyneW/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest4eyneW/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest4eyneW/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-329ef83dbc1dd86d.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-ddf1f863d2eaf653.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace-a26391313b65d7dc.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-0ad3fa8cdd0b9704.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-4d45b60ed1186ef3.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-b5491b1ffbbc1406.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-4228aae7223b6103.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-99ea320c6f5b5eea.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-6595b4030f6ddb0e.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-d930cd0bdea4797b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-8bde4cd3b4c45eea.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-6450bce6cc789177.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-992f5c26c2d9c590.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-0f189b415a69de4e.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:05:52] 
[02:05:52] error: aborting due to previous error
[02:05:52] 
[02:05:52] Couldn't compile the test.
---
travis_fold:start:after_failure.2
travis_time:start:02d82b18
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Jun 22 10:07 .
-rw-------@  1 travis  staff  13742 Jun 22 10:07 overflow_2019-06-22-100745_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Jun 22 10:07 foo_2019-06-22-100720_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1418 Jun 22 10:06 m4_2019-06-22-100644_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 Jun 22 10:06 b_2019-06-22-100631_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Jun 22 10:06 bar_2019-06-22-100631_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Jun 22 10:06 bar_2019-06-22-100630_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62246 Jun 22 09:19 a_2019-06-22-091924-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 Jun 22 09:19 a_2019-06-22-091924_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37410 Jun 22 09:19 a_2019-06-22-091915-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60386 Jun 22 09:19 a_2019-06-22-091915_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Jun 22 09:19 a_2019-06-22-091908_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Jun 22 09:19 a_2019-06-22-091902_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Jun 22 09:19 a_2019-06-22-091900-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 Jun 22 09:19 a_2019-06-22-091900_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10033 Jun 22 09:18 a_2019-06-22-091822_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63059 Jun 22 09:18 a_2019-06-22-091808_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65081 Jun 22 09:18 a_2019-06-22-091807-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63914 Jun 22 09:18 a_2019-06-22-091807-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64276 Jun 22 09:18 a_2019-06-22-091807_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11712 Jun 22 09:15 a_2019-06-22-091553_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9895 Jun 22 09:15 a_2019-06-22-091501_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Jun 22 09:13 a_2019-06-22-091347_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Jun 22 09:13 a_2019-06-22-091323-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 Jun 22 09:13 a_2019-06-22-091323-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10489 Jun 22 09:13 a_2019-06-22-091323_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:072fd070
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-22-091323-1_Traviss-Mac-1044.crash
Process:               a [42366]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [42361]
Responsible:           a [42366]
User ID:               501
Date/Time:             2019-06-22 09:12:43.303 +0000
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
0   libstd-329ef83dbc1dd86d.dylib  0x000000010d5699de std::panicking::rust_panic_with_hook::h756d0bba11076be5 + 142
1   a                              0x000000010d53b6c5 std::panicking::begin_panic::haff5b930812bcdc4 + 37
2   a                              0x000000010d538d6c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x000000010d538589 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x000000010d538d43 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x000000010d53a2aa backtrace::main::hc9a5bc8fc93ded64 + 5210 (backtrace.rs:119)
6   a                              0x000000010d538276 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc80788536e97b081 + 6 (rt.rs:64)
7   libstd-329ef83dbc1dd86d.dylib  0x000000010d569458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
8   libstd-329ef83dbc1dd86d.dylib  0x000000010d57903f __rust_maybe_catch_panic + 31
9   libstd-329ef83dbc1dd86d.dylib  0x000000010d569f3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
10  a                              0x000000010d53abf9 main + 41
11  libdyld.dylib                  0x00007fff66170115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee26c93e8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010d5b9e42  rbp: 0x00007ffee26c94e0  rsp: 0x00007ffee26c9410
   r8: 0xffffffff00000100   r9: 0x000000010d5edae0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010d5b8c38  r14: 0x000000010d53d480  r15: 0x00007ffee26c94f0
  rip: 0x000000010d5699de  rfl: 0x0000000000010206  cr2: 0x0000000100cca000
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10d534000 -        0x10d53cff7 +a (0) <DE32EEFB-76CF-3561-9EC6-A683DB68D532> /Users/USER/*/a
       0x10d546000 -        0x10d5e5fff +libstd-329ef83dbc1dd86d.dylib (0) <171D9096-1D04-3EF5-AA0F-9CA103E886FD> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x111ac1000 -        0x111b0b98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff639da000 -     0x7fff63a0dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff63eec000 -     0x7fff63eedff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff641a2000 -     0x7fff641f8fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff641f9000 -     0x7fff6421dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6556f000 -     0x7fff659603b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff65a2d000 -     0x7fff65a49ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff66007000 -     0x7fff6600bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6600c000 -     0x7fff66016ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff66017000 -     0x7fff6601efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6601f000 -     0x7fff66027ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff66028000 -     0x7fff660adfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff66135000 -     0x7fff6616eff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6616f000 -     0x7fff6618cff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6618d000 -     0x7fff6618dffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6619b000 -     0x7fff6619bff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6619c000 -     0x7fff661a0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff661a1000 -     0x7fff661a3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff661a4000 -     0x7fff661a5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff661a6000 -     0x7fff661bdfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff661be000 -     0x7fff661befff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff661bf000 -     0x7fff66248ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff66249000 -     0x7fff6624cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6624d000 -     0x7fff66250ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff66251000 -     0x7fff66252fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff66253000 -     0x7fff66259ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6625a000 -     0x7fff662a3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff662a4000 -     0x7fff662c9ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff662ca000 -     0x7fff66315fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff66316000 -     0x7fff66335fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff66336000 -     0x7fff663daff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff663db000 -     0x7fff663e5ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff663e6000 -     0x7fff663efff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff663f0000 -     0x7fff663f7ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff663f8000 -     0x7fff66403fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff66404000 -     0x7fff66407ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff66408000 -     0x7fff66409ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6640a000 -     0x7fff66411ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff66412000 -     0x7fff66425ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff66427000 -     0x7fff6642cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6642d000 -     0x7fff66459ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2080
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-22-091323-2_Traviss-Mac-1044.crash
Process:               a [41586]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41584]
Responsible:           a [41586]
User ID:               501
Date/Time:             2019-06-22 09:12:14.592 +0000
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
0   a                              0x000000010bd507fe abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 30
1   a                              0x000000010bd4fbf9 std::panicking::try::do_call::h279475168e6f9cd6 (.llvm.15361624866771944484) + 9
2   libstd-329ef83dbc1dd86d.dylib  0x000000010bd8c03f __rust_maybe_catch_panic + 31
3   a                              0x000000010bd50a51 abort_on_c_abi::main::he771bf881fc862e3 + 593
4   a                              0x000000010bd4f0d6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hbc930dad328663fe + 6
5   libstd-329ef83dbc1dd86d.dylib  0x000000010bd7c458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
6   libstd-329ef83dbc1dd86d.dylib  0x000000010bd8c03f __rust_maybe_catch_panic + 31
7   libstd-329ef83dbc1dd86d.dylib  0x000000010bd7cf3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
8   a                              0x000000010bd50d59 main + 41
9   libdyld.dylib                  0x00007fff66170115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007f9141500010  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee3eaec78  rsi: 0x00000000ffffffc3  rbp: 0x00007ffee3eaf6d0  rsp: 0x00007ffee3eaf6d0
   r8: 0x0000000014150006   r9: 0x0000000000000004  r10: 0x000000011731b8c0  r11: 0x00007fff6642796c
  r12: 0x000000010c0a0000  r13: 0x0000000000000000  r14: 0x00007ffee3eaf7f0  r15: 0x00007ffee3eaf738
  rip: 0x000000010bd507fe  rfl: 0x0000000000010206  cr2: 0x000000010bdcb998
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10bd4e000 -        0x10bd51fff +a (0) <B9B7BC2A-8B83-375C-A5CD-F87771AAF509> /Users/USER/*/a
       0x10bd59000 -        0x10bdf8fff +libstd-329ef83dbc1dd86d.dylib (0) <171D9096-1D04-3EF5-AA0F-9CA103E886FD> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x1172c9000 -        0x11731398f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff639da000 -     0x7fff63a0dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff63eec000 -     0x7fff63eedff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff641a2000 -     0x7fff641f8fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff641f9000 -     0x7fff6421dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6556f000 -     0x7fff659603b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff65a2d000 -     0x7fff65a49ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff66007000 -     0x7fff6600bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6600c000 -     0x7fff66016ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff66017000 -     0x7fff6601efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6601f000 -     0x7fff66027ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff66028000 -     0x7fff660adfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff66135000 -     0x7fff6616eff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6616f000 -     0x7fff6618cff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6618d000 -     0x7fff6618dffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6619b000 -     0x7fff6619bff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6619c000 -     0x7fff661a0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff661a1000 -     0x7fff661a3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff661a4000 -     0x7fff661a5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff661a6000 -     0x7fff661bdfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff661be000 -     0x7fff661befff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff661bf000 -     0x7fff66248ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff66249000 -     0x7fff6624cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6624d000 -     0x7fff66250ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff66251000 -     0x7fff66252fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff66253000 -     0x7fff66259ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6625a000 -     0x7fff662a3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff662a4000 -     0x7fff662c9ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff662ca000 -     0x7fff66315fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff66316000 -     0x7fff66335fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff66336000 -     0x7fff663daff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff663db000 -     0x7fff663e5ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff663e6000 -     0x7fff663efff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff663f0000 -     0x7fff663f7ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff663f8000 -     0x7fff66403fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff66404000 -     0x7fff66407ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff66408000 -     0x7fff66409ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6640a000 -     0x7fff66411ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff66412000 -     0x7fff66425ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff66427000 -     0x7fff6642cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6642d000 -     0x7fff66459ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2080
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
__TEXT                            9656K       44 
===========                     =======  ======= 
TOTAL                            277.1M      109 
TOTAL                            277.1M      109 
TOTAL, minus reserved VM space   276.9M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-22-091323_Traviss-Mac-1044.crash
Process:               a [42365]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        a [42361]
Responsible:           a [42365]
User ID:               501
Date/Time:             2019-06-22 09:12:43.282 +0000
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
0   libstd-329ef83dbc1dd86d.dylib  0x000000010a7fe9de std::panicking::rust_panic_with_hook::h756d0bba11076be5 + 142
1   a                              0x000000010a7ce6c5 std::panicking::begin_panic::haff5b930812bcdc4 + 37
2   a                              0x000000010a7cbd6c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x000000010a7cb589 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x000000010a7cbd43 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x000000010a7cd2aa backtrace::main::hc9a5bc8fc93ded64 + 5210 (backtrace.rs:119)
6   a                              0x000000010a7cb276 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc80788536e97b081 + 6 (rt.rs:64)
7   libstd-329ef83dbc1dd86d.dylib  0x000000010a7fe458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
8   libstd-329ef83dbc1dd86d.dylib  0x000000010a80e03f __rust_maybe_catch_panic + 31
9   libstd-329ef83dbc1dd86d.dylib  0x000000010a7fef3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
10  a                              0x000000010a7cdbf9 main + 41
11  libdyld.dylib                  0x00007fff66170115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee5436408  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010a84ee42  rbp: 0x00007ffee5436500  rsp: 0x00007ffee5436430
   r8: 0xffffffff00000100   r9: 0x000000010a882ae0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010a84dc38  r14: 0x000000010a7d0480  r15: 0x00007ffee5436510
  rip: 0x000000010a7fe9de  rfl: 0x0000000000010202  cr2: 0x00007fc82840f2ce
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10a7c7000 -        0x10a7cfff7 +a (0) <DE32EEFB-76CF-3561-9EC6-A683DB68D532> /Users/USER/*/a
       0x10a7db000 -        0x10a87afff +libstd-329ef83dbc1dd86d.dylib (0) <171D9096-1D04-3EF5-AA0F-9CA103E886FD> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x1163ff000 -        0x11644998f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff639da000 -     0x7fff63a0dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff63eec000 -     0x7fff63eedff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff641a2000 -     0x7fff641f8fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff641f9000 -     0x7fff6421dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6556f000 -     0x7fff659603b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff65a2d000 -     0x7fff65a49ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff66007000 -     0x7fff6600bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6600c000 -     0x7fff66016ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff66017000 -     0x7fff6601efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6601f000 -     0x7fff66027ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff66028000 -     0x7fff660adfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff66135000 -     0x7fff6616eff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6616f000 -     0x7fff6618cff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6618d000 -     0x7fff6618dffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6619b000 -     0x7fff6619bff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6619c000 -     0x7fff661a0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff661a1000 -     0x7fff661a3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff661a4000 -     0x7fff661a5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff661a6000 -     0x7fff661bdfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff661be000 -     0x7fff661befff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff661bf000 -     0x7fff66248ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff66249000 -     0x7fff6624cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6624d000 -     0x7fff66250ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff66251000 -     0x7fff66252fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff66253000 -     0x7fff66259ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6625a000 -     0x7fff662a3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff662a4000 -     0x7fff662c9ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff662ca000 -     0x7fff66315fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff66316000 -     0x7fff66335fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff66336000 -     0x7fff663daff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff663db000 -     0x7fff663e5ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff663e6000 -     0x7fff663efff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff663f0000 -     0x7fff663f7ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff663f8000 -     0x7fff66403fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff66404000 -     0x7fff66407ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff66408000 -     0x7fff66409ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6640a000 -     0x7fff66411ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff66412000 -     0x7fff66425ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff66427000 -     0x7fff6642cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6642d000 -     0x7fff66459ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2080
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-22-091347_Traviss-Mac-1044.crash
Process:               a [44095]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44094]
Responsible:           a [44095]
User ID:               501
Date/Time:             2019-06-22 09:13:45.431 +0000
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
0   libsystem_kernel.dylib         0x00007fff662bfe3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff663fe150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff6621c312 abort + 127
3   libstd-329ef83dbc1dd86d.dylib  0x000000010e4b0579 std::sys::unix::abort_internal::hb0d87c939ffeb797 + 9
4   libstd-329ef83dbc1dd86d.dylib  0x000000010e4a0900 rust_oom + 32
5   libstd-329ef83dbc1dd86d.dylib  0x000000010e4c6539 alloc::alloc::handle_alloc_error::h4044521f3c8241d3 + 9
6   a                              0x000000010e47407f default_alloc_error_hook::main::h0fe124586986ad5d + 767
7   a                              0x000000010e474736 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h417afde3001e038b + 6
8   libstd-329ef83dbc1dd86d.dylib  0x000000010e4a1458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
9   libstd-329ef83dbc1dd86d.dylib  0x000000010e4b103f __rust_maybe_catch_panic + 31
10  libstd-329ef83dbc1dd86d.dylib  0x000000010e4a1f3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
11  a                              0x000000010e4741d9 main + 41
12  libdyld.dylib                  0x00007fff66170115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff9ee96340  rcx: 0x00007ffee178b648  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee178b680  rsp: 0x00007ffee178b648
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff662bfe3e  rfl: 0x0000000000000206  cr2: 0x00007fff9ee74148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10e472000 -        0x10e474fff +a (0) <0A4F8E0E-5AEE-386A-AFF2-393BFC812577> /Users/USER/*/a
       0x10e47e000 -        0x10e51dfff +libstd-329ef83dbc1dd86d.dylib (0) <171D9096-1D04-3EF5-AA0F-9CA103E886FD> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x11968b000 -        0x1196d598f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff639da000 -     0x7fff63a0dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff63eec000 -     0x7fff63eedff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff641a2000 -     0x7fff641f8fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff641f9000 -     0x7fff6421dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6556f000 -     0x7fff659603b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff65a2d000 -     0x7fff65a49ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff66007000 -     0x7fff6600bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6600c000 -     0x7fff66016ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff66017000 -     0x7fff6601efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6601f000 -     0x7fff66027ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff66028000 -     0x7fff660adfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff66135000 -     0x7fff6616eff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6616f000 -     0x7fff6618cff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6618d000 -     0x7fff6618dffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6619b000 -     0x7fff6619bff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6619c000 -     0x7fff661a0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff661a1000 -     0x7fff661a3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff661a4000 -     0x7fff661a5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff661a6000 -     0x7fff661bdfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff661be000 -     0x7fff661befff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff661bf000 -     0x7fff66248ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff66249000 -     0x7fff6624cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6624d000 -     0x7fff66250ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff66251000 -     0x7fff66252fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff66253000 -     0x7fff66259ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6625a000 -     0x7fff662a3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff662a4000 -     0x7fff662c9ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff662ca000 -     0x7fff66315fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff66316000 -     0x7fff66335fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff66336000 -     0x7fff663daff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff663db000 -     0x7fff663e5ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff663e6000 -     0x7fff663efff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff663f0000 -     0x7fff663f7ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff663f8000 -     0x7fff66403fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff66404000 -     0x7fff66407ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff66408000 -     0x7fff66409ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6640a000 -     0x7fff66411ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff66412000 -     0x7fff66425ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff66427000 -     0x7fff6642cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6642d000 -     0x7fff66459ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
