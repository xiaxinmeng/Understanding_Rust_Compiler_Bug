plain
[00:03:00]       Memory: 8 GB
[00:03:00]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:00]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:00]       SMC Version (system): 2.8f0
[00:03:00]       Serial Number (system): VMO2zAasTE/n
[00:03:00] 
[00:03:00] hw.ncpu: 4
[00:03:00] hw.byteorder: 1234
[00:03:00] hw.memsize: 8589934592
---
[02:10:20]   = note: #[warn(deprecated)] on by default
[02:10:20] 
[02:10:20] error: linking with `cc` failed: signal: 4
[02:10:20]   |
[02:10:20]   = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestqDGBO8/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestqDGBO8/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestqDGBO8/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestqDGBO8/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestqDGBO8/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestqDGBO8/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestqDGBO8/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libstd-a818a4b7490f1bef.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libpanic_unwind-782be5cba0a53fa8.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libbacktrace_sys-1405c679deb4b710.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_demangle-022938c840899ee8.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libunwind-1ecb62ae80798d9b.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liblibc-e1bf62a52accec8f.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liballoc-5cc329465c73523e.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_core-635c35a1095754d1.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcore-a9d318fb39582bdb.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-78e8331284e7e75f.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:10:20] 
[02:10:20] error: aborting due to previous error
[02:10:20] 
[02:10:20] thread '/Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/listing/warn-by-default.md - Warn_by_default_lints::deprecated (line 50)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[02:10:20] 
[02:10:20] 
[02:10:20] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:10:20] Build completed unsuccessfully in 0:52:50
[02:10:20] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17b5cf59
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 24 12:33:45 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:0ec9c694
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1176
-rw-------@  1 travis  staff   1387 Mar 24 12:33 foo_2019-03-24-123303_Traviss-Mac-1044.crash
drwx------  26 travis  staff    884 Mar 24 12:33 .
-rw-------@  1 travis  staff   1376 Mar 24 12:32 m4_2019-03-24-123233_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9899 Mar 24 12:32 b_2019-03-24-123225_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1403 Mar 24 12:32 bar_2019-03-24-123225-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1391 Mar 24 12:32 bar_2019-03-24-123225_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  57366 Mar 24 11:56 a_2019-03-24-115646-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34790 Mar 24 11:56 a_2019-03-24-115646_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55585 Mar 24 11:56 a_2019-03-24-115638-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34717 Mar 24 11:56 a_2019-03-24-115638_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 Mar 24 11:56 a_2019-03-24-115633_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9166 Mar 24 11:56 a_2019-03-24-115627_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9171 Mar 24 11:56 a_2019-03-24-115621-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8936 Mar 24 11:56 a_2019-03-24-115621_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9304 Mar 24 11:55 a_2019-03-24-115541_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58238 Mar 24 11:55 a_2019-03-24-115532_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 Mar 24 11:55 a_2019-03-24-115527-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60372 Mar 24 11:55 a_2019-03-24-115527-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59503 Mar 24 11:55 a_2019-03-24-115527_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10881 Mar 24 11:53 a_2019-03-24-115321_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 Mar 24 11:52 a_2019-03-24-115221_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 Mar 24 11:51 a_2019-03-24-115105_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 Mar 24 11:50 a_2019-03-24-115010-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 Mar 24 11:50 a_2019-03-24-115010_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9483 Mar 24 11:50 a_2019-03-24-115000_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0063e778
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115000_Traviss-Mac-1044.crash
Process:               a [37382]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [37381]
Responsible:           a [37382]
User ID:               501
Date/Time:             2019-03-24 11:49:35.201 +0000
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
0   a                              0x000b2b0e abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 46
1   a                              0x000b1f5b std::panicking::try::do_call::h70a3c61b79532463 (.llvm.8375664522836110742) + 11
2   libstd-a818a4b7490f1bef.dylib  0x00254a4d __rust_maybe_catch_panic + 29
3   a                              0x000b2d75 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 613
4   a                              0x000b15bb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb3f0cb28b66a3895 + 11
5   libstd-a818a4b7490f1bef.dylib  0x002437fc std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
6   libstd-a818a4b7490f1bef.dylib  0x00245ef4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
7   libstd-a818a4b7490f1bef.dylib  0x00254a4d __rust_maybe_catch_panic + 29
8   libstd-a818a4b7490f1bef.dylib  0x00246997 std::rt::lang_start_internal::h94f504c75da6b468 + 631
9   a                              0x000b30ac main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x7bf79cc0  ebx: 0xbff4e868  ecx: 0x00000000  edx: 0x00000000
  edi: 0x00254a3e  esi: 0x00000000  ebp: 0xbff4e808  esp: 0xbff4e7f0
   ss: 0x00000023  efl: 0x00010296  eip: 0x000b2b0e   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0028e120
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xb0000 -    0xb3ffb +a (0) <5B96394E-A082-39DE-B651-1056B96041E0> /Users/USER/*/a
  0x19c000 -   0x1e1fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x225000 -   0x2b4fff +libstd-a818a4b7490f1bef.dylib (0) <AC017488-35D2-3C21-AB79-9C50A6F5675A> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2363
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
__DATA                            3572K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9344K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.6M      134 
TOTAL                            569.6M      134 
TOTAL, minus reserved VM space   569.4M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115010-1_Traviss-Mac-1044.crash
Process:               a [38197]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [38192]
Responsible:           a [38197]
User ID:               501
Date/Time:             2019-03-24 11:50:06.904 +0000
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
0   libstd-a818a4b7490f1bef.dylib  0x000ff403 std::panicking::rust_panic_with_hook::h8e0c35d23a6f7bfa + 115
1   a                              0x0002cbff std::panicking::begin_panic::h0d9f547d0675ac23 + 47 (panicking.rs:408)
2   a                              0x0002a6e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::h5db4af68aad48a7d + 36 (backtrace.rs:24)
3   a                              0x00029c7b core::ptr::real_drop_in_place::h0f2bd336b3b26fd2 + 11
4   a                              0x0002a6b3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x0002b9c8 backtrace::main::hcde7a1a1c3c85e77 + 4568 (backtrace.rs:103)
6   a                              0x00029bbb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h1ff5f8c969ea1487 + 11 (rt.rs:64)
7   libstd-a818a4b7490f1bef.dylib  0x000fc7fc std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
8   libstd-a818a4b7490f1bef.dylib  0x000feef4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
9   libstd-a818a4b7490f1bef.dylib  0x0010da4d __rust_maybe_catch_panic + 29
10  libstd-a818a4b7490f1bef.dylib  0x000ff997 std::rt::lang_start_internal::h94f504c75da6b468 + 631
11  a                              0x0002c21c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbffd75c8  ebx: 0xbffd7610  ecx: 0xbffd74c0  edx: 0xa7702ec6
  edi: 0x00147468  esi: 0x000ff39e  ebp: 0xbffd7668  esp: 0xbffd75e0
   ss: 0x00000023  efl: 0x00010286  eip: 0x000ff403   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x003cf214
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x27000 -    0x2dff7 +a (0) <98C0E6D3-6329-38FD-8ECC-86BCDFA3C2F6> /Users/USER/*/a
   0x55000 -    0x9afdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
   0xde000 -   0x16dfff +libstd-a818a4b7490f1bef.dylib (0) <AC017488-35D2-3C21-AB79-9C50A6F5675A> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2363
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=73.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9600K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3572K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115010_Traviss-Mac-1044.crash
Process:               a [38200]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [38192]
Responsible:           a [38200]
User ID:               501
Date/Time:             2019-03-24 11:50:07.027 +0000
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
0   libstd-a818a4b7490f1bef.dylib  0x001b9403 std::panicking::rust_panic_with_hook::h8e0c35d23a6f7bfa + 115
1   a                              0x0000ebff std::panicking::begin_panic::h0d9f547d0675ac23 + 47 (panicking.rs:408)
2   a                              0x0000c6e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::h5db4af68aad48a7d + 36 (backtrace.rs:24)
3   a                              0x0000bc7b core::ptr::real_drop_in_place::h0f2bd336b3b26fd2 + 11
4   a                              0x0000c6b3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x0000d9c8 backtrace::main::hcde7a1a1c3c85e77 + 4568 (backtrace.rs:103)
6   a                              0x0000bbbb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h1ff5f8c969ea1487 + 11 (rt.rs:64)
7   libstd-a818a4b7490f1bef.dylib  0x001b67fc std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
8   libstd-a818a4b7490f1bef.dylib  0x001b8ef4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
9   libstd-a818a4b7490f1bef.dylib  0x001c7a4d __rust_maybe_catch_panic + 29
10  libstd-a818a4b7490f1bef.dylib  0x001b9997 std::rt::lang_start_internal::h94f504c75da6b468 + 631
11  a                              0x0000e21c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbfff55b8  ebx: 0xbfff5600  ecx: 0xbfff54b0  edx: 0xa7702ec6
  edi: 0x00201468  esi: 0x001b939e  ebp: 0xbfff5658  esp: 0xbfff55d0
   ss: 0x00000023  efl: 0x00010282  eip: 0x001b9403   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x001cc700
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
    0x9000 -     0xfff7 +a (0) <98C0E6D3-6329-38FD-8ECC-86BCDFA3C2F6> /Users/USER/*/a
  0x10f000 -   0x154fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x198000 -   0x227fff +libstd-a818a4b7490f1bef.dylib (0) <AC017488-35D2-3C21-AB79-9C50A6F5675A> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2363
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=82.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=82.6M(100%)
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
__DATA                            3572K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9356K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            577.9M      135 
TOTAL                            577.9M      135 
TOTAL, minus reserved VM space   577.8M      135 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115105_Traviss-Mac-1044.crash
Process:               a [39865]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [39864]
Responsible:           a [39865]
User ID:               501
Date/Time:             2019-03-24 11:51:04.211 +0000
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
3   libstd-a818a4b7490f1bef.dylib  0x0019cecb std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   libstd-a818a4b7490f1bef.dylib  0x0018e580 rust_oom + 48
5   libstd-a818a4b7490f1bef.dylib  0x001af6f4 alloc::alloc::handle_alloc_error::h548689f8166a49e8 + 20
6   a                              0x0007b6bd default_alloc_error_hook::main::hbf2d06db626d002e + 781
7   a                              0x0007b83b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::he2b2d8ee9e61762f + 11
8   libstd-a818a4b7490f1bef.dylib  0x0018c7fc std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
9   libstd-a818a4b7490f1bef.dylib  0x0018eef4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
10  libstd-a818a4b7490f1bef.dylib  0x0019da4d __rust_maybe_catch_panic + 29
11  libstd-a818a4b7490f1bef.dylib  0x0018f997 std::rt::lang_start_internal::h94f504c75da6b468 + 631
12  a                              0x0007b81c main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff8477c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff847a8  esp: 0xbff8477c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x7a000 -    0x7bffb +a (0) <C00F646C-896A-3E6C-A6EF-A4609F4F0A70> /Users/USER/*/a
   0xe5000 -   0x12afdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x16e000 -   0x1fdfff +libstd-a818a4b7490f1bef.dylib (0) <AC017488-35D2-3C21-AB79-9C50A6F5675A> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115527-2_Traviss-Mac-1044.crash
Process:               a [46225]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [46223]
Responsible:           a [46225]
User ID:               501
Date/Time:             2019-03-24 11:55:26.393 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5600 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b02a3eb8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb02a3eb8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b02a3000-00000000b02a4000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b02a4000-00000000b04a5000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-a818a4b7490f1bef.dylib  0x0023e800 std::sys::unix::thread::Thread::join::h3b1c68437a185a2f + 32
4   a                              0x000eb036 std::thread::JoinHandle$LT$T$GT$::join::hc73736f0cc1164cc + 70
5   a                              0x000ea70d out_of_stack::main::hfb05bc1bb33cf0c4 + 2605
6   a                              0x000e91bb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc5ab2f91b133232a + 11
7   libstd-a818a4b7490f1bef.dylib  0x0022e7fc std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
8   libstd-a818a4b7490f1bef.dylib  0x00230ef4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
9   libstd-a818a4b7490f1bef.dylib  0x0023fa4d __rust_maybe_catch_panic + 29
10  libstd-a818a4b7490f1bef.dylib  0x00231997 std::rt::lang_start_internal::h94f504c75da6b468 + 631
11  a                              0x000eaacc main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-a818a4b7490f1bef.dylib  0x0023eecb std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   libstd-a818a4b7490f1bef.dylib  0x0022f7d9 std::sys_common::util::abort::hf163c3c491a97f58 + 73
5   libstd-a818a4b7490f1bef.dylib  0x0023e298 std::sys::unix::stack_overflow::imp::signal_handler::hbc4abd09b25618b5 + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-a818a4b7490f1bef.dylib  0x0023dee0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h6296221f2cab524d + 80
9   a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
10  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
11  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
12  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
13  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
14  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
15  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
16  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
17  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
18  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
19  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
20  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
21  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
22  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
23  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
24  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
25  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
26  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
27  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
28  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
29  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
30  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
31  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
32  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
33  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
34  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
35  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
36  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
37  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
38  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
39  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
40  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
41  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
42  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
43  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
44  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
45  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
46  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
47  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
48  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
49  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
50  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
51  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
52  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
53  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
54  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
55  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
56  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
57  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
58  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
59  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
60  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
61  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
62  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
63  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
64  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
65  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
66  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
67  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
68  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
69  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
70  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
71  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
72  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
73  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
74  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
75  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
76  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
77  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
78  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
79  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
80  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
81  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
82  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
83  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
84  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
85  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
86  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
87  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
88  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
89  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
90  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
91  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
92  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
93  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
94  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
95  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
96  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
97  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
98  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
99  a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
100 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
101 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
102 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
103 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
104 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
105 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
106 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
107 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
108 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
109 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
110 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
111 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
112 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
113 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
114 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
115 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
116 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
117 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
118 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
119 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
120 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
121 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
122 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
123 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
124 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
125 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
126 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
127 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
128 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
129 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
130 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
131 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
132 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
133 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
134 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
135 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
136 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
137 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
138 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
139 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
140 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
141 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
142 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
143 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
144 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
145 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
146 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
147 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
148 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
149 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
150 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
151 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
152 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
153 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
154 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
155 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
156 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
157 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
158 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
159 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
160 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
161 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
162 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
163 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
164 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
165 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
166 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
167 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
168 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
169 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
170 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
171 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
172 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
173 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
174 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
175 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
176 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
177 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
178 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
179 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
180 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
181 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
182 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
183 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
184 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
185 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
186 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
187 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
188 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
189 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
190 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
191 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
192 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
193 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
194 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
195 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
196 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
197 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
198 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
199 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
200 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
201 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
202 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
203 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
204 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
205 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
206 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
207 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
208 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
209 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
210 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
211 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
212 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
213 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
214 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
215 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
216 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
217 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
218 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
219 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
220 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
221 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
222 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
223 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
224 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
225 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
226 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
227 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
228 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
229 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
230 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
231 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
232 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
233 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
234 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
235 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
236 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
237 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
238 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
239 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
240 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
241 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
242 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
243 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
244 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
245 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
246 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
247 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
248 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
249 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
250 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
251 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
252 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
253 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
254 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
255 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
256 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
257 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
258 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
259 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
260 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
261 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
262 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
263 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
264 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
265 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
266 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
267 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
268 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
269 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
270 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
271 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
272 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
273 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
274 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
275 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
276 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
277 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
278 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
279 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
280 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
281 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
282 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
283 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
284 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
285 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
286 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
287 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
288 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
289 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
290 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
291 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
292 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
293 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
294 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
295 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
296 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
297 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
298 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
299 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
300 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
301 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
302 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
303 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
304 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
305 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
306 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
307 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
308 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
309 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
310 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
311 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
312 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
313 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
314 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
315 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
316 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
317 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
318 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
319 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
320 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
321 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
322 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
323 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
324 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
325 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
326 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
327 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
328 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
329 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
330 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
331 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
332 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
333 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
334 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
335 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
336 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
337 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
338 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
339 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
340 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
341 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
342 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
343 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
344 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
345 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
346 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
347 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
348 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
349 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
350 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
351 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
352 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
353 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
354 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
355 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
356 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
357 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
358 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
359 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
360 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
361 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
362 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
363 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
364 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
365 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
366 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
367 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
368 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
369 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
370 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
371 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
372 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
373 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
374 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
375 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
376 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
377 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
378 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
379 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
380 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
381 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
382 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
383 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
384 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
385 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
386 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
387 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
388 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
389 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
390 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
391 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
392 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
393 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
394 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
395 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
396 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
397 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
398 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
399 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
400 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
401 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
402 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
403 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
404 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
405 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
406 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
407 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
408 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
409 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
410 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
411 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
412 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
413 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
414 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
415 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
416 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
417 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
418 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
419 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
420 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
421 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
422 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
423 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
424 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
425 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
426 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
427 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
428 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
429 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
430 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
431 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
432 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
433 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
434 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
435 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
436 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
437 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
438 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
439 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
440 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
441 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
442 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
443 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
444 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
445 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
446 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
447 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
448 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
449 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
450 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
451 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
452 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
453 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
454 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
455 a                              0x000e9c78 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
---
===========                     =======  ======= 
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115541_Traviss-Mac-1044.crash
Process:               a [46450]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [46449]
Responsible:           a [46450]
User ID:               501
Date/Time:             2019-03-24 11:55:40.473 +0000
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
3   a                              0x0003d5ab panic_abort::__rust_start_panic::abort::ha3c705416e7f7175 + 11
4   a                              0x0003d59b __rust_start_panic + 11
5   a                              0x00031acb rust_panic + 11
6   a                              0x000316b5 std::panicking::rust_panic_with_hook::h8e0c35d23a6f7bfa + 997
7   a                              0x0004332a std::panicking::begin_panic::hb6db9f05650430f8 + 42
8   a                              0x0003048d lto_abort::main::h9419a0043b6e0505 + 2909
9   a                              0x0004348b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf1a7859257dd6080 + 11
10  a                              0x0003d41c std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
11  a                              0x00030868 main + 984
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffcf74c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffcf778  esp: 0xbffcf74c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x2f000 -    0x52ffb +a (0) <E1A74460-56CD-367C-8CCA-46B8472F7B6B> /Users/USER/*/a
   0xc4000 -   0x109fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2617
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115621-1_Traviss-Mac-1044.crash
Process:               a [47443]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47442]
Responsible:           a [47443]
User ID:               501
Date/Time:             2019-03-24 11:56:19.385 +0000
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
    __TEXT                 000000000005f000-0000000000062000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00060f72 segfault_no_out_of_stack::main::haec5f9680e9a04a3 + 2034
1   a                              0x0005f99b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd9acd0ba19943a0d + 11
2   libstd-a818a4b7490f1bef.dylib  0x002047fc std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
3   libstd-a818a4b7490f1bef.dylib  0x00206ef4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
4   libstd-a818a4b7490f1bef.dylib  0x00215a4d __rust_maybe_catch_panic + 29
5   libstd-a818a4b7490f1bef.dylib  0x00207997 std::rt::lang_start_internal::h94f504c75da6b468 + 631
6   a                              0x0006124c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x7a610a30  ecx: 0x00000000  edx: 0x00000000
  edi: 0x00215a3e  esi: 0xbff9f890  ebp: 0xbff9f978  esp: 0xbff9f7d0
   ss: 0x00000023  efl: 0x00010246  eip: 0x00060f72   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     0
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x5f000 -    0x61ffb +a (0) <2BECF4EB-F7A2-381B-8F9E-164D29D29416> /Users/USER/*/a
  0x15d000 -   0x1a2fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1e6000 -   0x275fff +libstd-a818a4b7490f1bef.dylib (0) <AC017488-35D2-3C21-AB79-9C50A6F5675A> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2617
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
__DATA                            3572K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.6M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115621_Traviss-Mac-1044.crash
Process:               a [47416]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47411]
Responsible:           a [47416]
User ID:               501
Date/Time:             2019-03-24 11:56:18.472 +0000
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
3   libstd-a818a4b7490f1bef.dylib  0x00174ecb std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   libstd-a818a4b7490f1bef.dylib  0x001657d9 std::sys_common::util::abort::hf163c3c491a97f58 + 73
5   libstd-a818a4b7490f1bef.dylib  0x00167712 rust_panic + 98
6   libstd-a818a4b7490f1bef.dylib  0x001675de std::panicking::rust_panic_with_hook::h8e0c35d23a6f7bfa + 590
7   a                              0x000469cf std::panicking::begin_panic::h492b4fbd30659c42 + 47
8   a                              0x00047efc main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffb974c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffb9778  esp: 0xbffb974c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x45000 -    0x48fff +a (0) <53393FF3-B276-34C7-B0AF-5B87D97BCD98> /Users/USER/*/a
   0xbd000 -   0x102fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x146000 -   0x1d5fff +libstd-a818a4b7490f1bef.dylib (0) <AC017488-35D2-3C21-AB79-9C50A6F5675A> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2617
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
__DATA                            3572K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115627_Traviss-Mac-1044.crash
Process:               a [47626]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47625]
Responsible:           a [47626]
User ID:               501
Date/Time:             2019-03-24 11:56:27.157 +0000
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
    __TEXT                 0000000000069000-000000000006c000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x0006b5d4 signal_exit_status::main::h013a3960fc5c363d + 436
1   a                              0x0006a47b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h33c45f87a006e61b + 11
2   libstd-a818a4b7490f1bef.dylib  0x0011c7fc std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
3   libstd-a818a4b7490f1bef.dylib  0x0011eef4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
4   libstd-a818a4b7490f1bef.dylib  0x0012da4d __rust_maybe_catch_panic + 29
5   libstd-a818a4b7490f1bef.dylib  0x0011f997 std::rt::lang_start_internal::h94f504c75da6b468 + 631
6   a                              0x0006b6ac main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x796112b0
  edi: 0x79611340  esi: 0xbff958f0  ebp: 0xbff95988  esp: 0xbff95870
   ss: 0x00000023  efl: 0x00010246  eip: 0x0006b5d4   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     1
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x69000 -    0x6bff7 +a (0) <69D5EC92-AD2B-34A6-979B-8E8CB4EC1548> /Users/USER/*/a
   0x75000 -    0xbafdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
   0xfe000 -   0x18dfff +libstd-a818a4b7490f1bef.dylib (0) <AC017488-35D2-3C21-AB79-9C50A6F5675A> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2617
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
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                        132K        3 
__DATA                            3572K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115633_Traviss-Mac-1044.crash
Process:               a [47734]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47728]
Responsible:           a [47734]
User ID:               501
Date/Time:             2019-03-24 11:56:32.484 +0000
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
0   a                              0x000eeec6 simd_target_feature_mixup::test::id_avx512_512::h78a86a7fc3f1e93a + 102
1   a                              0x000edc7f simd_target_feature_mixup::test::main::h379367934b9623dc + 1647
2   a                              0x000f01f0 simd_target_feature_mixup::main::h4f60990077aff357 + 896
3   a                              0x000ed38b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h08b0578e0d501144 + 11
4   libstd-a818a4b7490f1bef.dylib  0x002347fc std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
5   libstd-a818a4b7490f1bef.dylib  0x00236ef4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
6   libstd-a818a4b7490f1bef.dylib  0x00245a4d __rust_maybe_catch_panic + 29
7   libstd-a818a4b7490f1bef.dylib  0x00237997 std::rt::lang_start_internal::h94f504c75da6b468 + 631
8   a                              0x000f03cc main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff125c0  ebx: 0xbff12540  ecx: 0x000eee6e  edx: 0xbff12540
  edi: 0x000ed624  esi: 0x00000000  ebp: 0xbff12538  esp: 0xbff12500
   ss: 0x00000023  efl: 0x00010246  eip: 0x000eeec6   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000eeb00
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xec000 -    0xf0fef +a (0) <8E13AD5A-BC8E-3462-8D28-914C9E0B5BF1> /Users/USER/*/a
  0x18d000 -   0x1d2fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x216000 -   0x2a5fff +libstd-a818a4b7490f1bef.dylib (0) <AC017488-35D2-3C21-AB79-9C50A6F5675A> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2617
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
__DATA                            3572K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115638-1_Traviss-Mac-1044.crash
Process:               a [47871]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47869]
Responsible:           a [47871]
User ID:               501
Date/Time:             2019-03-24 11:56:37.746 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5700 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbff6998
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbff6998:
    Stack Guard            00000000bbff5000-00000000bbff6000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbff6000-00000000bbff7000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbff7000-00000000bfff5000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-a818a4b7490f1bef.dylib  0x00126ecb std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   libstd-a818a4b7490f1bef.dylib  0x001177d9 std::sys_common::util::abort::hf163c3c491a97f58 + 73
5   libstd-a818a4b7490f1bef.dylib  0x00126298 std::sys::unix::stack_overflow::imp::signal_handler::hbc4abd09b25618b5 + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-a818a4b7490f1bef.dylib  0x00125ee0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h6296221f2cab524d + 80
9   a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
265 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
266 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
267 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
268 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
269 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
270 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
271 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
272 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
273 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
274 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
275 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
276 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
277 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
278 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
279 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
280 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
281 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
282 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
283 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
284 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
285 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
286 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
287 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
288 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
289 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
290 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
291 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
292 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
293 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
294 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
295 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
296 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
297 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
298 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
299 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
300 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
301 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
302 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
303 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
304 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
305 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
306 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
307 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
308 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
309 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
310 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
311 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
312 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
313 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
314 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
315 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
316 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
317 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
318 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
319 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
320 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
321 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
322 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
323 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
324 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
325 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
326 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
327 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
328 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
329 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
330 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
331 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
332 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
333 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
334 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
335 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
336 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
337 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
338 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
339 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
340 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
341 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
342 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
343 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
344 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
345 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
346 a                              0x0000e950 stack_probes::recurse::h24283d9484398da0 + 48
---
===========                     =======  ======= 
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115638_Traviss-Mac-1044.crash
Process:               a [47873]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47869]
Responsible:           a [47873]
User ID:               501
Date/Time:             2019-03-24 11:56:37.749 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5700 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b04edea8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb04edea8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b04ed000-00000000b04ee000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b04ee000-00000000b06ef000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-a818a4b7490f1bef.dylib  0x00219800 std::sys::unix::thread::Thread::join::h3b1c68437a185a2f + 32
4   a                              0x000895e6 std::thread::JoinHandle$LT$T$GT$::join::h5b2667e68613625d + 70
5   a                              0x00088805 stack_probes::main::hc5f49a55fd7e038b + 597
6   a                              0x0008761b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hec3c4a6375b3cdb3 + 11
7   libstd-a818a4b7490f1bef.dylib  0x002097fc std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
8   libstd-a818a4b7490f1bef.dylib  0x0020bef4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
9   libstd-a818a4b7490f1bef.dylib  0x0021aa4d __rust_maybe_catch_panic + 29
10  libstd-a818a4b7490f1bef.dylib  0x0020c997 std::rt::lang_start_internal::h94f504c75da6b468 + 631
11  a                              0x0008922c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-a818a4b7490f1bef.dylib  0x00219ecb std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   libstd-a818a4b7490f1bef.dylib  0x0020a7d9 std::sys_common::util::abort::hf163c3c491a97f58 + 73
5   libstd-a818a4b7490f1bef.dylib  0x00219298 std::sys::unix::stack_overflow::imp::signal_handler::hbc4abd09b25618b5 + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-a818a4b7490f1bef.dylib  0x00218ee0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h6296221f2cab524d + 80
9   a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x00088950 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x0008767d std::sys_common::backtrace::__rust_begin_short_backtrace::h50a627a5caed610d (.llvm.4877210633173626813) + 29
265 libstd-a818a4b7490f1bef.dylib  0x0021aa4d __rust_maybe_catch_panic + 29
266 a                              0x00089b13 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h02905e968a88e75f + 131
267 libstd-a818a4b7490f1bef.dylib  0x0021973b std::sys::unix::thread::Thread::new::thread_start::h571fbd6e46da83e4 + 187
268 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
269 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
270 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb06ee000  ecx: 0x000dcb0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x000dcb38  esp: 0x000dcb0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00219320
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0x86000 -    0x8afff +a (0) <707DB57A-C828-3F9F-BE0D-319D975E6135> /Users/USER/*/a
  0x162000 -   0x1a7fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1eb000 -   0x27afff +libstd-a818a4b7490f1bef.dylib (0) <AC017488-35D2-3C21-AB79-9C50A6F5675A> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2617
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
MALLOC guard page                   16K        5 
Stack Guard                          8K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3572K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9348K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            571.7M      137 
TOTAL                            571.7M      137 
TOTAL, minus reserved VM space   571.6M      137 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115646-1_Traviss-Mac-1044.crash
Process:               a [47987]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47986]
Responsible:           a [47987]
User ID:               501
Date/Time:             2019-03-24 11:56:44.379 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5700 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbfba9d8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbfba9d8:
    Stack Guard            00000000bbfb9000-00000000bbfba000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbfba000-00000000bbfbb000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbfbb000-00000000bffb9000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x0004c12b std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   a                              0x0004c119 std::sys_common::util::abort::hf163c3c491a97f58 + 73
5   a                              0x0005ac1c std::sys::unix::stack_overflow::imp::signal_handler::hbc4abd09b25618b5 + 860
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x0005a8c0 rust_begin_unwind + 16
9   a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
10  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
11  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
12  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
13  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
14  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
15  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
16  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
17  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
18  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
19  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
20  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
21  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
22  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
23  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
24  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
25  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
26  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
27  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
28  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
29  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
30  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
31  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
32  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
33  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
34  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
35  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
36  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
37  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
38  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
39  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
40  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
41  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
42  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
43  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
44  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
45  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
46  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
47  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
48  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
49  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
50  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
51  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
52  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
53  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
54  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
55  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
56  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
57  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
58  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
59  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
60  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
61  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
62  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
63  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
64  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
65  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
66  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
67  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
68  a                              0x00049b08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
---
===========                     =======  ======= 
TOTAL                            565.6M      130 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-24-115646_Traviss-Mac-1044.crash
Process:               a [47989]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47986]
Responsible:           a [47989]
User ID:               501
Date/Time:             2019-03-24 11:56:44.388 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5700 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b07bcf08
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb07bcf08:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b07bc000-00000000b07bd000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b07bd000-00000000b09be000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   a                              0x000ba659 stack_probes_lto::main::h7f9d4bbc5a99dd1a + 2457
4   a                              0x000d34eb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h8fa609264f6694d3 + 11
5   a                              0x000cbcbc std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
6   a                              0x000bbd7d main + 589
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000bd12b std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   a                              0x000bd119 std::sys_common::util::abort::hf163c3c491a97f58 + 73
5   a                              0x000cbc1c std::sys::unix::stack_overflow::imp::signal_handler::hbc4abd09b25618b5 + 860
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000cb8c0 rust_begin_unwind + 16
9   a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
10  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
11  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
12  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
13  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
14  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
15  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
16  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
17  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
18  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
19  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
20  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
21  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
22  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
23  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
24  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
25  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
26  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
27  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
28  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
29  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
30  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
31  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
32  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
33  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
34  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
35  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
36  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
37  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
38  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
39  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
40  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
41  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
42  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
43  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
44  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
45  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
46  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
47  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
48  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
49  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
50  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
51  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
52  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
53  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
54  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
55  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
56  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
57  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
58  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
59  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
60  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
61  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
62  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
63  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
64  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
65  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
66  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
67  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
68  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
69  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
70  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
71  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
72  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
73  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
74  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
75  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
76  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
77  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
78  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
79  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
80  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
81  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
82  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
83  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
84  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
85  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
86  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
87  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
88  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
89  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
90  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
91  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
92  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
93  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
94  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
95  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
96  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
97  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
98  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
99  a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
100 a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
101 a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
102 a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
103 a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
104 a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
105 a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
106 a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
107 a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
108 a                              0x000bab08 stack_probes_lto::recurse::h907252696a8f0ddd + 40
