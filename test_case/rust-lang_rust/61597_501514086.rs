plain
[00:03:29]       Memory: 8 GB
[00:03:29]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:29]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:29]       SMC Version (system): 2.8f0
[00:03:29]       Serial Number (system): VM9iLfuGaLuP
[00:03:29] 
[00:03:29] hw.ncpu: 4
[00:03:29] hw.byteorder: 1234
[00:03:29] hw.memsize: 8589934592
---
[01:41:41] 
[01:41:41] ---- alloc.rs - alloc::alloc_zeroed (line 148) stdout ----
[01:41:41] error: linking with `cc` failed: signal: 4
[01:41:41]   |
[01:41:41]   = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out.rust_out.7rcbfp3g-cgu.10.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out.rust_out.7rcbfp3g-cgu.11.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out.rust_out.7rcbfp3g-cgu.6.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out.rust_out.7rcbfp3g-cgu.7.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out.rust_out.7rcbfp3g-cgu.8.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out.rust_out.7rcbfp3g-cgu.9.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestff9n4L/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1-std/i686-apple-darwin/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1-std/i686-apple-darwin/release/build/compiler_builtins-d7c3da045db09769/out" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1-std/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libstd-7f610d5e9627464e.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libpanic_unwind-47c273a59a33e413.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libbacktrace-8437e0dafd1fffc6.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libbacktrace_sys-a217f92577a9b3cc.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/librustc_demangle-5a010823a37e6f01.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libcfg_if-7baaa299e2e2dc93.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libhashbrown-7fa6c149d950f8eb.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_alloc-eb52043434d3159b.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libunwind-af16fc0796b3befc.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/liblibc-10c9cfc6f6ba5bdd.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/liballoc-101fade3a60e1e01.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_core-1289226b8282c55e.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libcore-bfc5ef2087e01f43.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-d940584a0bad99e6.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[01:41:41] 
[01:41:41] error: aborting due to previous error
[01:41:41] 
[01:41:41] Couldn't compile the test.
---
travis_fold:start:after_failure.2
travis_time:start:00ee3518
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1120
drwx------  21 travis  staff    714 Jun 13 01:08 .
-rw-------@  1 travis  staff  57366 Jun 13 01:08 a_2019-06-13-010828_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34964 Jun 13 01:08 a_2019-06-13-010827_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55585 Jun 13 01:08 a_2019-06-13-010817-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34863 Jun 13 01:08 a_2019-06-13-010817_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 Jun 13 01:08 a_2019-06-13-010812_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9166 Jun 13 01:08 a_2019-06-13-010806_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9171 Jun 13 01:08 a_2019-06-13-010803-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8936 Jun 13 01:08 a_2019-06-13-010803_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9304 Jun 13 01:07 a_2019-06-13-010723_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 Jun 13 01:07 a_2019-06-13-010711-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59516 Jun 13 01:07 a_2019-06-13-010711-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58251 Jun 13 01:07 a_2019-06-13-010711-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60372 Jun 13 01:07 a_2019-06-13-010711_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10880 Jun 13 01:04 a_2019-06-13-010455_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 Jun 13 01:04 a_2019-06-13-010406_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 Jun 13 01:02 a_2019-06-13-010249_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 Jun 13 01:01 a_2019-06-13-010150-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9780 Jun 13 01:01 a_2019-06-13-010150_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9483 Jun 13 01:01 a_2019-06-13-010149_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:049fe958
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-13-010149_Traviss-Mac-1044.crash
Process:               a [37507]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [37506]
Responsible:           a [37507]
User ID:               501
Date/Time:             2019-06-13 01:01:20.762 +0000
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
0   a                              0x00060a4e abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 46
1   a                              0x0005feab std::panicking::try::do_call::h6b40d4a4929115b5 (.llvm.1022360936634227900) + 11
2   libstd-7f610d5e9627464e.dylib  0x001acced __rust_maybe_catch_panic + 29
3   a                              0x00060cb5 abort_on_c_abi::main::he771bf881fc862e3 + 613
4   a                              0x0005f50b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h912fc2338677bf1f + 11
5   libstd-7f610d5e9627464e.dylib  0x0019b11c std::sys_common::backtrace::__rust_begin_short_backtrace::h019858fb3770b29a + 12
6   libstd-7f610d5e9627464e.dylib  0x0019e414 std::panicking::try::do_call::h042aa15d53ce64b9 + 20
7   libstd-7f610d5e9627464e.dylib  0x001acced __rust_maybe_catch_panic + 29
8   libstd-7f610d5e9627464e.dylib  0x0019eeb7 std::rt::lang_start_internal::heab10cd4e35d85ce + 631
9   a                              0x00060fec main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x7a814130  ebx: 0xbffa0b08  ecx: 0x00000000  edx: 0x00000000
  edi: 0x001accde  esi: 0x00000000  ebp: 0xbffa0aa8  esp: 0xbffa0a90
   ss: 0x00000023  efl: 0x00010296  eip: 0x00060a4e   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x001eb3c0
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x5e000 -    0x61ffb +a (0) <D30C8C53-022A-3AE6-B33F-0183E151A35E> /Users/USER/*/a
   0xf4000 -   0x139fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x17d000 -   0x211ffb +libstd-7f610d5e9627464e.dylib (0) <223C5EB4-5E50-38EA-9A60-BB6302D40F2E> /Users/USER/*/libstd-7f610d5e9627464e.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2092
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=27.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=27.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            19.0M        9 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3548K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9364K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            578.6M      135 
TOTAL                            578.6M      135 
TOTAL, minus reserved VM space   578.4M      135 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-13-010150-1_Traviss-Mac-1044.crash
Process:               a [38282]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [38277]
Responsible:           a [38282]
User ID:               501
Date/Time:             2019-06-13 01:01:49.752 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-7f610d5e9627464e.dylib  0x00210923 std::panicking::rust_panic_with_hook::hd4d7cd7c522096e6 + 115
1   a                              0x000c3a0f std::panicking::begin_panic::hfe32fd5ce1ab4b4f + 47 (panicking.rs:409)
2   a                              0x000c1164 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 36 (backtrace.rs:25)
3   a                              0x000c0b3b core::ptr::real_drop_in_place::hd5524a8477e0f1c0 + 11
4   a                              0x000c1133 backtrace::double::h35adec2a6f63ef6c + 51
5   a                              0x000c2788 backtrace::main::hc9a5bc8fc93ded64 + 5400 (backtrace.rs:119)
6   a                              0x000c061b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h452c9645fa5d92f4 + 11 (rt.rs:64)
7   libstd-7f610d5e9627464e.dylib  0x0020d11c std::sys_common::backtrace::__rust_begin_short_backtrace::h019858fb3770b29a + 12
8   libstd-7f610d5e9627464e.dylib  0x00210414 std::panicking::try::do_call::h042aa15d53ce64b9 + 20
9   libstd-7f610d5e9627464e.dylib  0x0021eced __rust_maybe_catch_panic + 29
10  libstd-7f610d5e9627464e.dylib  0x00210eb7 std::rt::lang_start_internal::heab10cd4e35d85ce + 631
11  a                              0x000c302c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff41828  ebx: 0xbff41870  ecx: 0xbff41720  edx: 0xa7702ec6
  edi: 0x0025d728  esi: 0x002108be  ebp: 0xbff418c8  esp: 0xbff41840
   ss: 0x00000023  efl: 0x00010282  eip: 0x00210923   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00251920
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xbd000 -    0xc4fff +a (0) <95322B2A-942D-3585-BEF8-26B434E845DE> /Users/USER/*/a
  0x166000 -   0x1abfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1ef000 -   0x283ffb +libstd-7f610d5e9627464e.dylib (0) <223C5EB4-5E50-38EA-9A60-BB6302D40F2E> /Users/USER/*/libstd-7f610d5e9627464e.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2092
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=17.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9632K        9 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3548K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9380K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.0M      135 
TOTAL                            569.0M      135 
TOTAL, minus reserved VM space   568.9M      135 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-13-010150_Traviss-Mac-1044.crash
Process:               a [38281]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [38277]
Responsible:           a [38281]
User ID:               501
Date/Time:             2019-06-13 01:01:49.573 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-7f610d5e9627464e.dylib  0x0021f923 std::panicking::rust_panic_with_hook::hd4d7cd7c522096e6 + 115
1   a                              0x000bfa0f std::panicking::begin_panic::hfe32fd5ce1ab4b4f + 47 (panicking.rs:409)
2   a                              0x000bd164 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 36 (backtrace.rs:25)
3   a                              0x000bcb3b core::ptr::real_drop_in_place::hd5524a8477e0f1c0 + 11
4   a                              0x000bd133 backtrace::double::h35adec2a6f63ef6c + 51
5   a                              0x000be788 backtrace::main::hc9a5bc8fc93ded64 + 5400 (backtrace.rs:119)
6   a                              0x000bc61b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h452c9645fa5d92f4 + 11 (rt.rs:64)
7   libstd-7f610d5e9627464e.dylib  0x0021c11c std::sys_common::backtrace::__rust_begin_short_backtrace::h019858fb3770b29a + 12
8   libstd-7f610d5e9627464e.dylib  0x0021f414 std::panicking::try::do_call::h042aa15d53ce64b9 + 20
9   libstd-7f610d5e9627464e.dylib  0x0022dced __rust_maybe_catch_panic + 29
10  libstd-7f610d5e9627464e.dylib  0x0021feb7 std::rt::lang_start_internal::heab10cd4e35d85ce + 631
11  a                              0x000bf02c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff45838  ebx: 0xbff45880  ecx: 0xbff45730  edx: 0xa7702ec6
  edi: 0x0026c728  esi: 0x0021f8be  ebp: 0xbff458d8  esp: 0xbff45850
   ss: 0x00000023  efl: 0x00010286  eip: 0x0021f923   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x7b674000
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xb9000 -    0xc0fff +a (0) <95322B2A-942D-3585-BEF8-26B434E845DE> /Users/USER/*/a
  0x175000 -   0x1bafdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1fe000 -   0x292ffb +libstd-7f610d5e9627464e.dylib (0) <223C5EB4-5E50-38EA-9A60-BB6302D40F2E> /Users/USER/*/libstd-7f610d5e9627464e.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2092
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=17.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9632K        9 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3548K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9380K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.0M      135 
TOTAL                            569.0M      135 
TOTAL, minus reserved VM space   568.9M      135 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-13-010249_Traviss-Mac-1044.crash
Process:               a [39996]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [39994]
Responsible:           a [39996]
User ID:               501
Date/Time:             2019-06-13 01:02:49.052 +0000
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
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-7f610d5e9627464e.dylib  0x0013d16b std::sys::unix::abort_internal::ha5111d5275a9814a + 11
4   libstd-7f610d5e9627464e.dylib  0x0012e9b0 rust_oom + 48
5   libstd-7f610d5e9627464e.dylib  0x001548a4 alloc::alloc::handle_alloc_error::hcfc92d053cb11f4f + 20
6   a                              0x0002a69d default_alloc_error_hook::main::h0fe124586986ad5d + 781
7   a                              0x00029eab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h281c8e911b4e41bb + 11
8   libstd-7f610d5e9627464e.dylib  0x0012c11c std::sys_common::backtrace::__rust_begin_short_backtrace::h019858fb3770b29a + 12
9   libstd-7f610d5e9627464e.dylib  0x0012f414 std::panicking::try::do_call::h042aa15d53ce64b9 + 20
10  libstd-7f610d5e9627464e.dylib  0x0013dced __rust_maybe_catch_panic + 29
11  libstd-7f610d5e9627464e.dylib  0x0012feb7 std::rt::lang_start_internal::heab10cd4e35d85ce + 631
12  a                              0x0002a7fc main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffd5a1c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffd5a48  esp: 0xbffd5a1c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x29000 -    0x2affb +a (0) <10CB53D7-3AAB-3DF9-ACBC-6B5C16E6D584> /Users/USER/*/a
   0x85000 -    0xcafdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x10e000 -   0x1a2ffb +libstd-7f610d5e9627464e.dylib (0) <223C5EB4-5E50-38EA-9A60-BB6302D40F2E> /Users/USER/*/libstd-7f610d5e9627464e.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
---
===========                     =======  ======= 
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-13-010711-2_Traviss-Mac-1044.crash
Process:               a [46440]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [46438]
Responsible:           a [46440]
User ID:               501
Date/Time:             2019-06-13 01:07:10.398 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b019dfec
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb019dfec:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b019d000-00000000b019e000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b019e000-00000000b039f000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-7f610d5e9627464e.dylib  0x001c4970 std::sys::unix::thread::Thread::join::h0092f35e0e8cc34a + 32
4   a                              0x00102036 std::thread::JoinHandle$LT$T$GT$::join::h44c5a67401d14583 + 70
5   a                              0x00100cef out_of_stack::main::h51a6e88d0363a411 + 255
6   a                              0x001000fb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h7572bf37fe831ab7 + 11
7   libstd-7f610d5e9627464e.dylib  0x001b411c std::sys_common::backtrace::__rust_begin_short_backtrace::h019858fb3770b29a + 12
8   libstd-7f610d5e9627464e.dylib  0x001b7414 std::panicking::try::do_call::h042aa15d53ce64b9 + 20
9   libstd-7f610d5e9627464e.dylib  0x001c5ced __rust_maybe_catch_panic + 29
10  libstd-7f610d5e9627464e.dylib  0x001b7eb7 std::rt::lang_start_internal::heab10cd4e35d85ce + 631
11  a                              0x00101acc main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-7f610d5e9627464e.dylib  0x001c516b std::sys::unix::abort_internal::ha5111d5275a9814a + 11
4   libstd-7f610d5e9627464e.dylib  0x001b5bd9 std::sys_common::util::abort::hdff96fefabfe0dbc + 73
5   libstd-7f610d5e9627464e.dylib  0x001c440f std::sys::unix::stack_overflow::imp::signal_handler::h374d3af5ce3e1cc2 + 943
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-7f610d5e9627464e.dylib  0x001c4060 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h4fdd21c580c680ac + 80
9   libstd-7f610d5e9627464e.dylib  0x001a31ec _$LT$std..io..buffered..LineWriter$LT$W$GT$$u20$as$u20$std..io..Write$GT$::write::hf3bda037c01ff98f + 220
10  libstd-7f610d5e9627464e.dylib  0x001a7ca5 std::io::Write::write_all::hba716ea456265b7b + 101
11  libstd-7f610d5e9627464e.dylib  0x001a8143 _$LT$std..io..Write..write_fmt..Adaptor$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$::write_str::he133ee05f1263bed + 35
12  libstd-7f610d5e9627464e.dylib  0x001ea55d core::fmt::write::h4525a2ca55d60faf + 701
13  libstd-7f610d5e9627464e.dylib  0x001a5d56 _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::h7770b6777df16ec8 + 182
14  libstd-7f610d5e9627464e.dylib  0x001a728c std::io::stdio::_print::h2bcb723ad2073385 + 396
15  a                              0x00100bdf out_of_stack::loud_recurse::h48447a21f69c373e + 63
16  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
17  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
18  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
19  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
20  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
21  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
22  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
23  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
24  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
25  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
26  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
27  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
28  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
29  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
30  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
31  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
32  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
33  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
34  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
35  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
36  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
37  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
38  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
39  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
40  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
41  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
42  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
43  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
44  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
45  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
46  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
47  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
48  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
49  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
50  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
51  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
52  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
53  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
54  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
55  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
56  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
57  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
58  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
59  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
60  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
61  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
62  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
63  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
64  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
65  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
66  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
67  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
68  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
69  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
70  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
71  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
72  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
73  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
74  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
75  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
76  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
77  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
78  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
79  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
80  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
81  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
82  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
83  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
84  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
85  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
86  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
87  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
88  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
89  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
90  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
91  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
92  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
93  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
94  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
95  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
96  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
97  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
98  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
99  a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
100 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
101 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
102 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
103 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
104 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
105 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
106 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
107 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
108 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
109 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
110 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
111 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
112 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
113 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
114 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
115 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
116 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
117 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
118 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
119 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
120 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
121 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
122 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
123 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
124 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
125 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
126 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
127 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
128 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
129 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
130 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
131 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
132 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
133 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
134 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
135 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
136 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
137 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
138 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
139 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
140 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
141 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
142 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
143 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
144 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
145 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
146 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
147 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
148 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
149 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
150 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
151 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
152 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
153 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
154 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
155 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
156 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
157 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
158 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
159 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
160 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
161 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
162 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
163 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
164 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
165 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
166 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
167 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
168 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
169 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
170 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
171 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
172 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
173 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
174 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
175 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
176 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
177 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
178 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
179 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
180 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
181 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
182 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
183 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
184 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
185 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
186 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
187 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
188 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
189 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
190 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
191 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
192 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
193 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
194 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
195 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
196 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
197 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
198 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
199 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
200 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
201 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
202 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
203 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
204 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
205 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
206 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
207 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
208 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
209 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
210 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
211 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
212 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
213 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
214 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
215 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
216 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
217 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
218 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
219 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
220 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
221 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
222 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
223 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
224 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
225 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
226 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
227 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
228 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
229 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
230 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
231 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
232 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
233 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
234 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
235 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
236 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
237 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
238 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
239 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
240 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
241 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
242 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
243 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
244 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
245 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
246 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
247 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
248 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
249 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
250 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
251 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
252 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
253 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
254 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
255 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
256 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
257 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
258 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
259 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
260 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
261 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
262 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
263 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
264 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
265 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
266 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
267 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
268 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
269 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
270 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
271 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
272 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
273 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
274 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
275 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
276 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
277 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
278 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
279 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
280 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
281 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
282 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
283 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
284 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
285 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
286 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
287 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
288 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
289 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
290 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
291 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
292 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
293 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
294 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
295 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
296 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
297 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
298 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
299 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
300 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
301 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
302 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
303 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
304 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
305 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
306 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
307 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
308 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
309 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
310 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
311 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
312 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
313 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
314 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
315 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
316 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
317 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
318 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
319 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
320 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
321 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
322 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
323 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
324 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
325 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
326 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
327 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
328 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
329 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
330 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
331 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
332 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
333 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
334 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
335 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
336 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
337 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
338 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
339 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
340 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
341 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
342 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
343 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
344 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
345 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
346 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
347 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
348 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
349 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
350 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
351 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
352 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
353 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
354 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
355 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
356 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
357 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
358 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
359 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
360 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
361 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
362 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
363 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
364 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
365 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
366 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
367 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
368 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
369 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
370 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
371 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
372 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
373 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
374 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
375 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
376 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
377 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
378 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
379 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
380 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
381 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
382 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
383 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
384 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
385 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
386 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
387 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
388 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
389 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
390 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
391 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
392 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
393 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
394 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
395 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
396 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
397 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
398 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
399 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
400 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
401 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
402 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
403 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
404 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
405 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
406 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
407 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
408 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
409 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
410 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
411 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
412 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
413 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
414 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
415 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
416 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
417 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
418 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
419 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
420 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
421 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
422 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
423 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
424 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
425 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
426 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
427 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
428 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
429 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
430 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
431 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
432 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
433 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
434 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
435 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
436 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
437 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
438 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
439 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
440 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
441 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
442 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
443 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
444 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
445 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
446 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
447 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
448 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
449 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
450 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
451 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
452 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
453 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
454 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
455 a                              0x00100be4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
---
===========                     =======  ======= 
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-13-010711_Traviss-Mac-1044.crash
Process:               a [46439]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [46438]
Responsible:           a [46439]
User ID:               501
Date/Time:             2019-06-13 01:07:10.273 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b04d5e74
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb04d5e74:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b04d5000-00000000b04d6000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b04d6000-00000000b06d7000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-7f610d5e9627464e.dylib  0x001d7970 std::sys::unix::thread::Thread::join::h0092f35e0e8cc34a + 32
4   a                              0x000b0036 std::thread::JoinHandle$LT$T$GT$::join::h44c5a67401d14583 + 70
5   a                              0x000af5b1 out_of_stack::main::h51a6e88d0363a411 + 2497
6   a                              0x000ae0fb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h7572bf37fe831ab7 + 11
7   libstd-7f610d5e9627464e.dylib  0x001c711c std::sys_common::backtrace::__rust_begin_short_backtrace::h019858fb3770b29a + 12
8   libstd-7f610d5e9627464e.dylib  0x001ca414 std::panicking::try::do_call::h042aa15d53ce64b9 + 20
9   libstd-7f610d5e9627464e.dylib  0x001d8ced __rust_maybe_catch_panic + 29
10  libstd-7f610d5e9627464e.dylib  0x001caeb7 std::rt::lang_start_internal::heab10cd4e35d85ce + 631
11  a                              0x000afacc main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-7f610d5e9627464e.dylib  0x001d816b std::sys::unix::abort_internal::ha5111d5275a9814a + 11
4   libstd-7f610d5e9627464e.dylib  0x001c8bd9 std::sys_common::util::abort::hdff96fefabfe0dbc + 73
5   libstd-7f610d5e9627464e.dylib  0x001d740f std::sys::unix::stack_overflow::imp::signal_handler::h374d3af5ce3e1cc2 + 943
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-7f610d5e9627464e.dylib  0x001d7060 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h4fdd21c580c680ac + 80
9   a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
10  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
11  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
12  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
13  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
14  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
15  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
16  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
17  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
18  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
19  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
20  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
21  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
22  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
23  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
24  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
25  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
26  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
27  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
28  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
29  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
30  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
31  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
32  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
33  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
34  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
35  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
36  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
37  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
38  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
39  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
40  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
41  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
42  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
43  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
44  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
45  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
46  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
47  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
48  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
49  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
50  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
51  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
52  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
53  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
54  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
55  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
56  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
57  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
58  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
59  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
60  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
61  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
62  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
63  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
64  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
65  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
66  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
67  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
68  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
69  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
70  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
71  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
72  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
73  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
74  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
75  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
76  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
77  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
78  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
79  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
80  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
81  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
82  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
83  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
84  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
85  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
86  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
87  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
88  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
89  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
90  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
91  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
92  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
93  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
94  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
95  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
96  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
97  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
98  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
99  a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
100 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
101 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
102 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
103 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
104 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
105 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
106 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
107 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
108 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
109 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
110 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
111 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
112 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
113 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
114 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
115 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
116 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
117 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
118 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
119 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
120 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
121 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
122 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
123 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
124 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
125 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
126 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
127 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
128 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
129 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
130 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
131 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
132 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
133 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
134 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
135 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
136 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
137 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
138 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
139 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
140 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
141 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
142 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
143 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
144 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
145 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
146 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
147 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
148 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
149 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
150 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
151 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
152 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
153 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
154 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
155 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
156 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
157 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
158 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
159 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
160 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
161 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
162 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
163 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
164 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
165 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
166 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
167 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
168 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
169 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
170 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
171 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
172 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
173 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
174 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
175 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
176 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
177 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
178 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
179 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
180 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
181 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
182 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
183 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
184 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
185 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
186 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
187 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
188 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
189 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
190 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
191 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
192 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
193 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
194 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
195 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
196 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
197 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
198 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
199 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
200 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
201 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
202 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
203 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
204 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
205 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
206 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
207 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
208 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
209 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
210 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
211 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
212 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
213 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
214 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
215 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
216 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
217 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
218 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
219 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
220 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
221 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
222 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
223 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
224 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
225 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
226 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
227 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
228 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
229 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
230 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
231 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
232 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
233 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
234 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
235 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
236 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
237 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
238 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
239 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
240 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
241 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
242 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
243 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
244 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
245 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
246 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
247 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
248 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
249 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
250 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
251 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
252 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
253 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
254 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
255 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
256 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
257 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
258 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
259 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
260 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
261 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
262 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
263 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
264 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
265 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
266 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
267 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
268 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
269 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
270 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
271 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
272 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
273 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
274 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
275 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
276 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
277 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
278 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
279 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
280 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
281 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
282 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
283 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
284 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
285 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
286 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
287 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
288 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
289 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
290 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
291 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
292 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
293 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
294 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
295 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
296 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
297 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
298 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
299 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
300 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
301 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
302 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
303 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
304 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
305 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
306 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
307 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
308 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
309 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
310 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
311 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
312 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
313 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
314 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
315 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
316 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
317 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
318 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
319 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
320 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
321 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
322 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
323 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
324 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
325 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
326 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
327 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
328 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
329 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
330 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
331 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
332 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
333 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
334 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
335 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
336 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
337 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
338 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
339 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
340 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
341 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
342 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
343 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
344 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
345 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
346 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
347 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
348 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
349 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
350 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
351 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
352 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
353 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
354 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
355 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
356 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
357 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
358 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
359 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
360 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
361 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
362 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
363 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
364 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
365 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
366 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
367 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
368 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
369 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
370 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
371 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
372 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
373 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
374 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
375 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
376 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
377 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
378 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
379 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
380 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
381 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
382 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
383 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
384 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
385 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
386 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
387 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
388 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
389 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
390 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
391 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
392 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
393 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
394 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
395 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
396 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
397 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
398 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
399 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
400 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
401 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
402 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
403 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
404 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
405 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
406 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
407 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
408 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
409 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
410 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
411 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
412 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
413 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
414 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
415 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
416 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
417 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
418 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
419 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
420 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
421 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
422 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
423 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
424 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
425 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
426 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
427 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
428 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
429 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
430 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
431 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
432 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
433 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
434 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
435 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
436 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
437 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
438 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
439 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
440 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
441 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
442 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
443 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
444 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
445 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
446 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
447 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
448 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
449 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
450 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
451 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
452 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
453 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
454 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
455 a                              0x000aeb96 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
---
===========                     =======  ======= 
TOTAL                            568.6M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-13-010803_Traviss-Mac-1044.crash
Process:               a [47611]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47604]
Responsible:           a [47611]
User ID:               501
Date/Time:             2019-06-13 01:08:02.044 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-7f610d5e9627464e.dylib  0x001e816b std::sys::unix::abort_internal::ha5111d5275a9814a + 11
4   libstd-7f610d5e9627464e.dylib  0x001d8bd9 std::sys_common::util::abort::hdff96fefabfe0dbc + 73
5   libstd-7f610d5e9627464e.dylib  0x001dac32 rust_panic + 98
6   libstd-7f610d5e9627464e.dylib  0x001daafe std::panicking::rust_panic_with_hook::hd4d7cd7c522096e6 + 590
7   a                              0x000329cf std::panicking::begin_panic::hce6299dbe7204bf4 + 47
8   a                              0x00033b0c main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffcd9ec  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffcda18  esp: 0xbffcd9ec
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x31000 -    0x34fff +a (0) <2DEABA3B-67D8-399B-875F-EA742D999072> /Users/USER/*/a
  0x130000 -   0x175fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1b9000 -   0x24dffb +libstd-7f610d5e9627464e.dylib (0) <223C5EB4-5E50-38EA-9A60-BB6302D40F2E> /Users/USER/*/libstd-7f610d5e9627464e.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2340
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=18.2M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=18.2M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
__DATA                            3548K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__OBJC                              36K        6 
__TEXT                            9364K       44 
mapped file                      408.7M       21 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.4M      132 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-13-010806_Traviss-Mac-1044.crash
Process:               a [47715]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47714]
Responsible:           a [47715]
User ID:               501
Date/Time:             2019-06-13 01:08:05.784 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGSEGV)
Exception Codes:       KERN_INVALID_ADDRESS at 0x0000000000000001
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Segmentation fault: 11
Termination Reason:    Namespace SIGNAL, Code 0xb
Terminating Process:   exc handler [0]
VM Regions Near 0x1:
--> 
    __TEXT                 00000000000f4000-00000000000f7000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000f65f4 signal_exit_status::main::hc6663d816ec186eb + 436
1   a                              0x000f549b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb111cffbc1462816 + 11
2   libstd-7f610d5e9627464e.dylib  0x001f911c std::sys_common::backtrace::__rust_begin_short_backtrace::h019858fb3770b29a + 12
3   libstd-7f610d5e9627464e.dylib  0x001fc414 std::panicking::try::do_call::h042aa15d53ce64b9 + 20
4   libstd-7f610d5e9627464e.dylib  0x0020aced __rust_maybe_catch_panic + 29
5   libstd-7f610d5e9627464e.dylib  0x001fceb7 std::rt::lang_start_internal::heab10cd4e35d85ce + 631
6   a                              0x000f66cc main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x7a633a40
  edi: 0x7a633ad0  esi: 0xbff0ab90  ebp: 0xbff0ac28  esp: 0xbff0ab10
   ss: 0x00000023  efl: 0x00010246  eip: 0x000f65f4   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     2
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xf4000 -    0xf6ff7 +a (0) <F6AA49AF-20EC-3BDF-B383-3FD49EA1BE21> /Users/USER/*/a
  0x152000 -   0x197fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1db000 -   0x26fffb +libstd-7f610d5e9627464e.dylib (0) <223C5EB4-5E50-38EA-9A60-BB6302D40F2E> /Users/USER/*/libstd-7f610d5e9627464e.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2340
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=17.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                        132K        3 
__DATA                            3548K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.6M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-13-010812_Traviss-Mac-1044.crash
Process:               a [47819]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47811]
Responsible:           a [47819]
User ID:               501
Date/Time:             2019-06-13 01:08:11.203 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4900 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000f48e6 simd_target_feature_mixup::test::id_avx512_512::hf3a1395d43161fbe + 102
1   a                              0x000f369f simd_target_feature_mixup::test::main::h611f15c116e6c273 + 1647
2   a                              0x000f5d60 simd_target_feature_mixup::main::hfcef50d014c08dda + 896
3   a                              0x000f2dab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h6e7d59d2b0dfe81f + 11
4   libstd-7f610d5e9627464e.dylib  0x001bd11c std::sys_common::backtrace::__rust_begin_short_backtrace::h019858fb3770b29a + 12
5   libstd-7f610d5e9627464e.dylib  0x001c0414 std::panicking::try::do_call::h042aa15d53ce64b9 + 20
6   libstd-7f610d5e9627464e.dylib  0x001ceced __rust_maybe_catch_panic + 29
7   libstd-7f610d5e9627464e.dylib  0x001c0eb7 std::rt::lang_start_internal::heab10cd4e35d85ce + 631
8   a                              0x000f5f3c main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff0c840  ebx: 0xbff0c7c0  ecx: 0x000f488e  edx: 0xbff0c7c0
  edi: 0x000f3044  esi: 0x00000000  ebp: 0xbff0c7b8  esp: 0xbff0c780
   ss: 0x00000023  efl: 0x00010246  eip: 0x000f48e6   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000f4520
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xf2000 -    0xf6fcf +a (0) <F5096512-0444-3AEA-A9B3-499C8A7DEE56> /Users/USER/*/a
  0x116000 -   0x15bfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x19f000 -   0x233ffb +libstd-7f610d5e9627464e.dylib (0) <223C5EB4-5E50-38EA-9A60-BB6302D40F2E> /Users/USER/*/libstd-7f610d5e9627464e.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2340
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=17.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3548K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9368K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            568.6M      134 
TOTAL                            568.6M      134 
TOTAL, minus reserved VM space   568.4M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-13-010817-1_Traviss-Mac-1044.crash
Process:               a [47966]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47963]
Responsible:           a [47966]
User ID:               501
Date/Time:             2019-06-13 01:08:16.837 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4900 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bf7a6b48
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbf7a6b48:
    Stack Guard            00000000bbfa6000-00000000bf7a6000 [ 56.0M] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bf7a6000-00000000bf7a7000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bf7a7000-00000000bffa6000 [ 8188K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-7f610d5e9627464e.dylib  0x001d516b std::sys::unix::abort_internal::ha5111d5275a9814a + 11
4   libstd-7f610d5e9627464e.dylib  0x001c5bd9 std::sys_common::util::abort::hdff96fefabfe0dbc + 73
5   libstd-7f610d5e9627464e.dylib  0x001d440f std::sys::unix::stack_overflow::imp::signal_handler::h374d3af5ce3e1cc2 + 943
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-7f610d5e9627464e.dylib  0x001d4060 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h4fdd21c580c680ac + 80
9   a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
10  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
11  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
12  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
13  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
14  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
15  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
16  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
17  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
18  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
19  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
20  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
21  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
22  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
23  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
24  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
25  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
26  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
27  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
28  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
29  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
30  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
31  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
32  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
33  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
34  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
35  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
36  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
37  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
38  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
39  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
40  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
41  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
42  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
43  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
44  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
45  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
46  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
47  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
48  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
49  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
50  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
51  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
52  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
53  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
54  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
55  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
56  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
57  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
58  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
59  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
60  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
61  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
62  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
63  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
64  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
65  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
66  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
67  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
68  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
69  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
70  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
71  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
72  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
73  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
74  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
75  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
76  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
77  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
78  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
79  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
80  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
81  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
82  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
83  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
84  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
85  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
86  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
87  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
88  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
89  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
90  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
91  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
92  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
93  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
94  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
95  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
96  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
97  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
98  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
99  a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
100 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
101 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
102 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
103 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
104 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
105 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
106 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
107 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
108 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
109 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
110 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
111 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
112 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
113 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
114 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
115 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
116 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
117 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
118 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
119 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
120 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
121 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
122 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
123 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
124 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
125 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
126 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
127 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
128 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
129 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
130 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
131 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
132 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
133 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
134 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
135 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
136 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
137 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
138 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
139 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
140 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
141 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
142 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
143 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
144 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
145 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
146 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
147 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
148 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
149 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
150 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
151 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
152 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
153 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
154 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
155 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
156 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
157 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
158 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
159 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
160 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
161 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
162 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
163 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
164 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
165 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
166 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
167 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
168 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
169 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
170 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
171 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
172 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
173 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
174 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
175 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
176 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
177 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
178 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
179 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
180 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
181 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
182 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
183 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
184 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
185 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
186 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
187 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
188 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
189 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
190 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
191 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
192 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
193 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
194 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
195 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
196 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
197 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
198 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
199 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
200 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
201 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
202 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
203 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
204 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
205 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
206 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
207 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
208 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
209 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
210 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
211 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
212 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
213 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
214 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
215 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
216 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
217 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
218 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
219 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
220 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
221 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
222 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
223 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
224 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
225 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
226 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
227 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
228 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
229 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
230 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
231 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
232 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
233 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
234 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
235 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
236 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
237 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
238 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
239 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
240 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
241 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
242 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
243 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
244 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
245 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
246 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
247 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
248 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
249 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
250 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
251 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
252 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
253 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
254 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
255 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
256 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
257 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
258 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
259 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
260 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
261 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
262 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
263 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
264 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
265 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
266 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
267 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
268 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
269 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
270 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
271 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
272 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
273 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
274 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
275 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
276 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
277 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
278 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
279 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
280 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
281 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
282 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
283 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
284 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
285 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
286 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
287 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
288 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
289 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
290 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
291 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
292 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
293 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
294 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
295 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
296 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
297 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
298 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
299 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
300 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
301 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
302 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
303 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
304 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
305 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
306 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
307 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
308 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
309 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
310 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
311 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
312 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
313 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
314 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
315 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
316 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
317 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
318 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
319 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
320 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
321 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
322 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
323 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
324 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
325 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
326 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
327 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
328 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
329 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
330 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
331 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
332 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
333 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
334 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
335 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
336 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
337 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
338 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
339 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
340 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
341 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
342 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
343 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
344 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
345 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
346 a                              0x0005c9c0 stack_probes::recurse::h35e743424666f0f3 + 48
---
===========                     =======  ======= 
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-13-010817_Traviss-Mac-1044.crash
Process:               a [47968]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47963]
Responsible:           a [47968]
User ID:               501
Date/Time:             2019-06-13 01:08:16.839 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4900 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b010ce68
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb010ce68:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b010c000-00000000b010d000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b010d000-00000000b030e000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-7f610d5e9627464e.dylib  0x00162970 std::sys::unix::thread::Thread::join::h0092f35e0e8cc34a + 32
4   a                              0x0005f656 std::thread::JoinHandle$LT$T$GT$::join::h96cff75cf5159470 + 70
5   a                              0x0005e875 stack_probes::main::h385e455299c91d04 + 597
6   a                              0x0005d69b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h3196027e06c00c06 + 11
7   libstd-7f610d5e9627464e.dylib  0x0015211c std::sys_common::backtrace::__rust_begin_short_backtrace::h019858fb3770b29a + 12
8   libstd-7f610d5e9627464e.dylib  0x00155414 std::panicking::try::do_call::h042aa15d53ce64b9 + 20
9   libstd-7f610d5e9627464e.dylib  0x00163ced __rust_maybe_catch_panic + 29
10  libstd-7f610d5e9627464e.dylib  0x00155eb7 std::rt::lang_start_internal::heab10cd4e35d85ce + 631
11  a                              0x0005f29c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-7f610d5e9627464e.dylib  0x0016316b std::sys::unix::abort_internal::ha5111d5275a9814a + 11
4   libstd-7f610d5e9627464e.dylib  0x00153bd9 std::sys_common::util::abort::hdff96fefabfe0dbc + 73
5   libstd-7f610d5e9627464e.dylib  0x0016240f std::sys::unix::stack_overflow::imp::signal_handler::h374d3af5ce3e1cc2 + 943
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-7f610d5e9627464e.dylib  0x00162060 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h4fdd21c580c680ac + 80
9   a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
10  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
11  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
12  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
13  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
14  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
15  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
16  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
17  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
18  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
19  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
20  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
21  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
22  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
23  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
24  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
25  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
26  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
27  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
28  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
29  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
30  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
31  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
32  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
33  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
34  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
35  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
36  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
37  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
38  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
39  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
40  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
41  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
42  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
43  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
44  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
45  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
46  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
47  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
48  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
49  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
50  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
51  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
52  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
53  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
54  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
55  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
56  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
57  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
58  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
59  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
60  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
61  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
62  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
63  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
64  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
65  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
66  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
67  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
68  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
69  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
70  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
71  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
72  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
73  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
74  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
75  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
76  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
77  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
78  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
79  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
80  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
81  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
82  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
83  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
84  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
85  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
86  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
87  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
88  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
89  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
90  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
91  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
92  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
93  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
94  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
95  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
96  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
97  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
98  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
99  a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
100 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
101 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
102 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
103 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
104 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
105 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
106 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
107 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
108 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
109 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
110 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
111 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
112 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
113 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
114 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
115 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
116 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
117 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
118 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
119 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
120 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
121 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
122 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
123 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
124 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
125 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
126 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
127 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
128 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
129 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
130 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
131 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
132 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
133 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
134 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
135 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
136 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
137 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
138 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
139 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
140 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
141 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
142 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
143 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
144 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
145 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
146 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
147 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
148 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
149 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
150 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
151 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
152 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
153 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
154 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
155 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
156 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
157 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
158 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
159 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
160 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
161 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
162 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
163 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
164 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
165 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
166 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
167 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
168 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
169 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
170 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
171 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
172 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
173 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
174 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
175 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
176 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
177 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
178 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
179 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
180 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
181 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
182 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
183 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
184 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
185 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
186 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
187 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
188 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
189 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
190 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
191 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
192 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
193 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
194 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
195 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
196 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
197 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
198 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
199 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
200 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
201 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
202 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
203 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
204 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
205 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
206 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
207 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
208 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
209 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
210 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
211 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
212 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
213 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
214 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
215 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
216 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
217 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
218 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
219 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
220 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
221 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
222 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
223 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
224 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
225 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
226 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
227 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
228 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
229 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
230 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
231 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
232 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
233 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
234 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
235 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
236 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
237 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
238 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
239 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
240 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
241 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
242 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
243 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
244 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
245 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
246 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
247 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
248 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
249 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
250 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
251 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
252 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
253 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
254 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
255 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
256 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
257 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
258 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
259 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
260 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
261 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
262 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
263 a                              0x0005e9c0 stack_probes::recurse::h35e743424666f0f3 + 48
264 a                              0x0005d73d std::sys_common::backtrace::__rust_begin_short_backtrace::h64ee07bddc87b34b + 29
265 libstd-7f610d5e9627464e.dylib  0x00163ced __rust_maybe_catch_panic + 29
266 a                              0x0005f9b3 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h0c86ad2265a55c61 + 131
267 libstd-7f610d5e9627464e.dylib  0x0013aa61 _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h022ae92e25896d0f + 65
268 libstd-7f610d5e9627464e.dylib  0x001628a8 std::sys::unix::thread::Thread::new::thread_start::haf22678fd9d44e69 + 184
269 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
270 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
271 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb030d000  ecx: 0x00477b0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x00477b38  esp: 0x00477b0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00162490
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0x5d000 -    0x60ffb +a (0) <E7BFF0E3-96AA-332E-BF7E-286CCCE958E2> /Users/USER/*/a
   0xab000 -    0xf0fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x134000 -   0x1c8ffb +libstd-7f610d5e9627464e.dylib (0) <223C5EB4-5E50-38EA-9A60-BB6302D40F2E> /Users/USER/*/libstd-7f610d5e9627464e.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2340
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=20.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=20.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3548K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9364K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            571.7M      137 
TOTAL                            571.7M      137 
TOTAL, minus reserved VM space   571.6M      137 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-13-010827_Traviss-Mac-1044.crash
Process:               a [48130]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [48125]
Responsible:           a [48130]
User ID:               501
Date/Time:             2019-06-13 01:08:25.635 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4900 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b03b7ec8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb03b7ec8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b03b7000-00000000b03b8000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b03b8000-00000000b05b9000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   a                              0x000a9f21 stack_probes_lto::main::haeaf2f44e7253541 + 2545
4   a                              0x000c628b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc70e2eda093e2029 + 11
5   a                              0x000b908c std::sys_common::backtrace::__rust_begin_short_backtrace::h019858fb3770b29a + 12
6   a                              0x000ab69d main + 589
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000aca2b std::sys::unix::abort_internal::ha5111d5275a9814a + 11
4   a                              0x000aca19 std::sys_common::util::abort::hdff96fefabfe0dbc + 73
5   a                              0x000b8fe3 std::sys::unix::stack_overflow::imp::signal_handler::h374d3af5ce3e1cc2 + 851
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000b8c90 rust_begin_unwind + 16
9   a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
10  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
11  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
12  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
13  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
14  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
15  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
16  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
17  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
18  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
19  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
20  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
21  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
22  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
23  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
24  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
25  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
26  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
27  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
28  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
29  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
30  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
31  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
32  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
33  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
34  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
35  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
36  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
37  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
38  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
39  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
40  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
41  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
42  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
43  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
44  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
45  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
46  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
47  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
48  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
49  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
50  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
51  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
52  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
53  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
54  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
55  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
56  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
57  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
58  a                              0x000aa428 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
