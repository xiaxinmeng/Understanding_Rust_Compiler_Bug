plain
[00:04:19]       Memory: 8 GB
[00:04:19]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:04:19]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:04:19]       SMC Version (system): 2.8f0
[00:04:19]       Serial Number (system): VMYoirPbj3sr
[00:04:19] 
[00:04:19] hw.ncpu: 4
[00:04:19] hw.byteorder: 1234
[00:04:19] hw.memsize: 8589934592
---
[01:26:08] 
[01:26:08] ---- alloc.rs - alloc::alloc (line 62) stdout ----
[01:26:08] error: linking with `cc` failed: signal: 4
[01:26:08]   |
[01:26:08]   = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestNmGs6B/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestNmGs6B/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestNmGs6B/rust_out.rust_out.7rcbfp3g-cgu.10.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestNmGs6B/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestNmGs6B/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestNmGs6B/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestNmGs6B/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestNmGs6B/rust_out.rust_out.7rcbfp3g-cgu.6.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestNmGs6B/rust_out.rust_out.7rcbfp3g-cgu.7.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestNmGs6B/rust_out.rust_out.7rcbfp3g-cgu.8.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestNmGs6B/rust_out.rust_out.7rcbfp3g-cgu.9.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestNmGs6B/rust_out" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1-std/i686-apple-darwin/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1-std/i686-apple-darwin/release/build/compiler_builtins-d3c8b83e73dae572/out" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1-std/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib" "-lstd-312c1b89c01a0dbc" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage1/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-ef64ecc728e2572b.rlib" "-lSystem" "-lresolv" "-lpthread" "-lc" "-lm"
[01:26:08] 
[01:26:08] thread 'alloc.rs - alloc::alloc (line 62)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:26:08] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:26:08] 
---
[01:26:08] 
[01:26:08] error: test failed, to rerun pass '--doc'
[01:26:08] 
[01:26:08] 
[01:26:08] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0/bin/cargo" "test" "--target" "i686-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:26:08] 
[01:26:08] 
[01:26:08] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:26:08] Build completed unsuccessfully in 0:32:42
[01:26:08] Build completed unsuccessfully in 0:32:42
[01:26:08] make: *** [check] Error 1
travis_time:end:0041bb8a:start=1535745777207002000,finish=1535750945694350000,duration=5168487348000

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17c90e8e
---
travis_fold:start:after_failure.2
travis_time:start:24943fd1
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1120
drwx------  21 travis  staff    714 Aug 31 21:14 .
-rw-------@  1 travis  staff  34853 Aug 31 21:14 a_2018-08-31-211423-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  57427 Aug 31 21:14 a_2018-08-31-211423_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55548 Aug 31 21:14 a_2018-08-31-211416-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34763 Aug 31 21:14 a_2018-08-31-211416_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9412 Aug 31 21:14 a_2018-08-31-211406_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9158 Aug 31 21:14 a_2018-08-31-211402_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9161 Aug 31 21:13 a_2018-08-31-211352_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9011 Aug 31 21:13 a_2018-08-31-211350_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9244 Aug 31 21:13 a_2018-08-31-211322_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58311 Aug 31 21:13 a_2018-08-31-211313_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59069 Aug 31 21:13 a_2018-08-31-211308_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59526 Aug 31 21:13 a_2018-08-31-211307-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60338 Aug 31 21:13 a_2018-08-31-211307_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10915 Aug 31 21:10 a_2018-08-31-211047_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9182 Aug 31 21:09 a_2018-08-31-210942_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9543 Aug 31 21:08 a_2018-08-31-210829_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9769 Aug 31 21:07 a_2018-08-31-210742-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9767 Aug 31 21:07 a_2018-08-31-210742_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9476 Aug 31 21:07 a_2018-08-31-210741_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:03bc3c60
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-31-210741_Traviss-Mac-1044.crash
Process:               a [34668]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [34666]
Responsible:           a [34668]
User ID:               501
Date/Time:             2018-08-31 21:07:19.739 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000879de abort_on_c_abi::panic_in_ffi::h305168285df224eb + 46
1   a                              0x00086e2b std::panicking::try::do_call::h325fa2095685ded4 (.llvm.10369727252715004473) + 11
2   libstd-312c1b89c01a0dbc.dylib  0x00228b5d __rust_maybe_catch_panic + 29
3   a                              0x00087c45 abort_on_c_abi::main::h82c0574e6cdd7b93 + 613
4   a                              0x000862ab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h55ad69be6c72a0c7 + 11
5   libstd-312c1b89c01a0dbc.dylib  0x00217d47 std::panicking::try::do_call::h9314641171ec181d (.llvm.16980015207478962662) + 23
6   libstd-312c1b89c01a0dbc.dylib  0x00228b5d __rust_maybe_catch_panic + 29
7   libstd-312c1b89c01a0dbc.dylib  0x00217cbe std::panicking::try::h808ee7bbea2f45e8 + 62
8   libstd-312c1b89c01a0dbc.dylib  0x001ff5f5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
9   a                              0x00087f7c main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00815060  ebx: 0xbff7a628  ecx: 0x00000000  edx: 0x00000000
  edi: 0x00228b4e  esi: 0x00000000  ebp: 0xbff7a5c8  esp: 0xbff7a5b0
   ss: 0x00000023  efl: 0x00010292  eip: 0x000879de   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x002a04b4
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x85000 -    0x88ff3 +a (0) <6036C8D1-980D-3EEC-813A-06082FFE0357> /Users/USER/*/a
  0x15e000 -   0x1a3fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1e7000 -   0x2b7fcf +libstd-312c1b89c01a0dbc.dylib (0) <4F44522F-249F-3EBA-9BBA-BC2433800A6E> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
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
    task_for_pid: 2633
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=75.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3332K       45 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9604K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            572.7M      139 
TOTAL                            572.7M      139 
TOTAL, minus reserved VM space   572.6M      139 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-31-210742-1_Traviss-Mac-1044.crash
Process:               a [35332]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [35324]
Responsible:           a [35332]
User ID:               501
Date/Time:             2018-08-31 21:07:42.851 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-312c1b89c01a0dbc.dylib  0x001f3226 std::panicking::rust_panic_with_hook::h0589b757ac5dff86 + 118
1   a                              0x000d5b5f std::panicking::begin_panic::h1dfd76ce48beac77 + 47 (panicking.rs:411)
2   a                              0x000d35e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hee8ed3175b25d327 + 36 (backtrace.rs:34)
3   a                              0x000d2fcb core::ptr::drop_in_place::h39ab9c7eb9f9aa98 + 11
4   a                              0x000d35b3 backtrace::double::h3a79da7ae181846f + 51
5   a                              0x000d48c2 backtrace::main::h9751c5dfa7559d82 + 4562 (backtrace.rs:113)
6   a                              0x000d2acb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha211c2951cb49c7d + 11 (rt.rs:74)
7   libstd-312c1b89c01a0dbc.dylib  0x001f2d47 std::panicking::try::do_call::h9314641171ec181d (.llvm.16980015207478962662) + 23
8   libstd-312c1b89c01a0dbc.dylib  0x00203b5d __rust_maybe_catch_panic + 29
9   libstd-312c1b89c01a0dbc.dylib  0x001f2cbe std::panicking::try::h808ee7bbea2f45e8 + 62
10  libstd-312c1b89c01a0dbc.dylib  0x001da5f5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
11  a                              0x000d514c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x001f31be  ecx: 0x00000000  edx: 0xa7702ec6
  edi: 0x0029a100  esi: 0x002619d0  ebp: 0xbff30428  esp: 0xbff303c0
   ss: 0x00000023  efl: 0x00010286  eip: 0x001f3226   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0023a4a0
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xcf000 -    0xd6ffb +a (0) <398C9AF1-C3EA-3D15-879E-DC451AF69899> /Users/USER/*/a
  0x139000 -   0x17efdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1c2000 -   0x292fcf +libstd-312c1b89c01a0dbc.dylib (0) <4F44522F-249F-3EBA-9BBA-BC2433800A6E> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
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
    task_for_pid: 2633
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=76.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       3988K        5 
VM_ALLOCATE                       3988K        5 
VM_ALLOCATE (reserved)             240K        3         reserved VM address space (unallocated)
__DATA                            3332K       45 
__LINKEDIT                        74.2M        5 
__OBJC                              36K        6 
__TEXT                            9620K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            572.7M      141 
TOTAL                            572.7M      141 
TOTAL, minus reserved VM space   572.5M      141 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-31-210742_Traviss-Mac-1044.crash
Process:               a [35331]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [35324]
Responsible:           a [35331]
User ID:               501
Date/Time:             2018-08-31 21:07:42.832 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-312c1b89c01a0dbc.dylib  0x00284226 std::panicking::rust_panic_with_hook::h0589b757ac5dff86 + 118
1   a                              0x000c0b5f std::panicking::begin_panic::h1dfd76ce48beac77 + 47 (panicking.rs:411)
2   a                              0x000be5e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hee8ed3175b25d327 + 36 (backtrace.rs:34)
3   a                              0x000bdfcb core::ptr::drop_in_place::h39ab9c7eb9f9aa98 + 11
4   a                              0x000be5b3 backtrace::double::h3a79da7ae181846f + 51
5   a                              0x000bf8c2 backtrace::main::h9751c5dfa7559d82 + 4562 (backtrace.rs:113)
6   a                              0x000bdacb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha211c2951cb49c7d + 11 (rt.rs:74)
7   libstd-312c1b89c01a0dbc.dylib  0x00283d47 std::panicking::try::do_call::h9314641171ec181d (.llvm.16980015207478962662) + 23
8   libstd-312c1b89c01a0dbc.dylib  0x00294b5d __rust_maybe_catch_panic + 29
9   libstd-312c1b89c01a0dbc.dylib  0x00283cbe std::panicking::try::h808ee7bbea2f45e8 + 62
10  libstd-312c1b89c01a0dbc.dylib  0x0026b5f5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
11  a                              0x000c014c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x002841be  ecx: 0x00000000  edx: 0xa7702ec6
  edi: 0x0032b100  esi: 0x002f29d0  ebp: 0xbff45438  esp: 0xbff453d0
   ss: 0x00000023  efl: 0x00010282  eip: 0x00284226   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00554bd7
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xba000 -    0xc1ffb +a (0) <398C9AF1-C3EA-3D15-879E-DC451AF69899> /Users/USER/*/a
  0x1ca000 -   0x20ffdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x253000 -   0x323fcf +libstd-312c1b89c01a0dbc.dylib (0) <4F44522F-249F-3EBA-9BBA-BC2433800A6E> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
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
    task_for_pid: 2633
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=76.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3332K       45 
__LINKEDIT                        74.2M        5 
__OBJC                              36K        6 
__TEXT                            9620K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            572.7M      139 
TOTAL                            572.7M      139 
TOTAL, minus reserved VM space   572.6M      139 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-31-210829_Traviss-Mac-1044.crash
Process:               a [36732]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [36731]
Responsible:           a [36732]
User ID:               501
Date/Time:             2018-08-31 21:08:28.823 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4000 seconds
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
3   libstd-312c1b89c01a0dbc.dylib  0x00143dcb std::sys::unix::abort_internal::h88a26145a8b97680 + 11
4   libstd-312c1b89c01a0dbc.dylib  0x00130ad0 rust_oom + 48
5   libstd-312c1b89c01a0dbc.dylib  0x0019e774 alloc::alloc::handle_alloc_error::h531aecfe0c23f91b + 20
6   a                              0x00084648 default_alloc_error_hook::main::hbf3cf79eecbb97ff + 808
7   a                              0x0008370b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h8e8fd9e60f90c41f + 11
8   libstd-312c1b89c01a0dbc.dylib  0x00159d47 std::panicking::try::do_call::h9314641171ec181d (.llvm.16980015207478962662) + 23
9   libstd-312c1b89c01a0dbc.dylib  0x0016ab5d __rust_maybe_catch_panic + 29
10  libstd-312c1b89c01a0dbc.dylib  0x00159cbe std::panicking::try::h808ee7bbea2f45e8 + 62
11  libstd-312c1b89c01a0dbc.dylib  0x001415f5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
12  a                              0x000847bc main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff7c54c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff7c578  esp: 0xbff7c54c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x83000 -    0x84ff3 +a (0) <575DAC98-5F1B-356E-B067-5699FA42904F> /Users/USER/*/a
   0xa0000 -    0xe5fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x129000 -   0x1f9fcf +libstd-312c1b89c01a0dbc.dylib (0) <4F44522F-249F-3EBA-9BBA-BC2433800A6E> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
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
TOTAL                            572.7M      138 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-31-211313_Traviss-Mac-1044.crash
Process:               a [44208]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [44199]
Responsible:           a [44208]
User ID:               501
Date/Time:             2018-08-31 21:13:13.251 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4300 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbff1ffc
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbff1ffc:
    Stack Guard            00000000bbff0000-00000000bbff1000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbff1000-00000000bbff2000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbff2000-00000000bfff0000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-312c1b89c01a0dbc.dylib  0x0014ddcb std::sys::unix::abort_internal::h88a26145a8b97680 + 11
4   libstd-312c1b89c01a0dbc.dylib  0x0015e3d8 std::sys_common::util::abort::heae8a1f2fd58dc08 + 88
5   libstd-312c1b89c01a0dbc.dylib  0x00169611 std::sys::unix::stack_overflow::imp::signal_handler::hf542bf7dd946dc68 (.llvm.1389561392850443773) + 161
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-312c1b89c01a0dbc.dylib  0x00169570 std::sys::unix::os::exit::hce08a6fea2927a01 + 32
9   libstd-312c1b89c01a0dbc.dylib  0x0014f361 _$LT$std..io..buffered..BufWriter$LT$W$GT$$u20$as$u20$std..io..Write$GT$::write::hb8cab59c955cfed8 + 81
10  libstd-312c1b89c01a0dbc.dylib  0x0014f56e _$LT$std..io..buffered..LineWriter$LT$W$GT$$u20$as$u20$std..io..Write$GT$::write::hd1a65959ad23873a + 142
11  libstd-312c1b89c01a0dbc.dylib  0x00154ebf std::io::Write::write_all::hd4cd3ecf69c3361b + 95
12  libstd-312c1b89c01a0dbc.dylib  0x0015022b _$LT$std..io..Write..write_fmt..Adaptor$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..fmt..Write$GT$::write_str::hd0f3babb31e5b338 + 43
13  libstd-312c1b89c01a0dbc.dylib  0x001bdfb8 core::fmt::write::hed81c18d904637cb + 712
14  libstd-312c1b89c01a0dbc.dylib  0x00153bac _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::h6489d01deafd12af + 188
15  libstd-312c1b89c01a0dbc.dylib  0x0015930a _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::try_with::h227e4493cdf0e770 + 122
16  libstd-312c1b89c01a0dbc.dylib  0x0015493b std::io::stdio::_print::h3a6237a539b946da + 107
17  a                              0x0001295f out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 63
18  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
19  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
20  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
21  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
22  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
23  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
24  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
25  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
26  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
27  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
28  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
29  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
30  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
31  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
32  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
33  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
34  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
35  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
36  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
37  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
38  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
39  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
40  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
41  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
42  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
43  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
44  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
45  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
46  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
47  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
48  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
49  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
50  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
51  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
52  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
53  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
54  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
55  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
56  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
57  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
58  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
59  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
60  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
61  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
62  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
63  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
64  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
65  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
66  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
67  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
68  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
69  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
70  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
71  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
72  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
73  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
74  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
75  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
76  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
77  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
78  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
79  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
80  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
81  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
82  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
83  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
84  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
85  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
86  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
87  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
88  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
89  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
90  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
91  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
92  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
93  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
94  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
95  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
96  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
97  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
98  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
99  a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
100 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
101 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
102 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
103 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
104 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
105 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
106 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
107 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
108 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
109 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
110 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
111 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
112 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
113 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
114 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
115 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
116 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
117 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
118 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
119 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
120 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
121 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
122 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
123 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
124 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
125 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
126 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
127 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
128 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
129 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
130 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
131 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
132 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
133 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
134 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
135 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
136 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
137 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
138 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
139 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
140 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
141 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
142 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
143 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
144 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
145 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
146 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
147 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
148 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
149 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
150 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
151 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
152 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
153 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
154 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
155 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
156 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
157 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
158 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
159 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
160 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
161 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
162 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
163 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
164 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
165 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
166 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
167 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
168 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
169 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
170 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
171 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
172 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
173 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
174 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
175 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
176 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
177 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
178 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
179 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
180 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
181 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
182 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
183 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
184 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
185 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
186 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
187 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
188 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
189 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
190 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
191 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
192 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
193 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
194 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
195 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
196 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
197 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
198 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
199 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
200 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
201 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
202 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
203 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
204 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
205 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
206 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
207 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
208 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
209 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
210 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
211 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
212 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
213 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
214 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
215 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
216 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
217 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
218 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
219 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
220 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
221 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
222 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
223 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
224 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
225 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
226 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
227 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
228 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
229 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
230 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
231 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
232 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
233 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
234 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
235 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
236 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
237 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
238 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
239 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
240 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
241 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
242 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
243 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
244 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
245 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
246 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
247 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
248 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
249 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
250 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
251 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
252 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
253 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
254 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
255 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
256 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
257 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
258 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
259 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
260 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
261 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
262 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
263 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
264 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
265 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
266 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
267 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
268 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
269 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
270 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
271 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
272 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
273 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
274 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
275 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
276 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
277 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
278 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
279 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
280 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
281 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
282 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
283 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
284 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
285 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
286 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
287 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
288 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
289 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
290 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
291 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
292 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
293 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
294 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
295 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
296 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
297 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
298 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
299 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
300 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
301 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
302 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
303 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
304 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
305 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
306 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
307 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
308 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
309 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
310 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
311 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
312 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
313 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
314 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
315 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
316 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
317 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
318 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
319 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
320 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
321 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
322 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
323 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
324 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
325 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
326 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
327 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
328 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
329 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
330 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
331 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
332 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
333 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
334 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
335 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
336 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
337 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
338 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
339 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
340 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
341 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
342 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
343 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
344 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
345 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
346 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
347 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
348 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
349 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
350 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
351 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
352 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
353 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
354 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
355 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
356 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
357 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
358 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
359 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
360 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
361 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
362 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
363 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
364 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
365 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
366 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
367 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
368 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
369 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
370 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
371 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
372 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
373 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
374 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
375 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
376 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
377 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
378 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
379 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
380 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
381 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
382 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
383 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
384 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
385 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
386 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
387 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
388 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
389 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
390 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
391 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
392 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
393 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
394 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
395 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
396 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
397 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
398 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
399 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
400 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
401 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
402 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
403 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
404 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
405 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
406 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
407 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
408 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
409 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
410 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
411 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
412 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
413 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
414 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
415 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
416 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
417 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
418 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
419 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
420 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
421 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
422 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
423 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
424 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
425 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
426 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
427 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
428 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
429 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
430 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
431 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
432 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
433 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
434 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
435 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
436 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
437 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
438 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
439 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
440 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
441 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
442 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
443 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
444 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
445 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
446 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
447 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
448 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
449 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
450 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
451 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
452 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
453 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
454 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
455 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
456 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
457 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
458 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
459 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
460 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
461 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
462 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
463 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
464 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
465 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
466 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
467 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
468 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
469 a                              0x00012964 out_of_stack::loud_recurse::h8dc50be7b1b1be99 + 68
---
===========                     =======  ======= 
TOTAL                            572.7M      138 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-31-211322_Traviss-Mac-1044.crash
Process:               a [44445]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [44441]
Responsible:           a [44445]
User ID:               501
Date/Time:             2018-08-31 21:13:21.865 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4300 seconds
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
3   a                              0x000c028b panic_abort::__rust_start_panic::abort::h8e92c8e295aa6470 + 11
4   a                              0x000c027b __rust_start_panic + 11
5   a                              0x000be7e1 std::panicking::rust_panic_with_hook::h0589b757ac5dff86 + 1793
6   a                              0x000c4d2a std::panicking::begin_panic::h3adcacae23dc4118 + 42
7   a                              0x000b14bd lto_abort::main::h93d7449bf6778972 + 2893
8   a                              0x000c4e8b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h15424d6a3ebc0a85 + 11
9   a                              0x000b1f4c std::sys_common::backtrace::__rust_begin_short_backtrace::hefee785841cfcd1d + 12
10  a                              0x000b18b9 main + 1017
11  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff4f52c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff4f558  esp: 0xbff4f52c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xb0000 -    0xfefdf +a (0) <2FE99AAC-3ECA-3D30-87FE-0291E69B1838> /Users/USER/*/a
  0x150000 -   0x195fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2875
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=82.6M resident=0K(0%) swapped_out_or_unallocated=82.6M(100%)
Writable regions: Total=75.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            1344K       44 
__LINKEDIT                        73.7M        4 
__OBJC                              36K        6 
__TEXT                            9068K       43 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            569.8M      134 
TOTAL                            569.8M      134 
TOTAL, minus reserved VM space   569.7M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-31-211350_Traviss-Mac-1044.crash
Process:               a [45265]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [45260]
Responsible:           a [45265]
User ID:               501
Date/Time:             2018-08-31 21:13:50.810 +0000
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
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-312c1b89c01a0dbc.dylib  0x001badcb std::sys::unix::abort_internal::h88a26145a8b97680 + 11
4   libstd-312c1b89c01a0dbc.dylib  0x001cb3d8 std::sys_common::util::abort::heae8a1f2fd58dc08 + 88
5   libstd-312c1b89c01a0dbc.dylib  0x001d14d8 rust_panic.llvm.16980015207478962662 + 104
6   libstd-312c1b89c01a0dbc.dylib  0x001d1399 std::panicking::rust_panic_with_hook::h0589b757ac5dff86 + 489
7   a                              0x000209bf std::panicking::begin_panic::he072a440972f9cca + 47
8   a                              0x00021c1c main + 2668
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffe057c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffe05a8  esp: 0xbffe057c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x1f000 -    0x22ff7 +a (0) <32F4E304-7662-3DF0-88EA-D8638B924246> /Users/USER/*/a
  0x117000 -   0x15cfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1a0000 -   0x270fcf +libstd-312c1b89c01a0dbc.dylib (0) <4F44522F-249F-3EBA-9BBA-BC2433800A6E> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
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
    task_for_pid: 2875
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=75.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4096K        3 
__DATA                            3332K       45 
__LINKEDIT                        74.1M        5 
---
===========                     =======  ======= 
TOTAL                            572.6M      137 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-31-211352_Traviss-Mac-1044.crash
Process:               a [45291]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [45288]
Responsible:           a [45291]
User ID:               501
Date/Time:             2018-08-31 21:13:51.785 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4400 seconds
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
    __TEXT                 00000000000e2000-00000000000e5000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000e3f14 segfault_no_out_of_stack::main::h4bc020f9e0c050bb + 2132
1   a                              0x000e26fb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd045dd389f3521ab + 11
2   libstd-312c1b89c01a0dbc.dylib  0x00249d47 std::panicking::try::do_call::h9314641171ec181d (.llvm.16980015207478962662) + 23
3   libstd-312c1b89c01a0dbc.dylib  0x0025ab5d __rust_maybe_catch_panic + 29
4   libstd-312c1b89c01a0dbc.dylib  0x00249cbe std::panicking::try::h808ee7bbea2f45e8 + 62
5   libstd-312c1b89c01a0dbc.dylib  0x002315f5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
6   a                              0x000e41ec main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x00000000
  edi: 0x00818040  esi: 0xbff1d668  ebp: 0xbff1d738  esp: 0xbff1d590
   ss: 0x00000023  efl: 0x00010246  eip: 0x000e3f14   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     0
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xe2000 -    0xe4ff3 +a (0) <F86F9C9C-D63F-30F2-B3B4-1B0EB8025DDC> /Users/USER/*/a
  0x190000 -   0x1d5fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x219000 -   0x2e9fcf +libstd-312c1b89c01a0dbc.dylib (0) <4F44522F-249F-3EBA-9BBA-BC2433800A6E> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
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
    task_for_pid: 2875
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=75.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4228K        5 
__DATA                            3332K       45 
__LINKEDIT                        74.1M        5 
---
===========                     =======  ======= 
TOTAL                            572.7M      139 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-31-211402_Traviss-Mac-1044.crash
Process:               a [45445]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [45444]
Responsible:           a [45445]
User ID:               501
Date/Time:             2018-08-31 21:14:01.538 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4400 seconds
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
    __TEXT                 00000000000e1000-00000000000e4000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000e3594 signal_exit_status::main::h5e838d36792e8f94 + 436
1   a                              0x000e226b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h4aa733f82e7c1c80 + 11
2   libstd-312c1b89c01a0dbc.dylib  0x00213d47 std::panicking::try::do_call::h9314641171ec181d (.llvm.16980015207478962662) + 23
3   libstd-312c1b89c01a0dbc.dylib  0x00224b5d __rust_maybe_catch_panic + 29
4   libstd-312c1b89c01a0dbc.dylib  0x00213cbe std::panicking::try::h808ee7bbea2f45e8 + 62
5   libstd-312c1b89c01a0dbc.dylib  0x001fb5f5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
6   a                              0x000e366c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x00818020
  edi: 0x00818040  esi: 0xbff1e6b0  ebp: 0xbff1e748  esp: 0xbff1e630
   ss: 0x00000023  efl: 0x00010246  eip: 0x000e3594   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     0
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xe1000 -    0xe3ff7 +a (0) <460E43FE-8874-3081-9280-68B37266F0AD> /Users/USER/*/a
  0x15a000 -   0x19ffdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1e3000 -   0x2b3fcf +libstd-312c1b89c01a0dbc.dylib (0) <4F44522F-249F-3EBA-9BBA-BC2433800A6E> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
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
    task_for_pid: 2875
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=75.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4228K        5 
__DATA                            3332K       45 
__LINKEDIT                        74.1M        5 
---
===========                     =======  ======= 
TOTAL                            572.7M      139 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-31-211406_Traviss-Mac-1044.crash
Process:               a [45523]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [45516]
Responsible:           a [45523]
User ID:               501
Date/Time:             2018-08-31 21:14:06.200 +0000
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
0   a                              0x00045710 simd_target_feature_mixup::test::id_avx512_512::h96f843e8b3f4d2b1 + 112
1   a                              0x00044468 simd_target_feature_mixup::test::main::h1209a6983816d331 + 1848
2   a                              0x00046999 simd_target_feature_mixup::main::h9cd2e2449b07bb04 + 937
3   a                              0x00046bab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h398bc92ea1b82c97 + 11
4   libstd-312c1b89c01a0dbc.dylib  0x0019bd47 std::panicking::try::do_call::h9314641171ec181d (.llvm.16980015207478962662) + 23
5   libstd-312c1b89c01a0dbc.dylib  0x001acb5d __rust_maybe_catch_panic + 29
6   libstd-312c1b89c01a0dbc.dylib  0x0019bcbe std::panicking::try::h808ee7bbea2f45e8 + 62
7   libstd-312c1b89c01a0dbc.dylib  0x001835f5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
8   a                              0x00046b8c main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x000456ac  ebx: 0xbffbc2c0  ecx: 0xbffbc3c0  edx: 0xbffbc2c0
  edi: 0x00043d44  esi: 0x00000000  ebp: 0xbffbc2b8  esp: 0xbffbc280
   ss: 0x00000023  efl: 0x00010246  eip: 0x00045710   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000452f0
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x43000 -    0x47fc7 +a (0) <BA702CB3-AD7E-347B-907A-389B822F8720> /Users/USER/*/a
   0xe2000 -   0x127fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x16b000 -   0x23bfcf +libstd-312c1b89c01a0dbc.dylib (0) <4F44522F-249F-3EBA-9BBA-BC2433800A6E> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
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
    task_for_pid: 2875
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=75.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3332K       45 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9608K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            572.7M      139 
TOTAL                            572.7M      139 
TOTAL, minus reserved VM space   572.6M      139 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-31-211416-1_Traviss-Mac-1044.crash
Process:               a [45752]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [45751]
Responsible:           a [45752]
User ID:               501
Date/Time:             2018-08-31 21:14:15.750 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf33748
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf33748:
    Stack Guard            00000000bbf32000-00000000bbf33000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf33000-00000000bbf34000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf34000-00000000bff32000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-312c1b89c01a0dbc.dylib  0x00188dcb std::sys::unix::abort_internal::h88a26145a8b97680 + 11
4   libstd-312c1b89c01a0dbc.dylib  0x001993d8 std::sys_common::util::abort::heae8a1f2fd58dc08 + 88
5   libstd-312c1b89c01a0dbc.dylib  0x001a4611 std::sys::unix::stack_overflow::imp::signal_handler::hf542bf7dd946dc68 (.llvm.1389561392850443773) + 161
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-312c1b89c01a0dbc.dylib  0x001a4570 std::sys::unix::os::exit::hce08a6fea2927a01 + 32
9   a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
10  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
11  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
12  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
13  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
14  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
15  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
16  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
17  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
18  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
19  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
20  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
21  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
22  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
23  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
24  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
25  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
26  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
27  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
28  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
29  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
30  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
31  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
32  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
33  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
34  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
35  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
36  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
37  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
38  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
39  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
40  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
41  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
42  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
43  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
44  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
45  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
46  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
47  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
48  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
49  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
50  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
51  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
52  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
53  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
54  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
55  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
56  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
57  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
58  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
59  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
60  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
61  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
62  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
63  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
64  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
65  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
66  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
67  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
68  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
69  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
70  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
71  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
72  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
73  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
74  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
75  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
76  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
77  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
78  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
79  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
80  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
81  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
82  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
83  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
84  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
85  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
86  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
87  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
88  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
89  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
90  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
91  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
92  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
93  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
94  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
95  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
96  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
97  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
98  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
99  a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
100 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
101 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
102 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
103 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
104 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
105 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
106 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
107 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
108 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
109 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
110 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
111 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
112 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
113 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
114 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
115 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
116 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
117 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
118 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
119 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
120 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
121 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
122 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
123 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
124 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
125 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
126 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
127 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
128 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
129 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
130 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
131 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
132 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
133 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
134 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
135 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
136 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
137 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
138 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
139 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
140 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
141 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
142 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
143 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
144 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
145 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
146 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
147 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
148 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
149 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
150 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
151 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
152 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
153 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
154 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
155 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
156 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
157 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
158 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
159 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
160 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
161 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
162 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
163 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
164 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
165 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
166 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
167 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
168 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
169 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
170 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
171 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
172 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
173 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
174 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
175 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
176 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
177 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
178 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
179 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
180 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
181 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
182 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
183 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
184 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
185 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
186 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
187 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
188 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
189 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
190 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
191 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
192 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
193 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
194 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
195 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
196 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
197 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
198 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
199 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
200 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
201 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
202 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
203 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
204 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
205 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
206 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
207 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
208 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
209 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
210 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
211 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
212 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
213 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
214 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
215 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
216 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
217 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
218 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
219 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
220 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
221 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
222 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
223 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
224 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
225 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
226 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
227 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
228 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
229 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
230 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
231 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
232 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
233 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
234 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
235 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
236 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
237 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
238 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
239 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
240 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
241 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
242 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
243 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
244 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
245 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
246 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
247 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
248 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
249 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
250 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
251 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
252 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
253 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
254 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
255 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
256 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
257 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
258 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
259 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
260 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
261 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
262 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
263 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
264 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
265 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
266 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
267 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
268 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
269 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
270 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
271 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
272 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
273 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
274 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
275 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
276 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
277 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
278 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
279 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
280 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
281 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
282 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
283 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
284 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
285 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
286 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
287 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
288 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
289 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
290 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
291 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
292 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
293 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
294 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
295 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
296 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
297 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
298 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
299 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
300 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
301 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
302 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
303 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
304 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
305 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
306 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
307 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
308 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
309 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
310 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
311 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
312 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
313 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
314 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
315 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
316 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
317 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
318 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
319 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
320 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
321 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
322 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
323 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
324 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
325 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
326 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
327 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
328 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
329 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
330 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
331 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
332 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
333 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
334 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
335 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
336 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
337 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
338 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
339 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
340 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
341 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
342 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
343 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
344 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
345 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
346 a                              0x000d1880 stack_probes::recurse::h5b19fb02dc377c0b + 48
---
===========                     =======  ======= 
TOTAL                            572.7M      138 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-31-211416_Traviss-Mac-1044.crash
Process:               a [45754]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [45751]
Responsible:           a [45754]
User ID:               501
Date/Time:             2018-08-31 21:14:15.774 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4400 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b05f1e98
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb05f1e98:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b05f1000-00000000b05f2000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b05f2000-00000000b07f3000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-312c1b89c01a0dbc.dylib  0x000e53f0 std::sys::unix::thread::Thread::join::hd8ab4e4bc9ad521c + 32
4   a                              0x0003b7a6 _$LT$std..thread..JoinHandle$LT$T$GT$$GT$::join::hafc79c397df6dd89 + 70
5   a                              0x0003a735 stack_probes::main::h521fd0a37334d32c + 597
6   a                              0x0003938b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h1ad31c5e175b52d3 + 11
7   libstd-312c1b89c01a0dbc.dylib  0x0010dd47 std::panicking::try::do_call::h9314641171ec181d (.llvm.16980015207478962662) + 23
8   libstd-312c1b89c01a0dbc.dylib  0x0011eb5d __rust_maybe_catch_panic + 29
9   libstd-312c1b89c01a0dbc.dylib  0x0010dcbe std::panicking::try::h808ee7bbea2f45e8 + 62
10  libstd-312c1b89c01a0dbc.dylib  0x000f55f5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
11  a                              0x0003b1ac main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-312c1b89c01a0dbc.dylib  0x000f7dcb std::sys::unix::abort_internal::h88a26145a8b97680 + 11
4   libstd-312c1b89c01a0dbc.dylib  0x001083d8 std::sys_common::util::abort::heae8a1f2fd58dc08 + 88
5   libstd-312c1b89c01a0dbc.dylib  0x00113611 std::sys::unix::stack_overflow::imp::signal_handler::hf542bf7dd946dc68 (.llvm.1389561392850443773) + 161
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-312c1b89c01a0dbc.dylib  0x00113570 std::sys::unix::os::exit::hce08a6fea2927a01 + 32
9   a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
10  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
11  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
12  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
13  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
14  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
15  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
16  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
17  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
18  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
19  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
20  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
21  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
22  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
23  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
24  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
25  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
26  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
27  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
28  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
29  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
30  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
31  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
32  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
33  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
34  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
35  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
36  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
37  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
38  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
39  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
40  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
41  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
42  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
43  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
44  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
45  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
46  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
47  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
48  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
49  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
50  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
51  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
52  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
53  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
54  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
55  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
56  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
57  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
58  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
59  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
60  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
61  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
62  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
63  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
64  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
65  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
66  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
67  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
68  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
69  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
70  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
71  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
72  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
73  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
74  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
75  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
76  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
77  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
78  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
79  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
80  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
81  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
82  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
83  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
84  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
85  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
86  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
87  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
88  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
89  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
90  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
91  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
92  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
93  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
94  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
95  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
96  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
97  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
98  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
99  a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
100 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
101 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
102 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
103 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
104 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
105 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
106 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
107 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
108 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
109 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
110 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
111 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
112 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
113 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
114 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
115 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
116 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
117 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
118 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
119 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
120 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
121 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
122 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
123 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
124 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
125 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
126 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
127 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
128 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
129 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
130 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
131 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
132 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
133 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
134 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
135 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
136 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
137 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
138 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
139 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
140 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
141 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
142 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
143 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
144 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
145 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
146 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
147 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
148 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
149 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
150 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
151 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
152 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
153 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
154 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
155 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
156 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
157 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
158 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
159 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
160 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
161 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
162 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
163 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
164 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
165 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
166 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
167 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
168 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
169 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
170 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
171 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
172 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
173 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
174 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
175 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
176 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
177 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
178 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
179 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
180 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
181 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
182 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
183 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
184 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
185 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
186 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
187 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
188 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
189 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
190 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
191 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
192 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
193 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
194 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
195 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
196 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
197 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
198 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
199 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
200 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
201 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
202 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
203 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
204 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
205 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
206 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
207 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
208 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
209 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
210 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
211 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
212 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
213 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
214 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
215 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
216 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
217 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
218 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
219 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
220 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
221 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
222 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
223 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
224 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
225 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
226 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
227 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
228 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
229 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
230 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
231 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
232 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
233 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
234 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
235 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
236 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
237 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
238 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
239 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
240 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
241 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
242 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
243 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
244 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
245 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
246 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
247 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
248 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
249 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
250 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
251 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
252 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
253 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
254 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
255 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
256 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
257 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
258 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
259 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
260 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
261 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
262 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
263 a                              0x0003a880 stack_probes::recurse::h5b19fb02dc377c0b + 48
264 a                              0x0003941d std::sys_common::backtrace::__rust_begin_short_backtrace::h75043aa63ec0db1c + 29
265 libstd-312c1b89c01a0dbc.dylib  0x0011eb5d __rust_maybe_catch_panic + 29
266 a                              0x0003ba83 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h81a86fd07c13df25 + 131
267 libstd-312c1b89c01a0dbc.dylib  0x0011273b std::sys_common::thread::start_thread::h52dad84f78503b62 + 187
268 libstd-312c1b89c01a0dbc.dylib  0x000e5281 std::sys::unix::thread::Thread::new::thread_start::h7a084032ca4cb5ba + 17
269 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
270 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
271 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb07f2000  ecx: 0x0045eb4c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x0045eb78  esp: 0x0045eb4c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000e5000
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0x38000 -    0x3cff7 +a (0) <9AFF1783-39E3-3660-8800-666E833E527E> /Users/USER/*/a
   0x54000 -    0x99fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
   0xdd000 -   0x1adfcf +libstd-312c1b89c01a0dbc.dylib (0) <4F44522F-249F-3EBA-9BBA-BC2433800A6E> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
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
    task_for_pid: 2875
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=80.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=80.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            12.0M       11 
MALLOC guard page                   16K        5 
Stack Guard                          8K        3 
VM_ALLOCATE                       6276K        7 
VM_ALLOCATE                       6276K        7 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3332K       45 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9608K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            579.8M      145 
TOTAL                            579.8M      145 
TOTAL, minus reserved VM space   579.7M      145 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-31-211423-1_Traviss-Mac-1044.crash
Process:               a [45882]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [45879]
Responsible:           a [45882]
User ID:               501
Date/Time:             2018-08-31 21:14:21.793 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4400 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0097ef8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0097ef8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0097000-00000000b0098000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0098000-00000000b0299000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   a                              0x0005aee5 stack_probes_lto::main::h46e0341d7865a006 + 2709
4   a                              0x0007490b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc85022b643f5e2ea + 11
5   a                              0x0005ce9c std::sys_common::backtrace::__rust_begin_short_backtrace::hefee785841cfcd1d + 12
6   a                              0x0005c206 main + 582
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000662eb std::sys::unix::abort_internal::h88a26145a8b97680 + 11
4   a                              0x00069e62 std::sys_common::util::abort::heae8a1f2fd58dc08 + 82
5   a                              0x0006e761 std::sys::unix::stack_overflow::imp::signal_handler::hf542bf7dd946dc68 (.llvm.1389561392850443773) + 593
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x0006e510 std::sys::unix::os::error_string::h385ab9f8bc14b0c6 + 240
9   a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
10  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
11  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
12  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
13  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
14  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
15  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
16  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
17  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
18  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
19  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
20  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
21  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
22  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
23  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
24  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
25  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
26  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
27  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
28  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
29  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
30  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
31  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
32  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
33  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
34  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
35  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
36  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
37  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
38  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
39  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
40  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
41  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
42  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
43  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
44  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
45  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
46  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
47  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
48  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
49  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
50  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
51  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
52  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
53  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
54  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
55  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
56  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
57  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
58  a                              0x0005b408 stack_probes_lto::recurse::h6895313952b89569 + 40
