plain
[00:02:58]       Memory: 8 GB
[00:02:58]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:02:58]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:58]       SMC Version (system): 2.8f0
[00:02:58]       Serial Number (system): VMxJdlRSpDIv
[00:02:58] 
[00:02:58] hw.ncpu: 4
[00:02:58] hw.byteorder: 1234
[00:02:58] hw.memsize: 8589934592
---
[02:03:50] 
[02:03:50] ---- /Users/travis/build/rust-lang/rust/src/doc/rustdoc/src/documentation-tests.md - Documentation_tests::Documenting_macros (line 263) stdout ----
[02:03:50] error: linking with `cc` failed: signal: 4
[02:03:50]   |
[02:03:50]   = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestJoCE0N/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestJoCE0N/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestJoCE0N/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestJoCE0N/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestJoCE0N/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestJoCE0N/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestJoCE0N/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libstd-4ed43f4a317eb5c7.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libpanic_unwind-e9b55ddd85288dd7.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libbacktrace_sys-fd91892fbba08832.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_demangle-22a2db5c4c91d0ac.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libhashbrown-bbaa8196d55fa327.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_alloc-f544ab4a1582d7f1.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libunwind-260102d0cb679fa6.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liblibc-bc4cb3542854703f.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liballoc-d3223110ebde7328.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_core-4c9b9af89be4fb18.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcore-f01eb28d23d35654.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-708605a54f61eb16.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:03:50] 
[02:03:50] error: aborting due to previous error
[02:03:50] 
[02:03:50] thread '/Users/travis/build/rust-lang/rust/src/doc/rustdoc/src/documentation-tests.md - Documentation_tests::Documenting_macros (line 263)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[02:03:50] 
[02:03:50] 
[02:03:50] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:03:50] Build completed unsuccessfully in 0:48:35
[02:03:50] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04f85344
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 30 08:55:06 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:0fac0d6a
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1176
-rw-------@  1 travis  staff   1387 Apr 30 08:54 foo_2019-04-30-085442_Traviss-Mac-1044.crash
drwx------  26 travis  staff    884 Apr 30 08:54 .
-rw-------@  1 travis  staff   1376 Apr 30 08:54 m4_2019-04-30-085414_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1391 Apr 30 08:54 bar_2019-04-30-085405_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9899 Apr 30 08:54 b_2019-04-30-085404_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1403 Apr 30 08:54 bar_2019-04-30-085404_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  57366 Apr 30 08:22 a_2019-04-30-082204_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34964 Apr 30 08:22 a_2019-04-30-082203_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34892 Apr 30 08:21 a_2019-04-30-082156-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55585 Apr 30 08:21 a_2019-04-30-082156_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 Apr 30 08:21 a_2019-04-30-082150_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9166 Apr 30 08:21 a_2019-04-30-082145_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9171 Apr 30 08:21 a_2019-04-30-082138_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8936 Apr 30 08:21 a_2019-04-30-082137_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9304 Apr 30 08:21 a_2019-04-30-082101_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58249 Apr 30 08:20 a_2019-04-30-082054_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60372 Apr 30 08:20 a_2019-04-30-082048-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 Apr 30 08:20 a_2019-04-30-082048-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59516 Apr 30 08:20 a_2019-04-30-082048_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10879 Apr 30 08:18 a_2019-04-30-081847_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 Apr 30 08:17 a_2019-04-30-081747_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 Apr 30 08:16 a_2019-04-30-081636_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9780 Apr 30 08:15 a_2019-04-30-081537-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 Apr 30 08:15 a_2019-04-30-081537_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9482 Apr 30 08:15 a_2019-04-30-081533_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:16c5b448
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-30-081533_Traviss-Mac-1044.crash
Process:               a [40244]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [40243]
Responsible:           a [40244]
User ID:               501
Date/Time:             2019-04-30 08:15:08.911 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5100 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000eab0e abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 46
1   a                              0x000e9f5b std::panicking::try::do_call::hf7aa993ff875b21e (.llvm.801301855777072572) + 11
2   libstd-4ed43f4a317eb5c7.dylib  0x002952dd __rust_maybe_catch_panic + 29
3   a                              0x000ead75 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 613
4   a                              0x000e95bb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h1009499ed4d844c5 + 11
5   libstd-4ed43f4a317eb5c7.dylib  0x00283f3c std::sys_common::backtrace::__rust_begin_short_backtrace::hdddb286332b79fb7 + 12
6   libstd-4ed43f4a317eb5c7.dylib  0x00286a94 std::panicking::try::do_call::h5331467eede3c05e + 20
7   libstd-4ed43f4a317eb5c7.dylib  0x002952dd __rust_maybe_catch_panic + 29
8   libstd-4ed43f4a317eb5c7.dylib  0x00287537 std::rt::lang_start_internal::he35be8db163d2b8e + 631
9   a                              0x000eb0ac main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x7a73a9c0  ebx: 0xbff16638  ecx: 0x00000000  edx: 0x00000000
  edi: 0x002952ce  esi: 0x00000000  ebp: 0xbff165d8  esp: 0xbff165c0
   ss: 0x00000023  efl: 0x00010296  eip: 0x000eab0e   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x002cec18
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xe8000 -    0xebffb +a (0) <311A4EC1-41A0-3635-BA00-60C231589EBA> /Users/USER/*/a
  0x1dc000 -   0x221fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x265000 -   0x2f4ff3 +libstd-4ed43f4a317eb5c7.dylib (0) <F4A70709-CC7E-3ECD-A8C7-495C7787E62A> /Users/USER/*/libstd-4ed43f4a317eb5c7.dylib
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
    task_for_pid: 2377
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
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
__DATA                            3536K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9344K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.5M      134 
TOTAL                            569.5M      134 
TOTAL, minus reserved VM space   569.4M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-30-081537-1_Traviss-Mac-1044.crash
Process:               a [41023]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [41019]
Responsible:           a [41023]
User ID:               501
Date/Time:             2019-04-30 08:15:36.078 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5200 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-4ed43f4a317eb5c7.dylib  0x001d2fa3 std::panicking::rust_panic_with_hook::h1612a0d2724d4f01 + 115
1   a                              0x000dfbef std::panicking::begin_panic::hfa254b5f7556efb8 + 47 (panicking.rs:408)
2   a                              0x000dd6d4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 36 (backtrace.rs:24)
3   a                              0x000dd06b core::ptr::real_drop_in_place::hb4e245af875ded2a + 11
4   a                              0x000dd6a3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x000de9b8 backtrace::main::hcde7a1a1c3c85e77 + 4568 (backtrace.rs:103)
6   a                              0x000dcb8b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h97c914a78da5e141 + 11 (rt.rs:64)
7   libstd-4ed43f4a317eb5c7.dylib  0x001cff3c std::sys_common::backtrace::__rust_begin_short_backtrace::hdddb286332b79fb7 + 12
8   libstd-4ed43f4a317eb5c7.dylib  0x001d2a94 std::panicking::try::do_call::h5331467eede3c05e + 20
9   libstd-4ed43f4a317eb5c7.dylib  0x001e12dd __rust_maybe_catch_panic + 29
10  libstd-4ed43f4a317eb5c7.dylib  0x001d3537 std::rt::lang_start_internal::he35be8db163d2b8e + 631
11  a                              0x000df20c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff243a8  ebx: 0xbff243f0  ecx: 0xbff242a0  edx: 0xa7702ec6
  edi: 0x0021af58  esi: 0x001d2f3e  ebp: 0xbff24448  esp: 0xbff243c0
   ss: 0x00000023  efl: 0x00010286  eip: 0x001d2fa3   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00499c38
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xda000 -    0xe0fff +a (0) <BDA5881E-F2E5-3BC4-AC1D-E757D7D39701> /Users/USER/*/a
  0x128000 -   0x16dfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1b1000 -   0x240ff3 +libstd-4ed43f4a317eb5c7.dylib (0) <F4A70709-CC7E-3ECD-A8C7-495C7787E62A> /Users/USER/*/libstd-4ed43f4a317eb5c7.dylib
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
    task_for_pid: 2377
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=91.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=91.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            27.4M       12 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3536K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9356K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            586.9M      138 
TOTAL                            586.9M      138 
TOTAL, minus reserved VM space   586.8M      138 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-30-081537_Traviss-Mac-1044.crash
Process:               a [41024]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [41019]
Responsible:           a [41024]
User ID:               501
Date/Time:             2019-04-30 08:15:36.143 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5200 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-4ed43f4a317eb5c7.dylib  0x0018cfa3 std::panicking::rust_panic_with_hook::h1612a0d2724d4f01 + 115
1   a                              0x00063bef std::panicking::begin_panic::hfa254b5f7556efb8 + 47 (panicking.rs:408)
2   a                              0x000616d4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 36 (backtrace.rs:24)
3   a                              0x0006106b core::ptr::real_drop_in_place::hb4e245af875ded2a + 11
4   a                              0x000616a3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x000629b8 backtrace::main::hcde7a1a1c3c85e77 + 4568 (backtrace.rs:103)
6   a                              0x00060b8b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h97c914a78da5e141 + 11 (rt.rs:64)
7   libstd-4ed43f4a317eb5c7.dylib  0x00189f3c std::sys_common::backtrace::__rust_begin_short_backtrace::hdddb286332b79fb7 + 12
8   libstd-4ed43f4a317eb5c7.dylib  0x0018ca94 std::panicking::try::do_call::h5331467eede3c05e + 20
9   libstd-4ed43f4a317eb5c7.dylib  0x0019b2dd __rust_maybe_catch_panic + 29
10  libstd-4ed43f4a317eb5c7.dylib  0x0018d537 std::rt::lang_start_internal::he35be8db163d2b8e + 631
11  a                              0x0006320c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbffa0388  ebx: 0xbffa03d0  ecx: 0xbffa0280  edx: 0xa7702ec6
  edi: 0x001d4f58  esi: 0x0018cf3e  ebp: 0xbffa0428  esp: 0xbffa03a0
   ss: 0x00000023  efl: 0x00010282  eip: 0x0018cfa3   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00453c38
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x5e000 -    0x64fff +a (0) <BDA5881E-F2E5-3BC4-AC1D-E757D7D39701> /Users/USER/*/a
   0xe2000 -   0x127fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x16b000 -   0x1faff3 +libstd-4ed43f4a317eb5c7.dylib (0) <F4A70709-CC7E-3ECD-A8C7-495C7787E62A> /Users/USER/*/libstd-4ed43f4a317eb5c7.dylib
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
    task_for_pid: 2377
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=74.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=74.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.4M       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3536K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9356K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.9M      136 
TOTAL                            569.9M      136 
TOTAL, minus reserved VM space   569.8M      136 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-30-081636_Traviss-Mac-1044.crash
Process:               a [42725]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [42724]
Responsible:           a [42725]
User ID:               501
Date/Time:             2019-04-30 08:16:35.191 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5200 seconds
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
3   libstd-4ed43f4a317eb5c7.dylib  0x001566fb std::sys::unix::abort_internal::h0edd3eee133ef118 + 11
4   libstd-4ed43f4a317eb5c7.dylib  0x00148120 rust_oom + 48
5   libstd-4ed43f4a317eb5c7.dylib  0x00168fd4 alloc::alloc::handle_alloc_error::h1d590e159ffcfc9e + 20
6   a                              0x0002e71d default_alloc_error_hook::main::hbf2d06db626d002e + 781
7   a                              0x0002deab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h2354d4e86808fc83 + 11
8   libstd-4ed43f4a317eb5c7.dylib  0x00145f3c std::sys_common::backtrace::__rust_begin_short_backtrace::hdddb286332b79fb7 + 12
9   libstd-4ed43f4a317eb5c7.dylib  0x00148a94 std::panicking::try::do_call::h5331467eede3c05e + 20
10  libstd-4ed43f4a317eb5c7.dylib  0x001572dd __rust_maybe_catch_panic + 29
11  libstd-4ed43f4a317eb5c7.dylib  0x00149537 std::rt::lang_start_internal::he35be8db163d2b8e + 631
12  a                              0x0002e87c main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffd154c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffd1578  esp: 0xbffd154c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x2d000 -    0x2eff3 +a (0) <EAADE483-F7CB-3036-9204-307ABC9A8778> /Users/USER/*/a
   0x9e000 -    0xe3fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x127000 -   0x1b6ff3 +libstd-4ed43f4a317eb5c7.dylib (0) <F4A70709-CC7E-3ECD-A8C7-495C7787E62A> /Users/USER/*/libstd-4ed43f4a317eb5c7.dylib
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
---
===========                     =======  ======= 
TOTAL                            568.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-30-082048_Traviss-Mac-1044.crash
Process:               a [49073]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [49070]
Responsible:           a [49073]
User ID:               501
Date/Time:             2019-04-30 08:20:47.542 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b06b1fec
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb06b1fec:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b06b1000-00000000b06b2000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b06b2000-00000000b08b3000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-4ed43f4a317eb5c7.dylib  0x001b6f00 std::sys::unix::thread::Thread::join::hc0879e0958f00604 + 32
4   a                              0x000400b6 std::thread::JoinHandle$LT$T$GT$::join::h889cd3f3a1ab0cc0 + 70
5   a                              0x0003ee49 out_of_stack::main::hfb05bc1bb33cf0c4 + 233
6   a                              0x0003e23b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h63489e9f42c7cef3 + 11
7   libstd-4ed43f4a317eb5c7.dylib  0x001a6f3c std::sys_common::backtrace::__rust_begin_short_backtrace::hdddb286332b79fb7 + 12
8   libstd-4ed43f4a317eb5c7.dylib  0x001a9a94 std::panicking::try::do_call::h5331467eede3c05e + 20
9   libstd-4ed43f4a317eb5c7.dylib  0x001b82dd __rust_maybe_catch_panic + 29
10  libstd-4ed43f4a317eb5c7.dylib  0x001aa537 std::rt::lang_start_internal::he35be8db163d2b8e + 631
11  a                              0x0003fb4c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-4ed43f4a317eb5c7.dylib  0x001b76fb std::sys::unix::abort_internal::h0edd3eee133ef118 + 11
4   libstd-4ed43f4a317eb5c7.dylib  0x001a7f19 std::sys_common::util::abort::hf2a78ae510fb6eef + 73
5   libstd-4ed43f4a317eb5c7.dylib  0x001b6998 std::sys::unix::stack_overflow::imp::signal_handler::hc994e28bea134a6c + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-4ed43f4a317eb5c7.dylib  0x001b65e0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::heee5e4cc1aec4b76 + 80
9   libstd-4ed43f4a317eb5c7.dylib  0x00194d3c _$LT$std..io..buffered..LineWriter$LT$W$GT$$u20$as$u20$std..io..Write$GT$::write::h57b62ec9ae506531 + 220
10  libstd-4ed43f4a317eb5c7.dylib  0x00199a65 std::io::Write::write_all::hf6fafbd496763b34 + 101
11  libstd-4ed43f4a317eb5c7.dylib  0x00199d83 _$LT$std..io..Write..write_fmt..Adaptor$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$::write_str::h6540084b134a44d2 + 35
12  libstd-4ed43f4a317eb5c7.dylib  0x001d7c9d core::fmt::write::hab1379ed0cfbc217 + 701
13  libstd-4ed43f4a317eb5c7.dylib  0x00197a06 _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::h09adf890ce95ec9a + 182
14  libstd-4ed43f4a317eb5c7.dylib  0x00198f3c std::io::stdio::_print::h568513652811edd7 + 396
15  a                              0x0003ed4f out_of_stack::loud_recurse::hcd528ebf130a94fa + 63
16  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
17  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
18  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
19  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
20  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
21  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
22  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
23  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
24  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
25  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
26  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
27  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
28  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
29  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
30  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
31  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
32  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
33  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
34  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
35  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
36  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
37  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
38  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
39  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
40  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
41  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
42  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
43  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
44  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
45  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
46  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
47  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
48  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
49  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
50  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
51  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
52  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
53  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
54  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
55  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
56  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
57  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
58  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
59  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
60  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
61  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
62  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
63  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
64  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
65  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
66  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
67  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
68  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
69  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
70  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
71  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
72  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
73  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
74  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
75  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
76  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
77  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
78  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
79  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
80  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
81  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
82  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
83  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
84  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
85  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
86  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
87  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
88  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
89  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
90  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
91  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
92  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
93  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
94  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
95  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
96  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
97  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
98  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
99  a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
100 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
101 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
102 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
103 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
104 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
105 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
106 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
107 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
108 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
109 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
110 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
111 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
112 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
113 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
114 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
115 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
116 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
117 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
118 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
119 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
120 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
121 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
122 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
123 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
124 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
125 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
126 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
127 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
128 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
129 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
130 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
131 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
132 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
133 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
134 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
135 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
136 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
137 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
138 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
139 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
140 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
141 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
142 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
143 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
144 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
145 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
146 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
147 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
148 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
149 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
150 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
151 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
152 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
153 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
154 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
155 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
156 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
157 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
158 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
159 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
160 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
161 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
162 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
163 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
164 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
165 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
166 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
167 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
168 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
169 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
170 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
171 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
172 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
173 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
174 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
175 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
176 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
177 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
178 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
179 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
180 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
181 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
182 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
183 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
184 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
185 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
186 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
187 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
188 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
189 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
190 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
191 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
192 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
193 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
194 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
195 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
196 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
197 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
198 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
199 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
200 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
201 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
202 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
203 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
204 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
205 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
206 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
207 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
208 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
209 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
210 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
211 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
212 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
213 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
214 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
215 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
216 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
217 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
218 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
219 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
220 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
221 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
222 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
223 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
224 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
225 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
226 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
227 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
228 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
229 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
230 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
231 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
232 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
233 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
234 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
235 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
236 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
237 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
238 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
239 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
240 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
241 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
242 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
243 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
244 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
245 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
246 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
247 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
248 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
249 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
250 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
251 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
252 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
253 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
254 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
255 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
256 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
257 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
258 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
259 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
260 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
261 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
262 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
263 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
264 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
265 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
266 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
267 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
268 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
269 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
270 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
271 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
272 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
273 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
274 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
275 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
276 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
277 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
278 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
279 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
280 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
281 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
282 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
283 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
284 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
285 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
286 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
287 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
288 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
289 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
290 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
291 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
292 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
293 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
294 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
295 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
296 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
297 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
298 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
299 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
300 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
301 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
302 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
303 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
304 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
305 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
306 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
307 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
308 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
309 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
310 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
311 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
312 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
313 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
314 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
315 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
316 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
317 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
318 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
319 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
320 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
321 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
322 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
323 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
324 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
325 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
326 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
327 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
328 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
329 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
330 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
331 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
332 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
333 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
334 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
335 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
336 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
337 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
338 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
339 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
340 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
341 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
342 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
343 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
344 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
345 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
346 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
347 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
348 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
349 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
350 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
351 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
352 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
353 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
354 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
355 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
356 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
357 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
358 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
359 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
360 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
361 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
362 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
363 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
364 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
365 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
366 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
367 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
368 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
369 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
370 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
371 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
372 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
373 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
374 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
375 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
376 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
377 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
378 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
379 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
380 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
381 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
382 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
383 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
384 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
385 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
386 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
387 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
388 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
389 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
390 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
391 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
392 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
393 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
394 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
395 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
396 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
397 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
398 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
399 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
400 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
401 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
402 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
403 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
404 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
405 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
406 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
407 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
408 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
409 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
410 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
411 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
412 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
413 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
414 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
415 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
416 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
417 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
418 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
419 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
420 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
421 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
422 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
423 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
424 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
425 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
426 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
427 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
428 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
429 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
430 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
431 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
432 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
433 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
434 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
435 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
436 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
437 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
438 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
439 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
440 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
441 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
442 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
443 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
444 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
445 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
446 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
447 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
448 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
449 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
450 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
451 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
452 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
453 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
454 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
455 a                              0x0003ed54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
---
===========                     =======  ======= 
TOTAL                            577.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-30-082101_Traviss-Mac-1044.crash
Process:               a [49265]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [49262]
Responsible:           a [49265]
User ID:               501
Date/Time:             2019-04-30 08:21:00.607 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
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
3   a                              0x0006656b panic_abort::__rust_start_panic::abort::hf958b662c60a82f5 + 11
4   a                              0x0006655b __rust_start_panic + 11
5   a                              0x0005ac5b rust_panic + 11
6   a                              0x0005a845 std::panicking::rust_panic_with_hook::h1612a0d2724d4f01 + 997
7   a                              0x0006c2ea std::panicking::begin_panic::h76382a8a08833bf1 + 42
8   a                              0x0005961d lto_abort::main::h9419a0043b6e0505 + 2877
9   a                              0x0006c42b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h96eae67d3af96e37 + 11
10  a                              0x000663dc std::sys_common::backtrace::__rust_begin_short_backtrace::hdddb286332b79fb7 + 12
11  a                              0x000599f8 main + 984
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffa651c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffa6548  esp: 0xbffa651c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x58000 -    0x7bfff +a (0) <55391B4E-329C-342F-8A36-587405088758> /Users/USER/*/a
   0x95000 -    0xdafdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2642
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
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            565.6M      131 
TOTAL                            565.6M      131 
TOTAL, minus reserved VM space   565.5M      131 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-30-082137_Traviss-Mac-1044.crash
Process:               a [50241]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50236]
Responsible:           a [50241]
User ID:               501
Date/Time:             2019-04-30 08:21:37.020 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
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
3   libstd-4ed43f4a317eb5c7.dylib  0x001906fb std::sys::unix::abort_internal::h0edd3eee133ef118 + 11
4   libstd-4ed43f4a317eb5c7.dylib  0x00180f19 std::sys_common::util::abort::hf2a78ae510fb6eef + 73
5   libstd-4ed43f4a317eb5c7.dylib  0x001832b2 rust_panic + 98
6   libstd-4ed43f4a317eb5c7.dylib  0x0018317e std::panicking::rust_panic_with_hook::h1612a0d2724d4f01 + 590
7   a                              0x000829af std::panicking::begin_panic::h9edea083c9105d40 + 47
8   a                              0x00083aec main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff7d52c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff7d558  esp: 0xbff7d52c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x81000 -    0x84fff +a (0) <C216AC49-6368-3432-BD39-1EA6E9F6F679> /Users/USER/*/a
   0xd8000 -   0x11dfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x161000 -   0x1f0ff3 +libstd-4ed43f4a317eb5c7.dylib (0) <F4A70709-CC7E-3ECD-A8C7-495C7787E62A> /Users/USER/*/libstd-4ed43f4a317eb5c7.dylib
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
    task_for_pid: 2642
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=74.2M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=74.2M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
__DATA                            3536K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__OBJC                              36K        6 
__TEXT                            9344K       44 
mapped file                      408.7M       21 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.4M      132 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-30-082138_Traviss-Mac-1044.crash
Process:               a [50265]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50262]
Responsible:           a [50265]
User ID:               501
Date/Time:             2019-04-30 08:21:37.877 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
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
    __TEXT                 0000000000027000-000000000002a000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00028f72 segfault_no_out_of_stack::main::haec5f9680e9a04a3 + 2034
1   a                              0x0002799b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h74559608b24a7491 + 11
2   libstd-4ed43f4a317eb5c7.dylib  0x0017df3c std::sys_common::backtrace::__rust_begin_short_backtrace::hdddb286332b79fb7 + 12
3   libstd-4ed43f4a317eb5c7.dylib  0x00180a94 std::panicking::try::do_call::h5331467eede3c05e + 20
4   libstd-4ed43f4a317eb5c7.dylib  0x0018f2dd __rust_maybe_catch_panic + 29
5   libstd-4ed43f4a317eb5c7.dylib  0x00181537 std::rt::lang_start_internal::he35be8db163d2b8e + 631
6   a                              0x0002924c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x78f37880  ecx: 0x00000000  edx: 0x00000000
  edi: 0x0018f2ce  esi: 0xbffd7660  ebp: 0xbffd7748  esp: 0xbffd75a0
   ss: 0x00000023  efl: 0x00010246  eip: 0x00028f72   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     3
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x27000 -    0x29ff3 +a (0) <F33B86E7-DBE4-3F1C-8C43-73E99B1639D7> /Users/USER/*/a
   0xd6000 -   0x11bfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x15f000 -   0x1eeff3 +libstd-4ed43f4a317eb5c7.dylib (0) <F4A70709-CC7E-3ECD-A8C7-495C7787E62A> /Users/USER/*/libstd-4ed43f4a317eb5c7.dylib
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
    task_for_pid: 2642
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=82.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=82.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                        132K        3 
__DATA                            3536K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            577.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-30-082145_Traviss-Mac-1044.crash
Process:               a [50451]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50449]
Responsible:           a [50451]
User ID:               501
Date/Time:             2019-04-30 08:21:45.575 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
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
    __TEXT                 00000000000bf000-00000000000c2000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000c15d4 signal_exit_status::main::h013a3960fc5c363d + 436
1   a                              0x000c047b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h2a21a330214f8d90 + 11
2   libstd-4ed43f4a317eb5c7.dylib  0x001e6f3c std::sys_common::backtrace::__rust_begin_short_backtrace::hdddb286332b79fb7 + 12
3   libstd-4ed43f4a317eb5c7.dylib  0x001e9a94 std::panicking::try::do_call::h5331467eede3c05e + 20
4   libstd-4ed43f4a317eb5c7.dylib  0x001f82dd __rust_maybe_catch_panic + 29
5   libstd-4ed43f4a317eb5c7.dylib  0x001ea537 std::rt::lang_start_internal::he35be8db163d2b8e + 631
6   a                              0x000c16ac main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x786215e0
  edi: 0x78621670  esi: 0xbff3f6c0  ebp: 0xbff3f758  esp: 0xbff3f640
   ss: 0x00000023  efl: 0x00010246  eip: 0x000c15d4   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     3
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xbf000 -    0xc1fff +a (0) <BF1A571D-796F-3EE1-82BF-B67EE5DDB762> /Users/USER/*/a
  0x13f000 -   0x184fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1c8000 -   0x257ff3 +libstd-4ed43f4a317eb5c7.dylib (0) <F4A70709-CC7E-3ECD-A8C7-495C7787E62A> /Users/USER/*/libstd-4ed43f4a317eb5c7.dylib
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
    task_for_pid: 2642
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
VM_ALLOCATE                        132K        3 
__DATA                            3536K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-30-082150_Traviss-Mac-1044.crash
Process:               a [50550]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50545]
Responsible:           a [50550]
User ID:               501
Date/Time:             2019-04-30 08:21:50.104 +0000
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
0   a                              0x000c38a6 simd_target_feature_mixup::test::id_avx512_512::h967bf37482962390 + 102
1   a                              0x000c265f simd_target_feature_mixup::test::main::h379367934b9623dc + 1647
2   a                              0x000c4bd0 simd_target_feature_mixup::main::h4f60990077aff357 + 896
3   a                              0x000c4e4b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h4bdfb018b3267dfd + 11
4   libstd-4ed43f4a317eb5c7.dylib  0x00262f3c std::sys_common::backtrace::__rust_begin_short_backtrace::hdddb286332b79fb7 + 12
5   libstd-4ed43f4a317eb5c7.dylib  0x00265a94 std::panicking::try::do_call::h5331467eede3c05e + 20
6   libstd-4ed43f4a317eb5c7.dylib  0x002742dd __rust_maybe_catch_panic + 29
7   libstd-4ed43f4a317eb5c7.dylib  0x00266537 std::rt::lang_start_internal::he35be8db163d2b8e + 631
8   a                              0x000c4dac main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff3d380  ebx: 0xbff3d300  ecx: 0x000c384e  edx: 0xbff3d300
  edi: 0x000c2004  esi: 0x00000000  ebp: 0xbff3d2f8  esp: 0xbff3d2c0
   ss: 0x00000023  efl: 0x00010246  eip: 0x000c38a6   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000c34e0
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xc1000 -    0xc5fc7 +a (0) <849BAA88-DD75-3951-8CFC-9BCBF11E60EC> /Users/USER/*/a
  0x1bb000 -   0x200fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x244000 -   0x2d3ff3 +libstd-4ed43f4a317eb5c7.dylib (0) <F4A70709-CC7E-3ECD-A8C7-495C7787E62A> /Users/USER/*/libstd-4ed43f4a317eb5c7.dylib
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
    task_for_pid: 2642
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
__DATA                            3536K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9348K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            568.5M      134 
TOTAL                            568.5M      134 
TOTAL, minus reserved VM space   568.4M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-30-082156-1_Traviss-Mac-1044.crash
Process:               a [50696]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50688]
Responsible:           a [50696]
User ID:               501
Date/Time:             2019-04-30 08:21:55.682 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5600 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0058e68
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0058e68:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0058000-00000000b0059000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0059000-00000000b025a000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-4ed43f4a317eb5c7.dylib  0x00186f00 std::sys::unix::thread::Thread::join::hc0879e0958f00604 + 32
4   a                              0x00080646 std::thread::JoinHandle$LT$T$GT$::join::hbb152a86173b5f52 + 70
5   a                              0x0007f865 stack_probes::main::hc5f49a55fd7e038b + 597
6   a                              0x0007e67b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb65aef0aa6b669aa + 11
7   libstd-4ed43f4a317eb5c7.dylib  0x00176f3c std::sys_common::backtrace::__rust_begin_short_backtrace::hdddb286332b79fb7 + 12
8   libstd-4ed43f4a317eb5c7.dylib  0x00179a94 std::panicking::try::do_call::h5331467eede3c05e + 20
9   libstd-4ed43f4a317eb5c7.dylib  0x001882dd __rust_maybe_catch_panic + 29
10  libstd-4ed43f4a317eb5c7.dylib  0x0017a537 std::rt::lang_start_internal::he35be8db163d2b8e + 631
11  a                              0x0008028c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-4ed43f4a317eb5c7.dylib  0x001876fb std::sys::unix::abort_internal::h0edd3eee133ef118 + 11
4   libstd-4ed43f4a317eb5c7.dylib  0x00177f19 std::sys_common::util::abort::hf2a78ae510fb6eef + 73
5   libstd-4ed43f4a317eb5c7.dylib  0x00186998 std::sys::unix::stack_overflow::imp::signal_handler::hc994e28bea134a6c + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-4ed43f4a317eb5c7.dylib  0x001865e0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::heee5e4cc1aec4b76 + 80
9   a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x0007f9b0 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x0007e6dd std::sys_common::backtrace::__rust_begin_short_backtrace::he964fe1df7e63bb1 (.llvm.10301593783339668105) + 29
265 libstd-4ed43f4a317eb5c7.dylib  0x001882dd __rust_maybe_catch_panic + 29
266 a                              0x000809a3 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::ha73d015a80d53d80 + 131
267 libstd-4ed43f4a317eb5c7.dylib  0x0015e3e1 _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h5f53e6cdffc0bf7a + 65
268 libstd-4ed43f4a317eb5c7.dylib  0x00186e38 std::sys::unix::thread::Thread::new::thread_start::hc36863c11521331a + 184
269 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
270 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
271 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb0259000  ecx: 0x00492b0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x00492b38  esp: 0x00492b0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00186a20
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0x7d000 -    0x81ff3 +a (0) <BDD3DB76-BA31-313E-8C5E-EB300D643FA0> /Users/USER/*/a
   0xcf000 -   0x114fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x158000 -   0x1e7ff3 +libstd-4ed43f4a317eb5c7.dylib (0) <F4A70709-CC7E-3ECD-A8C7-495C7787E62A> /Users/USER/*/libstd-4ed43f4a317eb5c7.dylib
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
---
===========                     =======  ======= 
TOTAL                            568.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-30-082203_Traviss-Mac-1044.crash
Process:               a [50813]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50810]
Responsible:           a [50813]
User ID:               501
Date/Time:             2019-04-30 08:22:02.259 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5600 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0618ec8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0618ec8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0618000-00000000b0619000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0619000-00000000b081a000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   a                              0x000a67b9 stack_probes_lto::main::h7f9d4bbc5a99dd1a + 2457
4   a                              0x000bf4ab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h255bd7433f60aad4 + 11
5   a                              0x000b7cac std::sys_common::backtrace::__rust_begin_short_backtrace::hdddb286332b79fb7 + 12
6   a                              0x000a7edd main + 589
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000a928b std::sys::unix::abort_internal::h0edd3eee133ef118 + 11
4   a                              0x000a9279 std::sys_common::util::abort::hf2a78ae510fb6eef + 73
5   a                              0x000b7c0c std::sys::unix::stack_overflow::imp::signal_handler::hc994e28bea134a6c + 860
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000b78b0 rust_begin_unwind + 16
9   a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
10  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
11  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
12  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
13  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
14  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
15  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
16  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
17  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
18  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
19  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
20  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
21  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
22  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
23  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
24  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
25  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
26  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
27  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
28  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
29  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
30  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
31  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
32  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
33  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
34  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
35  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
36  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
37  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
38  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
39  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
40  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
41  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
42  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
43  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
44  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
45  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
46  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
47  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
48  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
49  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
50  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
51  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
52  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
53  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
54  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
55  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
56  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
57  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
58  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
59  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
60  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
61  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
62  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
63  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
64  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
65  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
66  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
67  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
68  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
69  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
70  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
71  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
72  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
73  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
74  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
75  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
76  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
77  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
78  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
79  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
80  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
81  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
82  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
83  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
84  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
85  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
86  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
87  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
88  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
89  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
90  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
91  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
92  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
93  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
94  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
95  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
96  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
97  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
98  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
99  a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
100 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
101 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
102 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
103 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
104 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
105 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
106 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
107 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
108 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
109 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
110 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
111 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
112 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
113 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
114 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
115 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
116 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
117 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
118 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
119 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
120 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
121 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
122 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
123 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
124 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
125 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
126 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
127 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
128 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
129 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
130 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
131 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
132 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
133 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
134 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
135 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
136 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
137 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
138 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
139 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
140 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
141 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
142 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
143 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
144 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
145 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
146 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
147 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
148 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
149 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
150 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
151 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
152 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
153 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
154 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
155 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
156 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
157 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
158 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
159 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
160 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
161 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
162 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
163 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
164 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
165 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
166 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
167 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
168 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
169 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
170 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
171 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
172 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
173 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
174 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
175 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
176 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
177 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
178 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
179 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
180 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
181 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
182 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
183 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
184 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
185 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
186 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
187 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
188 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
189 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
190 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
191 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
192 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
193 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
194 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
195 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
196 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
197 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
198 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
199 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
200 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
201 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
202 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
203 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
204 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
205 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
206 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
207 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
208 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
209 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
210 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
211 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
212 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
213 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
214 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
215 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
216 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
217 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
218 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
219 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
220 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
221 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
222 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
223 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
224 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
225 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
226 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
227 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
228 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
229 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
230 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
231 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
232 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
233 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
234 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
235 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
236 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
237 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
238 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
239 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
240 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
241 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
242 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
243 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
244 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
245 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
246 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
247 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
248 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
249 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
250 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
251 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
252 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
253 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
254 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
255 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
256 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
257 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
258 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
259 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
260 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
261 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
262 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
263 a                              0x000a6c68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
264 a                              0x000bee14 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hbe1211a7f59ba2d1 + 116
265 a                              0x000b7431 _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h5f53e6cdffc0bf7a + 65
266 a                              0x000b7f54 std::sys::unix::thread::Thread::new::thread_start::hc36863c11521331a + 180
267 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
268 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
269 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb0819000  ecx: 0x001b4b0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x001b4b38  esp: 0x001b4b0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000cc57d
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0xa5000 -    0xd0ff3 +a (0) <4FFE0A2C-549C-3476-B224-8F3B2273FA69> /Users/USER/*/a
  0x10c000 -   0x151fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2642
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=82.4M resident=0K(0%) swapped_out_or_unallocated=82.4M(100%)
Writable regions: Total=76.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                          8K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            1336K       43 
__LINKEDIT                        73.7M        4 
__OBJC                              36K        6 
__TEXT                            8928K       43 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            568.8M      134 
TOTAL                            568.8M      134 
TOTAL, minus reserved VM space   568.6M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-30-082204_Traviss-Mac-1044.crash
Process:               a [50812]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50810]
Responsible:           a [50812]
User ID:               501
Date/Time:             2019-04-30 08:22:02.257 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5600 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf697a8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf697a8:
    Stack Guard            00000000bbf68000-00000000bbf69000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf69000-00000000bbf6a000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf6a000-00000000bff68000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x0009d28b std::sys::unix::abort_internal::h0edd3eee133ef118 + 11
4   a                              0x0009d279 std::sys_common::util::abort::hf2a78ae510fb6eef + 73
5   a                              0x000abc0c std::sys::unix::stack_overflow::imp::signal_handler::hc994e28bea134a6c + 860
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000ab8b0 rust_begin_unwind + 16
9   a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
10  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
11  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
12  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
13  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
14  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
15  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
16  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
17  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
18  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
19  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
20  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
21  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
22  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
23  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
24  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
25  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
26  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
27  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
28  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
29  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
30  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
31  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
32  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
33  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
34  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
35  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
36  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
37  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
38  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
39  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
40  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
41  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
42  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
43  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
44  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
45  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
46  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
47  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
48  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
49  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
50  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
51  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
52  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
53  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
54  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
55  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
56  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
57  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
58  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
59  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
60  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
61  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
62  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
63  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
64  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
65  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
66  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
67  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
68  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
69  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
70  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
71  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
72  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
73  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
74  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
75  a                              0x0009ac68 stack_probes_lto::recurse::h907252696a8f0ddd + 40
