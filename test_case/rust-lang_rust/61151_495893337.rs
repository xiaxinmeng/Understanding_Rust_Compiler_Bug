plain
[00:03:21]       Memory: 8 GB
[00:03:21]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:21]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:21]       SMC Version (system): 2.8f0
[00:03:21]       Serial Number (system): VM6e/Fc2pNXx
[00:03:21] 
[00:03:21] hw.ncpu: 4
[00:03:21] hw.byteorder: 1234
[00:03:21] hw.memsize: 8589934592
---
[02:07:27] 
[02:07:27] ---- /Users/travis/build/rust-lang/rust/src/doc/rustdoc/src/documentation-tests.md - Documentation_tests::Documenting_macros (line 263) stdout ----
[02:07:27] error: linking with `cc` failed: signal: 4
[02:07:27]   |
[02:07:27]   = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnzbvyn/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnzbvyn/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnzbvyn/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnzbvyn/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnzbvyn/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnzbvyn/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestnzbvyn/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libstd-0e31801c7f918c72.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libpanic_unwind-53c261814f627a07.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libbacktrace_sys-c0d3309ba80e0f23.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_demangle-5509e73be26a0bd6.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libhashbrown-3132c0b871b3fff8.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_alloc-b2fa52a49bb284dc.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libunwind-0cdb42220e56e08b.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liblibc-b18dfaed99ca7a71.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liballoc-0a9841e408b889be.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_core-bc86d4a5a44cda78.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcore-75cce831a556c528.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-6ece06e05b87bd36.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:07:27] 
[02:07:27] error: aborting due to previous error
[02:07:27] 
[02:07:27] thread '/Users/travis/build/rust-lang/rust/src/doc/rustdoc/src/documentation-tests.md - Documentation_tests::Documenting_macros (line 263)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
---
[02:07:27] 
[02:07:27] 
[02:07:27] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:07:27] Build completed unsuccessfully in 0:50:59
[02:07:27] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:226579ef
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat May 25 09:16:22 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:1a2df8ae
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1176
-rw-------@  1 travis  staff   1387 May 25 09:15 foo_2019-05-25-091555_Traviss-Mac-1044.crash
drwx------  26 travis  staff    884 May 25 09:15 .
-rw-------@  1 travis  staff   1377 May 25 09:15 m4_2019-05-25-091525_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1391 May 25 09:15 bar_2019-05-25-091516-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1403 May 25 09:15 bar_2019-05-25-091516_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9899 May 25 09:15 b_2019-05-25-091515_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  57364 May 25 08:42 a_2019-05-25-084202-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34964 May 25 08:42 a_2019-05-25-084202_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34892 May 25 08:41 a_2019-05-25-084156-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55583 May 25 08:41 a_2019-05-25-084156_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 May 25 08:41 a_2019-05-25-084150_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9166 May 25 08:41 a_2019-05-25-084145_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9171 May 25 08:41 a_2019-05-25-084143_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8936 May 25 08:41 a_2019-05-25-084141_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9304 May 25 08:41 a_2019-05-25-084104_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58251 May 25 08:40 a_2019-05-25-084057_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 May 25 08:40 a_2019-05-25-084052-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60372 May 25 08:40 a_2019-05-25-084052-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59516 May 25 08:40 a_2019-05-25-084052_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10881 May 25 08:38 a_2019-05-25-083834_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 May 25 08:37 a_2019-05-25-083746_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 May 25 08:36 a_2019-05-25-083632_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 May 25 08:35 a_2019-05-25-083536-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9780 May 25 08:35 a_2019-05-25-083536_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9483 May 25 08:35 a_2019-05-25-083533_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:00a7f152
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-083533_Traviss-Mac-1044.crash
Process:               a [40555]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [40553]
Responsible:           a [40555]
User ID:               501
Date/Time:             2019-05-25 08:35:06.525 +0000
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
0   a                              0x000edafe abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 46
1   a                              0x000ecf4b std::panicking::try::do_call::h5a3ef39c95fb91cb (.llvm.7845620258863898592) + 11
2   libstd-0e31801c7f918c72.dylib  0x001d367d __rust_maybe_catch_panic + 29
3   a                              0x000edd65 abort_on_c_abi::main::he771bf881fc862e3 + 613
4   a                              0x000ec5ab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h2d28c74b2cc7c035 + 11
5   libstd-0e31801c7f918c72.dylib  0x001c1eac std::sys_common::backtrace::__rust_begin_short_backtrace::h2e5278a34b5b3cac + 12
6   libstd-0e31801c7f918c72.dylib  0x001c4a24 std::panicking::try::do_call::hfe83d4a945e58d59 + 20
7   libstd-0e31801c7f918c72.dylib  0x001d367d __rust_maybe_catch_panic + 29
8   libstd-0e31801c7f918c72.dylib  0x001c54c7 std::rt::lang_start_internal::h9aeb4048ce4bf257 + 631
9   a                              0x000ee09c main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x7966bdd0  ebx: 0xbff139c8  ecx: 0x00000000  edx: 0x00000000
  edi: 0x001d366e  esi: 0x00000000  ebp: 0xbff13968  esp: 0xbff13950
   ss: 0x00000023  efl: 0x00010296  eip: 0x000edafe   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0020cf3c
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xeb000 -    0xeeffb +a (0) <58570895-B077-35EE-B4EB-D1B99C61C9AD> /Users/USER/*/a
  0x11a000 -   0x15ffdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1a3000 -   0x232ffb +libstd-0e31801c7f918c72.dylib (0) <674D689C-6129-3C68-BDF2-AEB56FC106BF> /Users/USER/*/libstd-0e31801c7f918c72.dylib
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
    task_for_pid: 2341
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
__DATA                            3532K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9344K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            568.5M      134 
TOTAL                            568.5M      134 
TOTAL, minus reserved VM space   568.4M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-083536-1_Traviss-Mac-1044.crash
Process:               a [41361]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [41352]
Responsible:           a [41361]
User ID:               501
Date/Time:             2019-05-25 08:35:36.091 +0000
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
0   libstd-0e31801c7f918c72.dylib  0x00105f33 std::panicking::rust_panic_with_hook::h98cb1865e3cb8330 + 115
1   a                              0x00046bff std::panicking::begin_panic::hd14a5938fa400274 + 47 (panicking.rs:408)
2   a                              0x000446e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 36 (backtrace.rs:25)
3   a                              0x00043eab core::ptr::real_drop_in_place::ha0fd09e913297feb + 11
4   a                              0x000446b3 backtrace::double::h35adec2a6f63ef6c + 51
5   a                              0x000459c8 backtrace::main::hc9a5bc8fc93ded64 + 4568 (backtrace.rs:104)
6   a                              0x00043bbb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h29cb34e00104e506 + 11 (rt.rs:64)
7   libstd-0e31801c7f918c72.dylib  0x00102eac std::sys_common::backtrace::__rust_begin_short_backtrace::h2e5278a34b5b3cac + 12
8   libstd-0e31801c7f918c72.dylib  0x00105a24 std::panicking::try::do_call::hfe83d4a945e58d59 + 20
9   libstd-0e31801c7f918c72.dylib  0x0011467d __rust_maybe_catch_panic + 29
10  libstd-0e31801c7f918c72.dylib  0x001064c7 std::rt::lang_start_internal::h9aeb4048ce4bf257 + 631
11  a                              0x0004621c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbffbd718  ebx: 0xbffbd760  ecx: 0xbffbd610  edx: 0xa7702ec6
  edi: 0x0014e288  esi: 0x00105ece  ebp: 0xbffbd7b8  esp: 0xbffbd730
   ss: 0x00000023  efl: 0x00010286  eip: 0x00105f33   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x003cb889
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x41000 -    0x47ff7 +a (0) <CEEB8B5E-A783-3795-8AF3-D9635B5CE050> /Users/USER/*/a
   0x5b000 -    0xa0fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
   0xe4000 -   0x173ffb +libstd-0e31801c7f918c72.dylib (0) <674D689C-6129-3C68-BDF2-AEB56FC106BF> /Users/USER/*/libstd-0e31801c7f918c72.dylib
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
    task_for_pid: 2341
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=73.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9616K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3532K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9356K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            568.9M      136 
TOTAL                            568.9M      136 
TOTAL, minus reserved VM space   568.8M      136 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-083536_Traviss-Mac-1044.crash
Process:               a [41359]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [41352]
Responsible:           a [41359]
User ID:               501
Date/Time:             2019-05-25 08:35:36.064 +0000
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
0   libstd-0e31801c7f918c72.dylib  0x00126f33 std::panicking::rust_panic_with_hook::h98cb1865e3cb8330 + 115
1   a                              0x0006bbff std::panicking::begin_panic::hd14a5938fa400274 + 47 (panicking.rs:408)
2   a                              0x000696e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 36 (backtrace.rs:25)
3   a                              0x00068eab core::ptr::real_drop_in_place::ha0fd09e913297feb + 11
4   a                              0x000696b3 backtrace::double::h35adec2a6f63ef6c + 51
5   a                              0x0006a9c8 backtrace::main::hc9a5bc8fc93ded64 + 4568 (backtrace.rs:104)
6   a                              0x00068bbb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h29cb34e00104e506 + 11 (rt.rs:64)
7   libstd-0e31801c7f918c72.dylib  0x00123eac std::sys_common::backtrace::__rust_begin_short_backtrace::h2e5278a34b5b3cac + 12
8   libstd-0e31801c7f918c72.dylib  0x00126a24 std::panicking::try::do_call::hfe83d4a945e58d59 + 20
9   libstd-0e31801c7f918c72.dylib  0x0013567d __rust_maybe_catch_panic + 29
10  libstd-0e31801c7f918c72.dylib  0x001274c7 std::rt::lang_start_internal::h9aeb4048ce4bf257 + 631
11  a                              0x0006b21c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff98728  ebx: 0xbff98770  ecx: 0xbff98620  edx: 0xa7702ec6
  edi: 0x0016f288  esi: 0x00126ece  ebp: 0xbff987c8  esp: 0xbff98740
   ss: 0x00000023  efl: 0x00010282  eip: 0x00126f33   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x003ec889
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x66000 -    0x6cff7 +a (0) <CEEB8B5E-A783-3795-8AF3-D9635B5CE050> /Users/USER/*/a
   0x7c000 -    0xc1fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x105000 -   0x194ffb +libstd-0e31801c7f918c72.dylib (0) <674D689C-6129-3C68-BDF2-AEB56FC106BF> /Users/USER/*/libstd-0e31801c7f918c72.dylib
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
    task_for_pid: 2341
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
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3532K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9356K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            586.9M      137 
TOTAL                            586.9M      137 
TOTAL, minus reserved VM space   586.8M      137 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-083632_Traviss-Mac-1044.crash
Process:               a [43042]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [43041]
Responsible:           a [43042]
User ID:               501
Date/Time:             2019-05-25 08:36:32.546 +0000
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
3   libstd-0e31801c7f918c72.dylib  0x001a3a9b std::sys::unix::abort_internal::hf98dcaece2b04055 + 11
4   libstd-0e31801c7f918c72.dylib  0x00195090 rust_oom + 48
5   libstd-0e31801c7f918c72.dylib  0x001b63b4 alloc::alloc::handle_alloc_error::hb1eb2ca9c570c963 + 20
6   a                              0x000aa46d default_alloc_error_hook::main::h0fe124586986ad5d + 781
7   a                              0x000aa83b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h013725c100f2b5ec + 11
8   libstd-0e31801c7f918c72.dylib  0x00192eac std::sys_common::backtrace::__rust_begin_short_backtrace::h2e5278a34b5b3cac + 12
9   libstd-0e31801c7f918c72.dylib  0x00195a24 std::panicking::try::do_call::hfe83d4a945e58d59 + 20
10  libstd-0e31801c7f918c72.dylib  0x001a467d __rust_maybe_catch_panic + 29
11  libstd-0e31801c7f918c72.dylib  0x001964c7 std::rt::lang_start_internal::h9aeb4048ce4bf257 + 631
12  a                              0x000aa5cc main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff558dc  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff55908  esp: 0xbff558dc
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xa9000 -    0xaaffb +a (0) <E03186A2-223B-314E-B414-5C09BB9DC464> /Users/USER/*/a
   0xeb000 -   0x130fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x174000 -   0x203ffb +libstd-0e31801c7f918c72.dylib (0) <674D689C-6129-3C68-BDF2-AEB56FC106BF> /Users/USER/*/libstd-0e31801c7f918c72.dylib
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-084052-2_Traviss-Mac-1044.crash
Process:               a [49479]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [49477]
Responsible:           a [49479]
User ID:               501
Date/Time:             2019-05-25 08:40:51.358 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5600 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b013ce74
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb013ce74:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b013c000-00000000b013d000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b013d000-00000000b033e000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-0e31801c7f918c72.dylib  0x002842a0 std::sys::unix::thread::Thread::join::h99c3c5e9668159e7 + 32
4   a                              0x000fe0b6 std::thread::JoinHandle$LT$T$GT$::join::h585a7e453a8a707a + 70
5   a                              0x000fd78d out_of_stack::main::h51a6e88d0363a411 + 2605
6   a                              0x000fc24b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h6d1f43f99840bcce + 11
7   libstd-0e31801c7f918c72.dylib  0x00273eac std::sys_common::backtrace::__rust_begin_short_backtrace::h2e5278a34b5b3cac + 12
8   libstd-0e31801c7f918c72.dylib  0x00276a24 std::panicking::try::do_call::hfe83d4a945e58d59 + 20
9   libstd-0e31801c7f918c72.dylib  0x0028567d __rust_maybe_catch_panic + 29
10  libstd-0e31801c7f918c72.dylib  0x002774c7 std::rt::lang_start_internal::h9aeb4048ce4bf257 + 631
11  a                              0x000fdb4c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-0e31801c7f918c72.dylib  0x00284a9b std::sys::unix::abort_internal::hf98dcaece2b04055 + 11
4   libstd-0e31801c7f918c72.dylib  0x00274e89 std::sys_common::util::abort::h15892f92d50adb0b + 73
5   libstd-0e31801c7f918c72.dylib  0x00283d3f std::sys::unix::stack_overflow::imp::signal_handler::hb91daec24a8f4714 + 943
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-0e31801c7f918c72.dylib  0x00283990 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h9ee1814d6a6b1355 + 80
9   a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
10  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
11  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
12  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
13  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
14  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
15  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
16  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
17  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
18  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
19  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
20  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
21  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
22  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
23  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
24  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
25  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
26  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
27  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
28  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
29  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
30  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
31  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
32  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
33  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
34  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
35  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
36  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
37  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
38  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
39  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
40  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
41  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
42  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
43  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
44  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
45  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
46  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
47  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
48  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
49  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
50  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
51  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
52  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
53  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
54  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
55  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
56  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
57  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
58  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
59  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
60  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
61  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
62  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
63  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
64  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
65  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
66  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
67  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
68  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
69  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
70  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
71  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
72  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
73  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
74  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
75  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
76  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
77  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
78  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
79  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
80  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
81  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
82  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
83  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
84  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
85  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
86  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
87  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
88  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
89  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
90  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
91  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
92  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
93  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
94  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
95  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
96  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
97  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
98  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
99  a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
100 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
101 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
102 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
103 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
104 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
105 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
106 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
107 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
108 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
109 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
110 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
111 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
112 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
113 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
114 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
115 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
116 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
117 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
118 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
119 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
120 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
121 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
122 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
123 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
124 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
125 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
126 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
127 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
128 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
129 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
130 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
131 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
132 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
133 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
134 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
135 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
136 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
137 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
138 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
139 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
140 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
141 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
142 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
143 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
144 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
145 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
146 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
147 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
148 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
149 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
150 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
151 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
152 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
153 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
154 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
155 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
156 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
157 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
158 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
159 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
160 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
161 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
162 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
163 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
164 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
165 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
166 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
167 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
168 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
169 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
170 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
171 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
172 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
173 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
174 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
175 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
176 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
177 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
178 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
179 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
180 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
181 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
182 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
183 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
184 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
185 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
186 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
187 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
188 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
189 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
190 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
191 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
192 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
193 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
194 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
195 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
196 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
197 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
198 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
199 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
200 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
201 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
202 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
203 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
204 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
205 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
206 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
207 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
208 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
209 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
210 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
211 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
212 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
213 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
214 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
215 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
216 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
217 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
218 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
219 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
220 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
221 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
222 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
223 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
224 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
225 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
226 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
227 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
228 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
229 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
230 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
231 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
232 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
233 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
234 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
235 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
236 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
237 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
238 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
239 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
240 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
241 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
242 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
243 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
244 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
245 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
246 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
247 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
248 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
249 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
250 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
251 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
252 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
253 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
254 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
255 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
256 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
257 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
258 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
259 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
260 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
261 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
262 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
263 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
264 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
265 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
266 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
267 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
268 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
269 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
270 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
271 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
272 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
273 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
274 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
275 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
276 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
277 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
278 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
279 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
280 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
281 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
282 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
283 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
284 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
285 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
286 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
287 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
288 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
289 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
290 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
291 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
292 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
293 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
294 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
295 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
296 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
297 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
298 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
299 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
300 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
301 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
302 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
303 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
304 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
305 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
306 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
307 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
308 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
309 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
310 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
311 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
312 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
313 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
314 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
315 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
316 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
317 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
318 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
319 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
320 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
321 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
322 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
323 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
324 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
325 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
326 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
327 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
328 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
329 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
330 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
331 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
332 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
333 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
334 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
335 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
336 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
337 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
338 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
339 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
340 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
341 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
342 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
343 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
344 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
345 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
346 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
347 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
348 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
349 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
350 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
351 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
352 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
353 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
354 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
355 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
356 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
357 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
358 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
359 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
360 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
361 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
362 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
363 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
364 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
365 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
366 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
367 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
368 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
369 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
370 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
371 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
372 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
373 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
374 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
375 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
376 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
377 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
378 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
379 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
380 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
381 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
382 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
383 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
384 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
385 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
386 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
387 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
388 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
389 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
390 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
391 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
392 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
393 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
394 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
395 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
396 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
397 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
398 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
399 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
400 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
401 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
402 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
403 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
404 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
405 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
406 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
407 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
408 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
409 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
410 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
411 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
412 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
413 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
414 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
415 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
416 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
417 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
418 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
419 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
420 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
421 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
422 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
423 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
424 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
425 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
426 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
427 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
428 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
429 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
430 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
431 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
432 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
433 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
434 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
435 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
436 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
437 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
438 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
439 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
440 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
441 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
442 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
443 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
444 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
445 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
446 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
447 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
448 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
449 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
450 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
451 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
452 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
453 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
454 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
455 a                              0x000fcd06 out_of_stack::silent_recurse::h0010c6274b4315f5 + 38
---
===========                     =======  ======= 
TOTAL                            568.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-084104_Traviss-Mac-1044.crash
Process:               a [49677]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [49674]
Responsible:           a [49677]
User ID:               501
Date/Time:             2019-05-25 08:41:04.630 +0000
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
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x0008d57b panic_abort::__rust_start_panic::abort::hf66aecbaf63ab8c3 + 11
4   a                              0x0008d56b __rust_start_panic + 11
5   a                              0x00081c7b rust_panic + 11
6   a                              0x00081864 std::panicking::rust_panic_with_hook::h98cb1865e3cb8330 + 996
7   a                              0x000932ea std::panicking::begin_panic::h79dd2ca82d702cfe + 42
8   a                              0x0008065d lto_abort::main::hd3a7c21d2a5e6b65 + 2877
9   a                              0x0009342b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h89ed3e6dd3fc2318 + 11
10  a                              0x0008d3ec std::sys_common::backtrace::__rust_begin_short_backtrace::h2e5278a34b5b3cac + 12
11  a                              0x00080a38 main + 984
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff7f8ac  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff7f8d8  esp: 0xbff7f8ac
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x7f000 -    0xa2fff +a (0) <D00012E6-D846-3B1D-9F4E-A67C95715A86> /Users/USER/*/a
   0xee000 -   0x133fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2592
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-084141_Traviss-Mac-1044.crash
Process:               a [50653]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50644]
Responsible:           a [50653]
User ID:               501
Date/Time:             2019-05-25 08:41:40.910 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5700 seconds
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
3   libstd-0e31801c7f918c72.dylib  0x00214a9b std::sys::unix::abort_internal::hf98dcaece2b04055 + 11
4   libstd-0e31801c7f918c72.dylib  0x00204e89 std::sys_common::util::abort::h15892f92d50adb0b + 73
5   libstd-0e31801c7f918c72.dylib  0x00207242 rust_panic + 98
6   libstd-0e31801c7f918c72.dylib  0x0020710e std::panicking::rust_panic_with_hook::h98cb1865e3cb8330 + 590
7   a                              0x0007e9cf std::panicking::begin_panic::h9b30ae952326fc75 + 47
8   a                              0x0007fb0c main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff818ac  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff818d8  esp: 0xbff818ac
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x7d000 -    0x80ff7 +a (0) <21BFF263-FAD9-30C3-9BC4-D9B93B75C621> /Users/USER/*/a
  0x15c000 -   0x1a1fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1e5000 -   0x274ffb +libstd-0e31801c7f918c72.dylib (0) <674D689C-6129-3C68-BDF2-AEB56FC106BF> /Users/USER/*/libstd-0e31801c7f918c72.dylib
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
    task_for_pid: 2592
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
__DATA                            3532K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-084143_Traviss-Mac-1044.crash
Process:               a [50675]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50673]
Responsible:           a [50675]
User ID:               501
Date/Time:             2019-05-25 08:41:41.608 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5700 seconds
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
    __TEXT                 000000000002f000-0000000000032000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00030f62 segfault_no_out_of_stack::main::h579ee62fe996355a + 2034
1   a                              0x0002f98b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc202d5fdf888bf6b + 11
2   libstd-0e31801c7f918c72.dylib  0x000f6eac std::sys_common::backtrace::__rust_begin_short_backtrace::h2e5278a34b5b3cac + 12
3   libstd-0e31801c7f918c72.dylib  0x000f9a24 std::panicking::try::do_call::hfe83d4a945e58d59 + 20
4   libstd-0e31801c7f918c72.dylib  0x0010867d __rust_maybe_catch_panic + 29
5   libstd-0e31801c7f918c72.dylib  0x000fa4c7 std::rt::lang_start_internal::h9aeb4048ce4bf257 + 631
6   a                              0x0003123c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x7be29280  ecx: 0x00000000  edx: 0x00000000
  edi: 0x0010866e  esi: 0xbffcf9f0  ebp: 0xbffcfad8  esp: 0xbffcf930
   ss: 0x00000023  efl: 0x00010246  eip: 0x00030f62   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     2
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x2f000 -    0x31ff3 +a (0) <AE4644C1-7A79-3976-B6B8-C6F2C08800F7> /Users/USER/*/a
   0x4f000 -    0x94fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
   0xd8000 -   0x167ffb +libstd-0e31801c7f918c72.dylib (0) <674D689C-6129-3C68-BDF2-AEB56FC106BF> /Users/USER/*/libstd-0e31801c7f918c72.dylib
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
    task_for_pid: 2592
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
__DATA                            3532K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-084145_Traviss-Mac-1044.crash
Process:               a [50758]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50757]
Responsible:           a [50758]
User ID:               501
Date/Time:             2019-05-25 08:41:44.911 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5700 seconds
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
    __TEXT                 0000000000093000-0000000000096000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000955d4 signal_exit_status::main::hc6663d816ec186eb + 436
1   a                              0x0009447b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb10d0c6a170c2a16 + 11
2   libstd-0e31801c7f918c72.dylib  0x00222eac std::sys_common::backtrace::__rust_begin_short_backtrace::h2e5278a34b5b3cac + 12
3   libstd-0e31801c7f918c72.dylib  0x00225a24 std::panicking::try::do_call::hfe83d4a945e58d59 + 20
4   libstd-0e31801c7f918c72.dylib  0x0023467d __rust_maybe_catch_panic + 29
5   libstd-0e31801c7f918c72.dylib  0x002264c7 std::rt::lang_start_internal::h9aeb4048ce4bf257 + 631
6   a                              0x000956ac main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x7a6730a0
  edi: 0x7a673130  esi: 0xbff6ba50  ebp: 0xbff6bae8  esp: 0xbff6b9d0
   ss: 0x00000023  efl: 0x00010246  eip: 0x000955d4   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     0
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x93000 -    0x95fff +a (0) <024CE56F-199C-3E9C-B52B-ABCBDC1C45C2> /Users/USER/*/a
  0x17b000 -   0x1c0fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x204000 -   0x293ffb +libstd-0e31801c7f918c72.dylib (0) <674D689C-6129-3C68-BDF2-AEB56FC106BF> /Users/USER/*/libstd-0e31801c7f918c72.dylib
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
    task_for_pid: 2592
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
__DATA                            3532K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-084150_Traviss-Mac-1044.crash
Process:               a [50871]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50864]
Responsible:           a [50871]
User ID:               501
Date/Time:             2019-05-25 08:41:49.689 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5700 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000e0bf6 simd_target_feature_mixup::test::id_avx512_512::hf3a1395d43161fbe + 102
1   a                              0x000df9af simd_target_feature_mixup::test::main::h611f15c116e6c273 + 1647
2   a                              0x000e1f20 simd_target_feature_mixup::main::hfcef50d014c08dda + 896
3   a                              0x000e223b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h0f29aace0f882afd + 11
4   libstd-0e31801c7f918c72.dylib  0x001a2eac std::sys_common::backtrace::__rust_begin_short_backtrace::h2e5278a34b5b3cac + 12
5   libstd-0e31801c7f918c72.dylib  0x001a5a24 std::panicking::try::do_call::hfe83d4a945e58d59 + 20
6   libstd-0e31801c7f918c72.dylib  0x001b467d __rust_maybe_catch_panic + 29
7   libstd-0e31801c7f918c72.dylib  0x001a64c7 std::rt::lang_start_internal::h9aeb4048ce4bf257 + 631
8   a                              0x000e20fc main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff20700  ebx: 0xbff20680  ecx: 0x000e0b9e  edx: 0xbff20680
  edi: 0x000df354  esi: 0x00000000  ebp: 0xbff20678  esp: 0xbff20640
   ss: 0x00000023  efl: 0x00010246  eip: 0x000e0bf6   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000e0830
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xde000 -    0xe2fc7 +a (0) <76E2CBAC-AD50-3553-A087-BBFFF566C6BF> /Users/USER/*/a
   0xfb000 -   0x140fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x184000 -   0x213ffb +libstd-0e31801c7f918c72.dylib (0) <674D689C-6129-3C68-BDF2-AEB56FC106BF> /Users/USER/*/libstd-0e31801c7f918c72.dylib
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
    task_for_pid: 2592
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
__DATA                            3532K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-084156-1_Traviss-Mac-1044.crash
Process:               a [51014]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51012]
Responsible:           a [51014]
User ID:               501
Date/Time:             2019-05-25 08:41:55.370 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5700 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b077fe68
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb077fe68:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b077f000-00000000b0780000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0780000-00000000b0981000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-0e31801c7f918c72.dylib  0x0015f2a0 std::sys::unix::thread::Thread::join::h99c3c5e9668159e7 + 32
4   a                              0x000067b6 std::thread::JoinHandle$LT$T$GT$::join::hd0ea2b41ff0f4e12 + 70
5   a                              0x000059d5 stack_probes::main::h385e455299c91d04 + 597
6   a                              0x000047eb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hbf776e80e2ba52fe + 11
7   libstd-0e31801c7f918c72.dylib  0x0014eeac std::sys_common::backtrace::__rust_begin_short_backtrace::h2e5278a34b5b3cac + 12
8   libstd-0e31801c7f918c72.dylib  0x00151a24 std::panicking::try::do_call::hfe83d4a945e58d59 + 20
9   libstd-0e31801c7f918c72.dylib  0x0016067d __rust_maybe_catch_panic + 29
10  libstd-0e31801c7f918c72.dylib  0x001524c7 std::rt::lang_start_internal::h9aeb4048ce4bf257 + 631
11  a                              0x000063fc main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-0e31801c7f918c72.dylib  0x0015fa9b std::sys::unix::abort_internal::hf98dcaece2b04055 + 11
4   libstd-0e31801c7f918c72.dylib  0x0014fe89 std::sys_common::util::abort::h15892f92d50adb0b + 73
5   libstd-0e31801c7f918c72.dylib  0x0015ed3f std::sys::unix::stack_overflow::imp::signal_handler::hb91daec24a8f4714 + 943
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-0e31801c7f918c72.dylib  0x0015e990 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h9ee1814d6a6b1355 + 80
9   a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
10  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
11  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
12  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
13  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
14  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
15  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
16  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
17  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
18  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
19  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
20  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
21  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
22  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
23  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
24  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
25  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
26  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
27  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
28  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
29  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
30  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
31  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
32  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
33  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
34  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
35  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
36  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
37  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
38  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
39  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
40  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
41  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
42  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
43  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
44  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
45  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
46  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
47  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
48  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
49  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
50  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
51  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
52  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
53  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
54  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
55  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
56  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
57  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
58  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
59  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
60  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
61  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
62  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
63  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
64  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
65  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
66  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
67  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
68  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
69  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
70  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
71  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
72  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
73  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
74  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
75  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
76  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
77  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
78  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
79  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
80  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
81  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
82  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
83  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
84  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
85  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
86  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
87  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
88  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
89  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
90  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
91  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
92  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
93  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
94  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
95  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
96  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
97  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
98  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
99  a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
100 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
101 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
102 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
103 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
104 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
105 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
106 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
107 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
108 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
109 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
110 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
111 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
112 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
113 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
114 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
115 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
116 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
117 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
118 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
119 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
120 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
121 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
122 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
123 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
124 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
125 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
126 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
127 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
128 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
129 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
130 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
131 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
132 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
133 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
134 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
135 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
136 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
137 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
138 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
139 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
140 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
141 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
142 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
143 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
144 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
145 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
146 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
147 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
148 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
149 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
150 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
151 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
152 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
153 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
154 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
155 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
156 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
157 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
158 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
159 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
160 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
161 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
162 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
163 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
164 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
165 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
166 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
167 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
168 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
169 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
170 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
171 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
172 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
173 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
174 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
175 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
176 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
177 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
178 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
179 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
180 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
181 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
182 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
183 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
184 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
185 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
186 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
187 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
188 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
189 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
190 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
191 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
192 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
193 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
194 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
195 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
196 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
197 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
198 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
199 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
200 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
201 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
202 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
203 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
204 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
205 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
206 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
207 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
208 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
209 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
210 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
211 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
212 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
213 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
214 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
215 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
216 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
217 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
218 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
219 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
220 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
221 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
222 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
223 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
224 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
225 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
226 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
227 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
228 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
229 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
230 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
231 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
232 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
233 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
234 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
235 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
236 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
237 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
238 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
239 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
240 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
241 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
242 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
243 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
244 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
245 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
246 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
247 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
248 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
249 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
250 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
251 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
252 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
253 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
254 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
255 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
256 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
257 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
258 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
259 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
260 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
261 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
262 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
263 a                              0x00005b20 stack_probes::recurse::h35e743424666f0f3 + 48
264 a                              0x0000484d std::sys_common::backtrace::__rust_begin_short_backtrace::h14e6ee10d11e0ee3 (.llvm.12057741433189728042) + 29
265 libstd-0e31801c7f918c72.dylib  0x0016067d __rust_maybe_catch_panic + 29
266 a                              0x00006b13 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd95ff991ba2a68cb + 131
267 libstd-0e31801c7f918c72.dylib  0x001367b1 _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hfa972e553e3af5f8 + 65
268 libstd-0e31801c7f918c72.dylib  0x0015f1d8 std::sys::unix::thread::Thread::new::thread_start::hf9f66551bfbdabc7 + 184
269 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
270 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
271 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb0980000  ecx: 0x00059b0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x00059b38  esp: 0x00059b0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0015f280
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
    0x3000 -     0x7ffb +a (0) <10D3269A-1F88-391E-834F-E4F9B7B9E60D> /Users/USER/*/a
   0xa7000 -    0xecfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x130000 -   0x1bfffb +libstd-0e31801c7f918c72.dylib (0) <674D689C-6129-3C68-BDF2-AEB56FC106BF> /Users/USER/*/libstd-0e31801c7f918c72.dylib
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-084202-1_Traviss-Mac-1044.crash
Process:               a [51128]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [51126]
Responsible:           a [51128]
User ID:               501
Date/Time:             2019-05-25 08:42:01.656 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5700 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf54b38
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf54b38:
    Stack Guard            00000000bbf53000-00000000bbf54000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf54000-00000000bbf55000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf55000-00000000bff53000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000b228b std::sys::unix::abort_internal::hf98dcaece2b04055 + 11
4   a                              0x000b2279 std::sys_common::util::abort::h15892f92d50adb0b + 73
5   a                              0x000c0c33 std::sys::unix::stack_overflow::imp::signal_handler::hb91daec24a8f4714 + 851
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000c08e0 rust_begin_unwind + 16
9   a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
10  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
11  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
12  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
13  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
14  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
15  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
16  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
17  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
18  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
19  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
20  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
21  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
22  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
23  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
24  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
25  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
26  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
27  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
28  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
29  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
30  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
31  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
32  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
33  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
34  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
35  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
36  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
37  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
38  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
39  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
40  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
41  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
42  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
43  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
44  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
45  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
46  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
47  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
48  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
49  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
50  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
51  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
52  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
53  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
54  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
55  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
56  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
57  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
58  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
59  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
60  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
61  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
62  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
63  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
64  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
65  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
66  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
67  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
68  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
69  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
70  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
71  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
72  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
73  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
74  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
75  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
76  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
77  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
78  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
79  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
80  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
81  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
82  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
83  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
84  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
85  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
86  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
87  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
88  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
89  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
90  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
91  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
92  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
93  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
94  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
95  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
96  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
97  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
98  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
99  a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
100 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
101 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
102 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
103 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
104 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
105 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
106 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
107 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
108 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
109 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
110 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
111 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
112 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
113 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
114 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
115 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
116 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
117 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
118 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
119 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
120 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
121 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
122 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
123 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
124 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
125 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
126 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
127 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
128 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
129 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
130 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
131 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
132 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
133 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
134 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
135 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
136 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
137 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
138 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
139 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
140 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
141 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
142 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
143 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
144 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
145 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
146 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
147 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
148 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
149 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
150 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
151 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
152 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
153 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
154 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
155 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
156 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
157 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
158 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
159 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
160 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
161 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
162 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
163 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
164 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
165 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
166 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
167 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
168 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
169 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
170 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
171 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
172 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
173 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
174 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
175 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
176 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
177 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
178 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
179 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
180 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
181 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
182 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
183 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
184 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
185 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
186 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
187 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
188 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
189 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
190 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
191 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
192 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
193 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
194 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
195 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
196 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
197 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
198 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
199 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
200 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
201 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
202 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
203 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
204 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
205 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
206 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
207 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
208 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
209 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
210 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
211 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
212 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
213 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
214 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
215 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
216 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
217 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
218 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
219 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
220 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
221 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
222 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
223 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
224 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
225 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
226 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
227 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
228 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
229 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
230 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
231 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
232 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
233 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
234 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
235 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
236 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
237 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
238 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
239 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
240 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
241 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
242 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
243 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
244 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
245 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
246 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
247 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
248 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
249 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
250 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
251 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
252 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
253 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
254 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
255 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
256 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
257 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
258 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
259 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
260 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
261 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
262 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
263 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
264 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
265 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
266 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
267 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
268 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
269 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
270 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
271 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
272 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
273 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
274 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
275 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
276 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
277 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
278 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
279 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
280 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
281 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
282 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
283 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
284 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
285 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
286 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
287 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
288 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
289 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
290 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
291 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
292 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
293 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
294 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
295 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
296 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
297 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
298 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
299 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
300 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
301 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
302 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
303 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
304 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
305 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
306 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
307 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
308 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
309 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
310 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
311 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
312 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
313 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
314 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
315 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
316 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
317 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
318 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
319 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
320 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
321 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
322 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
323 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
324 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
325 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
326 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
327 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
328 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
329 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
330 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
331 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
332 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
333 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
334 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
335 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
336 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
337 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
338 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
339 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
340 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
341 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
342 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
343 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
344 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
345 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
346 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
347 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
348 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
349 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
350 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
351 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
352 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
353 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
354 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
355 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
356 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
357 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
358 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
359 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
360 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
361 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
362 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
363 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
364 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
365 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
366 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
367 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
368 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
369 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
370 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
371 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
372 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
373 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
374 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
375 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
376 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
377 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
378 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
379 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
380 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
381 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
382 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
383 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
384 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
385 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
386 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
387 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
388 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
389 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
390 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
391 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
392 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
393 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
394 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
395 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
396 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
397 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
398 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
399 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
400 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
401 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
402 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
403 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
404 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
405 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
406 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
407 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
408 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
409 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
410 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
411 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
412 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
413 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
414 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
415 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
416 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
417 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
418 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
419 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
420 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
421 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
422 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
423 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
424 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
425 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
426 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
427 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
428 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
429 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
430 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
431 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
432 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
433 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
434 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
435 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
436 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
437 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
438 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
439 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
440 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
441 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
442 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
443 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
444 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
445 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
446 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
447 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
448 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
449 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
450 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
451 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
452 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
453 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
454 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
455 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
456 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
457 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
458 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
459 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
460 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
461 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
462 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
463 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
464 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
465 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
466 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
467 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
468 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
469 a                              0x000afc88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
---
===========                     =======  ======= 
TOTAL                            565.6M      130 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-25-084202_Traviss-Mac-1044.crash
Process:               a [51129]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51126]
Responsible:           a [51129]
User ID:               501
Date/Time:             2019-05-25 08:42:01.699 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5700 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0135ec8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0135ec8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0135000-00000000b0136000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0136000-00000000b0337000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   a                              0x000b57d1 stack_probes_lto::main::haeaf2f44e7253541 + 2465
4   a                              0x000ce4bb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hee6c37cc5518ccb5 + 11
5   a                              0x000c6cdc std::sys_common::backtrace::__rust_begin_short_backtrace::h2e5278a34b5b3cac + 12
6   a                              0x000b6efd main + 589
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000b828b std::sys::unix::abort_internal::hf98dcaece2b04055 + 11
4   a                              0x000b8279 std::sys_common::util::abort::h15892f92d50adb0b + 73
5   a                              0x000c6c33 std::sys::unix::stack_overflow::imp::signal_handler::hb91daec24a8f4714 + 851
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000c68e0 rust_begin_unwind + 16
9   a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
10  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
11  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
12  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
13  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
14  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
15  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
16  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
17  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
18  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
19  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
20  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
21  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
22  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
23  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
24  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
25  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
26  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
27  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
28  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
29  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
30  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
31  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
32  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
33  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
34  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
35  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
36  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
37  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
38  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
39  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
40  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
41  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
42  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
43  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
44  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
45  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
46  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
47  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
48  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
49  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
50  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
51  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
52  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
53  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
54  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
55  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
56  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
57  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
58  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
59  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
60  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
61  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
62  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
63  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
64  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
65  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
66  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
67  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
68  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
69  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
70  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
71  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
72  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
73  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
74  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
75  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
76  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
77  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
78  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
79  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
80  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
81  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
82  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
83  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
84  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
85  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
86  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
87  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
88  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
89  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
90  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
91  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
92  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
93  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
94  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
95  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
96  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
97  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
98  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
99  a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
100 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
101 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
102 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
103 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
104 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
105 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
106 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
107 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
108 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
109 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
110 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
111 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
112 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
113 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
114 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
115 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
116 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
117 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
118 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
119 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
120 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
121 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
122 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
123 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
124 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
125 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
126 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
127 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
128 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
129 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
130 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
131 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
132 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
133 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
134 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
135 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
136 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
137 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
138 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
139 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
140 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
141 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
142 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
143 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
144 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
145 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
146 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
147 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
148 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
149 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
150 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
151 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
152 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
153 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
154 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
155 a                              0x000b5c88 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
156 a                       
