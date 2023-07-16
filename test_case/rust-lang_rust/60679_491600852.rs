plain
[00:03:23]       Memory: 8 GB
[00:03:23]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:23]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:23]       SMC Version (system): 2.8f0
[00:03:23]       Serial Number (system): VMD/EqgR2kXL
[00:03:23] 
[00:03:23] hw.ncpu: 4
[00:03:23] hw.byteorder: 1234
[00:03:23] hw.memsize: 8589934592
---
[02:09:55] 
[02:09:55] ---- /Users/travis/build/rust-lang/rust/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 13) stdout ----
[02:09:55] error: linking with `cc` failed: signal: 4
[02:09:55]   |
[02:09:55]   = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestdImJHP/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestdImJHP/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestdImJHP/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestdImJHP/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestdImJHP/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestdImJHP/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestdImJHP/rust_out.rust_out.7rcbfp3g-cgu.6.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestdImJHP/rust_out.rust_out.7rcbfp3g-cgu.7.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestdImJHP/rust_out.rust_out.7rcbfp3g-cgu.8.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestdImJHP/rust_out.rust_out.7rcbfp3g-cgu.9.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestdImJHP/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestdImJHP/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libstd-4abe846d76794026.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libpanic_unwind-609d5a7dc9a28eae.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libbacktrace_sys-7de7418f196502b0.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_demangle-da967535c682fd8d.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libhashbrown-16b5c4dad8b8bd9e.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_alloc-ad24c364747a80c1.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libunwind-430d09f1fd7bad92.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liblibc-b18dfaed99ca7a71.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liballoc-9be00977897baa0e.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_core-bc86d4a5a44cda78.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcore-75cce831a556c528.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-459a1a0e3e45fba1.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:09:55] 
[02:09:55] error: aborting due to previous error
[02:09:55] 
[02:09:55] thread '/Users/travis/build/rust-lang/rust/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:319:13
---
[02:09:55] 
[02:09:55] 
[02:09:55] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:09:55] Build completed unsuccessfully in 0:50:43
[02:09:55] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00f0e472
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May 12 14:31:04 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:146a29ee
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1176
drwx------  26 travis  staff    884 May 12 14:30 .
-rw-------@  1 travis  staff   1387 May 12 14:30 foo_2019-05-12-143027_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1377 May 12 14:29 m4_2019-05-12-142957_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1390 May 12 14:29 bar_2019-05-12-142948_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9899 May 12 14:29 b_2019-05-12-142947_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1403 May 12 14:29 bar_2019-05-12-142947_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34961 May 12 13:57 a_2019-05-12-135700-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  57364 May 12 13:57 a_2019-05-12-135700_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34892 May 12 13:56 a_2019-05-12-135651-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55583 May 12 13:56 a_2019-05-12-135651_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 May 12 13:56 a_2019-05-12-135646_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9251 May 12 13:56 a_2019-05-12-135640_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9262 May 12 13:56 a_2019-05-12-135633_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8936 May 12 13:56 a_2019-05-12-135632_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9304 May 12 13:55 a_2019-05-12-135553_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58251 May 12 13:55 a_2019-05-12-135545_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60372 May 12 13:55 a_2019-05-12-135538-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59516 May 12 13:55 a_2019-05-12-135538-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 May 12 13:55 a_2019-05-12-135538_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10879 May 12 13:53 a_2019-05-12-135325_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 May 12 13:52 a_2019-05-12-135233_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 May 12 13:51 a_2019-05-12-135116_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 May 12 13:50 a_2019-05-12-135015-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9780 May 12 13:50 a_2019-05-12-135015_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9484 May 12 13:50 a_2019-05-12-135011_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1c6aa7ec
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-12-135011_Traviss-Mac-1044.crash
Process:               a [40409]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [40406]
Responsible:           a [40409]
User ID:               501
Date/Time:             2019-05-12 13:49:44.880 +0000
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
0   a                              0x000ebafe abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 46
1   a                              0x000eaf4b std::panicking::try::do_call::h5fe737096e5e985d (.llvm.16598994130957019884) + 11
2   libstd-4abe846d76794026.dylib  0x001f738d __rust_maybe_catch_panic + 29
3   a                              0x000ebd65 abort_on_c_abi::main::he771bf881fc862e3 + 613
4   a                              0x000ea5ab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd453bee4204744f2 + 11
5   libstd-4abe846d76794026.dylib  0x001e5ccc std::sys_common::backtrace::__rust_begin_short_backtrace::h100d013749c77ce7 + 12
6   libstd-4abe846d76794026.dylib  0x001e8834 std::panicking::try::do_call::hb63ad230d246f403 + 20
7   libstd-4abe846d76794026.dylib  0x001f738d __rust_maybe_catch_panic + 29
8   libstd-4abe846d76794026.dylib  0x001e92d7 std::rt::lang_start_internal::h39a454825fe14ac7 + 631
9   a                              0x000ec09c main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x7861e3d0  ebx: 0xbff156a8  ecx: 0x00000000  edx: 0x00000000
  edi: 0x001f737e  esi: 0x00000000  ebp: 0xbff15648  esp: 0xbff15630
   ss: 0x00000023  efl: 0x00010296  eip: 0x000ebafe   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00230d44
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xe9000 -    0xecff3 +a (0) <466DC0AB-8F10-376D-B759-5C21193EF85B> /Users/USER/*/a
  0x13e000 -   0x183fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1c7000 -   0x256ff7 +libstd-4abe846d76794026.dylib (0) <CF625C04-A049-38F3-8F3D-1FE667C0A848> /Users/USER/*/libstd-4abe846d76794026.dylib
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
    task_for_pid: 2634
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
__DATA                            3552K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-12-135015-1_Traviss-Mac-1044.crash
Process:               a [41186]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [41180]
Responsible:           a [41186]
User ID:               501
Date/Time:             2019-05-12 13:50:14.789 +0000
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
0   libstd-4abe846d76794026.dylib  0x001f6d43 std::panicking::rust_panic_with_hook::h1d7088df4292c897 + 115
1   a                              0x000b1bef std::panicking::begin_panic::hd3c8acc7c0dbe265 + 47 (panicking.rs:408)
2   a                              0x000af6d4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 36 (backtrace.rs:24)
3   a                              0x000af0fb core::ptr::real_drop_in_place::ha0fd09e913297feb + 11
4   a                              0x000af6a3 backtrace::double::h35adec2a6f63ef6c + 51
5   a                              0x000b09b8 backtrace::main::hc9a5bc8fc93ded64 + 4568 (backtrace.rs:103)
6   a                              0x000aeb8b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h89f0713150257b1f + 11 (rt.rs:64)
7   libstd-4abe846d76794026.dylib  0x001f3ccc std::sys_common::backtrace::__rust_begin_short_backtrace::h100d013749c77ce7 + 12
8   libstd-4abe846d76794026.dylib  0x001f6834 std::panicking::try::do_call::hb63ad230d246f403 + 20
9   libstd-4abe846d76794026.dylib  0x0020538d __rust_maybe_catch_panic + 29
10  libstd-4abe846d76794026.dylib  0x001f72d7 std::rt::lang_start_internal::h39a454825fe14ac7 + 631
11  a                              0x000b120c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff523f8  ebx: 0xbff52440  ecx: 0xbff522f0  edx: 0xa7702ec6
  edi: 0x0023f088  esi: 0x001f6cde  ebp: 0xbff52498  esp: 0xbff52410
   ss: 0x00000023  efl: 0x00010282  eip: 0x001f6d43   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x004c19fd
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xac000 -    0xb2fff +a (0) <BA45A5E9-65A8-380C-837F-05DFF7DE3EF2> /Users/USER/*/a
  0x14c000 -   0x191fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1d5000 -   0x264ff7 +libstd-4abe846d76794026.dylib (0) <CF625C04-A049-38F3-8F3D-1FE667C0A848> /Users/USER/*/libstd-4abe846d76794026.dylib
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
    task_for_pid: 2634
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
__DATA                            3552K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9356K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            578.0M      135 
TOTAL                            578.0M      135 
TOTAL, minus reserved VM space   577.8M      135 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-12-135015_Traviss-Mac-1044.crash
Process:               a [41185]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [41180]
Responsible:           a [41185]
User ID:               501
Date/Time:             2019-05-12 13:50:14.754 +0000
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
0   libstd-4abe846d76794026.dylib  0x001dfd43 std::panicking::rust_panic_with_hook::h1d7088df4292c897 + 115
1   a                              0x00060bef std::panicking::begin_panic::hd3c8acc7c0dbe265 + 47 (panicking.rs:408)
2   a                              0x0005e6d4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 36 (backtrace.rs:24)
3   a                              0x0005e0fb core::ptr::real_drop_in_place::ha0fd09e913297feb + 11
4   a                              0x0005e6a3 backtrace::double::h35adec2a6f63ef6c + 51
5   a                              0x0005f9b8 backtrace::main::hc9a5bc8fc93ded64 + 4568 (backtrace.rs:103)
6   a                              0x0005db8b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h89f0713150257b1f + 11 (rt.rs:64)
7   libstd-4abe846d76794026.dylib  0x001dcccc std::sys_common::backtrace::__rust_begin_short_backtrace::h100d013749c77ce7 + 12
8   libstd-4abe846d76794026.dylib  0x001df834 std::panicking::try::do_call::hb63ad230d246f403 + 20
9   libstd-4abe846d76794026.dylib  0x001ee38d __rust_maybe_catch_panic + 29
10  libstd-4abe846d76794026.dylib  0x001e02d7 std::rt::lang_start_internal::h39a454825fe14ac7 + 631
11  a                              0x0006020c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbffa3418  ebx: 0xbffa3460  ecx: 0xbffa3310  edx: 0xa7702ec6
  edi: 0x00228088  esi: 0x001dfcde  ebp: 0xbffa34b8  esp: 0xbffa3430
   ss: 0x00000023  efl: 0x00010286  eip: 0x001dfd43   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x79339000
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x5b000 -    0x61fff +a (0) <BA45A5E9-65A8-380C-837F-05DFF7DE3EF2> /Users/USER/*/a
  0x135000 -   0x17afdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1be000 -   0x24dff7 +libstd-4abe846d76794026.dylib (0) <CF625C04-A049-38F3-8F3D-1FE667C0A848> /Users/USER/*/libstd-4abe846d76794026.dylib
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
    task_for_pid: 2634
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=91.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=91.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            27.4M       11 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3552K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9356K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            587.0M      137 
TOTAL                            587.0M      137 
TOTAL, minus reserved VM space   586.8M      137 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-12-135116_Traviss-Mac-1044.crash
Process:               a [42917]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [42916]
Responsible:           a [42917]
User ID:               501
Date/Time:             2019-05-12 13:51:15.568 +0000
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
3   libstd-4abe846d76794026.dylib  0x0021f7ab std::sys::unix::abort_internal::h02bcd6fe4227a27f + 11
4   libstd-4abe846d76794026.dylib  0x00210ec0 rust_oom + 48
5   libstd-4abe846d76794026.dylib  0x002320c4 alloc::alloc::handle_alloc_error::hc7311cf31e6ea2ad + 20
6   a                              0x000fa54d default_alloc_error_hook::main::h0fe124586986ad5d + 781
7   a                              0x000f9d9b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hbacb77ca45202771 + 11
8   libstd-4abe846d76794026.dylib  0x0020eccc std::sys_common::backtrace::__rust_begin_short_backtrace::h100d013749c77ce7 + 12
9   libstd-4abe846d76794026.dylib  0x00211834 std::panicking::try::do_call::hb63ad230d246f403 + 20
10  libstd-4abe846d76794026.dylib  0x0022038d __rust_maybe_catch_panic + 29
11  libstd-4abe846d76794026.dylib  0x002122d7 std::rt::lang_start_internal::h39a454825fe14ac7 + 631
12  a                              0x000fa6ac main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff055bc  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff055e8  esp: 0xbff055bc
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xf9000 -    0xfaffb +a (0) <9A31095F-D9E7-3BC2-A189-F3FB77B3DE32> /Users/USER/*/a
  0x167000 -   0x1acfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1f0000 -   0x27fff7 +libstd-4abe846d76794026.dylib (0) <CF625C04-A049-38F3-8F3D-1FE667C0A848> /Users/USER/*/libstd-4abe846d76794026.dylib
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
TOTAL                            577.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-12-135545_Traviss-Mac-1044.crash
Process:               a [49387]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [49376]
Responsible:           a [49387]
User ID:               501
Date/Time:             2019-05-12 13:55:44.690 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5800 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf9ffdc
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf9ffdc:
    Stack Guard            00000000bbf9e000-00000000bbf9f000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf9f000-00000000bbfa0000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbfa0000-00000000bff9e000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-4abe846d76794026.dylib  0x0017f7ab std::sys::unix::abort_internal::h02bcd6fe4227a27f + 11
4   libstd-4abe846d76794026.dylib  0x0016fca9 std::sys_common::util::abort::h5537abb85ab6294a + 73
5   libstd-4abe846d76794026.dylib  0x0017ea48 std::sys::unix::stack_overflow::imp::signal_handler::hf0cd0686a8b2e278 + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-4abe846d76794026.dylib  0x0017e690 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb725c872c06d0e95 + 80
9   libstd-4abe846d76794026.dylib  0x0015cc3c _$LT$std..io..buffered..LineWriter$LT$W$GT$$u20$as$u20$std..io..Write$GT$::write::h2a846c8c3cb9315d + 220
10  libstd-4abe846d76794026.dylib  0x00161805 std::io::Write::write_all::hdb5f5d4690361623 + 101
11  libstd-4abe846d76794026.dylib  0x00161b23 _$LT$std..io..Write..write_fmt..Adaptor$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$::write_str::h7d9fb1c2ac4fb086 + 35
12  libstd-4abe846d76794026.dylib  0x0019fdad core::fmt::write::hbce2d277ad333e62 + 701
13  libstd-4abe846d76794026.dylib  0x0015f7a6 _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::h74d4e21acdfb8716 + 182
14  libstd-4abe846d76794026.dylib  0x00160cdc std::io::stdio::_print::hb2ef31fab2a5a20e + 396
15  a                              0x00064d4f out_of_stack::loud_recurse::h48447a21f69c373e + 63
16  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
17  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
18  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
19  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
20  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
21  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
22  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
23  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
24  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
25  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
26  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
27  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
28  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
29  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
30  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
31  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
32  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
33  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
34  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
35  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
36  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
37  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
38  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
39  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
40  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
41  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
42  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
43  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
44  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
45  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
46  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
47  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
48  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
49  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
50  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
51  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
52  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
53  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
54  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
55  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
56  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
57  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
58  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
59  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
60  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
61  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
62  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
63  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
64  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
65  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
66  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
67  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
68  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
69  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
70  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
71  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
72  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
73  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
74  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
75  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
76  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
77  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
78  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
79  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
80  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
81  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
82  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
83  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
84  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
85  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
86  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
87  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
88  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
89  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
90  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
91  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
92  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
93  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
94  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
95  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
96  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
97  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
98  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
99  a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
100 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
101 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
102 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
103 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
104 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
105 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
106 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
107 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
108 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
109 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
110 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
111 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
112 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
113 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
114 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
115 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
116 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
117 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
118 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
119 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
120 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
121 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
122 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
123 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
124 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
125 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
126 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
127 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
128 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
129 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
130 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
131 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
132 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
133 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
134 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
135 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
136 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
137 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
138 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
139 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
140 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
141 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
142 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
143 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
144 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
145 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
146 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
147 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
148 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
149 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
150 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
151 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
152 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
153 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
154 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
155 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
156 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
157 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
158 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
159 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
160 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
161 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
162 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
163 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
164 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
165 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
166 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
167 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
168 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
169 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
170 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
171 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
172 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
173 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
174 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
175 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
176 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
177 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
178 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
179 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
180 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
181 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
182 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
183 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
184 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
185 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
186 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
187 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
188 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
189 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
190 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
191 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
192 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
193 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
194 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
195 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
196 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
197 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
198 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
199 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
200 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
201 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
202 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
203 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
204 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
205 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
206 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
207 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
208 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
209 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
210 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
211 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
212 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
213 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
214 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
215 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
216 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
217 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
218 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
219 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
220 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
221 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
222 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
223 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
224 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
225 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
226 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
227 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
228 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
229 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
230 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
231 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
232 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
233 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
234 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
235 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
236 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
237 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
238 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
239 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
240 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
241 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
242 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
243 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
244 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
245 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
246 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
247 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
248 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
249 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
250 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
251 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
252 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
253 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
254 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
255 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
256 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
257 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
258 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
259 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
260 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
261 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
262 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
263 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
264 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
265 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
266 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
267 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
268 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
269 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
270 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
271 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
272 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
273 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
274 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
275 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
276 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
277 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
278 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
279 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
280 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
281 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
282 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
283 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
284 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
285 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
286 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
287 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
288 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
289 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
290 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
291 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
292 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
293 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
294 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
295 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
296 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
297 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
298 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
299 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
300 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
301 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
302 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
303 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
304 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
305 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
306 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
307 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
308 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
309 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
310 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
311 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
312 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
313 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
314 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
315 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
316 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
317 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
318 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
319 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
320 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
321 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
322 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
323 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
324 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
325 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
326 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
327 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
328 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
329 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
330 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
331 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
332 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
333 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
334 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
335 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
336 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
337 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
338 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
339 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
340 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
341 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
342 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
343 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
344 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
345 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
346 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
347 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
348 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
349 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
350 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
351 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
352 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
353 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
354 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
355 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
356 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
357 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
358 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
359 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
360 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
361 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
362 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
363 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
364 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
365 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
366 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
367 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
368 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
369 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
370 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
371 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
372 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
373 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
374 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
375 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
376 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
377 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
378 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
379 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
380 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
381 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
382 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
383 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
384 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
385 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
386 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
387 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
388 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
389 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
390 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
391 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
392 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
393 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
394 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
395 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
396 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
397 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
398 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
399 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
400 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
401 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
402 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
403 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
404 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
405 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
406 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
407 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
408 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
409 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
410 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
411 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
412 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
413 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
414 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
415 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
416 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
417 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
418 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
419 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
420 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
421 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
422 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
423 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
424 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
425 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
426 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
427 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
428 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
429 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
430 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
431 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
432 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
433 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
434 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
435 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
436 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
437 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
438 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
439 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
440 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
441 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
442 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
443 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
444 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
445 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
446 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
447 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
448 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
449 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
450 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
451 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
452 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
453 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
454 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
455 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
456 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
457 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
458 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
459 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
460 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
461 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
462 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
463 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
464 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
465 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
466 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
467 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
468 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
469 a                              0x00064d54 out_of_stack::loud_recurse::h48447a21f69c373e + 68
---
===========                     =======  ======= 
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-12-135553_Traviss-Mac-1044.crash
Process:               a [49576]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [49574]
Responsible:           a [49576]
User ID:               501
Date/Time:             2019-05-12 13:55:52.717 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5800 seconds
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
3   a                              0x0002257b panic_abort::__rust_start_panic::abort::h618efac3971d931d + 11
4   a                              0x0002256b __rust_start_panic + 11
5   a                              0x00016c6b rust_panic + 11
6   a                              0x00016855 std::panicking::rust_panic_with_hook::h1d7088df4292c897 + 997
7   a                              0x000282ea std::panicking::begin_panic::hc824a1ed51317e55 + 42
8   a                              0x0001562d lto_abort::main::hd3a7c21d2a5e6b65 + 2877
9   a                              0x0002842b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd7f782ed9fe9f4c3 + 11
10  a                              0x000223ec std::sys_common::backtrace::__rust_begin_short_backtrace::h100d013749c77ce7 + 12
11  a                              0x00015a08 main + 984
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffea59c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffea5c8  esp: 0xbffea59c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x14000 -    0x37fff +a (0) <506D6E3F-B45A-3E0C-84A1-E855E74FB897> /Users/USER/*/a
   0x67000 -    0xacfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2634
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=82.3M resident=0K(0%) swapped_out_or_unallocated=82.3M(100%)
Writable regions: Total=75.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            11.0M        9 
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
TOTAL                            567.6M      132 
TOTAL                            567.6M      132 
TOTAL, minus reserved VM space   567.5M      132 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-12-135632_Traviss-Mac-1044.crash
Process:               a [50550]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50542]
Responsible:           a [50550]
User ID:               501
Date/Time:             2019-05-12 13:56:31.861 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5800 seconds
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
3   libstd-4abe846d76794026.dylib  0x002727ab std::sys::unix::abort_internal::h02bcd6fe4227a27f + 11
4   libstd-4abe846d76794026.dylib  0x00262ca9 std::sys_common::util::abort::h5537abb85ab6294a + 73
5   libstd-4abe846d76794026.dylib  0x00265052 rust_panic + 98
6   libstd-4abe846d76794026.dylib  0x00264f1e std::panicking::rust_panic_with_hook::h1d7088df4292c897 + 590
7   a                              0x000fa9cf std::panicking::begin_panic::h7d4a9e8c505f0bdb + 47
8   a                              0x000fbb0c main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff0559c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff055c8  esp: 0xbff0559c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xf9000 -    0xfcff7 +a (0) <CA004656-039C-3400-AD2D-4FD912B6E0A7> /Users/USER/*/a
  0x1ba000 -   0x1fffdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x243000 -   0x2d2ff7 +libstd-4abe846d76794026.dylib (0) <CF625C04-A049-38F3-8F3D-1FE667C0A848> /Users/USER/*/libstd-4abe846d76794026.dylib
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
    task_for_pid: 2634
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=73.2M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.2M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
__DATA                            3552K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__OBJC                              36K        6 
__TEXT                            9344K       44 
mapped file                      408.7M       21 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            568.4M      132 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-12-135633_Traviss-Mac-1044.crash
Process:               a [50572]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50568]
Responsible:           a [50572]
User ID:               501
Date/Time:             2019-05-12 13:56:32.531 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5800 seconds
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
    __TEXT                 00000000000b2000-00000000000b5000 [   12K] r-x/rwx SM=COW  ڝ [/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-pass/segfault-no-out-of-stack/a]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000b3f72 segfault_no_out_of_stack::main::h579ee62fe996355a + 2034
1   a                              0x000b299b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h13c7df73b052e880 + 11
2   libstd-4abe846d76794026.dylib  0x001a6ccc std::sys_common::backtrace::__rust_begin_short_backtrace::h100d013749c77ce7 + 12
3   libstd-4abe846d76794026.dylib  0x001a9834 std::panicking::try::do_call::hb63ad230d246f403 + 20
4   libstd-4abe846d76794026.dylib  0x001b838d __rust_maybe_catch_panic + 29
5   libstd-4abe846d76794026.dylib  0x001aa2d7 std::rt::lang_start_internal::h39a454825fe14ac7 + 631
6   a                              0x000b424c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x79660fb0  ecx: 0x00000000  edx: 0x00000000
  edi: 0x001b837e  esi: 0xbff4c6d0  ebp: 0xbff4c7b8  esp: 0xbff4c610
   ss: 0x00000023  efl: 0x00010246  eip: 0x000b3f72   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     1
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xb2000 -    0xb4ffb +a (0) <67695333-C2CC-3673-BDEF-137573636E0D> /Users/USER/*/a
   0xff000 -   0x144fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x188000 -   0x217ff7 +libstd-4abe846d76794026.dylib (0) <CF625C04-A049-38F3-8F3D-1FE667C0A848> /Users/USER/*/libstd-4abe846d76794026.dylib
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
    task_for_pid: 2634
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
__DATA                            3552K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.6M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-12-135640_Traviss-Mac-1044.crash
Process:               a [50772]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50771]
Responsible:           a [50772]
User ID:               501
Date/Time:             2019-05-12 13:56:40.352 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5800 seconds
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
    __TEXT                 000000000009e000-00000000000a1000 [   12K] r-x/rwx SM=COW  ڝ [/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-pass/signal-exit-status/a]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000a05d4 signal_exit_status::main::hc6663d816ec186eb + 436
1   a                              0x0009f47b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf5b2b9d166516e24 + 11
2   libstd-4abe846d76794026.dylib  0x00165ccc std::sys_common::backtrace::__rust_begin_short_backtrace::h100d013749c77ce7 + 12
3   libstd-4abe846d76794026.dylib  0x00168834 std::panicking::try::do_call::hb63ad230d246f403 + 20
4   libstd-4abe846d76794026.dylib  0x0017738d __rust_maybe_catch_panic + 29
5   libstd-4abe846d76794026.dylib  0x001692d7 std::rt::lang_start_internal::h39a454825fe14ac7 + 631
6   a                              0x000a06ac main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x78e17460
  edi: 0x78e174f0  esi: 0xbff60730  ebp: 0xbff607c8  esp: 0xbff606b0
   ss: 0x00000023  efl: 0x00010246  eip: 0x000a05d4   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     1
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x9e000 -    0xa0fff +a (0) <9A3A8741-C193-30E8-95A3-16E72F5199AA> /Users/USER/*/a
   0xbe000 -   0x103fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x147000 -   0x1d6ff7 +libstd-4abe846d76794026.dylib (0) <CF625C04-A049-38F3-8F3D-1FE667C0A848> /Users/USER/*/libstd-4abe846d76794026.dylib
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
    task_for_pid: 2634
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
__DATA                            3552K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-12-135646_Traviss-Mac-1044.crash
Process:               a [50877]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50870]
Responsible:           a [50877]
User ID:               501
Date/Time:             2019-05-12 13:56:45.316 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5900 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00071c46 simd_target_feature_mixup::test::id_avx512_512::hf3a1395d43161fbe + 102
1   a                              0x000709ff simd_target_feature_mixup::test::main::h611f15c116e6c273 + 1647
2   a                              0x00072f70 simd_target_feature_mixup::main::hfcef50d014c08dda + 896
3   a                              0x0007010b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h655ee21705291ae6 + 11
4   libstd-4abe846d76794026.dylib  0x00219ccc std::sys_common::backtrace::__rust_begin_short_backtrace::h100d013749c77ce7 + 12
5   libstd-4abe846d76794026.dylib  0x0021c834 std::panicking::try::do_call::hb63ad230d246f403 + 20
6   libstd-4abe846d76794026.dylib  0x0022b38d __rust_maybe_catch_panic + 29
7   libstd-4abe846d76794026.dylib  0x0021d2d7 std::rt::lang_start_internal::h39a454825fe14ac7 + 631
8   a                              0x0007314c main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff8f400  ebx: 0xbff8f380  ecx: 0x00071bee  edx: 0xbff8f380
  edi: 0x000703a4  esi: 0x00000000  ebp: 0xbff8f378  esp: 0xbff8f340
   ss: 0x00000023  efl: 0x00010246  eip: 0x00071c46   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00071880
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x6f000 -    0x73fc7 +a (0) <F2466E54-1DCA-3582-9C40-E8C1A535C289> /Users/USER/*/a
  0x172000 -   0x1b7fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1fb000 -   0x28aff7 +libstd-4abe846d76794026.dylib (0) <CF625C04-A049-38F3-8F3D-1FE667C0A848> /Users/USER/*/libstd-4abe846d76794026.dylib
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
    task_for_pid: 2634
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
__DATA                            3552K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-12-135651-1_Traviss-Mac-1044.crash
Process:               a [51027]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51023]
Responsible:           a [51027]
User ID:               501
Date/Time:             2019-05-12 13:56:51.110 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5900 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0558e68
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0558e68:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0558000-00000000b0559000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0559000-00000000b075a000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-4abe846d76794026.dylib  0x00275fb0 std::sys::unix::thread::Thread::join::h8832f54497ecedaa + 32
4   a                              0x000bf7b6 std::thread::JoinHandle$LT$T$GT$::join::h1d12cd3b37ad22ce + 70
5   a                              0x000be9d5 stack_probes::main::h385e455299c91d04 + 597
6   a                              0x000bd7eb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h4a81001de63cf2d2 + 11
7   libstd-4abe846d76794026.dylib  0x00265ccc std::sys_common::backtrace::__rust_begin_short_backtrace::h100d013749c77ce7 + 12
8   libstd-4abe846d76794026.dylib  0x00268834 std::panicking::try::do_call::hb63ad230d246f403 + 20
9   libstd-4abe846d76794026.dylib  0x0027738d __rust_maybe_catch_panic + 29
10  libstd-4abe846d76794026.dylib  0x002692d7 std::rt::lang_start_internal::h39a454825fe14ac7 + 631
11  a                              0x000bf3fc main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-4abe846d76794026.dylib  0x002767ab std::sys::unix::abort_internal::h02bcd6fe4227a27f + 11
4   libstd-4abe846d76794026.dylib  0x00266ca9 std::sys_common::util::abort::h5537abb85ab6294a + 73
5   libstd-4abe846d76794026.dylib  0x00275a48 std::sys::unix::stack_overflow::imp::signal_handler::hf0cd0686a8b2e278 + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-4abe846d76794026.dylib  0x00275690 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb725c872c06d0e95 + 80
9   a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
10  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
11  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
12  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
13  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
14  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
15  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
16  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
17  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
18  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
19  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
20  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
21  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
22  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
23  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
24  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
25  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
26  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
27  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
28  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
29  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
30  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
31  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
32  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
33  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
34  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
35  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
36  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
37  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
38  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
39  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
40  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
41  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
42  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
43  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
44  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
45  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
46  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
47  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
48  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
49  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
50  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
51  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
52  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
53  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
54  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
55  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
56  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
57  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
58  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
59  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
60  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
61  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
62  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
63  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
64  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
65  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
66  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
67  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
68  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
69  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
70  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
71  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
72  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
73  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
74  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
75  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
76  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
77  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
78  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
79  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
80  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
81  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
82  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
83  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
84  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
85  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
86  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
87  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
88  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
89  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
90  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
91  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
92  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
93  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
94  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
95  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
96  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
97  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
98  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
99  a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
100 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
101 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
102 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
103 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
104 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
105 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
106 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
107 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
108 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
109 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
110 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
111 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
112 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
113 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
114 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
115 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
116 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
117 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
118 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
119 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
120 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
121 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
122 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
123 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
124 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
125 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
126 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
127 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
128 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
129 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
130 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
131 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
132 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
133 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
134 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
135 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
136 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
137 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
138 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
139 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
140 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
141 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
142 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
143 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
144 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
145 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
146 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
147 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
148 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
149 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
150 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
151 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
152 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
153 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
154 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
155 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
156 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
157 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
158 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
159 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
160 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
161 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
162 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
163 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
164 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
165 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
166 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
167 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
168 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
169 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
170 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
171 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
172 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
173 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
174 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
175 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
176 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
177 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
178 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
179 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
180 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
181 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
182 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
183 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
184 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
185 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
186 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
187 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
188 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
189 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
190 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
191 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
192 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
193 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
194 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
195 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
196 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
197 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
198 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
199 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
200 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
201 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
202 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
203 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
204 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
205 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
206 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
207 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
208 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
209 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
210 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
211 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
212 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
213 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
214 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
215 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
216 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
217 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
218 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
219 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
220 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
221 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
222 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
223 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
224 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
225 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
226 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
227 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
228 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
229 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
230 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
231 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
232 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
233 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
234 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
235 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
236 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
237 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
238 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
239 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
240 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
241 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
242 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
243 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
244 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
245 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
246 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
247 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
248 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
249 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
250 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
251 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
252 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
253 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
254 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
255 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
256 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
257 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
258 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
259 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
260 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
261 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
262 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
263 a                              0x000beb20 stack_probes::recurse::h35e743424666f0f3 + 48
264 a                              0x000bd84d std::sys_common::backtrace::__rust_begin_short_backtrace::h94614072b48aaf73 (.llvm.17983259766672326343) + 29
265 libstd-4abe846d76794026.dylib  0x0027738d __rust_maybe_catch_panic + 29
266 a                              0x000bfb13 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd1acf7c9154cd998 + 131
267 libstd-4abe846d76794026.dylib  0x0024d2e1 _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h25c4185efb4f6b35 + 65
268 libstd-4abe846d76794026.dylib  0x00275ee8 std::sys::unix::thread::Thread::new::thread_start::hcdc42c44f6f551b4 + 184
269 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
270 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
271 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb0759000  ecx: 0x00112b0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x00112b38  esp: 0x00112b0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00275ad0
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0xbc000 -    0xc0ffb +a (0) <68A00D51-7ABD-36C1-BC66-2625B45ABCD4> /Users/USER/*/a
  0x1be000 -   0x203fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x247000 -   0x2d6ff7 +libstd-4abe846d76794026.dylib (0) <CF625C04-A049-38F3-8F3D-1FE667C0A848> /Users/USER/*/libstd-4abe846d76794026.dylib
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
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-12-135700-1_Traviss-Mac-1044.crash
Process:               a [51164]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [51155]
Responsible:           a [51164]
User ID:               501
Date/Time:             2019-05-12 13:56:58.894 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5900 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0311ec8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0311ec8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0311000-00000000b0312000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0312000-00000000b0513000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa77000b6 __bsdthread_create + 10
1   libsystem_pthread.dylib        0xa7832824 _pthread_create + 235
2   libsystem_pthread.dylib        0xa782f228 pthread_create + 28
3   a                              0x00082681 stack_probes_lto::main::haeaf2f44e7253541 + 2177
4   a                              0x0009b4ab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h833e36eee510b558 + 11
5   a                              0x00093cdc std::sys_common::backtrace::__rust_begin_short_backtrace::h100d013749c77ce7 + 12
6   a                              0x00083ebd main + 589
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x0008526b std::sys::unix::abort_internal::h02bcd6fe4227a27f + 11
4   a                              0x00085259 std::sys_common::util::abort::h5537abb85ab6294a + 73
5   a                              0x00093c3c std::sys::unix::stack_overflow::imp::signal_handler::hf0cd0686a8b2e278 + 860
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000938e0 rust_begin_unwind + 16
9   a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
10  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
11  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
12  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
13  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
14  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
15  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
16  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
17  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
18  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
19  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
20  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
21  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
22  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
23  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
24  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
25  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
26  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
27  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
28  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
29  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
30  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
31  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
32  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
33  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
34  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
35  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
36  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
37  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
38  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
39  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
40  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
41  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
42  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
43  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
44  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
45  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
46  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
47  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
48  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
49  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
50  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
51  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
52  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
53  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
54  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
55  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
56  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
57  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
58  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
59  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
60  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
61  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
62  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
63  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
64  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
65  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
66  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
67  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
68  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
69  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
70  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
71  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
72  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
73  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
74  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
75  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
76  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
77  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
78  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
79  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
80  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
81  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
82  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
83  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
84  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
85  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
86  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
87  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
88  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
89  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
90  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
91  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
92  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
93  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
94  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
95  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
96  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
97  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
98  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
99  a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
100 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
101 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
102 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
103 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
104 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
105 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
106 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
107 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
108 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
109 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
110 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
111 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
112 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
113 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
114 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
115 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
116 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
117 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
118 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
119 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
120 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
121 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
122 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
123 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
124 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
125 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
126 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
127 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
128 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
129 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
130 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
131 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
132 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
133 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
134 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
135 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
136 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
137 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
138 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
139 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
140 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
141 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
142 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
143 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
144 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
145 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
146 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
147 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
148 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
149 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
150 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
151 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
152 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
153 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
154 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
155 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
156 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
157 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
158 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
159 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
160 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
161 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
162 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
163 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
164 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
165 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
166 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
167 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
168 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
169 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
170 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
171 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
172 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
173 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
174 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
175 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
176 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
177 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
178 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
179 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
180 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
181 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
182 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
183 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
184 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
185 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
186 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
187 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
188 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
189 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
190 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
191 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
192 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
193 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
194 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
195 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
196 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
197 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
198 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
199 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
200 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
201 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
202 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
203 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
204 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
205 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
206 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
207 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
208 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
209 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
210 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
211 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
212 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
213 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
214 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
215 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
216 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
217 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
218 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
219 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
220 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
221 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
222 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
223 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
224 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
225 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
226 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
227 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
228 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
229 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
230 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
231 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
232 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
233 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
234 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
235 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
236 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
237 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
238 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
239 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
240 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
241 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
242 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
243 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
244 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
245 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
246 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
247 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
248 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
249 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
250 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
251 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
252 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
253 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
254 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
255 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
256 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
257 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
258 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
259 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
260 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
261 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
262 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
263 a                              0x00082c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
264 a                              0x0009ae44 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::he4cfa78f94b23829 + 116
265 a                              0x00093461 _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h25c4185efb4f6b35 + 65
266 a                              0x00093f84 std::sys::unix::thread::Thread::new::thread_start::hcdc42c44f6f551b4 + 180
267 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
268 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
269 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb0512000  ecx: 0x001abb0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x001abb38  esp: 0x001abb0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000a857d
Logical CPU:     0
Error Code:      0x00140168
Trap Number:     132
Binary Images:
   0x81000 -    0xacff3 +a (0) <64AAE2B3-7251-335A-B8F4-687C445A2CE9> /Users/USER/*/a
   0xe3000 -   0x128fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2634
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=82.4M resident=0K(0%) swapped_out_or_unallocated=82.4M(100%)
Writable regions: Total=86.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=86.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            20.0M        9 
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
TOTAL                            578.8M      135 
TOTAL                            578.8M      135 
TOTAL, minus reserved VM space   578.6M      135 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-12-135700_Traviss-Mac-1044.crash
Process:               a [51162]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [51155]
Responsible:           a [51162]
User ID:               501
Date/Time:             2019-05-12 13:56:58.829 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5900 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf5f828
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf5f828:
    Stack Guard            00000000bbf5e000-00000000bbf5f000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf5f000-00000000bbf60000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf60000-00000000bff5e000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000a726b std::sys::unix::abort_internal::h02bcd6fe4227a27f + 11
4   a                              0x000a7259 std::sys_common::util::abort::h5537abb85ab6294a + 73
5   a                              0x000b5c3c std::sys::unix::stack_overflow::imp::signal_handler::hf0cd0686a8b2e278 + 860
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000b58e0 rust_begin_unwind + 16
9   a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
10  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
11  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
12  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
13  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
14  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
15  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
16  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
17  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
18  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
19  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
20  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
21  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
22  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
23  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
24  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
25  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
26  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
27  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
28  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
29  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
30  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
31  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
32  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
33  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
34  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
35  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
36  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
37  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
38  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
39  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
40  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
41  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
42  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
43  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
44  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
45  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
46  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
47  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
48  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
49  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
50  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
51  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
52  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
53  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
54  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
55  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
56  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
57  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
58  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
59  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
60  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
61  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
62  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
63  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
64  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
65  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
66  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
67  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
68  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
69  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
70  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
71  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
72  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
73  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
74  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
75  a                              0x000a4c48 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
