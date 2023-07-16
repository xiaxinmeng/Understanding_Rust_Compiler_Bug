plain
[00:03:13]       Memory: 8 GB
[00:03:13]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:13]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:13]       SMC Version (system): 2.8f0
[00:03:13]       Serial Number (system): VMdBl+hpWEu3
[00:03:13] 
[00:03:13] hw.ncpu: 4
[00:03:13] hw.byteorder: 1234
[00:03:13] hw.memsize: 8589934592
---
[02:03:24] 
[02:03:24] ---- /Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/listing/allowed-by-default.md - Allowed_by_default_lints::anonymous_parameters (line 32) stdout ----
[02:03:24] error: linking with `cc` failed: signal: 4
[02:03:24]   |
[02:03:24]   = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestThgZJ6/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestThgZJ6/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestThgZJ6/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestThgZJ6/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestThgZJ6/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestThgZJ6/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestThgZJ6/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libstd-e19c6fd124ab426a.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libpanic_unwind-922c0f31abefcc87.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libbacktrace_sys-5f9a36a9e2c857d6.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_demangle-baf6c52b660f2145.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libhashbrown-c52850f3028d8246.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_alloc-34f211e0b5b4bb43.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libunwind-e4c87afe5f0e3a8b.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liblibc-b18dfaed99ca7a71.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liballoc-f86d19932541f7e5.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_core-bc86d4a5a44cda78.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcore-75cce831a556c528.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-41f371c652d8b71f.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:03:24] 
[02:03:24] error: aborting due to previous error
[02:03:24] 
[02:03:24] thread '/Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/listing/allowed-by-default.md - Allowed_by_default_lints::anonymous_parameters (line 32)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:319:13
---
[02:03:24] 
[02:03:24] 
[02:03:24] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:03:24] Build completed unsuccessfully in 0:49:10
[02:03:24] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a8a558a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May 19 01:55:03 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:0916d2d8
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1176
drwx------  26 travis  staff    884 May 19 01:54 .
-rw-------@  1 travis  staff   1387 May 19 01:54 foo_2019-05-19-015422_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1377 May 19 01:53 m4_2019-05-19-015352_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1391 May 19 01:53 bar_2019-05-19-015343_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9899 May 19 01:53 b_2019-05-19-015342_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1403 May 19 01:53 bar_2019-05-19-015342_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  57364 May 19 01:21 a_2019-05-19-012136_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34964 May 19 01:21 a_2019-05-19-012135_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55585 May 19 01:21 a_2019-05-19-012129-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34890 May 19 01:21 a_2019-05-19-012129_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 May 19 01:21 a_2019-05-19-012123_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9166 May 19 01:21 a_2019-05-19-012119_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9171 May 19 01:21 a_2019-05-19-012112_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8936 May 19 01:21 a_2019-05-19-012111_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9304 May 19 01:20 a_2019-05-19-012037_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58251 May 19 01:20 a_2019-05-19-012028_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59516 May 19 01:20 a_2019-05-19-012024-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 May 19 01:20 a_2019-05-19-012024_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60372 May 19 01:20 a_2019-05-19-012023_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10881 May 19 01:18 a_2019-05-19-011822_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 May 19 01:17 a_2019-05-19-011732_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 May 19 01:16 a_2019-05-19-011622_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 May 19 01:15 a_2019-05-19-011525-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9780 May 19 01:15 a_2019-05-19-011525_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9484 May 19 01:15 a_2019-05-19-011522_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0eb0efd8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-19-011522_Traviss-Mac-1044.crash
Process:               a [40422]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [40419]
Responsible:           a [40422]
User ID:               501
Date/Time:             2019-05-19 01:15:03.179 +0000
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
0   a                              0x00078b0e abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 46
1   a                              0x00077f5b std::panicking::try::do_call::h7e36174869ccfd96 (.llvm.12471302550891041630) + 11
2   libstd-e19c6fd124ab426a.dylib  0x001a635d __rust_maybe_catch_panic + 29
3   a                              0x00078d75 abort_on_c_abi::main::he771bf881fc862e3 + 613
4   a                              0x000775bb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::he632041fd3e68316 + 11
5   libstd-e19c6fd124ab426a.dylib  0x00194c6c std::sys_common::backtrace::__rust_begin_short_backtrace::h213ec381daa40f46 + 12
6   libstd-e19c6fd124ab426a.dylib  0x001977d4 std::panicking::try::do_call::h6e25e413421a370b + 20
7   libstd-e19c6fd124ab426a.dylib  0x001a635d __rust_maybe_catch_panic + 29
8   libstd-e19c6fd124ab426a.dylib  0x00198277 std::rt::lang_start_internal::h4758d469cab088c0 + 631
9   a                              0x000790ac main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x79655b20  ebx: 0xbff88b58  ecx: 0x00000000  edx: 0x00000000
  edi: 0x001a634e  esi: 0x00000000  ebp: 0xbff88af8  esp: 0xbff88ae0
   ss: 0x00000023  efl: 0x00010296  eip: 0x00078b0e   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x001dfd14
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x76000 -    0x79ffb +a (0) <BC0507C7-4409-3AD1-954E-DDF1A34E0608> /Users/USER/*/a
   0xed000 -   0x132fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x176000 -   0x205ff3 +libstd-e19c6fd124ab426a.dylib (0) <FF0426F0-BEFF-3AE8-A073-5B9841972296> /Users/USER/*/libstd-e19c6fd124ab426a.dylib
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
    task_for_pid: 2412
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
__DATA                            3560K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9344K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            568.6M      134 
TOTAL                            568.6M      134 
TOTAL, minus reserved VM space   568.4M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-19-011525-1_Traviss-Mac-1044.crash
Process:               a [41213]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [41205]
Responsible:           a [41213]
User ID:               501
Date/Time:             2019-05-19 01:15:24.407 +0000
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
0   libstd-e19c6fd124ab426a.dylib  0x001a2ce3 std::panicking::rust_panic_with_hook::he934d38f653d062b + 115
1   a                              0x0007bbff std::panicking::begin_panic::h0d1eb084ec9b9923 + 47 (panicking.rs:408)
2   a                              0x000796e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 36 (backtrace.rs:25)
3   a                              0x000790cb core::ptr::real_drop_in_place::ha0fd09e913297feb + 11
4   a                              0x000796b3 backtrace::double::h35adec2a6f63ef6c + 51
5   a                              0x0007a9c8 backtrace::main::hc9a5bc8fc93ded64 + 4568 (backtrace.rs:104)
6   a                              0x00078bbb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf526fee767d31873 + 11 (rt.rs:64)
7   libstd-e19c6fd124ab426a.dylib  0x0019fc6c std::sys_common::backtrace::__rust_begin_short_backtrace::h213ec381daa40f46 + 12
8   libstd-e19c6fd124ab426a.dylib  0x001a27d4 std::panicking::try::do_call::h6e25e413421a370b + 20
9   libstd-e19c6fd124ab426a.dylib  0x001b135d __rust_maybe_catch_panic + 29
10  libstd-e19c6fd124ab426a.dylib  0x001a3277 std::rt::lang_start_internal::h4758d469cab088c0 + 631
11  a                              0x0007b21c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff888a8  ebx: 0xbff888f0  ecx: 0xbff887a0  edx: 0xa7702ec6
  edi: 0x001eb058  esi: 0x001a2c7e  ebp: 0xbff88948  esp: 0xbff888c0
   ss: 0x00000023  efl: 0x00010286  eip: 0x001a2ce3   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0046fa65
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x76000 -    0x7cff7 +a (0) <5850BD58-636B-37CC-ABE0-F9990E78CD0C> /Users/USER/*/a
   0xf8000 -   0x13dfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x181000 -   0x210ff3 +libstd-e19c6fd124ab426a.dylib (0) <FF0426F0-BEFF-3AE8-A073-5B9841972296> /Users/USER/*/libstd-e19c6fd124ab426a.dylib
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
    task_for_pid: 2412
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=73.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9620K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3560K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9356K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.0M      136 
TOTAL                            569.0M      136 
TOTAL, minus reserved VM space   568.8M      136 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-19-011525_Traviss-Mac-1044.crash
Process:               a [41212]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [41205]
Responsible:           a [41212]
User ID:               501
Date/Time:             2019-05-19 01:15:24.366 +0000
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
0   libstd-e19c6fd124ab426a.dylib  0x0018dce3 std::panicking::rust_panic_with_hook::he934d38f653d062b + 115
1   a                              0x0006ebff std::panicking::begin_panic::h0d1eb084ec9b9923 + 47 (panicking.rs:408)
2   a                              0x0006c6e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 36 (backtrace.rs:25)
3   a                              0x0006c0cb core::ptr::real_drop_in_place::ha0fd09e913297feb + 11
4   a                              0x0006c6b3 backtrace::double::h35adec2a6f63ef6c + 51
5   a                              0x0006d9c8 backtrace::main::hc9a5bc8fc93ded64 + 4568 (backtrace.rs:104)
6   a                              0x0006bbbb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf526fee767d31873 + 11 (rt.rs:64)
7   libstd-e19c6fd124ab426a.dylib  0x0018ac6c std::sys_common::backtrace::__rust_begin_short_backtrace::h213ec381daa40f46 + 12
8   libstd-e19c6fd124ab426a.dylib  0x0018d7d4 std::panicking::try::do_call::h6e25e413421a370b + 20
9   libstd-e19c6fd124ab426a.dylib  0x0019c35d __rust_maybe_catch_panic + 29
10  libstd-e19c6fd124ab426a.dylib  0x0018e277 std::rt::lang_start_internal::h4758d469cab088c0 + 631
11  a                              0x0006e21c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff958c8  ebx: 0xbff95910  ecx: 0xbff957c0  edx: 0xa7702ec6
  edi: 0x001d6058  esi: 0x0018dc7e  ebp: 0xbff95968  esp: 0xbff958e0
   ss: 0x00000023  efl: 0x00010286  eip: 0x0018dce3   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0045aa65
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x69000 -    0x6fff7 +a (0) <5850BD58-636B-37CC-ABE0-F9990E78CD0C> /Users/USER/*/a
   0xe3000 -   0x128fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x16c000 -   0x1fbff3 +libstd-e19c6fd124ab426a.dylib (0) <FF0426F0-BEFF-3AE8-A073-5B9841972296> /Users/USER/*/libstd-e19c6fd124ab426a.dylib
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
    task_for_pid: 2412
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=73.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9620K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3560K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9356K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.0M      136 
TOTAL                            569.0M      136 
TOTAL, minus reserved VM space   568.8M      136 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-19-011622_Traviss-Mac-1044.crash
Process:               a [42930]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [42928]
Responsible:           a [42930]
User ID:               501
Date/Time:             2019-05-19 01:16:21.290 +0000
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
3   libstd-e19c6fd124ab426a.dylib  0x0021477b std::sys::unix::abort_internal::hc95bf10286d6296b + 11
4   libstd-e19c6fd124ab426a.dylib  0x00205e60 rust_oom + 48
5   libstd-e19c6fd124ab426a.dylib  0x00227094 alloc::alloc::handle_alloc_error::h3de3f6160e4e1656 + 20
6   a                              0x000754bd default_alloc_error_hook::main::h0fe124586986ad5d + 781
7   a                              0x0007491b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h06cfc317c8eb7693 + 11
8   libstd-e19c6fd124ab426a.dylib  0x00203c6c std::sys_common::backtrace::__rust_begin_short_backtrace::h213ec381daa40f46 + 12
9   libstd-e19c6fd124ab426a.dylib  0x002067d4 std::panicking::try::do_call::h6e25e413421a370b + 20
10  libstd-e19c6fd124ab426a.dylib  0x0021535d __rust_maybe_catch_panic + 29
11  libstd-e19c6fd124ab426a.dylib  0x00207277 std::rt::lang_start_internal::h4758d469cab088c0 + 631
12  a                              0x0007561c main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff8aa6c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff8aa98  esp: 0xbff8aa6c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x74000 -    0x75ff3 +a (0) <0817E120-F480-33BD-9EB5-904DC3DA9BA8> /Users/USER/*/a
  0x15c000 -   0x1a1fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1e5000 -   0x274ff3 +libstd-e19c6fd124ab426a.dylib (0) <FF0426F0-BEFF-3AE8-A073-5B9841972296> /Users/USER/*/libstd-e19c6fd124ab426a.dylib
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
TOTAL                            569.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-19-012028_Traviss-Mac-1044.crash
Process:               a [49383]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [49377]
Responsible:           a [49383]
User ID:               501
Date/Time:             2019-05-19 01:20:28.771 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf13fdc
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf13fdc:
    Stack Guard            00000000bbf12000-00000000bbf13000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf13000-00000000bbf14000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf14000-00000000bff12000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-e19c6fd124ab426a.dylib  0x0027177b std::sys::unix::abort_internal::hc95bf10286d6296b + 11
4   libstd-e19c6fd124ab426a.dylib  0x00261c49 std::sys_common::util::abort::hbac7dac51444ffae + 73
5   libstd-e19c6fd124ab426a.dylib  0x00270a18 std::sys::unix::stack_overflow::imp::signal_handler::h532e135313fe6f2f + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-e19c6fd124ab426a.dylib  0x00270660 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h970ffc97bbecde05 + 80
9   libstd-e19c6fd124ab426a.dylib  0x0024ebdc _$LT$std..io..buffered..LineWriter$LT$W$GT$$u20$as$u20$std..io..Write$GT$::write::hf30cd4758197fe7c + 220
10  libstd-e19c6fd124ab426a.dylib  0x002537a5 std::io::Write::write_all::h8e028d04af27fd42 + 101
11  libstd-e19c6fd124ab426a.dylib  0x00253b33 _$LT$std..io..Write..write_fmt..Adaptor$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$::write_str::hbd4f9829cdbda6fc + 35
12  libstd-e19c6fd124ab426a.dylib  0x00291d7d core::fmt::write::hbce2d277ad333e62 + 701
13  libstd-e19c6fd124ab426a.dylib  0x00251746 _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::h37bcb36c7cff8c7a + 182
14  libstd-e19c6fd124ab426a.dylib  0x00252c7c std::io::stdio::_print::hdb3f69b53a703576 + 396
15  a                              0x000f0d4f out_of_stack::loud_recurse::h48447a21f69c373e + 63
16  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
17  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
18  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
19  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
20  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
21  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
22  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
23  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
24  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
25  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
26  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
27  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
28  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
29  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
30  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
31  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
32  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
33  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
34  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
35  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
36  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
37  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
38  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
39  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
40  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
41  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
42  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
43  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
44  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
45  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
46  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
47  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
48  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
49  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
50  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
51  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
52  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
53  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
54  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
55  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
56  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
57  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
58  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
59  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
60  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
61  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
62  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
63  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
64  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
65  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
66  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
67  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
68  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
69  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
70  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
71  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
72  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
73  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
74  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
75  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
76  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
77  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
78  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
79  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
80  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
81  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
82  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
83  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
84  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
85  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
86  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
87  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
88  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
89  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
90  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
91  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
92  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
93  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
94  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
95  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
96  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
97  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
98  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
99  a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
100 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
101 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
102 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
103 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
104 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
105 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
106 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
107 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
108 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
109 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
110 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
111 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
112 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
113 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
114 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
115 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
116 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
117 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
118 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
119 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
120 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
121 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
122 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
123 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
124 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
125 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
126 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
127 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
128 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
129 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
130 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
131 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
132 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
133 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
134 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
135 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
136 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
137 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
138 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
139 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
140 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
141 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
142 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
143 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
144 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
145 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
146 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
147 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
148 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
149 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
150 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
151 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
152 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
153 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
154 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
155 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
156 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
157 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
158 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
159 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
160 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
161 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
162 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
163 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
164 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
165 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
166 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
167 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
168 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
169 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
170 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
171 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
172 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
173 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
174 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
175 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
176 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
177 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
178 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
179 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
180 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
181 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
182 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
183 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
184 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
185 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
186 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
187 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
188 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
189 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
190 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
191 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
192 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
193 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
194 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
195 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
196 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
197 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
198 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
199 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
200 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
201 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
202 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
203 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
204 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
205 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
206 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
207 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
208 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
209 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
210 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
211 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
212 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
213 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
214 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
215 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
216 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
217 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
218 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
219 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
220 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
221 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
222 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
223 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
224 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
225 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
226 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
227 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
228 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
229 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
230 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
231 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
232 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
233 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
234 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
235 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
236 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
237 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
238 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
239 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
240 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
241 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
242 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
243 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
244 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
245 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
246 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
247 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
248 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
249 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
250 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
251 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
252 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
253 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
254 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
255 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
256 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
257 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
258 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
259 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
260 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
261 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
262 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
263 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
264 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
265 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
266 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
267 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
268 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
269 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
270 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
271 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
272 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
273 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
274 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
275 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
276 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
277 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
278 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
279 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
280 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
281 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
282 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
283 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
284 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
285 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
286 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
287 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
288 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
289 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
290 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
291 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
292 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
293 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
294 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
295 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
296 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
297 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
298 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
299 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
300 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
301 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
302 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
303 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
304 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
305 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
306 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
307 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
308 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
309 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
310 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
311 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
312 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
313 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
314 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
315 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
316 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
317 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
318 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
319 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
320 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
321 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
322 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
323 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
324 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
325 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
326 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
327 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
328 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
329 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
330 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
331 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
332 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
333 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
334 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
335 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
336 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
337 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
338 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
339 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
340 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
341 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
342 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
343 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
344 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
345 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
346 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
347 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
348 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
349 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
350 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
351 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
352 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
353 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
354 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
355 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
356 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
357 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
358 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
359 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
360 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
361 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
362 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
363 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
364 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
365 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
366 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
367 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
368 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
369 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
370 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
371 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
372 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
373 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
374 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
375 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
376 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
377 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
378 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
379 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
380 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
381 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
382 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
383 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
384 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
385 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
386 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
387 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
388 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
389 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
390 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
391 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
392 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
393 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
394 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
395 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
396 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
397 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
398 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
399 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
400 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
401 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
402 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
403 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
404 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
405 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
406 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
407 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
408 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
409 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
410 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
411 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
412 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
413 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
414 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
415 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
416 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
417 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
418 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
419 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
420 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
421 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
422 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
423 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
424 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
425 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
426 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
427 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
428 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
429 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
430 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
431 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
432 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
433 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
434 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
435 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
436 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
437 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
438 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
439 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
440 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
441 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
442 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
443 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
444 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
445 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
446 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
447 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
448 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
449 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
450 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
451 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
452 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
453 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
454 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
455 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
456 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
457 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
458 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
459 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
460 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
461 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
462 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
463 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
464 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
465 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
466 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
467 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
468 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
469 a                              0x000f0d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
---
===========                     =======  ======= 
TOTAL                            578.6M      135 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-19-012037_Traviss-Mac-1044.crash
Process:               a [49581]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [49580]
Responsible:           a [49581]
User ID:               501
Date/Time:             2019-05-19 01:20:35.976 +0000
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
3   a                              0x000dd56b panic_abort::__rust_start_panic::abort::h8f85775f13b4187f + 11
4   a                              0x000dd55b __rust_start_panic + 11
5   a                              0x000d1c6b rust_panic + 11
6   a                              0x000d1855 std::panicking::rust_panic_with_hook::he934d38f653d062b + 997
7   a                              0x000e32ea std::panicking::begin_panic::h239a4969d92a0edc + 42
8   a                              0x000d062d lto_abort::main::hd3a7c21d2a5e6b65 + 2877
9   a                              0x000e344b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd95cfe66e335d4bd + 11
10  a                              0x000dd3dc std::sys_common::backtrace::__rust_begin_short_backtrace::h213ec381daa40f46 + 12
11  a                              0x000d0a08 main + 984
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff2fa4c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff2fa78  esp: 0xbff2fa4c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xcf000 -    0xf2fff +a (0) <6844AF44-479F-3203-A626-D1EE62EA7EA1> /Users/USER/*/a
  0x1d9000 -   0x21efdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2664
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-19-012111_Traviss-Mac-1044.crash
Process:               a [50549]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50544]
Responsible:           a [50549]
User ID:               501
Date/Time:             2019-05-19 01:21:10.951 +0000
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
3   libstd-e19c6fd124ab426a.dylib  0x001e777b std::sys::unix::abort_internal::hc95bf10286d6296b + 11
4   libstd-e19c6fd124ab426a.dylib  0x001d7c49 std::sys_common::util::abort::hbac7dac51444ffae + 73
5   libstd-e19c6fd124ab426a.dylib  0x001d9ff2 rust_panic + 98
6   libstd-e19c6fd124ab426a.dylib  0x001d9ebe std::panicking::rust_panic_with_hook::he934d38f653d062b + 590
7   a                              0x000a09cf std::panicking::begin_panic::h4602bea5f62e9c4a + 47
8   a                              0x000a1b0c main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff5fa4c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff5fa78  esp: 0xbff5fa4c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x9f000 -    0xa2fff +a (0) <DC91664C-97F9-37AB-A22D-EAC607E5EA38> /Users/USER/*/a
  0x12f000 -   0x174fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1b8000 -   0x247ff3 +libstd-e19c6fd124ab426a.dylib (0) <FF0426F0-BEFF-3AE8-A073-5B9841972296> /Users/USER/*/libstd-e19c6fd124ab426a.dylib
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
    task_for_pid: 2664
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
__DATA                            3560K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-19-012112_Traviss-Mac-1044.crash
Process:               a [50576]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50573]
Responsible:           a [50576]
User ID:               501
Date/Time:             2019-05-19 01:21:11.830 +0000
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
    __TEXT                 0000000000066000-0000000000069000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00067f72 segfault_no_out_of_stack::main::h579ee62fe996355a + 2034
1   a                              0x0006699b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h2638423b4083885d + 11
2   libstd-e19c6fd124ab426a.dylib  0x0016fc6c std::sys_common::backtrace::__rust_begin_short_backtrace::h213ec381daa40f46 + 12
3   libstd-e19c6fd124ab426a.dylib  0x001727d4 std::panicking::try::do_call::h6e25e413421a370b + 20
4   libstd-e19c6fd124ab426a.dylib  0x0018135d __rust_maybe_catch_panic + 29
5   libstd-e19c6fd124ab426a.dylib  0x00173277 std::rt::lang_start_internal::h4758d469cab088c0 + 631
6   a                              0x0006824c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x79656540  ecx: 0x00000000  edx: 0x00000000
  edi: 0x0018134e  esi: 0xbff98b80  ebp: 0xbff98c68  esp: 0xbff98ac0
   ss: 0x00000023  efl: 0x00010246  eip: 0x00067f72   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     3
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x66000 -    0x68ffb +a (0) <BFD46A66-E15A-38A3-B042-31BD330289FF> /Users/USER/*/a
   0xc8000 -   0x10dfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x151000 -   0x1e0ff3 +libstd-e19c6fd124ab426a.dylib (0) <FF0426F0-BEFF-3AE8-A073-5B9841972296> /Users/USER/*/libstd-e19c6fd124ab426a.dylib
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
    task_for_pid: 2664
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=84.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=84.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            20.0M       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                        132K        3 
__DATA                            3560K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            579.6M      136 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-19-012119_Traviss-Mac-1044.crash
Process:               a [50774]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50772]
Responsible:           a [50774]
User ID:               501
Date/Time:             2019-05-19 01:21:18.939 +0000
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
    __TEXT                 0000000000063000-0000000000066000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000655d4 signal_exit_status::main::hc6663d816ec186eb + 436
1   a                              0x0006447b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h441ce4164af07677 + 11
2   libstd-e19c6fd124ab426a.dylib  0x00201c6c std::sys_common::backtrace::__rust_begin_short_backtrace::h213ec381daa40f46 + 12
3   libstd-e19c6fd124ab426a.dylib  0x002047d4 std::panicking::try::do_call::h6e25e413421a370b + 20
4   libstd-e19c6fd124ab426a.dylib  0x0021335d __rust_maybe_catch_panic + 29
5   libstd-e19c6fd124ab426a.dylib  0x00205277 std::rt::lang_start_internal::h4758d469cab088c0 + 631
6   a                              0x000656ac main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x7be2d430
  edi: 0x7be2d4c0  esi: 0xbff9bbe0  ebp: 0xbff9bc78  esp: 0xbff9bb60
   ss: 0x00000023  efl: 0x00010246  eip: 0x000655d4   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     1
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x63000 -    0x65ff7 +a (0) <D2188246-EBBF-3CFE-A1FE-33AEA7CD624E> /Users/USER/*/a
  0x15a000 -   0x19ffdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1e3000 -   0x272ff3 +libstd-e19c6fd124ab426a.dylib (0) <FF0426F0-BEFF-3AE8-A073-5B9841972296> /Users/USER/*/libstd-e19c6fd124ab426a.dylib
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
    task_for_pid: 2664
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
__DATA                            3560K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.6M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-19-012123_Traviss-Mac-1044.crash
Process:               a [50877]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50872]
Responsible:           a [50877]
User ID:               501
Date/Time:             2019-05-19 01:21:23.534 +0000
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
0   a                              0x0009d966 simd_target_feature_mixup::test::id_avx512_512::hf3a1395d43161fbe + 102
1   a                              0x0009c71f simd_target_feature_mixup::test::main::h611f15c116e6c273 + 1647
2   a                              0x0009ec90 simd_target_feature_mixup::main::hfcef50d014c08dda + 896
3   a                              0x0009efab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h54f7f12b72da9875 + 11
4   libstd-e19c6fd124ab426a.dylib  0x00205c6c std::sys_common::backtrace::__rust_begin_short_backtrace::h213ec381daa40f46 + 12
5   libstd-e19c6fd124ab426a.dylib  0x002087d4 std::panicking::try::do_call::h6e25e413421a370b + 20
6   libstd-e19c6fd124ab426a.dylib  0x0021735d __rust_maybe_catch_panic + 29
7   libstd-e19c6fd124ab426a.dylib  0x00209277 std::rt::lang_start_internal::h4758d469cab088c0 + 631
8   a                              0x0009ee6c main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff638c0  ebx: 0xbff63840  ecx: 0x0009d90e  edx: 0xbff63840
  edi: 0x0009c0c4  esi: 0x00000000  ebp: 0xbff63838  esp: 0xbff63800
   ss: 0x00000023  efl: 0x00010246  eip: 0x0009d966   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0009d5a0
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x9b000 -    0x9ffef +a (0) <DBB36068-26DC-3FF2-A3E9-B5F5C92BD432> /Users/USER/*/a
  0x15e000 -   0x1a3fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1e7000 -   0x276ff3 +libstd-e19c6fd124ab426a.dylib (0) <FF0426F0-BEFF-3AE8-A073-5B9841972296> /Users/USER/*/libstd-e19c6fd124ab426a.dylib
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
    task_for_pid: 2664
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
__DATA                            3560K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9348K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            568.6M      134 
TOTAL                            568.6M      134 
TOTAL, minus reserved VM space   568.4M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-19-012129-1_Traviss-Mac-1044.crash
Process:               a [51020]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51019]
Responsible:           a [51020]
User ID:               501
Date/Time:             2019-05-19 01:21:28.937 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf92c88
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf92c88:
    Stack Guard            00000000bbf91000-00000000bbf92000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf92000-00000000bbf93000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf93000-00000000bff91000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-e19c6fd124ab426a.dylib  0x001d777b std::sys::unix::abort_internal::hc95bf10286d6296b + 11
4   libstd-e19c6fd124ab426a.dylib  0x001c7c49 std::sys_common::util::abort::hbac7dac51444ffae + 73
5   libstd-e19c6fd124ab426a.dylib  0x001d6a18 std::sys::unix::stack_overflow::imp::signal_handler::h532e135313fe6f2f + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-e19c6fd124ab426a.dylib  0x001d6660 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h970ffc97bbecde05 + 80
9   a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
10  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
11  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
12  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
13  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
14  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
15  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
16  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
17  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
18  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
19  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
20  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
21  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
22  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
23  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
24  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
25  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
26  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
27  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
28  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
29  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
30  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
31  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
32  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
33  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
34  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
35  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
36  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
37  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
38  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
39  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
40  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
41  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
42  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
43  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
44  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
45  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
46  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
47  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
48  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
49  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
50  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
51  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
52  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
53  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
54  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
55  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
56  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
57  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
58  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
59  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
60  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
61  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
62  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
63  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
64  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
65  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
66  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
67  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
68  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
69  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
70  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
71  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
72  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
73  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
74  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
75  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
76  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
77  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
78  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
79  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
80  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
81  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
82  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
83  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
84  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
85  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
86  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
87  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
88  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
89  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
90  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
91  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
92  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
93  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
94  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
95  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
96  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
97  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
98  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
99  a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
100 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
101 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
102 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
103 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
104 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
105 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
106 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
107 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
108 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
109 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
110 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
111 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
112 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
113 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
114 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
115 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
116 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
117 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
118 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
119 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
120 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
121 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
122 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
123 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
124 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
125 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
126 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
127 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
128 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
129 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
130 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
131 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
132 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
133 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
134 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
135 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
136 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
137 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
138 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
139 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
140 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
141 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
142 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
143 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
144 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
145 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
146 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
147 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
148 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
149 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
150 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
151 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
152 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
153 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
154 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
155 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
156 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
157 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
158 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
159 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
160 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
161 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
162 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
163 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
164 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
165 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
166 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
167 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
168 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
169 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
170 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
171 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
172 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
173 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
174 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
175 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
176 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
177 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
178 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
179 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
180 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
181 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
182 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
183 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
184 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
185 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
186 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
187 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
188 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
189 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
190 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
191 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
192 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
193 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
194 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
195 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
196 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
197 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
198 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
199 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
200 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
201 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
202 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
203 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
204 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
205 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
206 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
207 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
208 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
209 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
210 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
211 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
212 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
213 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
214 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
215 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
216 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
217 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
218 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
219 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
220 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
221 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
222 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
223 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
224 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
225 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
226 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
227 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
228 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
229 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
230 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
231 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
232 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
233 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
234 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
235 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
236 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
237 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
238 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
239 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
240 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
241 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
242 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
243 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
244 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
245 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
246 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
247 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
248 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
249 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
250 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
251 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
252 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
253 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
254 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
255 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
256 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
257 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
258 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
259 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
260 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
261 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
262 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
263 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
264 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
265 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
266 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
267 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
268 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
269 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
270 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
271 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
272 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
273 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
274 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
275 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
276 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
277 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
278 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
279 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
280 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
281 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
282 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
283 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
284 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
285 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
286 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
287 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
288 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
289 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
290 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
291 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
292 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
293 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
294 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
295 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
296 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
297 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
298 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
299 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
300 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
301 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
302 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
303 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
304 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
305 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
306 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
307 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
308 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
309 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
310 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
311 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
312 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
313 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
314 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
315 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
316 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
317 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
318 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
319 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
320 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
321 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
322 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
323 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
324 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
325 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
326 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
327 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
328 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
329 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
330 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
331 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
332 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
333 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
334 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
335 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
336 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
337 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
338 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
339 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
340 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
341 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
342 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
343 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
344 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
345 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
346 a                              0x00072b20 stack_probes::recurse::h35e743424666f0f3 + 48
---
===========                     =======  ======= 
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-19-012129_Traviss-Mac-1044.crash
Process:               a [51027]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51019]
Responsible:           a [51027]
User ID:               501
Date/Time:             2019-05-19 01:21:28.939 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b079ce68
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb079ce68:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b079c000-00000000b079d000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b079d000-00000000b099e000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-e19c6fd124ab426a.dylib  0x00183f80 std::sys::unix::thread::Thread::join::h77e41045cb0de7a3 + 32
4   a                              0x000c17b6 std::thread::JoinHandle$LT$T$GT$::join::h4b8f4d09dc48f4d7 + 70
5   a                              0x000c09d5 stack_probes::main::h385e455299c91d04 + 597
6   a                              0x000bf7eb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hefd28badadd722d1 + 11
7   libstd-e19c6fd124ab426a.dylib  0x00173c6c std::sys_common::backtrace::__rust_begin_short_backtrace::h213ec381daa40f46 + 12
8   libstd-e19c6fd124ab426a.dylib  0x001767d4 std::panicking::try::do_call::h6e25e413421a370b + 20
9   libstd-e19c6fd124ab426a.dylib  0x0018535d __rust_maybe_catch_panic + 29
10  libstd-e19c6fd124ab426a.dylib  0x00177277 std::rt::lang_start_internal::h4758d469cab088c0 + 631
11  a                              0x000c13fc main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-e19c6fd124ab426a.dylib  0x0018477b std::sys::unix::abort_internal::hc95bf10286d6296b + 11
4   libstd-e19c6fd124ab426a.dylib  0x00174c49 std::sys_common::util::abort::hbac7dac51444ffae + 73
5   libstd-e19c6fd124ab426a.dylib  0x00183a18 std::sys::unix::stack_overflow::imp::signal_handler::h532e135313fe6f2f + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-e19c6fd124ab426a.dylib  0x00183660 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h970ffc97bbecde05 + 80
9   a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
10  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
11  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
12  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
13  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
14  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
15  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
16  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
17  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
18  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
19  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
20  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
21  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
22  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
23  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
24  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
25  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
26  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
27  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
28  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
29  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
30  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
31  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
32  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
33  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
34  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
35  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
36  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
37  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
38  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
39  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
40  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
41  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
42  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
43  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
44  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
45  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
46  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
47  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
48  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
49  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
50  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
51  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
52  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
53  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
54  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
55  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
56  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
57  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
58  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
59  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
60  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
61  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
62  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
63  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
64  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
65  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
66  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
67  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
68  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
69  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
70  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
71  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
72  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
73  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
74  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
75  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
76  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
77  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
78  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
79  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
80  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
81  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
82  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
83  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
84  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
85  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
86  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
87  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
88  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
89  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
90  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
91  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
92  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
93  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
94  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
95  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
96  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
97  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
98  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
99  a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
100 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
101 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
102 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
103 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
104 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
105 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
106 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
107 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
108 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
109 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
110 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
111 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
112 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
113 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
114 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
115 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
116 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
117 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
118 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
119 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
120 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
121 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
122 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
123 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
124 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
125 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
126 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
127 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
128 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
129 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
130 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
131 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
132 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
133 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
134 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
135 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
136 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
137 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
138 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
139 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
140 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
141 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
142 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
143 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
144 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
145 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
146 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
147 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
148 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
149 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
150 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
151 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
152 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
153 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
154 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
155 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
156 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
157 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
158 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
159 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
160 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
161 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
162 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
163 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
164 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
165 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
166 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
167 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
168 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
169 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
170 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
171 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
172 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
173 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
174 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
175 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
176 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
177 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
178 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
179 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
180 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
181 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
182 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
183 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
184 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
185 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
186 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
187 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
188 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
189 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
190 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
191 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
192 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
193 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
194 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
195 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
196 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
197 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
198 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
199 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
200 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
201 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
202 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
203 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
204 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
205 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
206 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
207 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
208 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
209 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
210 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
211 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
212 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
213 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
214 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
215 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
216 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
217 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
218 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
219 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
220 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
221 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
222 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
223 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
224 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
225 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
226 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
227 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
228 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
229 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
230 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
231 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
232 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
233 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
234 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
235 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
236 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
237 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
238 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
239 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
240 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
241 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
242 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
243 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
244 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
245 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
246 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
247 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
248 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
249 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
250 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
251 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
252 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
253 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
254 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
255 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
256 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
257 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
258 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
259 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
260 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
261 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
262 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
263 a                              0x000c0b20 stack_probes::recurse::h35e743424666f0f3 + 48
264 a                              0x000bf84d std::sys_common::backtrace::__rust_begin_short_backtrace::h9e9423ed4ca621ce (.llvm.409427213745687080) + 29
265 libstd-e19c6fd124ab426a.dylib  0x0018535d __rust_maybe_catch_panic + 29
266 a                              0x000c1b13 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h1ca559b4a8762424 + 131
267 libstd-e19c6fd124ab426a.dylib  0x0015b281 _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hcf32d1da52b372ce + 65
268 libstd-e19c6fd124ab426a.dylib  0x00183eb8 std::sys::unix::thread::Thread::new::thread_start::h418301b9d93c06b2 + 184
269 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
270 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
271 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb099d000  ecx: 0x004beb0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x004beb38  esp: 0x004beb0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x08daf000
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0xbe000 -    0xc2ffb +a (0) <66B6B05C-71FB-3266-9D92-C7383D755ABB> /Users/USER/*/a
   0xcc000 -   0x111fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x155000 -   0x1e4ff3 +libstd-e19c6fd124ab426a.dylib (0) <FF0426F0-BEFF-3AE8-A073-5B9841972296> /Users/USER/*/libstd-e19c6fd124ab426a.dylib
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
    task_for_pid: 2664
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=76.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        4 
Stack Guard                          8K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3560K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9348K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            571.7M      136 
TOTAL                            571.7M      136 
TOTAL, minus reserved VM space   571.6M      136 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-19-012135_Traviss-Mac-1044.crash
Process:               a [51147]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51142]
Responsible:           a [51147]
User ID:               501
Date/Time:             2019-05-19 01:21:35.543 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b01a5ec8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb01a5ec8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b01a5000-00000000b01a6000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b01a6000-00000000b03a7000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   a                              0x0005c799 stack_probes_lto::main::haeaf2f44e7253541 + 2457
4   a                              0x0007549b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h64e02b3994e9b031 + 11
5   a                              0x0006dcbc std::sys_common::backtrace::__rust_begin_short_backtrace::h213ec381daa40f46 + 12
6   a                              0x0005debd main + 589
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x0005f24b std::sys::unix::abort_internal::hc95bf10286d6296b + 11
4   a                              0x0005f239 std::sys_common::util::abort::hbac7dac51444ffae + 73
5   a                              0x0006dc1c std::sys::unix::stack_overflow::imp::signal_handler::h532e135313fe6f2f + 860
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x0006d8c0 rust_begin_unwind + 16
9   a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
10  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
11  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
12  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
13  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
14  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
15  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
16  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
17  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
18  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
19  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
20  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
21  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
22  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
23  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
24  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
25  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
26  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
27  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
28  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
29  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
30  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
31  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
32  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
33  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
34  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
35  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
36  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
37  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
38  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
39  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
40  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
41  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
42  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
43  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
44  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
45  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
46  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
47  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
48  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
49  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
50  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
51  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
52  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
53  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
54  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
55  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
56  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
57  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
58  a                              0x0005cc48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
