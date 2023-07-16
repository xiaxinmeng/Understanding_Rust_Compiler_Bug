plain
[00:03:42]       Memory: 8 GB
[00:03:42]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:42]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:42]       SMC Version (system): 2.8f0
[00:03:42]       Serial Number (system): VMcOLr9d/RAr
[00:03:42] 
[00:03:43] hw.ncpu: 4
[00:03:43] hw.byteorder: 1234
[00:03:43] hw.memsize: 8589934592
---
[02:09:05] 
[02:09:05] ---- /Users/travis/build/rust-lang/rust/src/doc/unstable-book/src/language-features/asm.md - The_tracking_issue_for_this_feature_is__::Assembly_template (line 33) stdout ----
[02:09:05] error: linking with `cc` failed: signal: 4
[02:09:05]   |
[02:09:05]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestOANZX0/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestOANZX0/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestOANZX0/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestOANZX0/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestOANZX0/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestOANZX0/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestOANZX0/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-a20dac67422e025f.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-9f32e784311ef33a.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace-a264787c19fe31ff.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-c347686a4174c4db.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-9138e3d844afdefd.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-9237f8c591007519.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-27f3e12e625b8940.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-3531405ede979d40.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-60fd4dc4fcb3504e.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-d930cd0bdea4797b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-4dc4d3ebda24b0dc.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-6450bce6cc789177.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-992f5c26c2d9c590.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-13932ce48fdbd514.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:09:05] 
[02:09:05] error: aborting due to previous error
[02:09:05] 
[02:09:05] Couldn't compile the test.
---
travis_fold:start:after_failure.2
travis_time:start:2a88a47a
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 May 31 22:14 .
-rw-------@  1 travis  staff  13742 May 31 22:14 overflow_2019-05-31-221420_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 May 31 22:13 foo_2019-05-31-221358_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1417 May 31 22:13 m4_2019-05-31-221321_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 May 31 22:13 b_2019-05-31-221310_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 May 31 22:13 bar_2019-05-31-221310-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 May 31 22:13 bar_2019-05-31-221310_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62244 May 31 21:23 a_2019-05-31-212356-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 May 31 21:23 a_2019-05-31-212356_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60387 May 31 21:23 a_2019-05-31-212346-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37411 May 31 21:23 a_2019-05-31-212346_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 May 31 21:23 a_2019-05-31-212342_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 May 31 21:23 a_2019-05-31-212335_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 May 31 21:23 a_2019-05-31-212333_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 May 31 21:23 a_2019-05-31-212331_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 May 31 21:22 a_2019-05-31-212253_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65082 May 31 21:22 a_2019-05-31-212241-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63060 May 31 21:22 a_2019-05-31-212241-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64224 May 31 21:22 a_2019-05-31-212241-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 May 31 21:22 a_2019-05-31-212241_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11713 May 31 21:20 a_2019-05-31-212026_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 May 31 21:19 a_2019-05-31-211934_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 May 31 21:18 a_2019-05-31-211820-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 May 31 21:18 a_2019-05-31-211820-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 May 31 21:18 a_2019-05-31-211820-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10489 May 31 21:18 a_2019-05-31-211820_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:00b2247e
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-31-211820-1_Traviss-Mac-1044.crash
Process:               a [43849]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [43846]
Responsible:           a [43849]
User ID:               501
Date/Time:             2019-05-31 21:18:17.364 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff736dbe3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff7381a150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff73638312 abort + 127
3   libstd-a20dac67422e025f.dylib  0x00000001066945f9 std::sys::unix::abort_internal::ha8c94e6dd6135a83 + 9
4   libstd-a20dac67422e025f.dylib  0x00000001066846a0 rust_oom + 32
5   libstd-a20dac67422e025f.dylib  0x00000001066aa529 alloc::alloc::handle_alloc_error::ha97ebff8f1c02847 + 9
6   a                              0x000000010665d5ef default_alloc_error_hook::main::h0fe124586986ad5d + 767
7   a                              0x000000010665cdf6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h6ffa1a75254e7b33 + 6
8   libstd-a20dac67422e025f.dylib  0x00000001066851b8 std::panicking::try::do_call::hd4cf110fedfe114a + 24
9   libstd-a20dac67422e025f.dylib  0x00000001066950bf __rust_maybe_catch_panic + 31
10  libstd-a20dac67422e025f.dylib  0x0000000106685c9e std::rt::lang_start_internal::hc3d05ecc1e0ed4e0 + 542
11  a                              0x000000010665d749 main + 41
12  libdyld.dylib                  0x00007fff7358c115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fffac2b2340  rcx: 0x00007ffee95a2478  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee95a24b0  rsp: 0x00007ffee95a2478
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff736dbe3e  rfl: 0x0000000000000206  cr2: 0x00007fffac290148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10665b000 -        0x10665dff7 +a (0) <D6482C5C-40DC-32F8-B0F1-64CAF4757EB8> /Users/USER/*/a
       0x106662000 -        0x1066fdff7 +libstd-a20dac67422e025f.dylib (0) <ADD7B58C-431A-314A-9E69-EC602179831B> /Users/USER/*/libstd-a20dac67422e025f.dylib
       0x110c5b000 -        0x110ca598f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff70df6000 -     0x7fff70e29fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff71308000 -     0x7fff71309ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff715be000 -     0x7fff71614fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff71615000 -     0x7fff71639ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff7298b000 -     0x7fff72d7c3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff72e49000 -     0x7fff72e65ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff73423000 -     0x7fff73427ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff73428000 -     0x7fff73432ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff73433000 -     0x7fff7343afff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7343b000 -     0x7fff73443ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff73444000 -     0x7fff734c9fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff73551000 -     0x7fff7358aff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7358b000 -     0x7fff735a8ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff735a9000 -     0x7fff735a9ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff735b7000 -     0x7fff735b7ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff735b8000 -     0x7fff735bcffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff735bd000 -     0x7fff735bfff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff735c0000 -     0x7fff735c1ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff735c2000 -     0x7fff735d9fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff735da000 -     0x7fff735dafff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff735db000 -     0x7fff73664ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff73665000 -     0x7fff73668ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff73669000 -     0x7fff7366cffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7366d000 -     0x7fff7366efff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7366f000 -     0x7fff73675ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff73676000 -     0x7fff736bfff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff736c0000 -     0x7fff736e5ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff736e6000 -     0x7fff73731fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff73732000 -     0x7fff73751fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff73752000 -     0x7fff737f6ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff737f7000 -     0x7fff73801ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff73802000 -     0x7fff7380bff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7380c000 -     0x7fff73813ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff73814000 -     0x7fff7381ffff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff73820000 -     0x7fff73823ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff73824000 -     0x7fff73825ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff73826000 -     0x7fff7382dff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7382e000 -     0x7fff73841ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff73843000 -     0x7fff73848ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff73849000 -     0x7fff73875ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2082
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
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4520K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9636K       44 
===========                     =======  ======= 
TOTAL                            276.0M      109 
TOTAL                            276.0M      109 
TOTAL, minus reserved VM space   275.9M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-31-211820-2_Traviss-Mac-1044.crash
Process:               a [42131]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [42126]
Responsible:           a [42131]
User ID:               501
Date/Time:             2019-05-31 21:17:14.835 +0000
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
0   libstd-a20dac67422e025f.dylib  0x000000010320b73e std::panicking::rust_panic_with_hook::h6bd005e43362900c + 142
1   a                              0x00000001031db6d5 std::panicking::begin_panic::h001d52deddc12f24 + 37
2   a                              0x00000001031d8d9c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x00000001031d8379 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x00000001031d8d73 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x00000001031da2ba backtrace::main::hc9a5bc8fc93ded64 + 5178 (backtrace.rs:119)
6   a                              0x00000001031d82a6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha6addb1b69a9df14 + 6 (rt.rs:64)
7   libstd-a20dac67422e025f.dylib  0x000000010320b1b8 std::panicking::try::do_call::hd4cf110fedfe114a + 24
8   libstd-a20dac67422e025f.dylib  0x000000010321b0bf __rust_maybe_catch_panic + 31
9   libstd-a20dac67422e025f.dylib  0x000000010320bc9e std::rt::lang_start_internal::hc3d05ecc1e0ed4e0 + 542
10  a                              0x00000001031dac09 main + 41
11  libdyld.dylib                  0x00007fff7358c115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeeca29228  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000103258145  rbp: 0x00007ffeeca29320  rsp: 0x00007ffeeca29250
   r8: 0x0000000000000100   r9: 0x000000010328ba40  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000103256f98  r14: 0x00000001031dd480  r15: 0x00007ffeeca29330
  rip: 0x000000010320b73e  rfl: 0x0000000000010202  cr2: 0x000000010324bba0
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1031d4000 -        0x1031dcfff +a (0) <60194EDB-25AE-3427-8A90-703ECF4A1D73> /Users/USER/*/a
       0x1031e8000 -        0x103283ff7 +libstd-a20dac67422e025f.dylib (0) <ADD7B58C-431A-314A-9E69-EC602179831B> /Users/USER/*/libstd-a20dac67422e025f.dylib
       0x10b689000 -        0x10b6d398f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff70df6000 -     0x7fff70e29fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff71308000 -     0x7fff71309ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff715be000 -     0x7fff71614fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff71615000 -     0x7fff71639ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff7298b000 -     0x7fff72d7c3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff72e49000 -     0x7fff72e65ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff73423000 -     0x7fff73427ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff73428000 -     0x7fff73432ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff73433000 -     0x7fff7343afff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7343b000 -     0x7fff73443ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff73444000 -     0x7fff734c9fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff73551000 -     0x7fff7358aff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7358b000 -     0x7fff735a8ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff735a9000 -     0x7fff735a9ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff735b7000 -     0x7fff735b7ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff735b8000 -     0x7fff735bcffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff735bd000 -     0x7fff735bfff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff735c0000 -     0x7fff735c1ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff735c2000 -     0x7fff735d9fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff735da000 -     0x7fff735dafff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff735db000 -     0x7fff73664ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff73665000 -     0x7fff73668ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff73669000 -     0x7fff7366cffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7366d000 -     0x7fff7366efff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7366f000 -     0x7fff73675ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff73676000 -     0x7fff736bfff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff736c0000 -     0x7fff736e5ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff736e6000 -     0x7fff73731fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff73732000 -     0x7fff73751fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff73752000 -     0x7fff737f6ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff737f7000 -     0x7fff73801ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff73802000 -     0x7fff7380bff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7380c000 -     0x7fff73813ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff73814000 -     0x7fff7381ffff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff73820000 -     0x7fff73823ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff73824000 -     0x7fff73825ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff73826000 -     0x7fff7382dff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7382e000 -     0x7fff73841ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff73843000 -     0x7fff73848ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff73849000 -     0x7fff73875ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=17.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9712K        9 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4520K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9660K       44 
===========                     =======  ======= 
TOTAL                            276.5M      110 
TOTAL                            276.5M      110 
TOTAL, minus reserved VM space   276.3M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-31-211820-3_Traviss-Mac-1044.crash
Process:               a [41347]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41346]
Responsible:           a [41347]
User ID:               501
Date/Time:             2019-05-31 21:16:45.410 +0000
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
0   a                              0x000000010c7d180e abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 30
1   a                              0x000000010c7d0c09 std::panicking::try::do_call::h6a57c411fdd4d7f2 (.llvm.16641410772807917656) + 9
2   libstd-a20dac67422e025f.dylib  0x000000010c80d0bf __rust_maybe_catch_panic + 31
3   a                              0x000000010c7d1a61 abort_on_c_abi::main::he771bf881fc862e3 + 593
4   a                              0x000000010c7d0106 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd9c006987ee01de9 + 6
5   libstd-a20dac67422e025f.dylib  0x000000010c7fd1b8 std::panicking::try::do_call::hd4cf110fedfe114a + 24
6   libstd-a20dac67422e025f.dylib  0x000000010c80d0bf __rust_maybe_catch_panic + 31
7   libstd-a20dac67422e025f.dylib  0x000000010c7fdc9e std::rt::lang_start_internal::hc3d05ecc1e0ed4e0 + 542
8   a                              0x000000010c7d1d69 main + 41
9   libdyld.dylib                  0x00007fff7358c115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007fafa8d00010  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee342dab8  rsi: 0x00000000ffffffc3  rbp: 0x00007ffee342e510  rsp: 0x00007ffee342e510
   r8: 0x00000000fa8d0006   r9: 0x0000000000000004  r10: 0x000000011a7538c0  r11: 0x00007fff7384396c
  r12: 0x000000010cb0f000  r13: 0x0000000000000000  r14: 0x00007ffee342e630  r15: 0x00007ffee342e578
  rip: 0x000000010c7d180e  rfl: 0x0000000000010206  cr2: 0x000000010c848d60
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10c7cf000 -        0x10c7d2ff7 +a (0) <7E8A5A3B-B884-3CF0-9357-94BC159DD79D> /Users/USER/*/a
       0x10c7da000 -        0x10c875ff7 +libstd-a20dac67422e025f.dylib (0) <ADD7B58C-431A-314A-9E69-EC602179831B> /Users/USER/*/libstd-a20dac67422e025f.dylib
       0x11a701000 -        0x11a74b98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff70df6000 -     0x7fff70e29fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff71308000 -     0x7fff71309ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff715be000 -     0x7fff71614fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff71615000 -     0x7fff71639ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff7298b000 -     0x7fff72d7c3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff72e49000 -     0x7fff72e65ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff73423000 -     0x7fff73427ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff73428000 -     0x7fff73432ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff73433000 -     0x7fff7343afff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7343b000 -     0x7fff73443ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff73444000 -     0x7fff734c9fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff73551000 -     0x7fff7358aff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7358b000 -     0x7fff735a8ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff735a9000 -     0x7fff735a9ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff735b7000 -     0x7fff735b7ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff735b8000 -     0x7fff735bcffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff735bd000 -     0x7fff735bfff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff735c0000 -     0x7fff735c1ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff735c2000 -     0x7fff735d9fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff735da000 -     0x7fff735dafff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff735db000 -     0x7fff73664ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff73665000 -     0x7fff73668ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff73669000 -     0x7fff7366cffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7366d000 -     0x7fff7366efff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7366f000 -     0x7fff73675ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff73676000 -     0x7fff736bfff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff736c0000 -     0x7fff736e5ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff736e6000 -     0x7fff73731fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff73732000 -     0x7fff73751fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff73752000 -     0x7fff737f6ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff737f7000 -     0x7fff73801ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff73802000 -     0x7fff7380bff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7380c000 -     0x7fff73813ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff73814000 -     0x7fff7381ffff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff73820000 -     0x7fff73823ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff73824000 -     0x7fff73825ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff73826000 -     0x7fff7382dff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7382e000 -     0x7fff73841ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff73843000 -     0x7fff73848ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff73849000 -     0x7fff73875ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2082
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
__DATA                            4520K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            277.0M      109 
TOTAL                            277.0M      109 
TOTAL, minus reserved VM space   276.9M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-31-211820_Traviss-Mac-1044.crash
Process:               a [42130]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        a [42126]
Responsible:           a [42130]
User ID:               501
Date/Time:             2019-05-31 21:17:14.624 +0000
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
0   libstd-a20dac67422e025f.dylib  0x00000001004e473e std::panicking::rust_panic_with_hook::h6bd005e43362900c + 142
1   a                              0x00000001004b46d5 std::panicking::begin_panic::h001d52deddc12f24 + 37
2   a                              0x00000001004b1d9c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x00000001004b1379 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x00000001004b1d73 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x00000001004b32ba backtrace::main::hc9a5bc8fc93ded64 + 5178 (backtrace.rs:119)
6   a                              0x00000001004b12a6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha6addb1b69a9df14 + 6 (rt.rs:64)
7   libstd-a20dac67422e025f.dylib  0x00000001004e41b8 std::panicking::try::do_call::hd4cf110fedfe114a + 24
8   libstd-a20dac67422e025f.dylib  0x00000001004f40bf __rust_maybe_catch_panic + 31
9   libstd-a20dac67422e025f.dylib  0x00000001004e4c9e std::rt::lang_start_internal::hc3d05ecc1e0ed4e0 + 542
10  a                              0x00000001004b3c09 main + 41
11  libdyld.dylib                  0x00007fff7358c115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeef750238  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000100531145  rbp: 0x00007ffeef750330  rsp: 0x00007ffeef750260
   r8: 0x0000000000000100   r9: 0x0000000100564a40  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010052ff98  r14: 0x00000001004b6480  r15: 0x00007ffeef750340
  rip: 0x00000001004e473e  rfl: 0x0000000000010206  cr2: 0x0000000113de7d9e
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1004ad000 -        0x1004b5fff +a (0) <60194EDB-25AE-3427-8A90-703ECF4A1D73> /Users/USER/*/a
       0x1004c1000 -        0x10055cff7 +libstd-a20dac67422e025f.dylib (0) <ADD7B58C-431A-314A-9E69-EC602179831B> /Users/USER/*/libstd-a20dac67422e025f.dylib
       0x104296000 -        0x1042e098f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff70df6000 -     0x7fff70e29fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff71308000 -     0x7fff71309ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff715be000 -     0x7fff71614fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff71615000 -     0x7fff71639ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff7298b000 -     0x7fff72d7c3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff72e49000 -     0x7fff72e65ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff73423000 -     0x7fff73427ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff73428000 -     0x7fff73432ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff73433000 -     0x7fff7343afff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7343b000 -     0x7fff73443ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff73444000 -     0x7fff734c9fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff73551000 -     0x7fff7358aff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7358b000 -     0x7fff735a8ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff735a9000 -     0x7fff735a9ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff735b7000 -     0x7fff735b7ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff735b8000 -     0x7fff735bcffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff735bd000 -     0x7fff735bfff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff735c0000 -     0x7fff735c1ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff735c2000 -     0x7fff735d9fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff735da000 -     0x7fff735dafff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff735db000 -     0x7fff73664ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff73665000 -     0x7fff73668ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff73669000 -     0x7fff7366cffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7366d000 -     0x7fff7366efff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7366f000 -     0x7fff73675ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff73676000 -     0x7fff736bfff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff736c0000 -     0x7fff736e5ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff736e6000 -     0x7fff73731fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff73732000 -     0x7fff73751fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff73752000 -     0x7fff737f6ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff737f7000 -     0x7fff73801ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff73802000 -     0x7fff7380bff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7380c000 -     0x7fff73813ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff73814000 -     0x7fff7381ffff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff73820000 -     0x7fff73823ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff73824000 -     0x7fff73825ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff73826000 -     0x7fff7382dff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7382e000 -     0x7fff73841ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff73843000 -     0x7fff73848ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff73849000 -     0x7fff73875ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
