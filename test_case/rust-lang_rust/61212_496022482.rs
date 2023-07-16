plain
[00:03:26]       Memory: 8 GB
[00:03:26]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:26]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:26]       SMC Version (system): 2.8f0
[00:03:26]       Serial Number (system): VMTj2gVzVbc5
[00:03:26] 
[00:03:26] hw.ncpu: 4
[00:03:26] hw.byteorder: 1234
[00:03:26] hw.memsize: 8589934592
---
[02:08:54] 
[02:08:54] ---- /Users/travis/build/rust-lang/rust/src/doc/rustdoc/src/documentation-tests.md - Documentation_tests::Documenting_macros (line 263) stdout ----
[02:08:54] error: linking with `cc` failed: signal: 4
[02:08:54]   |
[02:08:54]   = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22ISu0/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22ISu0/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22ISu0/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22ISu0/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22ISu0/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22ISu0/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest22ISu0/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libstd-2fb68d5d0d4b1dbe.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libpanic_unwind-53c261814f627a07.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libbacktrace-31c5687969fb0606.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libbacktrace_sys-c0d3309ba80e0f23.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_demangle-5509e73be26a0bd6.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcfg_if-924da5957a5a1863.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libhashbrown-3132c0b871b3fff8.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_alloc-b2fa52a49bb284dc.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libunwind-0cdb42220e56e08b.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liblibc-b18dfaed99ca7a71.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liballoc-0a9841e408b889be.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_core-bc86d4a5a44cda78.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcore-75cce831a556c528.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-6ece06e05b87bd36.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:08:54] 
[02:08:54] error: aborting due to previous error
[02:08:54] 
[02:08:54] thread '/Users/travis/build/rust-lang/rust/src/doc/rustdoc/src/documentation-tests.md - Documentation_tests::Documenting_macros (line 263)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
---
[02:08:54] 
[02:08:54] 
[02:08:54] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:08:54] Build completed unsuccessfully in 0:51:49
[02:08:54] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04603a19
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May 26 18:45:59 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:231c4d88
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1176
drwx------  26 travis  staff    884 May 26 18:45 .
-rw-------@  1 travis  staff   1387 May 26 18:45 foo_2019-05-26-184532_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1377 May 26 18:45 m4_2019-05-26-184501_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1390 May 26 18:44 bar_2019-05-26-184451-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1403 May 26 18:44 bar_2019-05-26-184451_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9898 May 26 18:44 b_2019-05-26-184450_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34964 May 26 18:11 a_2019-05-26-181114-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  57366 May 26 18:11 a_2019-05-26-181114_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55585 May 26 18:11 a_2019-05-26-181105-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34870 May 26 18:11 a_2019-05-26-181105_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 May 26 18:11 a_2019-05-26-181100_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9166 May 26 18:10 a_2019-05-26-181054_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9171 May 26 18:10 a_2019-05-26-181052-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8936 May 26 18:10 a_2019-05-26-181052_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9304 May 26 18:10 a_2019-05-26-181012_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58249 May 26 18:10 a_2019-05-26-181004_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 May 26 18:09 a_2019-05-26-180959-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59516 May 26 18:09 a_2019-05-26-180959-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60372 May 26 18:09 a_2019-05-26-180959_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10880 May 26 18:07 a_2019-05-26-180746_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 May 26 18:06 a_2019-05-26-180656_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 May 26 18:05 a_2019-05-26-180537_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9780 May 26 18:04 a_2019-05-26-180435-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 May 26 18:04 a_2019-05-26-180435_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9483 May 26 18:04 a_2019-05-26-180424_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:04c60d68
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-26-180424_Traviss-Mac-1044.crash
Process:               a [41029]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [41026]
Responsible:           a [41029]
User ID:               501
Date/Time:             2019-05-26 18:03:58.976 +0000
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
0   a                              0x00020afe abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 46
1   a                              0x0001ff4b std::panicking::try::do_call::h169a9c9a99ce0580 (.llvm.6988681596543106467) + 11
2   libstd-2fb68d5d0d4b1dbe.dylib  0x001452ed __rust_maybe_catch_panic + 29
3   a                              0x00020d65 abort_on_c_abi::main::he771bf881fc862e3 + 613
4   a                              0x0001f5ab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hde08d08afbec4db4 + 11
5   libstd-2fb68d5d0d4b1dbe.dylib  0x001336cc std::sys_common::backtrace::__rust_begin_short_backtrace::h5ceb27c8f2988436 + 12
6   libstd-2fb68d5d0d4b1dbe.dylib  0x001369b4 std::panicking::try::do_call::he685ce30b66c42e0 + 20
7   libstd-2fb68d5d0d4b1dbe.dylib  0x001452ed __rust_maybe_catch_panic + 29
8   libstd-2fb68d5d0d4b1dbe.dylib  0x00137457 std::rt::lang_start_internal::h561b2bf9cb1e5263 + 631
9   a                              0x0002109c main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x79f72da0  ebx: 0xbffe08b8  ecx: 0x00000000  edx: 0x00000000
  edi: 0x001452de  esi: 0x00000000  ebp: 0xbffe0858  esp: 0xbffe0840
   ss: 0x00000023  efl: 0x00010292  eip: 0x00020afe   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0017fa90
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x1e000 -    0x21ff3 +a (0) <FDFDA203-E222-3979-A416-19CE43B4D5CC> /Users/USER/*/a
   0x8d000 -    0xd2fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x116000 -   0x1a5ffb +libstd-2fb68d5d0d4b1dbe.dylib (0) <7C371CDB-5960-3DF5-9DC8-1C02708CA171> /Users/USER/*/libstd-2fb68d5d0d4b1dbe.dylib
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
    task_for_pid: 2357
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
__DATA                            3516K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-26-180435-1_Traviss-Mac-1044.crash
Process:               a [41830]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [41825]
Responsible:           a [41830]
User ID:               501
Date/Time:             2019-05-26 18:04:31.581 +0000
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
0   libstd-2fb68d5d0d4b1dbe.dylib  0x00241ec3 std::panicking::rust_panic_with_hook::h4369f3163f6f9f59 + 115
1   a                              0x000d3c0f std::panicking::begin_panic::hbdff5281e68719c4 + 47 (panicking.rs:408)
2   a                              0x000d16f4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 36 (backtrace.rs:25)
3   a                              0x000d0d8b core::ptr::real_drop_in_place::ha0fd09e913297feb + 11
4   a                              0x000d16c3 backtrace::double::h35adec2a6f63ef6c + 51
5   a                              0x000d29d8 backtrace::main::hc9a5bc8fc93ded64 + 4568 (backtrace.rs:104)
6   a                              0x000d0bcb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::haf9833fd4215e07e + 11 (rt.rs:64)
7   libstd-2fb68d5d0d4b1dbe.dylib  0x0023e6cc std::sys_common::backtrace::__rust_begin_short_backtrace::h5ceb27c8f2988436 + 12
8   libstd-2fb68d5d0d4b1dbe.dylib  0x002419b4 std::panicking::try::do_call::he685ce30b66c42e0 + 20
9   libstd-2fb68d5d0d4b1dbe.dylib  0x002502ed __rust_maybe_catch_panic + 29
10  libstd-2fb68d5d0d4b1dbe.dylib  0x00242457 std::rt::lang_start_internal::h561b2bf9cb1e5263 + 631
11  a                              0x000d322c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff30628  ebx: 0xbff30670  ecx: 0xbff30520  edx: 0xa7702ec6
  edi: 0x0028adf8  esi: 0x00241e5e  ebp: 0xbff306c8  esp: 0xbff30640
   ss: 0x00000023  efl: 0x00010282  eip: 0x00241ec3   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x7af77000
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xce000 -    0xd4fff +a (0) <E44B236D-ADCA-3F0A-9BD9-8CDEADC9E0E5> /Users/USER/*/a
  0x198000 -   0x1ddfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x221000 -   0x2b0ffb +libstd-2fb68d5d0d4b1dbe.dylib (0) <7C371CDB-5960-3DF5-9DC8-1C02708CA171> /Users/USER/*/libstd-2fb68d5d0d4b1dbe.dylib
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
    task_for_pid: 2357
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=82.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=82.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.4M        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3516K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9356K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            577.9M      135 
TOTAL                            577.9M      135 
TOTAL, minus reserved VM space   577.8M      135 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-26-180435_Traviss-Mac-1044.crash
Process:               a [41831]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [41825]
Responsible:           a [41831]
User ID:               501
Date/Time:             2019-05-26 18:04:31.743 +0000
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
0   libstd-2fb68d5d0d4b1dbe.dylib  0x001e4ec3 std::panicking::rust_panic_with_hook::h4369f3163f6f9f59 + 115
1   a                              0x00049c0f std::panicking::begin_panic::hbdff5281e68719c4 + 47 (panicking.rs:408)
2   a                              0x000476f4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 36 (backtrace.rs:25)
3   a                              0x00046d8b core::ptr::real_drop_in_place::ha0fd09e913297feb + 11
4   a                              0x000476c3 backtrace::double::h35adec2a6f63ef6c + 51
5   a                              0x000489d8 backtrace::main::hc9a5bc8fc93ded64 + 4568 (backtrace.rs:104)
6   a                              0x00046bcb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::haf9833fd4215e07e + 11 (rt.rs:64)
7   libstd-2fb68d5d0d4b1dbe.dylib  0x001e16cc std::sys_common::backtrace::__rust_begin_short_backtrace::h5ceb27c8f2988436 + 12
8   libstd-2fb68d5d0d4b1dbe.dylib  0x001e49b4 std::panicking::try::do_call::he685ce30b66c42e0 + 20
9   libstd-2fb68d5d0d4b1dbe.dylib  0x001f32ed __rust_maybe_catch_panic + 29
10  libstd-2fb68d5d0d4b1dbe.dylib  0x001e5457 std::rt::lang_start_internal::h561b2bf9cb1e5263 + 631
11  a                              0x0004922c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbffba608  ebx: 0xbffba650  ecx: 0xbffba500  edx: 0xa7702ec6
  edi: 0x0022ddf8  esi: 0x001e4e5e  ebp: 0xbffba6a8  esp: 0xbffba620
   ss: 0x00000023  efl: 0x00010286  eip: 0x001e4ec3   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x001c8720
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x44000 -    0x4afff +a (0) <E44B236D-ADCA-3F0A-9BD9-8CDEADC9E0E5> /Users/USER/*/a
  0x13b000 -   0x180fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1c4000 -   0x253ffb +libstd-2fb68d5d0d4b1dbe.dylib (0) <7C371CDB-5960-3DF5-9DC8-1C02708CA171> /Users/USER/*/libstd-2fb68d5d0d4b1dbe.dylib
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
    task_for_pid: 2357
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=73.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9624K        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3516K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9356K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            568.9M      135 
TOTAL                            568.9M      135 
TOTAL, minus reserved VM space   568.8M      135 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-26-180537_Traviss-Mac-1044.crash
Process:               a [43512]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [43511]
Responsible:           a [43512]
User ID:               501
Date/Time:             2019-05-26 18:05:35.112 +0000
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
3   libstd-2fb68d5d0d4b1dbe.dylib  0x0028676b std::sys::unix::abort_internal::ha73ed347b2c8bece + 11
4   libstd-2fb68d5d0d4b1dbe.dylib  0x00277f50 rust_oom + 48
5   libstd-2fb68d5d0d4b1dbe.dylib  0x00299f64 alloc::alloc::handle_alloc_error::hb1eb2ca9c570c963 + 20
6   a                              0x000ce46d default_alloc_error_hook::main::h0fe124586986ad5d + 781
7   a                              0x000ce5eb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h05fb1dfa58b097a9 + 11
8   libstd-2fb68d5d0d4b1dbe.dylib  0x002756cc std::sys_common::backtrace::__rust_begin_short_backtrace::h5ceb27c8f2988436 + 12
9   libstd-2fb68d5d0d4b1dbe.dylib  0x002789b4 std::panicking::try::do_call::he685ce30b66c42e0 + 20
10  libstd-2fb68d5d0d4b1dbe.dylib  0x002872ed __rust_maybe_catch_panic + 29
11  libstd-2fb68d5d0d4b1dbe.dylib  0x00279457 std::rt::lang_start_internal::h561b2bf9cb1e5263 + 631
12  a                              0x000ce5cc main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff317cc  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff317f8  esp: 0xbff317cc
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xcd000 -    0xceffb +a (0) <71A49469-3AE5-3B83-9D12-1004F1D665FB> /Users/USER/*/a
  0x1cf000 -   0x214fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x258000 -   0x2e7ffb +libstd-2fb68d5d0d4b1dbe.dylib (0) <7C371CDB-5960-3DF5-9DC8-1C02708CA171> /Users/USER/*/libstd-2fb68d5d0d4b1dbe.dylib
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-26-180959-2_Traviss-Mac-1044.crash
Process:               a [49952]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [49947]
Responsible:           a [49952]
User ID:               501
Date/Time:             2019-05-26 18:09:58.427 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5700 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b03a0fec
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb03a0fec:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b03a0000-00000000b03a1000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b03a1000-00000000b05a2000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-2fb68d5d0d4b1dbe.dylib  0x0021cf70 std::sys::unix::thread::Thread::join::hb34e2b7ffc44e46a + 32
4   a                              0x000ad0b6 std::thread::JoinHandle$LT$T$GT$::join::h07afe131e2a109df + 70
5   a                              0x000abe59 out_of_stack::main::h51a6e88d0363a411 + 233
6   a                              0x000ab25b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h942ab43f41a36a46 + 11
7   libstd-2fb68d5d0d4b1dbe.dylib  0x0020c6cc std::sys_common::backtrace::__rust_begin_short_backtrace::h5ceb27c8f2988436 + 12
8   libstd-2fb68d5d0d4b1dbe.dylib  0x0020f9b4 std::panicking::try::do_call::he685ce30b66c42e0 + 20
9   libstd-2fb68d5d0d4b1dbe.dylib  0x0021e2ed __rust_maybe_catch_panic + 29
10  libstd-2fb68d5d0d4b1dbe.dylib  0x00210457 std::rt::lang_start_internal::h561b2bf9cb1e5263 + 631
11  a                              0x000acb4c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-2fb68d5d0d4b1dbe.dylib  0x0021d76b std::sys::unix::abort_internal::ha73ed347b2c8bece + 11
4   libstd-2fb68d5d0d4b1dbe.dylib  0x0020e179 std::sys_common::util::abort::h5a4bdba44b04c461 + 73
5   libstd-2fb68d5d0d4b1dbe.dylib  0x0021ca0f std::sys::unix::stack_overflow::imp::signal_handler::h14af32901beb880b + 943
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-2fb68d5d0d4b1dbe.dylib  0x0021c660 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb8c6e3b98880b638 + 80
9   libstd-2fb68d5d0d4b1dbe.dylib  0x001fb84c _$LT$std..io..buffered..LineWriter$LT$W$GT$$u20$as$u20$std..io..Write$GT$::write::h80beae42929aca94 + 220
10  libstd-2fb68d5d0d4b1dbe.dylib  0x00200215 std::io::Write::write_all::hc8522cfee96bfe99 + 101
11  libstd-2fb68d5d0d4b1dbe.dylib  0x002007a3 _$LT$std..io..Write..write_fmt..Adaptor$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$::write_str::he53ab791a4d0d034 + 35
12  libstd-2fb68d5d0d4b1dbe.dylib  0x0023ec2d core::fmt::write::hbce2d277ad333e62 + 701
13  libstd-2fb68d5d0d4b1dbe.dylib  0x001fe3b6 _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::hf0a125ab529d1bae + 182
14  libstd-2fb68d5d0d4b1dbe.dylib  0x001ff8ec std::io::stdio::_print::h637ba9758f64a9f3 + 396
15  a                              0x000abd5f out_of_stack::loud_recurse::h48447a21f69c373e + 63
16  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
17  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
18  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
19  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
20  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
21  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
22  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
23  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
24  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
25  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
26  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
27  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
28  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
29  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
30  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
31  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
32  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
33  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
34  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
35  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
36  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
37  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
38  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
39  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
40  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
41  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
42  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
43  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
44  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
45  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
46  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
47  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
48  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
49  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
50  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
51  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
52  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
53  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
54  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
55  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
56  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
57  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
58  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
59  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
60  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
61  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
62  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
63  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
64  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
65  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
66  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
67  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
68  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
69  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
70  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
71  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
72  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
73  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
74  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
75  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
76  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
77  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
78  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
79  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
80  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
81  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
82  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
83  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
84  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
85  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
86  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
87  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
88  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
89  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
90  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
91  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
92  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
93  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
94  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
95  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
96  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
97  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
98  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
99  a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
100 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
101 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
102 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
103 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
104 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
105 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
106 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
107 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
108 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
109 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
110 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
111 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
112 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
113 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
114 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
115 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
116 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
117 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
118 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
119 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
120 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
121 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
122 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
123 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
124 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
125 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
126 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
127 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
128 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
129 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
130 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
131 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
132 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
133 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
134 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
135 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
136 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
137 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
138 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
139 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
140 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
141 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
142 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
143 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
144 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
145 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
146 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
147 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
148 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
149 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
150 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
151 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
152 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
153 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
154 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
155 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
156 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
157 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
158 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
159 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
160 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
161 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
162 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
163 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
164 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
165 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
166 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
167 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
168 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
169 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
170 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
171 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
172 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
173 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
174 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
175 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
176 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
177 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
178 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
179 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
180 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
181 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
182 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
183 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
184 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
185 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
186 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
187 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
188 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
189 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
190 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
191 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
192 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
193 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
194 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
195 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
196 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
197 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
198 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
199 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
200 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
201 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
202 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
203 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
204 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
205 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
206 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
207 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
208 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
209 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
210 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
211 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
212 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
213 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
214 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
215 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
216 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
217 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
218 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
219 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
220 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
221 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
222 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
223 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
224 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
225 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
226 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
227 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
228 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
229 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
230 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
231 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
232 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
233 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
234 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
235 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
236 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
237 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
238 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
239 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
240 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
241 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
242 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
243 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
244 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
245 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
246 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
247 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
248 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
249 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
250 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
251 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
252 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
253 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
254 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
255 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
256 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
257 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
258 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
259 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
260 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
261 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
262 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
263 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
264 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
265 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
266 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
267 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
268 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
269 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
270 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
271 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
272 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
273 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
274 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
275 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
276 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
277 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
278 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
279 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
280 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
281 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
282 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
283 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
284 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
285 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
286 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
287 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
288 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
289 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
290 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
291 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
292 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
293 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
294 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
295 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
296 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
297 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
298 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
299 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
300 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
301 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
302 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
303 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
304 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
305 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
306 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
307 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
308 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
309 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
310 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
311 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
312 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
313 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
314 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
315 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
316 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
317 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
318 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
319 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
320 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
321 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
322 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
323 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
324 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
325 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
326 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
327 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
328 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
329 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
330 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
331 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
332 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
333 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
334 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
335 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
336 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
337 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
338 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
339 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
340 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
341 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
342 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
343 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
344 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
345 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
346 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
347 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
348 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
349 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
350 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
351 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
352 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
353 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
354 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
355 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
356 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
357 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
358 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
359 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
360 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
361 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
362 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
363 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
364 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
365 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
366 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
367 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
368 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
369 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
370 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
371 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
372 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
373 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
374 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
375 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
376 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
377 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
378 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
379 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
380 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
381 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
382 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
383 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
384 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
385 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
386 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
387 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
388 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
389 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
390 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
391 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
392 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
393 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
394 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
395 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
396 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
397 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
398 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
399 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
400 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
401 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
402 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
403 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
404 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
405 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
406 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
407 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
408 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
409 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
410 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
411 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
412 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
413 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
414 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
415 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
416 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
417 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
418 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
419 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
420 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
421 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
422 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
423 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
424 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
425 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
426 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
427 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
428 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
429 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
430 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
431 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
432 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
433 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
434 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
435 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
436 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
437 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
438 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
439 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
440 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
441 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
442 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
443 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
444 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
445 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
446 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
447 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
448 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
449 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
450 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
451 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
452 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
453 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
454 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
455 a                              0x000abd64 out_of_stack::loud_recurse::h48447a21f69c373e + 68
---
===========                     =======  ======= 
TOTAL                            568.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-26-181012_Traviss-Mac-1044.crash
Process:               a [50149]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50147]
Responsible:           a [50149]
User ID:               501
Date/Time:             2019-05-26 18:10:11.784 +0000
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
3   a                              0x000b500b panic_abort::__rust_start_panic::abort::hf66aecbaf63ab8c3 + 11
4   a                              0x000b4ffb __rust_start_panic + 11
5   a                              0x000ab89b rust_panic + 11
6   a                              0x000ab484 std::panicking::rust_panic_with_hook::h4369f3163f6f9f59 + 996
7   a                              0x000bc4ea std::panicking::begin_panic::h1a0295b08b61c427 + 42
8   a                              0x000aa27d lto_abort::main::hd3a7c21d2a5e6b65 + 2877
9   a                              0x000bc62b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd4ee642c695f5761 + 11
10  a                              0x000b4e7c std::sys_common::backtrace::__rust_begin_short_backtrace::h5ceb27c8f2988436 + 12
11  a                              0x000aa658 main + 984
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff5679c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff567c8  esp: 0xbff5679c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xa8000 -    0xcbff7 +a (0) <E03148AF-915B-34FD-AA28-0245DF52DB8A> /Users/USER/*/a
  0x1af000 -   0x1f4fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2611
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-26-181052-1_Traviss-Mac-1044.crash
Process:               a [51153]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51150]
Responsible:           a [51153]
User ID:               501
Date/Time:             2019-05-26 18:10:50.776 +0000
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
    __TEXT                 00000000000f9000-00000000000fc000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000faf72 segfault_no_out_of_stack::main::h579ee62fe996355a + 2034
1   a                              0x000f999b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc674c807cba5f32f + 11
2   libstd-2fb68d5d0d4b1dbe.dylib  0x0028f6cc std::sys_common::backtrace::__rust_begin_short_backtrace::h5ceb27c8f2988436 + 12
3   libstd-2fb68d5d0d4b1dbe.dylib  0x002929b4 std::panicking::try::do_call::he685ce30b66c42e0 + 20
4   libstd-2fb68d5d0d4b1dbe.dylib  0x002a12ed __rust_maybe_catch_panic + 29
5   libstd-2fb68d5d0d4b1dbe.dylib  0x00293457 std::rt::lang_start_internal::h561b2bf9cb1e5263 + 631
6   a                              0x000fb24c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x79620930  ecx: 0x00000000  edx: 0x00000000
  edi: 0x002a12de  esi: 0xbff058e0  ebp: 0xbff059c8  esp: 0xbff05820
   ss: 0x00000023  efl: 0x00010246  eip: 0x000faf72   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     1
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xf9000 -    0xfbffb +a (0) <34762377-9AEF-38E0-AABC-5B6899D1A8F1> /Users/USER/*/a
  0x1e9000 -   0x22efdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x272000 -   0x301ffb +libstd-2fb68d5d0d4b1dbe.dylib (0) <7C371CDB-5960-3DF5-9DC8-1C02708CA171> /Users/USER/*/libstd-2fb68d5d0d4b1dbe.dylib
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
    task_for_pid: 2611
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
__DATA                            3516K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-26-181052_Traviss-Mac-1044.crash
Process:               a [51128]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51122]
Responsible:           a [51128]
User ID:               501
Date/Time:             2019-05-26 18:10:50.407 +0000
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
3   libstd-2fb68d5d0d4b1dbe.dylib  0x001d776b std::sys::unix::abort_internal::ha73ed347b2c8bece + 11
4   libstd-2fb68d5d0d4b1dbe.dylib  0x001c8179 std::sys_common::util::abort::h5a4bdba44b04c461 + 73
5   libstd-2fb68d5d0d4b1dbe.dylib  0x001ca1d2 rust_panic + 98
6   libstd-2fb68d5d0d4b1dbe.dylib  0x001ca09e std::panicking::rust_panic_with_hook::h4369f3163f6f9f59 + 590
7   a                              0x0007b9cf std::panicking::begin_panic::hac772f5135b78e2a + 47
8   a                              0x0007cb0c main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff847ac  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff847d8  esp: 0xbff847ac
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x7a000 -    0x7dfff +a (0) <AAA185C1-273F-3A02-8EFD-E57F5B5DCF06> /Users/USER/*/a
  0x120000 -   0x165fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1a9000 -   0x238ffb +libstd-2fb68d5d0d4b1dbe.dylib (0) <7C371CDB-5960-3DF5-9DC8-1C02708CA171> /Users/USER/*/libstd-2fb68d5d0d4b1dbe.dylib
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
    task_for_pid: 2611
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
__DATA                            3516K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-26-181054_Traviss-Mac-1044.crash
Process:               a [51234]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51233]
Responsible:           a [51234]
User ID:               501
Date/Time:             2019-05-26 18:10:54.059 +0000
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
    __TEXT                 0000000000037000-000000000003a000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000395d4 signal_exit_status::main::hc6663d816ec186eb + 436
1   a                              0x0003847b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h2e9eaed1f28709d8 + 11
2   libstd-2fb68d5d0d4b1dbe.dylib  0x001576cc std::sys_common::backtrace::__rust_begin_short_backtrace::h5ceb27c8f2988436 + 12
3   libstd-2fb68d5d0d4b1dbe.dylib  0x0015a9b4 std::panicking::try::do_call::he685ce30b66c42e0 + 20
4   libstd-2fb68d5d0d4b1dbe.dylib  0x001692ed __rust_maybe_catch_panic + 29
5   libstd-2fb68d5d0d4b1dbe.dylib  0x0015b457 std::rt::lang_start_internal::h561b2bf9cb1e5263 + 631
6   a                              0x000396ac main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x78e52e80
  edi: 0x78e52f10  esi: 0xbffc7940  ebp: 0xbffc79d8  esp: 0xbffc78c0
   ss: 0x00000023  efl: 0x00010246  eip: 0x000395d4   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     2
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x37000 -    0x39fff +a (0) <B2728581-326B-352B-8429-3B21B35AA8D6> /Users/USER/*/a
   0xb1000 -    0xf6fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x13a000 -   0x1c9ffb +libstd-2fb68d5d0d4b1dbe.dylib (0) <7C371CDB-5960-3DF5-9DC8-1C02708CA171> /Users/USER/*/libstd-2fb68d5d0d4b1dbe.dylib
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
    task_for_pid: 2611
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
__DATA                            3516K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-26-181100_Traviss-Mac-1044.crash
Process:               a [51342]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51332]
Responsible:           a [51342]
User ID:               501
Date/Time:             2019-05-26 18:10:59.059 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5800 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00063856 simd_target_feature_mixup::test::id_avx512_512::hf3a1395d43161fbe + 102
1   a                              0x0006260f simd_target_feature_mixup::test::main::h611f15c116e6c273 + 1647
2   a                              0x00064b80 simd_target_feature_mixup::main::hfcef50d014c08dda + 896
3   a                              0x00061d1b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hda1d67090c04b2bc + 11
4   libstd-2fb68d5d0d4b1dbe.dylib  0x002066cc std::sys_common::backtrace::__rust_begin_short_backtrace::h5ceb27c8f2988436 + 12
5   libstd-2fb68d5d0d4b1dbe.dylib  0x002099b4 std::panicking::try::do_call::he685ce30b66c42e0 + 20
6   libstd-2fb68d5d0d4b1dbe.dylib  0x002182ed __rust_maybe_catch_panic + 29
7   libstd-2fb68d5d0d4b1dbe.dylib  0x0020a457 std::rt::lang_start_internal::h561b2bf9cb1e5263 + 631
8   a                              0x00064d5c main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff9d600  ebx: 0xbff9d580  ecx: 0x000637fe  edx: 0xbff9d580
  edi: 0x00061fb4  esi: 0x00000000  ebp: 0xbff9d578  esp: 0xbff9d540
   ss: 0x00000023  efl: 0x00010246  eip: 0x00063856   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00063490
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x61000 -    0x65fc7 +a (0) <AFE59430-236B-3B92-9F6F-9EF8735E85C1> /Users/USER/*/a
  0x160000 -   0x1a5fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1e9000 -   0x278ffb +libstd-2fb68d5d0d4b1dbe.dylib (0) <7C371CDB-5960-3DF5-9DC8-1C02708CA171> /Users/USER/*/libstd-2fb68d5d0d4b1dbe.dylib
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
    task_for_pid: 2611
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
__DATA                            3516K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-26-181105-1_Traviss-Mac-1044.crash
Process:               a [51481]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51479]
Responsible:           a [51481]
User ID:               501
Date/Time:             2019-05-26 18:11:04.860 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5800 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf379e8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf379e8:
    Stack Guard            00000000bbf36000-00000000bbf37000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf37000-00000000bbf38000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf38000-00000000bff36000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-2fb68d5d0d4b1dbe.dylib  0x001cd76b std::sys::unix::abort_internal::ha73ed347b2c8bece + 11
4   libstd-2fb68d5d0d4b1dbe.dylib  0x001be179 std::sys_common::util::abort::h5a4bdba44b04c461 + 73
5   libstd-2fb68d5d0d4b1dbe.dylib  0x001cca0f std::sys::unix::stack_overflow::imp::signal_handler::h14af32901beb880b + 943
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-2fb68d5d0d4b1dbe.dylib  0x001cc660 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb8c6e3b98880b638 + 80
9   a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
10  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
11  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
12  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
13  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
14  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
15  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
16  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
17  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
18  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
19  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
20  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
21  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
22  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
23  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
24  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
25  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
26  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
27  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
28  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
29  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
30  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
31  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
32  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
33  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
34  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
35  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
36  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
37  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
38  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
39  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
40  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
41  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
42  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
43  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
44  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
45  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
46  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
47  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
48  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
49  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
50  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
51  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
52  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
53  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
54  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
55  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
56  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
57  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
58  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
59  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
60  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
61  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
62  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
63  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
64  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
65  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
66  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
67  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
68  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
69  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
70  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
71  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
72  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
73  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
74  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
75  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
76  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
77  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
78  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
79  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
80  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
81  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
82  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
83  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
84  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
85  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
86  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
87  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
88  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
89  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
90  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
91  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
92  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
93  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
94  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
95  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
96  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
97  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
98  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
99  a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
100 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
101 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
102 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
103 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
104 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
105 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
106 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
107 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
108 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
109 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
110 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
111 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
112 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
113 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
114 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
115 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
116 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
117 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
118 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
119 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
120 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
121 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
122 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
123 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
124 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
125 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
126 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
127 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
128 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
129 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
130 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
131 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
132 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
133 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
134 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
135 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
136 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
137 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
138 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
139 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
140 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
141 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
142 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
143 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
144 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
145 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
146 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
147 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
148 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
149 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
150 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
151 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
152 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
153 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
154 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
155 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
156 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
157 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
158 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
159 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
160 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
161 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
162 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
163 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
164 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
165 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
166 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
167 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
168 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
169 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
170 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
171 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
172 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
173 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
174 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
175 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
176 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
177 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
178 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
179 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
180 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
181 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
182 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
183 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
184 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
185 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
186 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
187 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
188 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
189 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
190 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
191 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
192 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
193 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
194 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
195 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
196 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
197 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
198 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
199 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
200 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
201 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
202 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
203 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
204 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
205 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
206 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
207 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
208 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
209 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
210 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
211 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
212 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
213 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
214 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
215 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
216 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
217 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
218 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
219 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
220 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
221 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
222 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
223 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
224 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
225 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
226 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
227 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
228 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
229 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
230 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
231 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
232 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
233 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
234 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
235 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
236 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
237 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
238 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
239 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
240 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
241 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
242 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
243 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
244 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
245 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
246 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
247 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
248 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
249 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
250 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
251 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
252 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
253 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
254 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
255 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
256 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
257 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
258 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
259 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
260 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
261 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
262 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
263 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
264 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
265 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
266 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
267 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
268 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
269 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
270 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
271 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
272 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
273 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
274 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
275 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
276 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
277 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
278 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
279 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
280 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
281 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
282 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
283 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
284 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
285 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
286 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
287 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
288 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
289 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
290 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
291 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
292 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
293 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
294 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
295 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
296 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
297 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
298 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
299 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
300 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
301 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
302 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
303 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
304 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
305 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
306 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
307 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
308 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
309 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
310 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
311 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
312 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
313 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
314 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
315 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
316 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
317 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
318 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
319 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
320 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
321 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
322 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
323 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
324 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
325 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
326 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
327 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
328 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
329 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
330 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
331 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
332 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
333 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
334 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
335 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
336 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
337 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
338 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
339 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
340 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
341 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
342 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
343 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
344 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
345 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
346 a                              0x000cdb10 stack_probes::recurse::h35e743424666f0f3 + 48
---
===========                     =======  ======= 
TOTAL                            568.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-26-181105_Traviss-Mac-1044.crash
Process:               a [51485]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51479]
Responsible:           a [51485]
User ID:               501
Date/Time:             2019-05-26 18:11:04.866 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5800 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b010ae68
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb010ae68:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b010a000-00000000b010b000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b010b000-00000000b030c000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa77000b6 __bsdthread_create + 10
1   libsystem_pthread.dylib        0xa7832824 _pthread_create + 235
2   libsystem_pthread.dylib        0xa782f228 pthread_create + 28
3   libstd-2fb68d5d0d4b1dbe.dylib  0x001dcb84 std::sys::unix::thread::Thread::new::h725ebc66e8cb797e + 244
4   a                              0x00065916 std::thread::spawn::h4eaed9a29d2ce2d3 + 230
5   a                              0x000649ba stack_probes::main::h385e455299c91d04 + 586
6   a                              0x000637db std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::he6b9eb11842b2249 + 11
7   libstd-2fb68d5d0d4b1dbe.dylib  0x001cc6cc std::sys_common::backtrace::__rust_begin_short_backtrace::h5ceb27c8f2988436 + 12
8   libstd-2fb68d5d0d4b1dbe.dylib  0x001cf9b4 std::panicking::try::do_call::he685ce30b66c42e0 + 20
9   libstd-2fb68d5d0d4b1dbe.dylib  0x001de2ed __rust_maybe_catch_panic + 29
10  libstd-2fb68d5d0d4b1dbe.dylib  0x001d0457 std::rt::lang_start_internal::h561b2bf9cb1e5263 + 631
11  a                              0x000653ec main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-2fb68d5d0d4b1dbe.dylib  0x001dd76b std::sys::unix::abort_internal::ha73ed347b2c8bece + 11
4   libstd-2fb68d5d0d4b1dbe.dylib  0x001ce179 std::sys_common::util::abort::h5a4bdba44b04c461 + 73
5   libstd-2fb68d5d0d4b1dbe.dylib  0x001dca0f std::sys::unix::stack_overflow::imp::signal_handler::h14af32901beb880b + 943
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-2fb68d5d0d4b1dbe.dylib  0x001dc660 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb8c6e3b98880b638 + 80
9   a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
10  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
11  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
12  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
13  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
14  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
15  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
16  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
17  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
18  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
19  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
20  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
21  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
22  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
23  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
24  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
25  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
26  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
27  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
28  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
29  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
30  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
31  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
32  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
33  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
34  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
35  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
36  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
37  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
38  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
39  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
40  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
41  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
42  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
43  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
44  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
45  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
46  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
47  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
48  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
49  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
50  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
51  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
52  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
53  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
54  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
55  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
56  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
57  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
58  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
59  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
60  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
61  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
62  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
63  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
64  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
65  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
66  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
67  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
68  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
69  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
70  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
71  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
72  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
73  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
74  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
75  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
76  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
77  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
78  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
79  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
80  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
81  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
82  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
83  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
84  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
85  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
86  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
87  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
88  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
89  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
90  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
91  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
92  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
93  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
94  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
95  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
96  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
97  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
98  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
99  a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
100 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
101 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
102 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
103 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
104 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
105 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
106 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
107 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
108 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
109 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
110 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
111 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
112 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
113 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
114 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
115 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
116 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
117 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
118 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
119 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
120 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
121 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
122 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
123 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
124 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
125 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
126 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
127 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
128 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
129 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
130 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
131 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
132 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
133 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
134 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
135 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
136 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
137 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
138 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
139 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
140 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
141 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
142 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
143 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
144 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
145 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
146 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
147 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
148 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
149 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
150 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
151 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
152 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
153 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
154 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
155 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
156 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
157 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
158 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
159 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
160 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
161 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
162 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
163 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
164 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
165 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
166 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
167 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
168 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
169 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
170 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
171 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
172 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
173 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
174 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
175 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
176 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
177 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
178 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
179 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
180 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
181 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
182 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
183 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
184 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
185 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
186 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
187 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
188 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
189 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
190 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
191 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
192 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
193 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
194 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
195 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
196 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
197 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
198 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
199 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
200 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
201 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
202 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
203 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
204 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
205 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
206 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
207 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
208 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
209 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
210 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
211 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
212 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
213 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
214 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
215 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
216 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
217 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
218 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
219 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
220 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
221 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
222 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
223 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
224 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
225 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
226 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
227 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
228 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
229 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
230 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
231 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
232 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
233 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
234 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
235 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
236 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
237 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
238 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
239 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
240 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
241 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
242 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
243 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
244 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
245 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
246 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
247 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
248 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
249 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
250 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
251 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
252 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
253 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
254 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
255 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
256 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
257 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
258 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
259 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
260 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
261 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
262 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
263 a                              0x00064b10 stack_probes::recurse::h35e743424666f0f3 + 48
264 a                              0x0006383d std::sys_common::backtrace::__rust_begin_short_backtrace::h8e110e11ce67e4ad (.llvm.10484649999625559362) + 29
265 libstd-2fb68d5d0d4b1dbe.dylib  0x001de2ed __rust_maybe_catch_panic + 29
266 a                              0x00065b03 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hafdcae48e732805c + 131
267 libstd-2fb68d5d0d4b1dbe.dylib  0x001b50e1 _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::ha8b64f65a0cfa690 + 65
268 libstd-2fb68d5d0d4b1dbe.dylib  0x001dcea8 std::sys::unix::thread::Thread::new::thread_start::h0bb903c71c01413a + 184
269 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
270 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
271 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb030b000  ecx: 0x000b8b0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x000b8b38  esp: 0x000b8b0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x62000 -    0x66ff3 +a (0) <002E58AF-1413-39DD-954D-C6FA611B2B0E> /Users/USER/*/a
  0x126000 -   0x16bfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1af000 -   0x23effb +libstd-2fb68d5d0d4b1dbe.dylib (0) <7C371CDB-5960-3DF5-9DC8-1C02708CA171> /Users/USER/*/libstd-2fb68d5d0d4b1dbe.dylib
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
    task_for_pid: 2611
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=84.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=84.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                          8K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3516K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9348K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            579.7M      137 
TOTAL                            579.7M      137 
TOTAL, minus reserved VM space   579.5M      137 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-26-181114-1_Traviss-Mac-1044.crash
Process:               a [51612]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51605]
Responsible:           a [51612]
User ID:               501
Date/Time:             2019-05-26 18:11:11.763 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5800 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b069aec8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb069aec8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b069a000-00000000b069b000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b069b000-00000000b089c000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   a                              0x00054481 stack_probes_lto::main::haeaf2f44e7253541 + 2545
4   a                              0x0006c66b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h715664bfc3e64c0a + 11
5   a                              0x000636ec std::sys_common::backtrace::__rust_begin_short_backtrace::h5ceb27c8f2988436 + 12
6   a                              0x00055bfd main + 589
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x00056f8b std::sys::unix::abort_internal::ha73ed347b2c8bece + 11
4   a                              0x00056f79 std::sys_common::util::abort::h5a4bdba44b04c461 + 73
5   a                              0x00063643 std::sys::unix::stack_overflow::imp::signal_handler::h14af32901beb880b + 851
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000632f0 rust_begin_unwind + 16
9   a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
10  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
11  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
12  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
13  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
14  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
15  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
16  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
17  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
18  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
19  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
20  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
21  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
22  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
23  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
24  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
25  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
26  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
27  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
28  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
29  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
30  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
31  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
32  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
33  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
34  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
35  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
36  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
37  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
38  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
39  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
40  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
41  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
42  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
43  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
44  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
45  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
46  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
47  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
48  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
49  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
50  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
51  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
52  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
53  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
54  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
55  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
56  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
57  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
58  a                              0x00054988 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
