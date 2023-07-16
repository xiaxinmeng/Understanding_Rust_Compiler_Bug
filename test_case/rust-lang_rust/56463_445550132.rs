plain
[00:02:49]       Memory: 8 GB
[00:02:49]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:49]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:49]       SMC Version (system): 2.8f0
[00:02:49]       Serial Number (system): VM10lll+IVTa
[00:02:49] 
[00:02:49] hw.ncpu: 4
[00:02:49] hw.byteorder: 1234
[00:02:49] hw.memsize: 8589934592
---
[01:52:44] 
[01:52:44] ---- ../stdsimd/coresimd/macros.rs - coresimd::x86::__m128 (line 292) stdout ----
[01:52:44] error: linking with `cc` failed: signal: 4
[01:52:44]   |
[01:52:44]   = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestSgrFCp/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestSgrFCp/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestSgrFCp/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestSgrFCp/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestSgrFCp/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestSgrFCp/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestSgrFCp/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestSgrFCp/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1-std/i686-apple-darwin/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1-std/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libstd-a864e4fc172b78eb.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libpanic_unwind-d8128fb70a58c95a.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libunwind-c86a7fa8f92d373c.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/liblibc-87bdb3db8afad601.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/liballoc-910506085d5eca6e.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libcore-3aea89fac6a9ea37.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-043b9c2a09c27e31.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[01:52:44] 
[01:52:44] 
[01:52:44] thread '../stdsimd/coresimd/macros.rs - coresimd::x86::__m128 (line 292)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:52:44] 
[01:52:44] 
[01:52:44] failures:
[01:52:44]     ../stdsimd/coresimd/macros.rs - coresimd::x86::__m128 (line 292)
[01:52:44]     ../stdsimd/coresimd/macros.rs - coresimd::x86::__m128 (line 292)
[01:52:44] 
[01:52:44] test result: FAILED. 2208 passed; 1 failed; 8 ignored; 0 measured; 0 filtered out
[01:52:44] 
[01:52:44] error: test failed, to rerun pass '--doc'
[01:52:44] 
[01:52:44] 
[01:52:44] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0/bin/cargo" "test" "--target" "i686-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/libstd/Cargo.toml" "-p" "core" "--"
[01:52:44] 
[01:52:44] 
[01:52:44] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:52:44] Build completed unsuccessfully in 0:44:14
[01:52:44] Build completed unsuccessfully in 0:44:14
[01:52:44] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:34da0820
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec  9 16:12:40 GMT 2018
---
travis_fold:start:after_failure.2
travis_time:start:30724896
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1120
drwx------  21 travis  staff    714 Dec  9 15:45 .
-rw-------@  1 travis  staff  57366 Dec  9 15:45 a_2018-12-09-154540-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34790 Dec  9 15:45 a_2018-12-09-154540_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55585 Dec  9 15:45 a_2018-12-09-154531-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34698 Dec  9 15:45 a_2018-12-09-154531_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 Dec  9 15:45 a_2018-12-09-154525_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9166 Dec  9 15:45 a_2018-12-09-154518_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9171 Dec  9 15:45 a_2018-12-09-154509_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8937 Dec  9 15:45 a_2018-12-09-154508_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9305 Dec  9 15:44 a_2018-12-09-154425_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58266 Dec  9 15:44 a_2018-12-09-154415_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 Dec  9 15:44 a_2018-12-09-154409-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59540 Dec  9 15:44 a_2018-12-09-154409_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60381 Dec  9 15:44 a_2018-12-09-154408_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10895 Dec  9 15:41 a_2018-12-09-154154_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 Dec  9 15:40 a_2018-12-09-154059_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 Dec  9 15:39 a_2018-12-09-153939_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9784 Dec  9 15:38 a_2018-12-09-153838-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9780 Dec  9 15:38 a_2018-12-09-153838_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9484 Dec  9 15:38 a_2018-12-09-153811_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:04707703
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-153811_Traviss-Mac-1044.crash
Process:               a [37170]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [37168]
Responsible:           a [37170]
User ID:               501
Date/Time:             2018-12-09 15:37:57.631 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x0007d9ce abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 46
1   a                              0x0007ce1b std::panicking::try::do_call::h43336942a367bcee (.llvm.13876530554090135727) + 11
2   libstd-a864e4fc172b78eb.dylib  0x001e80dd __rust_maybe_catch_panic + 29
3   a                              0x0007dc35 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 613
4   a                              0x0007c3ab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hdc21b773cf71beba + 11
5   libstd-a864e4fc172b78eb.dylib  0x001cbdfc std::sys_common::backtrace::__rust_begin_short_backtrace::h6b4c7e0ceaf651d6 + 12
6   libstd-a864e4fc172b78eb.dylib  0x001ce414 std::panicking::try::do_call::h70e72bd270b82310 + 20
7   libstd-a864e4fc172b78eb.dylib  0x001e80dd __rust_maybe_catch_panic + 29
8   libstd-a864e4fc172b78eb.dylib  0x001cee04 std::rt::lang_start_internal::h45fa6e15141fcd98 + 436
9   a                              0x0007df6c main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x7977cc90  ebx: 0xbff84228  ecx: 0x00000000  edx: 0x00000000
  edi: 0x001e80ce  esi: 0x00000000  ebp: 0xbff841c8  esp: 0xbff841b0
   ss: 0x00000023  efl: 0x00010292  eip: 0x0007d9ce   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x001e7e80
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x7b000 -    0x7effb +a (0) <544E3F62-8F73-34E0-955A-48B37AC54370> /Users/USER/*/a
  0x124000 -   0x169fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1ad000 -   0x236ffb +libstd-a864e4fc172b78eb.dylib (0) <8F9B4A93-C7F9-321A-91F5-511459BD77AE> /Users/USER/*/libstd-a864e4fc172b78eb.dylib
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
    task_for_pid: 2571
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
__DATA                            3524K       45 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9320K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            569.5M      135 
TOTAL                            569.5M      135 
TOTAL, minus reserved VM space   569.4M      135 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-153838-1_Traviss-Mac-1044.crash
Process:               a [37968]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [37963]
Responsible:           a [37968]
User ID:               501
Date/Time:             2018-12-09 15:38:36.886 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-a864e4fc172b78eb.dylib  0x001f8af7 std::panicking::rust_panic_with_hook::h9e8510b21d0870f6 + 583
1   a                              0x0007ab6f std::panicking::begin_panic::h49740845cde64402 + 47 (panicking.rs:421)
2   a                              0x000785f4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hf67a6b0d57ebdabe + 36 (backtrace.rs:34)
3   a                              0x00077f5b core::ptr::real_drop_in_place::h4979e5af03c472e3 + 11
4   a                              0x000785c3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x000798f8 backtrace::main::hcde7a1a1c3c85e77 + 4600 (backtrace.rs:113)
6   a                              0x00077a7b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hff5aa043ee2e70e4 + 11 (rt.rs:74)
7   libstd-a864e4fc172b78eb.dylib  0x001f5dfc std::sys_common::backtrace::__rust_begin_short_backtrace::h6b4c7e0ceaf651d6 + 12
8   libstd-a864e4fc172b78eb.dylib  0x001f8414 std::panicking::try::do_call::h70e72bd270b82310 + 20
9   libstd-a864e4fc172b78eb.dylib  0x002120dd __rust_maybe_catch_panic + 29
10  libstd-a864e4fc172b78eb.dylib  0x001f8e04 std::rt::lang_start_internal::h45fa6e15141fcd98 + 436
11  a                              0x0007a17c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff8af78  ebx: 0xbff8afc0  ecx: 0xbff8ae60  edx: 0xa7702ec6
  edi: 0x0023b30c  esi: 0x001f88be  ebp: 0xbff8b018  esp: 0xbff8af90
   ss: 0x00000023  efl: 0x00010286  eip: 0x001f8af7   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x004b4f02
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x74000 -    0x7bff7 +a (0) <8407A827-53A1-3E28-B988-672D2DAA3C7C> /Users/USER/*/a
  0x14e000 -   0x193fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1d7000 -   0x260ffb +libstd-a864e4fc172b78eb.dylib (0) <8F9B4A93-C7F9-321A-91F5-511459BD77AE> /Users/USER/*/libstd-a864e4fc172b78eb.dylib
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
    task_for_pid: 2571
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=100.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=100.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            36.4M       11 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3524K       45 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9336K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            595.9M      138 
TOTAL                            595.9M      138 
TOTAL, minus reserved VM space   595.7M      138 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-153838_Traviss-Mac-1044.crash
Process:               a [37967]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [37963]
Responsible:           a [37967]
User ID:               501
Date/Time:             2018-12-09 15:38:36.830 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-a864e4fc172b78eb.dylib  0x0020baf7 std::panicking::rust_panic_with_hook::h9e8510b21d0870f6 + 583
1   a                              0x000b5b6f std::panicking::begin_panic::h49740845cde64402 + 47 (panicking.rs:421)
2   a                              0x000b35f4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hf67a6b0d57ebdabe + 36 (backtrace.rs:34)
3   a                              0x000b2f5b core::ptr::real_drop_in_place::h4979e5af03c472e3 + 11
4   a                              0x000b35c3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x000b48f8 backtrace::main::hcde7a1a1c3c85e77 + 4600 (backtrace.rs:113)
6   a                              0x000b2a7b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hff5aa043ee2e70e4 + 11 (rt.rs:74)
7   libstd-a864e4fc172b78eb.dylib  0x00208dfc std::sys_common::backtrace::__rust_begin_short_backtrace::h6b4c7e0ceaf651d6 + 12
8   libstd-a864e4fc172b78eb.dylib  0x0020b414 std::panicking::try::do_call::h70e72bd270b82310 + 20
9   libstd-a864e4fc172b78eb.dylib  0x002250dd __rust_maybe_catch_panic + 29
10  libstd-a864e4fc172b78eb.dylib  0x0020be04 std::rt::lang_start_internal::h45fa6e15141fcd98 + 436
11  a                              0x000b517c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff4ff88  ebx: 0xbff4ffd0  ecx: 0xbff4fe70  edx: 0xa7702ec6
  edi: 0x0024e30c  esi: 0x0020b8be  ebp: 0xbff50028  esp: 0xbff4ffa0
   ss: 0x00000023  efl: 0x00010282  eip: 0x0020baf7   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x004c7f02
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xaf000 -    0xb6ff7 +a (0) <8407A827-53A1-3E28-B988-672D2DAA3C7C> /Users/USER/*/a
  0x161000 -   0x1a6fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1ea000 -   0x273ffb +libstd-a864e4fc172b78eb.dylib (0) <8F9B4A93-C7F9-321A-91F5-511459BD77AE> /Users/USER/*/libstd-a864e4fc172b78eb.dylib
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
    task_for_pid: 2571
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=73.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9604K        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3524K       45 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9336K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            568.9M      136 
TOTAL                            568.9M      136 
TOTAL, minus reserved VM space   568.7M      136 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-153939_Traviss-Mac-1044.crash
Process:               a [39560]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [39559]
Responsible:           a [39560]
User ID:               501
Date/Time:             2018-12-09 15:39:38.706 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4900 seconds
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
3   libstd-a864e4fc172b78eb.dylib  0x0019d66b std::sys::unix::abort_internal::h11df38a9e25d2d5a + 11
4   libstd-a864e4fc172b78eb.dylib  0x0018e8a0 rust_oom + 48
5   libstd-a864e4fc172b78eb.dylib  0x001ae2c4 alloc::alloc::handle_alloc_error::hbc8bbea36adecdda + 20
6   a                              0x00095606 default_alloc_error_hook::main::hbf2d06db626d002e + 790
7   a                              0x0009578b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h6f5245cf34af3430 + 11
8   libstd-a864e4fc172b78eb.dylib  0x0018cdfc std::sys_common::backtrace::__rust_begin_short_backtrace::h6b4c7e0ceaf651d6 + 12
9   libstd-a864e4fc172b78eb.dylib  0x0018f414 std::panicking::try::do_call::h70e72bd270b82310 + 20
10  libstd-a864e4fc172b78eb.dylib  0x001a90dd __rust_maybe_catch_panic + 29
11  libstd-a864e4fc172b78eb.dylib  0x0018fe04 std::rt::lang_start_internal::h45fa6e15141fcd98 + 436
12  a                              0x0009576c main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff6b13c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff6b168  esp: 0xbff6b13c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x94000 -    0x95ffb +a (0) <EC1EF574-7703-3221-B2AD-F16BD73D4B7B> /Users/USER/*/a
   0xe5000 -   0x12afdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x16e000 -   0x1f7ffb +libstd-a864e4fc172b78eb.dylib (0) <8F9B4A93-C7F9-321A-91F5-511459BD77AE> /Users/USER/*/libstd-a864e4fc172b78eb.dylib
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
TOTAL                            568.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-154409_Traviss-Mac-1044.crash
Process:               a [45848]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [45846]
Responsible:           a [45848]
User ID:               501
Date/Time:             2018-12-09 15:44:08.649 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5200 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b003cfec
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb003cfec:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b003c000-00000000b003d000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b003d000-00000000b023e000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-a864e4fc172b78eb.dylib  0x001ef040 std::sys::unix::thread::Thread::join::hb704f80f5c5986df + 32
4   a                              0x000593f6 _$LT$std..thread..JoinHandle$LT$T$GT$$GT$::join::h90290ff26b108e21 + 70
5   a                              0x00057c19 out_of_stack::main::hfb05bc1bb33cf0c4 + 233
6   a                              0x0005700b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h1a17ee7ca8d525c2 + 11
7   libstd-a864e4fc172b78eb.dylib  0x001dedfc std::sys_common::backtrace::__rust_begin_short_backtrace::h6b4c7e0ceaf651d6 + 12
8   libstd-a864e4fc172b78eb.dylib  0x001e1414 std::panicking::try::do_call::h70e72bd270b82310 + 20
9   libstd-a864e4fc172b78eb.dylib  0x001fb0dd __rust_maybe_catch_panic + 29
10  libstd-a864e4fc172b78eb.dylib  0x001e1e04 std::rt::lang_start_internal::h45fa6e15141fcd98 + 436
11  a                              0x0005893c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-a864e4fc172b78eb.dylib  0x001ef66b std::sys::unix::abort_internal::h11df38a9e25d2d5a + 11
4   libstd-a864e4fc172b78eb.dylib  0x001df962 std::sys_common::util::abort::hf9f7115afc09ee95 + 82
5   libstd-a864e4fc172b78eb.dylib  0x001eeaa0 std::sys::unix::stack_overflow::imp::signal_handler::hf1a9b448623e5488 + 608
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-a864e4fc172b78eb.dylib  0x001ee840 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h9ecf6a2b7f565b3c + 80
9   libstd-a864e4fc172b78eb.dylib  0x001ceb67 _$LT$std..io..stdio..StdoutLock$LT$$u27$a$GT$$u20$as$u20$std..io..Write$GT$::write::h603e75dcb3cd2e14 + 263
10  libstd-a864e4fc172b78eb.dylib  0x001d0597 std::io::Write::write_all::h8d1784796cf92b68 + 71
11  libstd-a864e4fc172b78eb.dylib  0x001d0843 _$LT$std..io..Write..write_fmt..Adaptor$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..fmt..Write$GT$::write_str::h5019903c37ad7d4b + 35
12  libstd-a864e4fc172b78eb.dylib  0x0020dd04 core::fmt::write::h2819f76745b9e88c + 740
13  libstd-a864e4fc172b78eb.dylib  0x001ce936 _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::hcf23c2957e80ed91 + 182
14  libstd-a864e4fc172b78eb.dylib  0x001cfbfc std::io::stdio::_print::h789cae5c4766ebb0 + 396
15  a                              0x00057b1f out_of_stack::loud_recurse::hcd528ebf130a94fa + 63
16  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
17  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
18  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
19  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
20  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
21  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
22  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
23  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
24  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
25  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
26  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
27  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
28  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
29  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
30  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
31  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
32  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
33  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
34  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
35  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
36  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
37  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
38  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
39  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
40  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
41  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
42  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
43  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
44  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
45  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
46  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
47  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
48  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
49  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
50  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
51  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
52  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
53  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
54  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
55  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
56  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
57  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
58  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
59  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
60  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
61  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
62  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
63  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
64  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
65  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
66  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
67  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
68  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
69  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
70  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
71  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
72  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
73  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
74  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
75  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
76  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
77  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
78  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
79  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
80  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
81  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
82  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
83  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
84  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
85  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
86  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
87  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
88  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
89  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
90  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
91  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
92  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
93  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
94  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
95  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
96  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
97  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
98  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
99  a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
100 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
101 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
102 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
103 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
104 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
105 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
106 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
107 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
108 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
109 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
110 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
111 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
112 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
113 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
114 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
115 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
116 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
117 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
118 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
119 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
120 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
121 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
122 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
123 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
124 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
125 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
126 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
127 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
128 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
129 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
130 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
131 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
132 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
133 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
134 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
135 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
136 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
137 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
138 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
139 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
140 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
141 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
142 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
143 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
144 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
145 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
146 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
147 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
148 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
149 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
150 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
151 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
152 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
153 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
154 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
155 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
156 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
157 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
158 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
159 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
160 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
161 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
162 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
163 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
164 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
165 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
166 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
167 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
168 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
169 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
170 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
171 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
172 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
173 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
174 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
175 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
176 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
177 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
178 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
179 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
180 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
181 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
182 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
183 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
184 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
185 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
186 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
187 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
188 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
189 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
190 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
191 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
192 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
193 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
194 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
195 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
196 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
197 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
198 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
199 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
200 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
201 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
202 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
203 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
204 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
205 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
206 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
207 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
208 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
209 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
210 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
211 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
212 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
213 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
214 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
215 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
216 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
217 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
218 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
219 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
220 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
221 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
222 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
223 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
224 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
225 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
226 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
227 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
228 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
229 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
230 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
231 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
232 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
233 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
234 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
235 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
236 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
237 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
238 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
239 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
240 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
241 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
242 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
243 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
244 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
245 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
246 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
247 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
248 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
249 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
250 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
251 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
252 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
253 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
254 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
255 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
256 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
257 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
258 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
259 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
260 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
261 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
262 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
263 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
264 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
265 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
266 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
267 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
268 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
269 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
270 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
271 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
272 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
273 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
274 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
275 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
276 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
277 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
278 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
279 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
280 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
281 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
282 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
283 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
284 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
285 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
286 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
287 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
288 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
289 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
290 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
291 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
292 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
293 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
294 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
295 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
296 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
297 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
298 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
299 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
300 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
301 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
302 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
303 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
304 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
305 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
306 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
307 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
308 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
309 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
310 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
311 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
312 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
313 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
314 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
315 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
316 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
317 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
318 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
319 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
320 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
321 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
322 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
323 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
324 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
325 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
326 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
327 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
328 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
329 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
330 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
331 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
332 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
333 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
334 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
335 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
336 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
337 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
338 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
339 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
340 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
341 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
342 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
343 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
344 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
345 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
346 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
347 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
348 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
349 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
350 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
351 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
352 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
353 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
354 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
355 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
356 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
357 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
358 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
359 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
360 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
361 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
362 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
363 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
364 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
365 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
366 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
367 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
368 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
369 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
370 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
371 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
372 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
373 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
374 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
375 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
376 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
377 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
378 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
379 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
380 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
381 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
382 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
383 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
384 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
385 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
386 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
387 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
388 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
389 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
390 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
391 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
392 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
393 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
394 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
395 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
396 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
397 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
398 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
399 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
400 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
401 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
402 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
403 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
404 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
405 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
406 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
407 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
408 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
409 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
410 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
411 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
412 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
413 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
414 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
415 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
416 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
417 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
418 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
419 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
420 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
421 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
422 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
423 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
424 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
425 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
426 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
427 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
428 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
429 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
430 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
431 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
432 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
433 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
434 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
435 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
436 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
437 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
438 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
439 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
440 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
441 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
442 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
443 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
444 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
445 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
446 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
447 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
448 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
449 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
450 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
451 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
452 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
453 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
454 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
455 a                              0x00057b24 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
---
===========                     =======  ======= 
TOTAL                            568.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-154425_Traviss-Mac-1044.crash
Process:               a [46033]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [46032]
Responsible:           a [46033]
User ID:               501
Date/Time:             2018-12-09 15:44:24.636 +0000
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
3   a                              0x000e04ab panic_abort::__rust_start_panic::abort::h09929f5c6db85081 + 11
4   a                              0x000e049b __rust_start_panic + 11
5   a                              0x000d41fb rust_panic + 11
6   a                              0x000d3d89 std::panicking::rust_panic_with_hook::h9e8510b21d0870f6 + 1321
7   a                              0x000e4dea std::panicking::begin_panic::h070e4f16f4d5ca14 + 42
8   a                              0x000d2b9f lto_abort::main::h9419a0043b6e0505 + 3023
9   a                              0x000e4f5b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf5889b1a07ff9f94 + 11
10  a                              0x000e020c std::sys_common::backtrace::__rust_begin_short_backtrace::h6b4c7e0ceaf651d6 + 12
11  a                              0x000d2f84 main + 996
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff2e11c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff2e148  esp: 0xbff2e11c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xd1000 -    0xf4ff3 +a (0) <C7260C41-73F9-3DC9-8A5D-5CC029925598> /Users/USER/*/a
  0x113000 -   0x158fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2571
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
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-154508_Traviss-Mac-1044.crash
Process:               a [47053]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47048]
Responsible:           a [47053]
User ID:               501
Date/Time:             2018-12-09 15:45:08.063 +0000
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
3   libstd-a864e4fc172b78eb.dylib  0x001dd66b std::sys::unix::abort_internal::h11df38a9e25d2d5a + 11
4   libstd-a864e4fc172b78eb.dylib  0x001cd962 std::sys_common::util::abort::hf9f7115afc09ee95 + 82
5   libstd-a864e4fc172b78eb.dylib  0x001cfc4b rust_panic + 107
6   libstd-a864e4fc172b78eb.dylib  0x001cfb0b std::panicking::rust_panic_with_hook::h9e8510b21d0870f6 + 603
7   a                              0x000c2d3f std::panicking::begin_panic::had33920219affb71 + 47
8   a                              0x000c3e8c main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff3e11c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff3e148  esp: 0xbff3e11c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xc1000 -    0xc4fff +a (0) <0E12294A-A58F-30BE-A211-B0F61E8322F5> /Users/USER/*/a
  0x125000 -   0x16afdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1ae000 -   0x237ffb +libstd-a864e4fc172b78eb.dylib (0) <8F9B4A93-C7F9-321A-91F5-511459BD77AE> /Users/USER/*/libstd-a864e4fc172b78eb.dylib
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
    task_for_pid: 2571
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
Writable regions: Total=73.2M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.2M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
__DATA                            3524K       45 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__OBJC                              36K        6 
__TEXT                            9320K       44 
mapped file                      408.7M       21 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            568.4M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-154509_Traviss-Mac-1044.crash
Process:               a [47079]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47073]
Responsible:           a [47079]
User ID:               501
Date/Time:             2018-12-09 15:45:09.021 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5200 seconds
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
    __TEXT                 00000000000d5000-00000000000d8000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000d6f0e segfault_no_out_of_stack::main::haec5f9680e9a04a3 + 2110
1   a                              0x000d581b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha3a9f54a77425a30 + 11
2   libstd-a864e4fc172b78eb.dylib  0x00259dfc std::sys_common::backtrace::__rust_begin_short_backtrace::h6b4c7e0ceaf651d6 + 12
3   libstd-a864e4fc172b78eb.dylib  0x0025c414 std::panicking::try::do_call::h70e72bd270b82310 + 20
4   libstd-a864e4fc172b78eb.dylib  0x002760dd __rust_maybe_catch_panic + 29
5   libstd-a864e4fc172b78eb.dylib  0x0025ce04 std::rt::lang_start_internal::h45fa6e15141fcd98 + 436
6   a                              0x000d71dc main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x7be4b620  ecx: 0x00000000  edx: 0x00000000
  edi: 0x002760ce  esi: 0xbff2a270  ebp: 0xbff2a338  esp: 0xbff2a190
   ss: 0x00000023  efl: 0x00010246  eip: 0x000d6f0e   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     2
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xd5000 -    0xd7ffb +a (0) <9C1E9AAD-8F7D-3CB1-AE77-070A25CBDDBA> /Users/USER/*/a
  0x1b2000 -   0x1f7fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x23b000 -   0x2c4ffb +libstd-a864e4fc172b78eb.dylib (0) <8F9B4A93-C7F9-321A-91F5-511459BD77AE> /Users/USER/*/libstd-a864e4fc172b78eb.dylib
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
    task_for_pid: 2571
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
__DATA                            3524K       45 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.5M      135 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-154518_Traviss-Mac-1044.crash
Process:               a [47263]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47262]
Responsible:           a [47263]
User ID:               501
Date/Time:             2018-12-09 15:45:18.338 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5200 seconds
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
    __TEXT                 000000000007a000-000000000007d000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x0007c584 signal_exit_status::main::h013a3960fc5c363d + 436
1   a                              0x0007b35b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h62cede598c0ade76 + 11
2   libstd-a864e4fc172b78eb.dylib  0x001d3dfc std::sys_common::backtrace::__rust_begin_short_backtrace::h6b4c7e0ceaf651d6 + 12
3   libstd-a864e4fc172b78eb.dylib  0x001d6414 std::panicking::try::do_call::h70e72bd270b82310 + 20
4   libstd-a864e4fc172b78eb.dylib  0x001f00dd __rust_maybe_catch_panic + 29
5   libstd-a864e4fc172b78eb.dylib  0x001d6e04 std::rt::lang_start_internal::h45fa6e15141fcd98 + 436
6   a                              0x0007c65c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x7b6112a0
  edi: 0x7b611330  esi: 0xbff852b0  ebp: 0xbff85348  esp: 0xbff85230
   ss: 0x00000023  efl: 0x00010246  eip: 0x0007c584   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     3
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x7a000 -    0x7cfff +a (0) <5DCA1ACC-0D5D-3C33-9B83-D76CE6F23C3B> /Users/USER/*/a
  0x12c000 -   0x171fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1b5000 -   0x23effb +libstd-a864e4fc172b78eb.dylib (0) <8F9B4A93-C7F9-321A-91F5-511459BD77AE> /Users/USER/*/libstd-a864e4fc172b78eb.dylib
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
    task_for_pid: 2571
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
__DATA                            3524K       45 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.5M      135 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-154525_Traviss-Mac-1044.crash
Process:               a [47351]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47345]
Responsible:           a [47351]
User ID:               501
Date/Time:             2018-12-09 15:45:24.625 +0000
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
0   a                              0x00006722 simd_target_feature_mixup::test::id_avx512_512::h6d002d563db01d8b + 114
1   a                              0x00005478 simd_target_feature_mixup::test::main::h379367934b9623dc + 1848
2   a                              0x000079a9 simd_target_feature_mixup::main::h4f60990077aff357 + 937
3   a                              0x000080fb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h981a4e1a70b17d5a + 11
4   libstd-a864e4fc172b78eb.dylib  0x0017edfc std::sys_common::backtrace::__rust_begin_short_backtrace::h6b4c7e0ceaf651d6 + 12
5   libstd-a864e4fc172b78eb.dylib  0x00181414 std::panicking::try::do_call::h70e72bd270b82310 + 20
6   libstd-a864e4fc172b78eb.dylib  0x0019b0dd __rust_maybe_catch_panic + 29
7   libstd-a864e4fc172b78eb.dylib  0x00181e04 std::rt::lang_start_internal::h45fa6e15141fcd98 + 436
8   a                              0x00007b9c main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbfffafc0  ebx: 0xbfffaec0  ecx: 0x000066be  edx: 0xbfffaec0
  edi: 0x00004d54  esi: 0x00000000  ebp: 0xbfffaeb8  esp: 0xbfffae80
   ss: 0x00000023  efl: 0x00010246  eip: 0x00006722   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00006300
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
    0x4000 -     0x8fff +a (0) <555910C8-7052-351B-8016-E38927F0B38F> /Users/USER/*/a
   0xd7000 -   0x11cfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x160000 -   0x1e9ffb +libstd-a864e4fc172b78eb.dylib (0) <8F9B4A93-C7F9-321A-91F5-511459BD77AE> /Users/USER/*/libstd-a864e4fc172b78eb.dylib
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
    task_for_pid: 2571
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
Writable regions: Total=82.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=82.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3524K       45 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9324K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            577.5M      134 
TOTAL                            577.5M      134 
TOTAL, minus reserved VM space   577.4M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-154531-1_Traviss-Mac-1044.crash
Process:               a [47499]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47496]
Responsible:           a [47499]
User ID:               501
Date/Time:             2018-12-09 15:45:31.374 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5300 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbfa6348
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbfa6348:
    Stack Guard            00000000bbfa5000-00000000bbfa6000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbfa6000-00000000bbfa7000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbfa7000-00000000bffa5000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-a864e4fc172b78eb.dylib  0x0013466b std::sys::unix::abort_internal::h11df38a9e25d2d5a + 11
4   libstd-a864e4fc172b78eb.dylib  0x00124962 std::sys_common::util::abort::hf9f7115afc09ee95 + 82
5   libstd-a864e4fc172b78eb.dylib  0x00133aa0 std::sys::unix::stack_overflow::imp::signal_handler::hf1a9b448623e5488 + 608
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-a864e4fc172b78eb.dylib  0x00133840 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h9ecf6a2b7f565b3c + 80
9   a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
265 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
266 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
267 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
268 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
269 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
270 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
271 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
272 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
273 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
274 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
275 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
276 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
277 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
278 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
279 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
280 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
281 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
282 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
283 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
284 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
285 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
286 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
287 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
288 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
289 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
290 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
291 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
292 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
293 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
294 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
295 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
296 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
297 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
298 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
299 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
300 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
301 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
302 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
303 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
304 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
305 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
306 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
307 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
308 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
309 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
310 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
311 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
312 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
313 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
314 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
315 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
316 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
317 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
318 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
319 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
320 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
321 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
322 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
323 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
324 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
325 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
326 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
327 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
328 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
329 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
330 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
331 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
332 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
333 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
334 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
335 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
336 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
337 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
338 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
339 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
340 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
341 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
342 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
343 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
344 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
345 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
346 a                              0x0005e8c0 stack_probes::recurse::h24283d9484398da0 + 48
---
===========                     =======  ======= 
TOTAL                            568.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-154531_Traviss-Mac-1044.crash
Process:               a [47502]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47496]
Responsible:           a [47502]
User ID:               501
Date/Time:             2018-12-09 15:45:31.381 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5300 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b06b9ea8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb06b9ea8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b06b9000-00000000b06ba000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b06ba000-00000000b08bb000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-a864e4fc172b78eb.dylib  0x001a4040 std::sys::unix::thread::Thread::join::hb704f80f5c5986df + 32
4   a                              0x000047a6 _$LT$std..thread..JoinHandle$LT$T$GT$$GT$::join::h5a4f2f3901b16eca + 70
5   a                              0x00003775 stack_probes::main::hc5f49a55fd7e038b + 597
6   a                              0x0000250b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h3ad456ede7d3fcd2 + 11
7   libstd-a864e4fc172b78eb.dylib  0x00193dfc std::sys_common::backtrace::__rust_begin_short_backtrace::h6b4c7e0ceaf651d6 + 12
8   libstd-a864e4fc172b78eb.dylib  0x00196414 std::panicking::try::do_call::h70e72bd270b82310 + 20
9   libstd-a864e4fc172b78eb.dylib  0x001b00dd __rust_maybe_catch_panic + 29
10  libstd-a864e4fc172b78eb.dylib  0x00196e04 std::rt::lang_start_internal::h45fa6e15141fcd98 + 436
11  a                              0x0000419c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-a864e4fc172b78eb.dylib  0x001a466b std::sys::unix::abort_internal::h11df38a9e25d2d5a + 11
4   libstd-a864e4fc172b78eb.dylib  0x00194962 std::sys_common::util::abort::hf9f7115afc09ee95 + 82
5   libstd-a864e4fc172b78eb.dylib  0x001a3aa0 std::sys::unix::stack_overflow::imp::signal_handler::hf1a9b448623e5488 + 608
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-a864e4fc172b78eb.dylib  0x001a3840 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h9ecf6a2b7f565b3c + 80
9   a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x000038c0 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x0000249d std::sys_common::backtrace::__rust_begin_short_backtrace::h2236a37599c75dd8 + 29
265 libstd-a864e4fc172b78eb.dylib  0x001b00dd __rust_maybe_catch_panic + 29
266 a                              0x00004a83 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::ha63d42b932689466 + 131
267 libstd-a864e4fc172b78eb.dylib  0x001a3f7b std::sys::unix::thread::Thread::new::thread_start::h59da1619537370a3 + 187
268 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
269 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
270 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb08ba000  ecx: 0x00057b2c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x00057b58  esp: 0x00057b2c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
    0x1000 -     0x5fff +a (0) <E498DA73-569F-3003-82D9-2B45E38CB127> /Users/USER/*/a
   0xec000 -   0x131fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x175000 -   0x1feffb +libstd-a864e4fc172b78eb.dylib (0) <8F9B4A93-C7F9-321A-91F5-511459BD77AE> /Users/USER/*/libstd-a864e4fc172b78eb.dylib
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
    task_for_pid: 2571
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
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
__DATA                            3524K       45 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9324K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            571.6M      138 
TOTAL                            571.6M      138 
TOTAL, minus reserved VM space   571.5M      138 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-154540-1_Traviss-Mac-1044.crash
Process:               a [47606]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47603]
Responsible:           a [47606]
User ID:               501
Date/Time:             2018-12-09 15:45:38.730 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5300 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf78388
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf78388:
    Stack Guard            00000000bbf77000-00000000bbf78000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf78000-00000000bbf79000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf79000-00000000bff77000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x0008e3ab std::sys::unix::abort_internal::h11df38a9e25d2d5a + 11
4   a                              0x0008e392 std::sys_common::util::abort::hf9f7115afc09ee95 + 82
5   a                              0x0009d6c8 std::sys::unix::stack_overflow::imp::signal_handler::hf1a9b448623e5488 + 584
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x0009d480 rust_begin_unwind + 16
9   a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
10  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
11  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
12  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
13  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
14  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
15  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
16  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
17  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
18  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
19  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
20  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
21  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
22  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
23  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
24  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
25  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
26  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
27  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
28  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
29  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
30  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
31  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
32  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
33  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
34  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
35  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
36  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
37  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
38  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
39  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
40  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
41  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
42  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
43  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
44  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
45  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
46  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
47  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
48  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
49  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
50  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
51  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
52  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
53  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
54  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
55  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
56  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
57  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
58  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
59  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
60  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
61  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
62  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
63  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
64  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
65  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
66  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
67  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
68  a                              0x0008bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
---
===========                     =======  ======= 
TOTAL                            565.6M      130 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-09-154540_Traviss-Mac-1044.crash
Process:               a [47610]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47603]
Responsible:           a [47610]
User ID:               501
Date/Time:             2018-12-09 15:45:38.733 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5300 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0353f08
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0353f08:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0353000-00000000b0354000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0354000-00000000b0555000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   a                              0x0004b75e stack_probes_lto::main::h7f9d4bbc5a99dd1a + 2542
4   a                              0x00063dcb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h99550ee3228369a1 + 11
5   a                              0x0005d78c std::sys_common::backtrace::__rust_begin_short_backtrace::h6b4c7e0ceaf651d6 + 12
6   a                              0x0004cee0 main + 592
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x0004e3ab std::sys::unix::abort_internal::h11df38a9e25d2d5a + 11
4   a                              0x0004e392 std::sys_common::util::abort::hf9f7115afc09ee95 + 82
5   a                              0x0005d6c8 std::sys::unix::stack_overflow::imp::signal_handler::hf1a9b448623e5488 + 584
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x0005d480 rust_begin_unwind + 16
9   a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
10  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
11  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
12  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
13  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
14  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
15  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
16  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
17  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
18  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
19  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
20  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
21  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
22  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
23  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
24  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
25  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
26  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
27  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
28  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
29  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
30  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
31  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
32  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
33  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
34  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
35  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
36  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
37  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
38  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
39  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
40  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
41  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
42  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
43  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
44  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
45  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
46  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
47  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
48  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
49  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
50  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
51  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
52  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
53  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
54  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
55  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
56  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
57  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
58  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
59  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
60  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
61  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
62  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
63  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
64  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
65  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
66  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
67  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
68  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
69  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
70  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
71  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
72  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
73  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
74  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
75  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
76  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
77  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
78  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
79  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
80  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
81  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
82  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
83  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
84  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
85  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
86  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
87  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
88  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
89  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
90  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
91  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
92  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
93  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
94  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
95  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
96  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
97  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
98  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
99  a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
100 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
101 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
102 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
103 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
104 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
105 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
106 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
107 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
108 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
109 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
110 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
111 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
112 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
113 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
114 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
115 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
116 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
117 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
118 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
119 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
120 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
121 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
122 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
123 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
124 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
125 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
126 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
127 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
128 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
129 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
130 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
131 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
132 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
133 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
134 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
135 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
136 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
137 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
138 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
139 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
140 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
141 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
142 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
143 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
144 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
145 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
146 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
147 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
148 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
149 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
150 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
151 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
152 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
153 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
154 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
155 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
156 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
157 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
158 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
159 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
160 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
161 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
162 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
163 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
164 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
165 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
166 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
167 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
168 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
169 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
170 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
171 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
172 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
173 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
174 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
175 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
176 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
177 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
178 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
179 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
180 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
181 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
182 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
183 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
184 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
185 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
186 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
187 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
188 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
189 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
190 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
191 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
192 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
193 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
194 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
195 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
196 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
197 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
198 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
199 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
200 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
201 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
202 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
203 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
204 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
205 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
206 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
207 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
208 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
209 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
210 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
211 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
212 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
213 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
214 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
215 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
216 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
217 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
218 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
219 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
220 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
221 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
222 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
223 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
224 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
225 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
226 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
227 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
228 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
229 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
230 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
231 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
232 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
233 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
234 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
235 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
236 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
237 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
238 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
239 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
240 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
241 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
242 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
243 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
244 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
245 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
246 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
247 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
248 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
249 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
250 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
251 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
252 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
253 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
254 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
255 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
256 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
257 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
258 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
259 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
260 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
261 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
262 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
263 a                              0x0004bc48 stack_probes_lto::recurse::h907252696a8f0ddd + 40
264 a                              0x00063894 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h69723346175f7322 + 116
265 a                              0x0005db9b std::sys::unix::thread::Thread::new::thread_start::h59da1619537370a3 + 187
266 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
267 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
268 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb0554000  ecx: 0x000ceb3c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x000ceb68  esp: 0x000ceb3c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00071008
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0x4a000 -    0x75ff3 +a (0) <28E347CB-0E15-39D7-A687-FA23B2A25EB4> /Users/USER/*/a
  0x162000 -   0x1a7fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2571
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
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            568.8M      134 
TOTAL                            568.8M      134 
TOTAL, minus reserved VM space   568.6M      134 
travis_time:end:04707703:start=1544371967250998000,finish=1544371976124766000,duration=8873768000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f641e2d
travis_time:start:1f641e2d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07deb7f8
travis_time:start:07deb7f8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0dc7e078
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
