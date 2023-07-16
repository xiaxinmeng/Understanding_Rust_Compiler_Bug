plain
[00:03:13]       Memory: 8 GB
[00:03:13]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:13]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:13]       SMC Version (system): 2.8f0
[00:03:13]       Serial Number (system): VMfBCEMAMM/5
[00:03:13] 
[00:03:13] hw.ncpu: 4
[00:03:13] hw.byteorder: 1234
[00:03:13] hw.memsize: 8589934592
---
[02:01:50] 
[02:01:50] failures:
[02:01:50] 
[02:01:50] ---- /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/error-index.md - Rust_Compiler_Error_Index::E0370 (line 5311) stdout ----
[02:01:50] error: literal out of range for isize
[02:01:50]   |
[02:01:50] 4 |     X = 0x7fffffffffffffff,
[02:01:50]   |         ^^^^^^^^^^^^^^^^^^
[02:01:50]   |
[02:01:50]   |
[02:01:50]   = note: #[deny(overflowing_literals)] on by default
[02:01:50]   = note: the literal `0x7fffffffffffffff` (decimal `9223372036854775807`) does not fit into an `isize` and will become `-1isize`
[02:01:50] thread '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/error-index.md - Rust_Compiler_Error_Index::E0370 (line 5311)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[02:01:50] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[02:01:50] 
[02:01:50] ---- /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/error-index.md - Rust_Compiler_Error_Index::E0370 (line 5320) stdout ----
[02:01:50] ---- /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/error-index.md - Rust_Compiler_Error_Index::E0370 (line 5320) stdout ----
[02:01:50] error: literal out of range for isize
[02:01:50]   |
[02:01:50] 5 |     X = 0x7fffffffffffffff,
[02:01:50]   |         ^^^^^^^^^^^^^^^^^^
[02:01:50]   |
[02:01:50]   |
[02:01:50]   = note: #[deny(overflowing_literals)] on by default
[02:01:50]   = note: the literal `0x7fffffffffffffff` (decimal `9223372036854775807`) does not fit into an `isize` and will become `-1isize`
[02:01:50] thread '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/error-index.md - Rust_Compiler_Error_Index::E0370 (line 5320)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[02:01:50] 
[02:01:50] 
[02:01:50] failures:
---
[02:01:50] 
[02:01:50] 
[02:01:50] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:01:50] Build completed unsuccessfully in 0:52:31
[02:01:50] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00c5ff5b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 20 16:58:57 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:1f1610f6
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1120
drwx------  21 travis  staff    714 Feb 20 16:22 .
-rw-------@  1 travis  staff  57366 Feb 20 16:22 a_2019-02-20-162225-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34790 Feb 20 16:22 a_2019-02-20-162225_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34726 Feb 20 16:22 a_2019-02-20-162217-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55585 Feb 20 16:22 a_2019-02-20-162217_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 Feb 20 16:22 a_2019-02-20-162211_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9166 Feb 20 16:22 a_2019-02-20-162204_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9171 Feb 20 16:21 a_2019-02-20-162157_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8936 Feb 20 16:21 a_2019-02-20-162155_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9305 Feb 20 16:21 a_2019-02-20-162115_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58264 Feb 20 16:21 a_2019-02-20-162105_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59540 Feb 20 16:20 a_2019-02-20-162059-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60381 Feb 20 16:20 a_2019-02-20-162059-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 Feb 20 16:20 a_2019-02-20-162059_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10890 Feb 20 16:18 a_2019-02-20-161840_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 Feb 20 16:17 a_2019-02-20-161743_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 Feb 20 16:16 a_2019-02-20-161626_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 Feb 20 16:15 a_2019-02-20-161528-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9780 Feb 20 16:15 a_2019-02-20-161528_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9484 Feb 20 16:15 a_2019-02-20-161524_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:28ef8822
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-02-20-161524_Traviss-Mac-1044.crash
Process:               a [37039]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [37038]
Responsible:           a [37039]
User ID:               501
Date/Time:             2019-02-20 16:14:53.829 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000cda1e abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 46
1   a                              0x000cce6b std::panicking::try::do_call::h1264aa3b2ac9b2e7 (.llvm.13811582661879557506) + 11
2   libstd-3494f61bb4979964.dylib  0x0021802d __rust_maybe_catch_panic + 29
3   a                              0x000cdc85 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 613
4   a                              0x000cc4cb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h05126edae96e3b3f + 11
5   libstd-3494f61bb4979964.dylib  0x00206d7c std::sys_common::backtrace::__rust_begin_short_backtrace::h2d2e61db5180bd6a + 12
6   libstd-3494f61bb4979964.dylib  0x002091a4 std::panicking::try::do_call::h5093eb873c82e7fb + 20
7   libstd-3494f61bb4979964.dylib  0x0021802d __rust_maybe_catch_panic + 29
8   libstd-3494f61bb4979964.dylib  0x00209c47 std::rt::lang_start_internal::he96e9276ae2d7d1a + 631
9   a                              0x000cdfbc main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x7872a7d0  ebx: 0xbff34128  ecx: 0x00000000  edx: 0x00000000
  edi: 0x0021801e  esi: 0x00000000  ebp: 0xbff340c8  esp: 0xbff340b0
   ss: 0x00000023  efl: 0x00010292  eip: 0x000cda1e   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0025171c
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xcb000 -    0xceff3 +a (0) <F17D2D2B-89AD-375B-9A8F-746C00BD863B> /Users/USER/*/a
  0x160000 -   0x1a5fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1e9000 -   0x277ffb +libstd-3494f61bb4979964.dylib (0) <598CD2DD-712A-388D-8E6D-CC6364AC6AF0> /Users/USER/*/libstd-3494f61bb4979964.dylib
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
    task_for_pid: 2390
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
Writable regions: Total=74.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=74.3M(100%)
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
__DATA                            3520K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9340K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            569.5M      134 
TOTAL                            569.5M      134 
TOTAL, minus reserved VM space   569.4M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-02-20-161528-1_Traviss-Mac-1044.crash
Process:               a [37853]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [37846]
Responsible:           a [37853]
User ID:               501
Date/Time:             2019-02-20 16:15:26.311 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-3494f61bb4979964.dylib  0x001f26b3 std::panicking::rust_panic_with_hook::hcc322b8644c66ddd + 115
1   a                              0x000bebef std::panicking::begin_panic::hd091b7c71d4be77d + 47 (panicking.rs:412)
2   a                              0x000bc6d4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::h5ed13b737dc2b5ab + 36 (backtrace.rs:24)
3   a                              0x000bbedb core::ptr::real_drop_in_place::h457ec08aeb6814a9 + 11
4   a                              0x000bc6a3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x000bd9b8 backtrace::main::hcde7a1a1c3c85e77 + 4568 (backtrace.rs:103)
6   a                              0x000bbbab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h8316194a2e873d0a + 11 (rt.rs:64)
7   libstd-3494f61bb4979964.dylib  0x001efd7c std::sys_common::backtrace::__rust_begin_short_backtrace::h2d2e61db5180bd6a + 12
8   libstd-3494f61bb4979964.dylib  0x001f21a4 std::panicking::try::do_call::h5093eb873c82e7fb + 20
9   libstd-3494f61bb4979964.dylib  0x0020102d __rust_maybe_catch_panic + 29
10  libstd-3494f61bb4979964.dylib  0x001f2c47 std::rt::lang_start_internal::he96e9276ae2d7d1a + 631
11  a                              0x000be20c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff45e88  ebx: 0xbff45ed0  ecx: 0xbff45d80  edx: 0xa7702ec6
  edi: 0x0023a98c  esi: 0x001f264e  ebp: 0xbff45f28  esp: 0xbff45ea0
   ss: 0x00000023  efl: 0x00010282  eip: 0x001f26b3   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x08daf000
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xb9000 -    0xbfff7 +a (0) <86C9050F-D31D-31DC-9386-EA9ED6D97C8D> /Users/USER/*/a
  0x149000 -   0x18efdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1d2000 -   0x260ffb +libstd-3494f61bb4979964.dylib (0) <598CD2DD-712A-388D-8E6D-CC6364AC6AF0> /Users/USER/*/libstd-3494f61bb4979964.dylib
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
    task_for_pid: 2390
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=82.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=82.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.4M       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3520K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9352K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            577.9M      135 
TOTAL                            577.9M      135 
TOTAL, minus reserved VM space   577.8M      135 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-02-20-161528_Traviss-Mac-1044.crash
Process:               a [37852]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [37846]
Responsible:           a [37852]
User ID:               501
Date/Time:             2019-02-20 16:15:25.877 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-3494f61bb4979964.dylib  0x001ae6b3 std::panicking::rust_panic_with_hook::hcc322b8644c66ddd + 115
1   a                              0x00074bef std::panicking::begin_panic::hd091b7c71d4be77d + 47 (panicking.rs:412)
2   a                              0x000726d4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::h5ed13b737dc2b5ab + 36 (backtrace.rs:24)
3   a                              0x00071edb core::ptr::real_drop_in_place::h457ec08aeb6814a9 + 11
4   a                              0x000726a3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x000739b8 backtrace::main::hcde7a1a1c3c85e77 + 4568 (backtrace.rs:103)
6   a                              0x00071bab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h8316194a2e873d0a + 11 (rt.rs:64)
7   libstd-3494f61bb4979964.dylib  0x001abd7c std::sys_common::backtrace::__rust_begin_short_backtrace::h2d2e61db5180bd6a + 12
8   libstd-3494f61bb4979964.dylib  0x001ae1a4 std::panicking::try::do_call::h5093eb873c82e7fb + 20
9   libstd-3494f61bb4979964.dylib  0x001bd02d __rust_maybe_catch_panic + 29
10  libstd-3494f61bb4979964.dylib  0x001aec47 std::rt::lang_start_internal::he96e9276ae2d7d1a + 631
11  a                              0x0007420c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff8fe98  ebx: 0xbff8fee0  ecx: 0xbff8fd90  edx: 0xa7702ec6
  edi: 0x001f698c  esi: 0x001ae64e  ebp: 0xbff8ff38  esp: 0xbff8feb0
   ss: 0x00000023  efl: 0x00010282  eip: 0x001ae6b3   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00471663
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x6f000 -    0x75ff7 +a (0) <86C9050F-D31D-31DC-9386-EA9ED6D97C8D> /Users/USER/*/a
  0x105000 -   0x14afdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x18e000 -   0x21cffb +libstd-3494f61bb4979964.dylib (0) <598CD2DD-712A-388D-8E6D-CC6364AC6AF0> /Users/USER/*/libstd-3494f61bb4979964.dylib
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
    task_for_pid: 2390
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=73.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9612K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3520K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9352K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            568.9M      135 
TOTAL                            568.9M      135 
TOTAL, minus reserved VM space   568.8M      135 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-02-20-161626_Traviss-Mac-1044.crash
Process:               a [39510]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [39508]
Responsible:           a [39510]
User ID:               501
Date/Time:             2019-02-20 16:16:24.820 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5000 seconds
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
3   libstd-3494f61bb4979964.dylib  0x001744ab std::sys::unix::abort_internal::h6c504aa00bb24e4d + 11
4   libstd-3494f61bb4979964.dylib  0x00165650 rust_oom + 48
5   libstd-3494f61bb4979964.dylib  0x00186cc4 alloc::alloc::handle_alloc_error::h69bf1225c032a73a + 20
6   a                              0x0005828d default_alloc_error_hook::main::hbf2d06db626d002e + 781
7   a                              0x0005883b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf702d0c2f1207d97 + 11
8   libstd-3494f61bb4979964.dylib  0x00163d7c std::sys_common::backtrace::__rust_begin_short_backtrace::h2d2e61db5180bd6a + 12
9   libstd-3494f61bb4979964.dylib  0x001661a4 std::panicking::try::do_call::h5093eb873c82e7fb + 20
10  libstd-3494f61bb4979964.dylib  0x0017502d __rust_maybe_catch_panic + 29
11  libstd-3494f61bb4979964.dylib  0x00166c47 std::rt::lang_start_internal::he96e9276ae2d7d1a + 631
12  a                              0x000583ec main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffa804c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffa8078  esp: 0xbffa804c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x57000 -    0x58ff3 +a (0) <B66DBA6F-61A0-3693-9BBE-DAC83999E3BC> /Users/USER/*/a
   0xbd000 -   0x102fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x146000 -   0x1d4ffb +libstd-3494f61bb4979964.dylib (0) <598CD2DD-712A-388D-8E6D-CC6364AC6AF0> /Users/USER/*/libstd-3494f61bb4979964.dylib
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
TOTAL                            568.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-02-20-162105_Traviss-Mac-1044.crash
Process:               a [45837]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [45831]
Responsible:           a [45837]
User ID:               501
Date/Time:             2019-02-20 16:21:05.595 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5300 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf48ffc
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf48ffc:
    Stack Guard            00000000bbf47000-00000000bbf48000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf48000-00000000bbf49000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf49000-00000000bff47000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-3494f61bb4979964.dylib  0x002344ab std::sys::unix::abort_internal::h6c504aa00bb24e4d + 11
4   libstd-3494f61bb4979964.dylib  0x002248c9 std::sys_common::util::abort::h71a3cfc175de1003 + 73
5   libstd-3494f61bb4979964.dylib  0x00233928 std::sys::unix::stack_overflow::imp::signal_handler::hbbeeb5fd3bed4cd1 + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-3494f61bb4979964.dylib  0x00233570 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hd2b14336ff0ba1b2 + 80
9   libstd-3494f61bb4979964.dylib  0x00214d11 _$LT$std..io..stdio..StdoutLock$LT$$u27$_$GT$$u20$as$u20$std..io..Write$GT$::write::ha2808bcdd94c535e + 193
10  libstd-3494f61bb4979964.dylib  0x00216a27 std::io::Write::write_all::hc4089658ecd0248f + 71
11  libstd-3494f61bb4979964.dylib  0x00216d43 _$LT$std..io..Write..write_fmt..Adaptor$LT$$u27$_$C$$u20$T$GT$$u20$as$u20$core..fmt..Write$GT$::write_str::h0869d8c9ce8ab2a7 + 35
12  libstd-3494f61bb4979964.dylib  0x002549bd core::fmt::write::ha8eb31e9539b84e4 + 749
13  libstd-3494f61bb4979964.dylib  0x00214b26 _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::h50185f294186fc36 + 182
14  libstd-3494f61bb4979964.dylib  0x00215e7c std::io::stdio::_print::ha4c6cb42689179af + 396
15  a                              0x000bbccf out_of_stack::loud_recurse::hcd528ebf130a94fa + 63
16  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
17  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
18  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
19  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
20  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
21  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
22  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
23  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
24  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
25  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
26  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
27  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
28  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
29  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
30  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
31  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
32  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
33  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
34  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
35  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
36  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
37  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
38  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
39  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
40  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
41  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
42  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
43  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
44  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
45  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
46  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
47  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
48  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
49  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
50  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
51  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
52  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
53  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
54  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
55  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
56  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
57  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
58  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
59  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
60  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
61  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
62  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
63  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
64  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
65  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
66  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
67  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
68  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
69  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
70  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
71  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
72  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
73  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
74  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
75  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
76  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
77  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
78  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
79  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
80  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
81  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
82  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
83  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
84  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
85  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
86  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
87  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
88  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
89  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
90  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
91  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
92  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
93  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
94  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
95  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
96  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
97  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
98  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
99  a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
100 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
101 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
102 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
103 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
104 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
105 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
106 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
107 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
108 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
109 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
110 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
111 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
112 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
113 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
114 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
115 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
116 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
117 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
118 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
119 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
120 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
121 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
122 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
123 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
124 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
125 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
126 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
127 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
128 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
129 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
130 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
131 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
132 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
133 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
134 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
135 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
136 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
137 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
138 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
139 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
140 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
141 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
142 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
143 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
144 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
145 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
146 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
147 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
148 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
149 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
150 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
151 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
152 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
153 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
154 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
155 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
156 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
157 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
158 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
159 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
160 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
161 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
162 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
163 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
164 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
165 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
166 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
167 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
168 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
169 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
170 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
171 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
172 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
173 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
174 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
175 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
176 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
177 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
178 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
179 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
180 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
181 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
182 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
183 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
184 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
185 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
186 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
187 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
188 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
189 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
190 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
191 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
192 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
193 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
194 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
195 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
196 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
197 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
198 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
199 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
200 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
201 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
202 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
203 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
204 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
205 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
206 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
207 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
208 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
209 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
210 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
211 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
212 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
213 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
214 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
215 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
216 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
217 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
218 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
219 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
220 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
221 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
222 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
223 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
224 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
225 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
226 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
227 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
228 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
229 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
230 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
231 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
232 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
233 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
234 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
235 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
236 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
237 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
238 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
239 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
240 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
241 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
242 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
243 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
244 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
245 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
246 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
247 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
248 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
249 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
250 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
251 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
252 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
253 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
254 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
255 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
256 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
257 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
258 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
259 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
260 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
261 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
262 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
263 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
264 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
265 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
266 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
267 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
268 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
269 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
270 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
271 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
272 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
273 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
274 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
275 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
276 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
277 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
278 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
279 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
280 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
281 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
282 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
283 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
284 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
285 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
286 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
287 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
288 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
289 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
290 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
291 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
292 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
293 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
294 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
295 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
296 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
297 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
298 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
299 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
300 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
301 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
302 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
303 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
304 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
305 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
306 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
307 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
308 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
309 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
310 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
311 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
312 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
313 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
314 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
315 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
316 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
317 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
318 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
319 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
320 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
321 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
322 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
323 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
324 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
325 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
326 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
327 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
328 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
329 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
330 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
331 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
332 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
333 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
334 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
335 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
336 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
337 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
338 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
339 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
340 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
341 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
342 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
343 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
344 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
345 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
346 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
347 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
348 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
349 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
350 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
351 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
352 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
353 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
354 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
355 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
356 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
357 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
358 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
359 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
360 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
361 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
362 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
363 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
364 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
365 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
366 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
367 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
368 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
369 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
370 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
371 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
372 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
373 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
374 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
375 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
376 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
377 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
378 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
379 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
380 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
381 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
382 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
383 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
384 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
385 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
386 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
387 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
388 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
389 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
390 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
391 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
392 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
393 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
394 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
395 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
396 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
397 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
398 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
399 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
400 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
401 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
402 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
403 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
404 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
405 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
406 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
407 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
408 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
409 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
410 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
411 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
412 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
413 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
414 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
415 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
416 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
417 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
418 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
419 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
420 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
421 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
422 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
423 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
424 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
425 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
426 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
427 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
428 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
429 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
430 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
431 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
432 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
433 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
434 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
435 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
436 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
437 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
438 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
439 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
440 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
441 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
442 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
443 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
444 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
445 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
446 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
447 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
448 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
449 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
450 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
451 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
452 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
453 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
454 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
455 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
456 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
457 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
458 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
459 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
460 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
461 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
462 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
463 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
464 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
465 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
466 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
467 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
468 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
469 a                              0x000bbcd4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
---
===========                     =======  ======= 
TOTAL                            568.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-02-20-162115_Traviss-Mac-1044.crash
Process:               a [46063]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [46061]
Responsible:           a [46063]
User ID:               501
Date/Time:             2019-02-20 16:21:14.649 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5300 seconds
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
3   a                              0x0009c65b panic_abort::__rust_start_panic::abort::h04b844de1cba291f + 11
4   a                              0x0009c64b __rust_start_panic + 11
5   a                              0x00090e4b rust_panic + 11
6   a                              0x000909e5 std::panicking::rust_panic_with_hook::hcc322b8644c66ddd + 1301
7   a                              0x000a241a std::panicking::begin_panic::h3b08eea10eb485c3 + 42
8   a                              0x0008f6cf lto_abort::main::h9419a0043b6e0505 + 2895
9   a                              0x000a24eb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h26b731308b719d2b + 11
10  a                              0x0009c4cc std::sys_common::backtrace::__rust_begin_short_backtrace::h2d2e61db5180bd6a + 12
11  a                              0x0008faa8 main + 984
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff7101c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff71048  esp: 0xbff7101c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x8e000 -    0xb1ffb +a (0) <603CB435-D470-3F3B-8AFE-2EE7D1BA0F6B> /Users/USER/*/a
  0x10a000 -   0x14ffdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2390
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=82.3M resident=0K(0%) swapped_out_or_unallocated=82.3M(100%)
Writable regions: Total=73.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            1336K       43 
__LINKEDIT                        73.7M        4 
__OBJC                              36K        6 
__TEXT                            8896K       43 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            565.6M      131 
TOTAL                            565.6M      131 
TOTAL, minus reserved VM space   565.5M      131 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-02-20-162155_Traviss-Mac-1044.crash
Process:               a [47025]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47020]
Responsible:           a [47025]
User ID:               501
Date/Time:             2019-02-20 16:21:54.512 +0000
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
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-3494f61bb4979964.dylib  0x001674ab std::sys::unix::abort_internal::h6c504aa00bb24e4d + 11
4   libstd-3494f61bb4979964.dylib  0x001578c9 std::sys_common::util::abort::h71a3cfc175de1003 + 73
5   libstd-3494f61bb4979964.dylib  0x001599c2 rust_panic + 98
6   libstd-3494f61bb4979964.dylib  0x0015988e std::panicking::rust_panic_with_hook::hcc322b8644c66ddd + 590
7   a                              0x0005c9af std::panicking::begin_panic::h75b6999e27bb83a7 + 47
8   a                              0x0005dedc main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffa401c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffa4048  esp: 0xbffa401c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x5b000 -    0x5efff +a (0) <CD8FB52A-A43E-39A9-8986-C9E45241B30D> /Users/USER/*/a
   0xb0000 -    0xf5fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x139000 -   0x1c7ffb +libstd-3494f61bb4979964.dylib (0) <598CD2DD-712A-388D-8E6D-CC6364AC6AF0> /Users/USER/*/libstd-3494f61bb4979964.dylib
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
    task_for_pid: 2639
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
Writable regions: Total=83.2M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=83.2M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            19.0M        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
__DATA                            3520K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__OBJC                              36K        6 
__TEXT                            9340K       44 
mapped file                      408.7M       21 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            578.4M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-02-20-162157_Traviss-Mac-1044.crash
Process:               a [47057]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47056]
Responsible:           a [47057]
User ID:               501
Date/Time:             2019-02-20 16:21:55.673 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGSEGV)
Exception Codes:       KERN_INVALID_ADDRESS at 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Segmentation fault: 11
Termination Reason:    Namespace SIGNAL, Code 0xb
Terminating Process:   exc handler [0]
VM Regions Near 0:
--> 
    __TEXT                 00000000000a6000-00000000000a9000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000a7f72 segfault_no_out_of_stack::main::haec5f9680e9a04a3 + 2034
1   a                              0x000a699b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h16d3efcdf1d9e54f + 11
2   libstd-3494f61bb4979964.dylib  0x001d8d7c std::sys_common::backtrace::__rust_begin_short_backtrace::h2d2e61db5180bd6a + 12
3   libstd-3494f61bb4979964.dylib  0x001db1a4 std::panicking::try::do_call::h5093eb873c82e7fb + 20
4   libstd-3494f61bb4979964.dylib  0x001ea02d __rust_maybe_catch_panic + 29
5   libstd-3494f61bb4979964.dylib  0x001dbc47 std::rt::lang_start_internal::he96e9276ae2d7d1a + 631
6   a                              0x000a824c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x79e21a80  ecx: 0x00000000  edx: 0x00000000
  edi: 0x001ea01e  esi: 0xbff59150  ebp: 0xbff59238  esp: 0xbff59090
   ss: 0x00000023  efl: 0x00010246  eip: 0x000a7f72   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     0
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xa6000 -    0xa8ffb +a (0) <131B0CA3-2CF0-3EE2-815B-48F4A5EA6BEE> /Users/USER/*/a
  0x132000 -   0x177fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1bb000 -   0x249ffb +libstd-3494f61bb4979964.dylib (0) <598CD2DD-712A-388D-8E6D-CC6364AC6AF0> /Users/USER/*/libstd-3494f61bb4979964.dylib
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
    task_for_pid: 2639
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
Writable regions: Total=73.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                        132K        3 
__DATA                            3520K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-02-20-162204_Traviss-Mac-1044.crash
Process:               a [47233]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47232]
Responsible:           a [47233]
User ID:               501
Date/Time:             2019-02-20 16:22:04.076 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
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
    __TEXT                 000000000002f000-0000000000032000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000315d4 signal_exit_status::main::h013a3960fc5c363d + 436
1   a                              0x0003047b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::he2cf2a86c3ebd4b8 + 11
2   libstd-3494f61bb4979964.dylib  0x001a0d7c std::sys_common::backtrace::__rust_begin_short_backtrace::h2d2e61db5180bd6a + 12
3   libstd-3494f61bb4979964.dylib  0x001a31a4 std::panicking::try::do_call::h5093eb873c82e7fb + 20
4   libstd-3494f61bb4979964.dylib  0x001b202d __rust_maybe_catch_panic + 29
5   libstd-3494f61bb4979964.dylib  0x001a3c47 std::rt::lang_start_internal::he96e9276ae2d7d1a + 631
6   a                              0x000316ac main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x7be59af0
  edi: 0x7be59b80  esi: 0xbffd01b0  ebp: 0xbffd0248  esp: 0xbffd0130
   ss: 0x00000023  efl: 0x00010246  eip: 0x000315d4   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     1
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x2f000 -    0x31fff +a (0) <8FE33232-9E4C-3123-B01D-DD50AF22456A> /Users/USER/*/a
   0xfa000 -   0x13ffdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x183000 -   0x211ffb +libstd-3494f61bb4979964.dylib (0) <598CD2DD-712A-388D-8E6D-CC6364AC6AF0> /Users/USER/*/libstd-3494f61bb4979964.dylib
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
    task_for_pid: 2639
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
Writable regions: Total=73.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                        132K        3 
__DATA                            3520K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-02-20-162211_Traviss-Mac-1044.crash
Process:               a [47339]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47331]
Responsible:           a [47339]
User ID:               501
Date/Time:             2019-02-20 16:22:10.472 +0000
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
0   a                              0x00034c36 simd_target_feature_mixup::test::id_avx512_512::h5d16af711813be1e + 102
1   a                              0x000339ef simd_target_feature_mixup::test::main::h379367934b9623dc + 1647
2   a                              0x00035fe0 simd_target_feature_mixup::main::h4f60990077aff357 + 896
3   a                              0x000363cb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hed385be97fd1eada + 11
4   libstd-3494f61bb4979964.dylib  0x0018cd7c std::sys_common::backtrace::__rust_begin_short_backtrace::h2d2e61db5180bd6a + 12
5   libstd-3494f61bb4979964.dylib  0x0018f1a4 std::panicking::try::do_call::h5093eb873c82e7fb + 20
6   libstd-3494f61bb4979964.dylib  0x0019e02d __rust_maybe_catch_panic + 29
7   libstd-3494f61bb4979964.dylib  0x0018fc47 std::rt::lang_start_internal::he96e9276ae2d7d1a + 631
8   a                              0x000361bc main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbffcce80  ebx: 0xbffcce00  ecx: 0x00034bde  edx: 0xbffcce00
  edi: 0x00033394  esi: 0x00000000  ebp: 0xbffccdf8  esp: 0xbffccdc0
   ss: 0x00000023  efl: 0x00010246  eip: 0x00034c36   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00034870
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x32000 -    0x36fff +a (0) <2D1EF600-F232-3B81-8852-E6E6A2778DCF> /Users/USER/*/a
   0xe6000 -   0x12bfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x16f000 -   0x1fdffb +libstd-3494f61bb4979964.dylib (0) <598CD2DD-712A-388D-8E6D-CC6364AC6AF0> /Users/USER/*/libstd-3494f61bb4979964.dylib
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
    task_for_pid: 2639
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=73.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3520K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9344K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            568.5M      133 
TOTAL                            568.5M      133 
TOTAL, minus reserved VM space   568.4M      133 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-02-20-162217-1_Traviss-Mac-1044.crash
Process:               a [47478]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47476]
Responsible:           a [47478]
User ID:               501
Date/Time:             2019-02-20 16:22:16.621 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b00c2ea8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb00c2ea8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b00c2000-00000000b00c3000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b00c3000-00000000b02c4000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-3494f61bb4979964.dylib  0x0018ee90 std::sys::unix::thread::Thread::join::hf99a401ab18ebcf7 + 32
4   a                              0x00067826 _$LT$std..thread..JoinHandle$LT$T$GT$$GT$::join::h63373167649346f4 + 70
5   a                              0x000667f5 stack_probes::main::hc5f49a55fd7e038b + 597
6   a                              0x0006560b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h01a566ba33fdecb6 + 11
7   libstd-3494f61bb4979964.dylib  0x0017ed7c std::sys_common::backtrace::__rust_begin_short_backtrace::h2d2e61db5180bd6a + 12
8   libstd-3494f61bb4979964.dylib  0x001811a4 std::panicking::try::do_call::h5093eb873c82e7fb + 20
9   libstd-3494f61bb4979964.dylib  0x0019002d __rust_maybe_catch_panic + 29
10  libstd-3494f61bb4979964.dylib  0x00181c47 std::rt::lang_start_internal::he96e9276ae2d7d1a + 631
11  a                              0x0006721c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-3494f61bb4979964.dylib  0x0018f4ab std::sys::unix::abort_internal::h6c504aa00bb24e4d + 11
4   libstd-3494f61bb4979964.dylib  0x0017f8c9 std::sys_common::util::abort::h71a3cfc175de1003 + 73
5   libstd-3494f61bb4979964.dylib  0x0018e928 std::sys::unix::stack_overflow::imp::signal_handler::hbbeeb5fd3bed4cd1 + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-3494f61bb4979964.dylib  0x0018e570 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hd2b14336ff0ba1b2 + 80
9   a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x00066940 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x0006566d std::sys_common::backtrace::__rust_begin_short_backtrace::he721cfccf27d8bab (.llvm.8285367932403481863) + 29
265 libstd-3494f61bb4979964.dylib  0x0019002d __rust_maybe_catch_panic + 29
266 a                              0x00067b03 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hfe9aee6e9bb7e57f + 131
267 libstd-3494f61bb4979964.dylib  0x0018edcb std::sys::unix::thread::Thread::new::thread_start::h1c1084152d5c249d + 187
268 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
269 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
270 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb02c3000  ecx: 0x000bab0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x000bab38  esp: 0x000bab0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0018e9b0
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0x64000 -    0x68ff7 +a (0) <6E077AEE-542C-3F96-9FE0-8829E70CCA81> /Users/USER/*/a
   0xd8000 -   0x11dfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x161000 -   0x1efffb +libstd-3494f61bb4979964.dylib (0) <598CD2DD-712A-388D-8E6D-CC6364AC6AF0> /Users/USER/*/libstd-3494f61bb4979964.dylib
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
---
===========                     =======  ======= 
TOTAL                            568.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-02-20-162225-1_Traviss-Mac-1044.crash
Process:               a [47610]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47608]
Responsible:           a [47610]
User ID:               501
Date/Time:             2019-02-20 16:22:24.830 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf76298
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf76298:
    Stack Guard            00000000bbf75000-00000000bbf76000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf76000-00000000bbf77000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf77000-00000000bff75000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000905fb std::sys::unix::abort_internal::h6c504aa00bb24e4d + 11
4   a                              0x000905e9 std::sys_common::util::abort::h71a3cfc175de1003 + 73
5   a                              0x0009ed4c std::sys::unix::stack_overflow::imp::signal_handler::hbbeeb5fd3bed4cd1 + 860
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x0009e9f0 rust_begin_unwind + 16
9   a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
10  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
11  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
12  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
13  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
14  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
15  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
16  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
17  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
18  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
19  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
20  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
21  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
22  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
23  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
24  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
25  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
26  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
27  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
28  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
29  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
30  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
31  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
32  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
33  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
34  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
35  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
36  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
37  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
38  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
39  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
40  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
41  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
42  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
43  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
44  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
45  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
46  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
47  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
48  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
49  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
50  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
51  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
52  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
53  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
54  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
55  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
56  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
57  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
58  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
59  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
60  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
61  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
62  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
63  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
64  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
65  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
66  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
67  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
68  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
69  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
70  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
71  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
72  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
73  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
74  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
75  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
76  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
77  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
78  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
79  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
80  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
81  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
82  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
83  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
84  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
85  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
86  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
87  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
88  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
89  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
90  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
91  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
92  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
93  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
94  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
95  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
96  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
97  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
98  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
99  a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
100 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
101 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
102 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
103 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
104 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
105 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
106 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
107 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
108 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
109 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
110 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
111 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
112 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
113 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
114 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
115 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
116 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
117 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
118 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
119 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
120 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
121 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
122 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
123 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
124 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
125 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
126 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
127 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
128 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
129 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
130 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
131 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
132 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
133 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
134 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
135 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
136 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
137 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
138 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
139 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
140 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
141 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
142 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
143 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
144 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
145 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
146 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
147 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
148 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
149 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
150 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
151 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
152 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
153 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
154 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
155 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
156 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
157 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
158 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
159 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
160 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
161 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
162 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
163 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
164 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
165 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
166 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
167 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
168 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
169 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
170 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
171 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
172 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
173 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
174 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
175 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
176 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
177 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
178 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
179 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
180 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
181 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
182 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
183 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
184 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
185 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
186 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
187 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
188 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
189 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
190 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
191 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
192 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
193 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
194 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
195 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
196 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
197 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
198 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
199 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
200 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
201 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
202 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
203 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
204 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
205 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
206 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
207 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
208 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
209 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
210 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
211 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
212 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
213 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
214 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
215 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
216 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
217 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
218 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
219 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
220 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
221 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
222 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
223 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
224 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
225 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
226 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
227 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
228 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
229 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
230 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
231 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
232 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
233 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
234 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
235 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
236 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
237 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
238 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
239 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
240 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
241 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
242 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
243 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
244 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
245 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
246 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
247 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
248 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
249 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
250 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
251 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
252 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
253 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
254 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
255 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
256 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
257 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
258 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
259 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
260 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
261 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
262 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
263 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
264 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
265 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
266 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
267 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
268 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
269 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
270 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
271 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
272 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
273 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
274 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
275 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
276 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
277 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
278 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
279 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
280 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
281 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
282 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
283 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
284 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
285 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
286 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
287 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
288 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
289 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
290 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
291 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
292 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
293 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
294 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
295 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
296 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
297 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
298 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
299 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
300 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
301 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
302 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
303 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
304 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
305 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
306 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
307 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
308 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
309 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
310 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
311 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
312 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
313 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
314 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
315 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
316 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
317 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
318 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
319 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
320 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
321 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
322 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
323 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
324 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
325 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
326 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
327 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
328 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
329 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
330 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
331 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
332 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
333 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
334 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
335 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
336 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
337 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
338 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
339 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
340 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
341 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
342 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
343 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
344 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
345 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
346 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
347 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
348 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
349 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
350 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
351 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
352 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
353 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
354 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
355 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
356 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
357 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
358 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
359 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
360 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
361 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
362 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
363 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
364 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
365 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
366 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
367 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
368 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
369 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
370 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
371 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
372 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
373 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
374 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
375 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
376 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
377 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
378 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
379 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
380 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
381 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
382 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
383 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
384 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
385 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
386 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
387 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
388 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
389 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
390 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
391 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
392 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
393 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
394 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
395 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
396 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
397 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
398 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
399 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
400 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
401 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
402 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
403 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
404 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
405 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
406 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
407 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
408 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
409 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
410 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
411 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
412 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
413 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
414 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
415 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
416 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
417 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
418 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
419 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
420 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
421 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
422 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
423 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
424 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
425 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
426 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
427 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
428 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
429 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
430 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
431 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
432 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
433 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
434 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
435 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
436 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
437 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
438 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
439 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
440 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
441 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
442 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
443 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
444 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
445 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
446 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
447 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
448 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
449 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
450 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
451 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
452 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
453 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
454 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
455 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
456 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
457 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
458 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
459 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
460 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
461 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
462 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
463 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
464 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
465 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
466 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
467 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
468 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
469 a                              0x0008ddc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
