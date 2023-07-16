plain
[00:03:08]       Memory: 8 GB
[00:03:08]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:08]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:08]       SMC Version (system): 2.8f0
[00:03:08]       Serial Number (system): VMNGdaUgAyEh
[00:03:08] 
[00:03:08] hw.ncpu: 4
[00:03:08] hw.byteorder: 1234
[00:03:08] hw.memsize: 8589934592
---
[02:12:08] ---- /Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/levels.md - Lint_levels::Configuring_warning_levels::Via_an_attribute (line 202) stdout ----
[02:12:08] warning: missing documentation for crate
[02:12:08]  --> /Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/levels.md:201:1
[02:12:08]   |
[02:12:08] 1 | / #![allow(unused)]
[02:12:08] 2 | | #![warn(missing_docs, unused_variables)]
[02:12:08] 4 | | fn main() {
[02:12:08] 5 | | pub fn foo() {}
[02:12:08] 6 | | }
[02:12:08]   | |_^
---
[02:12:08]   |         ^^^^^^^^^^^^
[02:12:08] 
[02:12:08] error: linking with `cc` failed: signal: 4
[02:12:08]   |
[02:12:08]   = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEfFdML/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEfFdML/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEfFdML/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEfFdML/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEfFdML/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEfFdML/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEfFdML/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libstd-22969636c9515299.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libpanic_unwind-ade3f71f5c4745b0.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libbacktrace_sys-6e9496f6aa8a8aee.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_demangle-022938c840899ee8.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libunwind-e499ce42a6220ba0.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liblibc-35b711169c5a08cc.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liballoc-5cc329465c73523e.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_core-635c35a1095754d1.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcore-a9d318fb39582bdb.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-78e8331284e7e75f.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:12:08] 
[02:12:08] error: aborting due to previous error
[02:12:08] 
[02:12:08] thread '/Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/levels.md - Lint_levels::Configuring_warning_levels::Via_an_attribute (line 202)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[02:12:08] 
[02:12:08] 
[02:12:08] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:12:08] Build completed unsuccessfully in 0:53:54
[02:12:08] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:088c6632
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 20 14:39:58 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:2ecfc4f7
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1176
drwx------  26 travis  staff    884 Mar 20 14:39 .
-rw-------@  1 travis  staff   1387 Mar 20 14:39 foo_2019-03-20-143920_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1377 Mar 20 14:38 m4_2019-03-20-143848_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1391 Mar 20 14:38 bar_2019-03-20-143838-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1403 Mar 20 14:38 bar_2019-03-20-143838_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9899 Mar 20 14:38 b_2019-03-20-143837_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34790 Mar 20 14:02 a_2019-03-20-140203-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  57364 Mar 20 14:02 a_2019-03-20-140203_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55583 Mar 20 14:01 a_2019-03-20-140154-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34718 Mar 20 14:01 a_2019-03-20-140154_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 Mar 20 14:01 a_2019-03-20-140150_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9166 Mar 20 14:01 a_2019-03-20-140144_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8936 Mar 20 14:01 a_2019-03-20-140138-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9171 Mar 20 14:01 a_2019-03-20-140138_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9304 Mar 20 14:00 a_2019-03-20-140057_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58238 Mar 20 14:00 a_2019-03-20-140051_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60372 Mar 20 14:00 a_2019-03-20-140044-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59503 Mar 20 14:00 a_2019-03-20-140044-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 Mar 20 14:00 a_2019-03-20-140044_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10881 Mar 20 13:58 a_2019-03-20-135829_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 Mar 20 13:57 a_2019-03-20-135734_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 Mar 20 13:56 a_2019-03-20-135616_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 Mar 20 13:55 a_2019-03-20-135521-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 Mar 20 13:55 a_2019-03-20-135521_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9484 Mar 20 13:55 a_2019-03-20-135517_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:02f1f03d
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-20-135517_Traviss-Mac-1044.crash
Process:               a [37306]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [37303]
Responsible:           a [37306]
User ID:               501
Date/Time:             2019-03-20 13:54:47.289 +0000
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
0   a                              0x000ccafe abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 46
1   a                              0x000cbf4b std::panicking::try::do_call::hc1ccca6d267d6023 (.llvm.14945672356138856841) + 11
2   libstd-22969636c9515299.dylib  0x001bdb3d __rust_maybe_catch_panic + 29
3   a                              0x000ccd65 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 613
4   a                              0x000cb5ab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h0b74faa50be5cfe9 + 11
5   libstd-22969636c9515299.dylib  0x001ac90c std::sys_common::backtrace::__rust_begin_short_backtrace::h941085b4bbfcfc5d + 12
6   libstd-22969636c9515299.dylib  0x001aeb54 std::panicking::try::do_call::h82374a59315181b8 + 20
7   libstd-22969636c9515299.dylib  0x001bdb3d __rust_maybe_catch_panic + 29
8   libstd-22969636c9515299.dylib  0x001af5f7 std::rt::lang_start_internal::h7741962f5392b059 + 631
9   a                              0x000cd09c main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x78646090  ebx: 0xbff349a8  ecx: 0x00000000  edx: 0x00000000
  edi: 0x001bdb2e  esi: 0x00000000  ebp: 0xbff34948  esp: 0xbff34930
   ss: 0x00000023  efl: 0x00010296  eip: 0x000ccafe   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x001f71f0
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xca000 -    0xcdff3 +a (0) <DB0EC50F-5E79-33E0-90C5-141B335AF4B3> /Users/USER/*/a
  0x105000 -   0x14afdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x18e000 -   0x21dff3 +libstd-22969636c9515299.dylib (0) <9339C2F6-C93C-34A0-B00E-77391B2E6BDA> /Users/USER/*/libstd-22969636c9515299.dylib
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
    task_for_pid: 2345
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
__DATA                            3564K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-20-135521-1_Traviss-Mac-1044.crash
Process:               a [38118]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [38113]
Responsible:           a [38118]
User ID:               501
Date/Time:             2019-03-20 13:55:18.154 +0000
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
0   libstd-22969636c9515299.dylib  0x000f5063 std::panicking::rust_panic_with_hook::haacf198c21f4f49a + 115
1   a                              0x0001abff std::panicking::begin_panic::h94347040df0aab0c + 47 (panicking.rs:408)
2   a                              0x000186e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::h5ed13b737dc2b5ab + 36 (backtrace.rs:24)
3   a                              0x00017c0b core::ptr::real_drop_in_place::h0c56079745e4bd0d + 11
4   a                              0x000186b3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x000199c8 backtrace::main::hcde7a1a1c3c85e77 + 4568 (backtrace.rs:103)
6   a                              0x00017bbb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hcd2beb3d44141aba + 11 (rt.rs:64)
7   libstd-22969636c9515299.dylib  0x000f290c std::sys_common::backtrace::__rust_begin_short_backtrace::h941085b4bbfcfc5d + 12
8   libstd-22969636c9515299.dylib  0x000f4b54 std::panicking::try::do_call::h82374a59315181b8 + 20
9   libstd-22969636c9515299.dylib  0x00103b3d __rust_maybe_catch_panic + 29
10  libstd-22969636c9515299.dylib  0x000f55f7 std::rt::lang_start_internal::h7741962f5392b059 + 631
11  a                              0x0001a21c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbffe96f8  ebx: 0xbffe9740  ecx: 0xbffe95f0  edx: 0xa7702ec6
  edi: 0x0013d538  esi: 0x000f4ffe  ebp: 0xbffe9798  esp: 0xbffe9710
   ss: 0x00000023  efl: 0x00010282  eip: 0x000f5063   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x003c2c0b
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x15000 -    0x1bff7 +a (0) <49B72B8A-1079-3344-9930-82D35757FEBB> /Users/USER/*/a
   0x4b000 -    0x90fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
   0xd4000 -   0x163ff3 +libstd-22969636c9515299.dylib (0) <9339C2F6-C93C-34A0-B00E-77391B2E6BDA> /Users/USER/*/libstd-22969636c9515299.dylib
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
    task_for_pid: 2345
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=91.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=91.6M(100%)
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
__DATA                            3564K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9356K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            586.9M      138 
TOTAL                            586.9M      138 
TOTAL, minus reserved VM space   586.8M      138 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-20-135521_Traviss-Mac-1044.crash
Process:               a [38117]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [38113]
Responsible:           a [38117]
User ID:               501
Date/Time:             2019-03-20 13:55:18.150 +0000
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
0   libstd-22969636c9515299.dylib  0x0023d063 std::panicking::rust_panic_with_hook::haacf198c21f4f49a + 115
1   a                              0x0009bbff std::panicking::begin_panic::h94347040df0aab0c + 47 (panicking.rs:408)
2   a                              0x000996e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::h5ed13b737dc2b5ab + 36 (backtrace.rs:24)
3   a                              0x00098c0b core::ptr::real_drop_in_place::h0c56079745e4bd0d + 11
4   a                              0x000996b3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x0009a9c8 backtrace::main::hcde7a1a1c3c85e77 + 4568 (backtrace.rs:103)
6   a                              0x00098bbb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hcd2beb3d44141aba + 11 (rt.rs:64)
7   libstd-22969636c9515299.dylib  0x0023a90c std::sys_common::backtrace::__rust_begin_short_backtrace::h941085b4bbfcfc5d + 12
8   libstd-22969636c9515299.dylib  0x0023cb54 std::panicking::try::do_call::h82374a59315181b8 + 20
9   libstd-22969636c9515299.dylib  0x0024bb3d __rust_maybe_catch_panic + 29
10  libstd-22969636c9515299.dylib  0x0023d5f7 std::rt::lang_start_internal::h7741962f5392b059 + 631
11  a                              0x0009b21c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff68708  ebx: 0xbff68750  ecx: 0xbff68600  edx: 0xa7702ec6
  edi: 0x00285538  esi: 0x0023cffe  ebp: 0xbff687a8  esp: 0xbff68720
   ss: 0x00000023  efl: 0x00010286  eip: 0x0023d063   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xb2bb0fbc
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x96000 -    0x9cff7 +a (0) <49B72B8A-1079-3344-9930-82D35757FEBB> /Users/USER/*/a
  0x193000 -   0x1d8fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x21c000 -   0x2abff3 +libstd-22969636c9515299.dylib (0) <9339C2F6-C93C-34A0-B00E-77391B2E6BDA> /Users/USER/*/libstd-22969636c9515299.dylib
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
    task_for_pid: 2345
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=91.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=91.6M(100%)
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
__DATA                            3564K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-20-135616_Traviss-Mac-1044.crash
Process:               a [39789]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [39786]
Responsible:           a [39789]
User ID:               501
Date/Time:             2019-03-20 13:56:14.491 +0000
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
3   libstd-22969636c9515299.dylib  0x00124fbb std::sys::unix::abort_internal::h07a492646837a647 + 11
4   libstd-22969636c9515299.dylib  0x001161e0 rust_oom + 48
5   libstd-22969636c9515299.dylib  0x001377c4 alloc::alloc::handle_alloc_error::hf69a2b5e93d11fa8 + 20
6   a                              0x0001071d default_alloc_error_hook::main::hbf2d06db626d002e + 781
7   a                              0x0000f8eb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h7667eb2d46896510 + 11
8   libstd-22969636c9515299.dylib  0x0011490c std::sys_common::backtrace::__rust_begin_short_backtrace::h941085b4bbfcfc5d + 12
9   libstd-22969636c9515299.dylib  0x00116b54 std::panicking::try::do_call::h82374a59315181b8 + 20
10  libstd-22969636c9515299.dylib  0x00125b3d __rust_maybe_catch_panic + 29
11  libstd-22969636c9515299.dylib  0x001175f7 std::rt::lang_start_internal::h7741962f5392b059 + 631
12  a                              0x0001087c main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffef8bc  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffef8e8  esp: 0xbffef8bc
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
    0xf000 -    0x10ffb +a (0) <0395EFD4-3828-3044-9470-E6AB060D77E5> /Users/USER/*/a
   0x6d000 -    0xb2fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
   0xf6000 -   0x185ff3 +libstd-22969636c9515299.dylib (0) <9339C2F6-C93C-34A0-B00E-77391B2E6BDA> /Users/USER/*/libstd-22969636c9515299.dylib
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-20-140051_Traviss-Mac-1044.crash
Process:               a [46138]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [46130]
Responsible:           a [46138]
User ID:               501
Date/Time:             2019-03-20 14:00:49.036 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5700 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf90ffc
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf90ffc:
    Stack Guard            00000000bbf8f000-00000000bbf90000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf90000-00000000bbf91000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf91000-00000000bff8f000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-22969636c9515299.dylib  0x001edfbb std::sys::unix::abort_internal::h07a492646837a647 + 11
4   libstd-22969636c9515299.dylib  0x001de459 std::sys_common::util::abort::h85f0045aaa34085e + 73
5   libstd-22969636c9515299.dylib  0x001ed388 std::sys::unix::stack_overflow::imp::signal_handler::h907965a5b4fdf2cd + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-22969636c9515299.dylib  0x001ecfd0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb6457c1a448aaf7a + 80
9   libstd-22969636c9515299.dylib  0x001ce707 _$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$::write::he230939e675a1ef4 + 263
10  libstd-22969636c9515299.dylib  0x001d0127 std::io::Write::write_all::h468b76f2fd2524c6 + 71
11  libstd-22969636c9515299.dylib  0x001d0753 _$LT$std..io..Write..write_fmt..Adaptor$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$::write_str::hba77afa5ace19f21 + 35
12  libstd-22969636c9515299.dylib  0x0020e46d core::fmt::write::h4dcf555f975832d8 + 701
13  libstd-22969636c9515299.dylib  0x001ce4d6 _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::ha476003207bd3567 + 182
14  libstd-22969636c9515299.dylib  0x001cf82c std::io::stdio::_print::he0049ada803ac7cf + 396
15  a                              0x00073cbf out_of_stack::loud_recurse::hcd528ebf130a94fa + 63
16  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
17  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
18  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
19  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
20  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
21  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
22  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
23  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
24  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
25  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
26  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
27  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
28  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
29  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
30  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
31  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
32  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
33  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
34  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
35  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
36  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
37  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
38  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
39  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
40  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
41  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
42  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
43  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
44  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
45  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
46  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
47  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
48  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
49  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
50  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
51  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
52  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
53  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
54  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
55  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
56  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
57  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
58  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
59  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
60  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
61  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
62  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
63  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
64  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
65  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
66  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
67  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
68  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
69  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
70  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
71  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
72  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
73  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
74  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
75  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
76  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
77  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
78  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
79  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
80  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
81  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
82  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
83  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
84  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
85  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
86  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
87  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
88  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
89  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
90  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
91  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
92  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
93  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
94  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
95  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
96  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
97  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
98  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
99  a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
100 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
101 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
102 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
103 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
104 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
105 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
106 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
107 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
108 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
109 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
110 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
111 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
112 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
113 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
114 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
115 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
116 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
117 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
118 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
119 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
120 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
121 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
122 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
123 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
124 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
125 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
126 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
127 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
128 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
129 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
130 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
131 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
132 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
133 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
134 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
135 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
136 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
137 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
138 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
139 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
140 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
141 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
142 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
143 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
144 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
145 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
146 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
147 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
148 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
149 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
150 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
151 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
152 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
153 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
154 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
155 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
156 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
157 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
158 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
159 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
160 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
161 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
162 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
163 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
164 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
165 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
166 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
167 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
168 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
169 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
170 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
171 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
172 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
173 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
174 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
175 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
176 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
177 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
178 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
179 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
180 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
181 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
182 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
183 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
184 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
185 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
186 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
187 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
188 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
189 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
190 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
191 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
192 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
193 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
194 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
195 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
196 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
197 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
198 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
199 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
200 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
201 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
202 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
203 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
204 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
205 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
206 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
207 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
208 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
209 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
210 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
211 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
212 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
213 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
214 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
215 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
216 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
217 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
218 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
219 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
220 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
221 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
222 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
223 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
224 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
225 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
226 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
227 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
228 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
229 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
230 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
231 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
232 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
233 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
234 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
235 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
236 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
237 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
238 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
239 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
240 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
241 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
242 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
243 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
244 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
245 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
246 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
247 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
248 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
249 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
250 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
251 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
252 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
253 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
254 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
255 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
256 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
257 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
258 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
259 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
260 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
261 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
262 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
263 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
264 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
265 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
266 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
267 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
268 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
269 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
270 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
271 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
272 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
273 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
274 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
275 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
276 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
277 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
278 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
279 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
280 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
281 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
282 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
283 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
284 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
285 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
286 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
287 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
288 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
289 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
290 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
291 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
292 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
293 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
294 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
295 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
296 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
297 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
298 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
299 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
300 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
301 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
302 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
303 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
304 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
305 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
306 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
307 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
308 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
309 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
310 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
311 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
312 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
313 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
314 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
315 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
316 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
317 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
318 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
319 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
320 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
321 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
322 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
323 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
324 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
325 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
326 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
327 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
328 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
329 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
330 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
331 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
332 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
333 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
334 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
335 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
336 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
337 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
338 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
339 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
340 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
341 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
342 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
343 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
344 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
345 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
346 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
347 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
348 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
349 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
350 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
351 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
352 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
353 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
354 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
355 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
356 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
357 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
358 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
359 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
360 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
361 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
362 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
363 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
364 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
365 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
366 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
367 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
368 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
369 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
370 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
371 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
372 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
373 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
374 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
375 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
376 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
377 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
378 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
379 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
380 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
381 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
382 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
383 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
384 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
385 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
386 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
387 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
388 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
389 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
390 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
391 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
392 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
393 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
394 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
395 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
396 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
397 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
398 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
399 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
400 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
401 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
402 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
403 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
404 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
405 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
406 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
407 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
408 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
409 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
410 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
411 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
412 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
413 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
414 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
415 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
416 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
417 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
418 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
419 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
420 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
421 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
422 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
423 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
424 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
425 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
426 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
427 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
428 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
429 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
430 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
431 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
432 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
433 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
434 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
435 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
436 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
437 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
438 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
439 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
440 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
441 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
442 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
443 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
444 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
445 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
446 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
447 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
448 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
449 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
450 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
451 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
452 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
453 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
454 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
455 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
456 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
457 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
458 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
459 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
460 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
461 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
462 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
463 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
464 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
465 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
466 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
467 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
468 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
469 a                              0x00073cc4 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
---
===========                     =======  ======= 
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-20-140057_Traviss-Mac-1044.crash
Process:               a [46358]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [46356]
Responsible:           a [46358]
User ID:               501
Date/Time:             2019-03-20 14:00:57.272 +0000
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
3   a                              0x000fd57b panic_abort::__rust_start_panic::abort::h0752f9dad0e76c9b + 11
4   a                              0x000fd56b __rust_start_panic + 11
5   a                              0x000f1aab rust_panic + 11
6   a                              0x000f1695 std::panicking::rust_panic_with_hook::haacf198c21f4f49a + 997
7   a                              0x0010332a std::panicking::begin_panic::h024b527a81426a14 + 42
8   a                              0x000f04ad lto_abort::main::h9419a0043b6e0505 + 2909
9   a                              0x0010346b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb68b9bf9943537ef + 11
10  a                              0x000fd3ec std::sys_common::backtrace::__rust_begin_short_backtrace::h941085b4bbfcfc5d + 12
11  a                              0x000f0888 main + 984
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff0f88c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff0f8b8  esp: 0xbff0f88c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xef000 -   0x112ffb +a (0) <05B8BAE9-519E-38D4-A2DC-99AD3456FCBC> /Users/USER/*/a
  0x1a2000 -   0x1e7fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2591
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-20-140138-1_Traviss-Mac-1044.crash
Process:               a [47331]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47325]
Responsible:           a [47331]
User ID:               501
Date/Time:             2019-03-20 14:01:35.906 +0000
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
3   libstd-22969636c9515299.dylib  0x00117fbb std::sys::unix::abort_internal::h07a492646837a647 + 11
4   libstd-22969636c9515299.dylib  0x00108459 std::sys_common::util::abort::h85f0045aaa34085e + 73
5   libstd-22969636c9515299.dylib  0x0010a372 rust_panic + 98
6   libstd-22969636c9515299.dylib  0x0010a23e std::panicking::rust_panic_with_hook::haacf198c21f4f49a + 590
7   a                              0x000469bf std::panicking::begin_panic::hd6320424f447b8a4 + 47
8   a                              0x00047eec main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffb988c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffb98b8  esp: 0xbffb988c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x45000 -    0x48ff7 +a (0) <D547FC41-4312-3A7F-A542-81AE9FB6E88B> /Users/USER/*/a
   0x60000 -    0xa5fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
   0xe9000 -   0x178ff3 +libstd-22969636c9515299.dylib (0) <9339C2F6-C93C-34A0-B00E-77391B2E6BDA> /Users/USER/*/libstd-22969636c9515299.dylib
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
    task_for_pid: 2591
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=83.2M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=83.2M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            19.0M        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
__DATA                            3564K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__OBJC                              36K        6 
__TEXT                            9344K       44 
mapped file                      408.7M       21 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            578.4M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-20-140138_Traviss-Mac-1044.crash
Process:               a [47362]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47360]
Responsible:           a [47362]
User ID:               501
Date/Time:             2019-03-20 14:01:36.554 +0000
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
    __TEXT                 0000000000072000-0000000000075000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00073f72 segfault_no_out_of_stack::main::haec5f9680e9a04a3 + 2034
1   a                              0x0007299b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h855a18e5368d7134 + 11
2   libstd-22969636c9515299.dylib  0x0020790c std::sys_common::backtrace::__rust_begin_short_backtrace::h941085b4bbfcfc5d + 12
3   libstd-22969636c9515299.dylib  0x00209b54 std::panicking::try::do_call::h82374a59315181b8 + 20
4   libstd-22969636c9515299.dylib  0x00218b3d __rust_maybe_catch_panic + 29
5   libstd-22969636c9515299.dylib  0x0020a5f7 std::rt::lang_start_internal::h7741962f5392b059 + 631
6   a                              0x0007424c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x78616700  ecx: 0x00000000  edx: 0x00000000
  edi: 0x00218b2e  esi: 0xbff8c9d0  ebp: 0xbff8cab8  esp: 0xbff8c910
   ss: 0x00000023  efl: 0x00010246  eip: 0x00073f72   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     0
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x72000 -    0x74ffb +a (0) <2F4B1847-595D-327C-B943-163714E8AE0E> /Users/USER/*/a
  0x160000 -   0x1a5fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1e9000 -   0x278ff3 +libstd-22969636c9515299.dylib (0) <9339C2F6-C93C-34A0-B00E-77391B2E6BDA> /Users/USER/*/libstd-22969636c9515299.dylib
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
    task_for_pid: 2591
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
__DATA                            3564K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.6M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-20-140144_Traviss-Mac-1044.crash
Process:               a [47540]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47539]
Responsible:           a [47540]
User ID:               501
Date/Time:             2019-03-20 14:01:43.841 +0000
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
    __TEXT                 00000000000d2000-00000000000d5000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000d45d4 signal_exit_status::main::h013a3960fc5c363d + 436
1   a                              0x000d347b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h7465e93784988b39 + 11
2   libstd-22969636c9515299.dylib  0x0021890c std::sys_common::backtrace::__rust_begin_short_backtrace::h941085b4bbfcfc5d + 12
3   libstd-22969636c9515299.dylib  0x0021ab54 std::panicking::try::do_call::h82374a59315181b8 + 20
4   libstd-22969636c9515299.dylib  0x00229b3d __rust_maybe_catch_panic + 29
5   libstd-22969636c9515299.dylib  0x0021b5f7 std::rt::lang_start_internal::h7741962f5392b059 + 631
6   a                              0x000d46ac main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x7a6264b0
  edi: 0x7a626540  esi: 0xbff2ca30  ebp: 0xbff2cac8  esp: 0xbff2c9b0
   ss: 0x00000023  efl: 0x00010246  eip: 0x000d45d4   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     3
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xd2000 -    0xd4ff7 +a (0) <CF72F5AD-24B3-3CB2-825A-8544B68C503B> /Users/USER/*/a
  0x171000 -   0x1b6fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1fa000 -   0x289ff3 +libstd-22969636c9515299.dylib (0) <9339C2F6-C93C-34A0-B00E-77391B2E6BDA> /Users/USER/*/libstd-22969636c9515299.dylib
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
    task_for_pid: 2591
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
__DATA                            3564K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.6M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-20-140150_Traviss-Mac-1044.crash
Process:               a [47645]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47637]
Responsible:           a [47645]
User ID:               501
Date/Time:             2019-03-20 14:01:48.907 +0000
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
0   a                              0x000cfec6 simd_target_feature_mixup::test::id_avx512_512::h5d16af711813be1e + 102
1   a                              0x000cec7f simd_target_feature_mixup::test::main::h379367934b9623dc + 1647
2   a                              0x000d11f0 simd_target_feature_mixup::main::h4f60990077aff357 + 896
3   a                              0x000ce23b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hda17298e688ff22d + 11
4   libstd-22969636c9515299.dylib  0x0023890c std::sys_common::backtrace::__rust_begin_short_backtrace::h941085b4bbfcfc5d + 12
5   libstd-22969636c9515299.dylib  0x0023ab54 std::panicking::try::do_call::h82374a59315181b8 + 20
6   libstd-22969636c9515299.dylib  0x00249b3d __rust_maybe_catch_panic + 29
7   libstd-22969636c9515299.dylib  0x0023b5f7 std::rt::lang_start_internal::h7741962f5392b059 + 631
8   a                              0x000d13cc main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff31700  ebx: 0xbff31680  ecx: 0x000cfe6e  edx: 0xbff31680
  edi: 0x000ce624  esi: 0x00000000  ebp: 0xbff31678  esp: 0xbff31640
   ss: 0x00000023  efl: 0x00010246  eip: 0x000cfec6   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000cfb00
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xcd000 -    0xd1fef +a (0) <FBCF3D38-564F-34CC-A2AF-3FF78772D870> /Users/USER/*/a
  0x191000 -   0x1d6fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x21a000 -   0x2a9ff3 +libstd-22969636c9515299.dylib (0) <9339C2F6-C93C-34A0-B00E-77391B2E6BDA> /Users/USER/*/libstd-22969636c9515299.dylib
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
    task_for_pid: 2591
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
__DATA                            3564K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-20-140154-1_Traviss-Mac-1044.crash
Process:               a [47778]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [47777]
Responsible:           a [47778]
User ID:               501
Date/Time:             2019-03-20 14:01:54.273 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5800 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf0cad8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf0cad8:
    Stack Guard            00000000bbf0b000-00000000bbf0c000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf0c000-00000000bbf0d000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf0d000-00000000bff0b000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-22969636c9515299.dylib  0x0024efbb std::sys::unix::abort_internal::h07a492646837a647 + 11
4   libstd-22969636c9515299.dylib  0x0023f459 std::sys_common::util::abort::h85f0045aaa34085e + 73
5   libstd-22969636c9515299.dylib  0x0024e388 std::sys::unix::stack_overflow::imp::signal_handler::h907965a5b4fdf2cd + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-22969636c9515299.dylib  0x0024dfd0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb6457c1a448aaf7a + 80
9   a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
265 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
266 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
267 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
268 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
269 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
270 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
271 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
272 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
273 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
274 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
275 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
276 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
277 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
278 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
279 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
280 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
281 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
282 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
283 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
284 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
285 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
286 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
287 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
288 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
289 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
290 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
291 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
292 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
293 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
294 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
295 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
296 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
297 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
298 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
299 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
300 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
301 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
302 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
303 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
304 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
305 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
306 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
307 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
308 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
309 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
310 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
311 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
312 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
313 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
314 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
315 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
316 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
317 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
318 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
319 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
320 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
321 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
322 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
323 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
324 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
325 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
326 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
327 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
328 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
329 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
330 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
331 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
332 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
333 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
334 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
335 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
336 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
337 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
338 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
339 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
340 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
341 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
342 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
343 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
344 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
345 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
346 a                              0x000f8950 stack_probes::recurse::h24283d9484398da0 + 48
---
===========                     =======  ======= 
TOTAL                            569.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-20-140154_Traviss-Mac-1044.crash
Process:               a [47782]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47777]
Responsible:           a [47782]
User ID:               501
Date/Time:             2019-03-20 14:01:54.333 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5800 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0765ea8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0765ea8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0765000-00000000b0766000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0766000-00000000b0967000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-22969636c9515299.dylib  0x001958f0 std::sys::unix::thread::Thread::join::hd7e5c931c45c15e8 + 32
4   a                              0x000995e6 std::thread::JoinHandle$LT$T$GT$::join::h33e33cfdb2abe1eb + 70
5   a                              0x00098805 stack_probes::main::hc5f49a55fd7e038b + 597
6   a                              0x0009761b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h81a1e1a591ca2243 + 11
7   libstd-22969636c9515299.dylib  0x0018590c std::sys_common::backtrace::__rust_begin_short_backtrace::h941085b4bbfcfc5d + 12
8   libstd-22969636c9515299.dylib  0x00187b54 std::panicking::try::do_call::h82374a59315181b8 + 20
9   libstd-22969636c9515299.dylib  0x00196b3d __rust_maybe_catch_panic + 29
10  libstd-22969636c9515299.dylib  0x001885f7 std::rt::lang_start_internal::h7741962f5392b059 + 631
11  a                              0x0009922c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-22969636c9515299.dylib  0x00195fbb std::sys::unix::abort_internal::h07a492646837a647 + 11
4   libstd-22969636c9515299.dylib  0x00186459 std::sys_common::util::abort::h85f0045aaa34085e + 73
5   libstd-22969636c9515299.dylib  0x00195388 std::sys::unix::stack_overflow::imp::signal_handler::h907965a5b4fdf2cd + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-22969636c9515299.dylib  0x00194fd0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb6457c1a448aaf7a + 80
9   a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x00098950 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x0009767d std::sys_common::backtrace::__rust_begin_short_backtrace::hfddadf83d3ee3663 (.llvm.13914630499615147088) + 29
265 libstd-22969636c9515299.dylib  0x00196b3d __rust_maybe_catch_panic + 29
266 a                              0x00099b13 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h6c5f8e6e1e7f23c9 + 131
267 libstd-22969636c9515299.dylib  0x0019582b std::sys::unix::thread::Thread::new::thread_start::h67faff1a27fc81d6 + 187
268 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
269 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
270 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb0966000  ecx: 0x004a6b0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x004a6b38  esp: 0x004a6b0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x96000 -    0x9afff +a (0) <C3AC74DE-89EA-3C22-8374-C8F7291143D4> /Users/USER/*/a
   0xde000 -   0x123fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x167000 -   0x1f6ff3 +libstd-22969636c9515299.dylib (0) <9339C2F6-C93C-34A0-B00E-77391B2E6BDA> /Users/USER/*/libstd-22969636c9515299.dylib
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
    task_for_pid: 2591
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
__DATA                            3564K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-20-140203-1_Traviss-Mac-1044.crash
Process:               a [47898]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47896]
Responsible:           a [47898]
User ID:               501
Date/Time:             2019-03-20 14:02:00.924 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5800 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0148f08
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0148f08:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0148000-00000000b0149000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0149000-00000000b034a000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   a                              0x00039689 stack_probes_lto::main::h7f9d4bbc5a99dd1a + 2457
4   a                              0x000524fb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h28021a1bb2111190 + 11
5   a                              0x0004accc std::sys_common::backtrace::__rust_begin_short_backtrace::h941085b4bbfcfc5d + 12
6   a                              0x0003adad main + 589
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x0003c12b std::sys::unix::abort_internal::h07a492646837a647 + 11
4   a                              0x0003c119 std::sys_common::util::abort::h85f0045aaa34085e + 73
5   a                              0x0004ac2c std::sys::unix::stack_overflow::imp::signal_handler::h907965a5b4fdf2cd + 860
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x0004a8d0 rust_begin_unwind + 16
9   a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
10  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
11  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
12  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
13  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
14  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
15  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
16  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
17  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
18  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
19  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
20  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
21  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
22  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
23  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
24  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
25  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
26  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
27  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
28  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
29  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
30  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
31  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
32  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
33  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
34  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
35  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
36  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
37  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
38  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
39  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
40  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
41  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
42  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
43  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
44  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
45  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
46  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
47  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
48  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
49  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
50  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
51  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
52  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
53  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
54  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
55  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
56  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
57  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
58  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
59  a                              0x00039b38 stack_probes_lto::recurse::h907252696a8f0ddd + 40
