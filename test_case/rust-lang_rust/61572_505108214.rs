plain
[00:03:40]       Memory: 8 GB
[00:03:40]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:40]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:40]       SMC Version (system): 2.8f0
[00:03:40]       Serial Number (system): VMnfXbU4aHD3
[00:03:40] 
[00:03:40] hw.ncpu: 4
[00:03:40] hw.byteorder: 1234
[00:03:40] hw.memsize: 8589934592
---
[01:31:32] stdout:
[01:31:32] ------------------------------------------
[01:31:32] 
[01:31:32] running 3 tests
[01:31:32] test src/test/rustdoc/test_option_check/test.rs - Bar (line 15) ... FAILED
[01:31:32] test src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6) ... ok
[01:31:32] test src/test/rustdoc/test_option_check/test.rs - Foo (line 8) ... ok
[01:31:32] failures:
[01:31:32] 
[01:31:32] ---- src/test/rustdoc/test_option_check/test.rs - Bar (line 15) stdout ----
[01:31:32] error: linking with `cc` failed: signal: 4
[01:31:32] error: linking with `cc` failed: signal: 4
[01:31:32]   |
[01:31:32]   = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlZdIJ2/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlZdIJ2/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlZdIJ2/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlZdIJ2/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlZdIJ2/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlZdIJ2/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlZdIJ2/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestlZdIJ2/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/rustdoc/test_option_check/test/auxiliary" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libstd-b4e8967dfe831496.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libpanic_unwind-af3315b059b0b1d4.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libbacktrace-8437e0dafd1fffc6.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libbacktrace_sys-a217f92577a9b3cc.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_demangle-5a010823a37e6f01.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libhashbrown-7fa6c149d950f8eb.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_alloc-eb52043434d3159b.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libunwind-67a4c100d304f27d.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcfg_if-7baaa299e2e2dc93.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liblibc-10c9cfc6f6ba5bdd.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liballoc-101fade3a60e1e01.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_core-1289226b8282c55e.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcore-bfc5ef2087e01f43.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-d940584a0bad99e6.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[01:31:32] 
[01:31:32] error: aborting due to previous error
[01:31:32] 
[01:31:32] Couldn't compile the test.
---
[01:31:32] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:31:32] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:31:32] 
[01:31:32] 
[01:31:32] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/rustdoc" "--build-base" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/rustdoc" "--stage-id" "stage2-i686-apple-darwin" "--mode" "rustdoc" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0-rust-1.37.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:31:32] 
[01:31:32] 
[01:31:32] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:31:32] Build completed unsuccessfully in 1:27:50
---
travis_fold:start:after_failure.2
travis_time:start:0d6ddc4e
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1200
drwx------  22 travis  staff    748 Jun 24 17:35 .
-rw-------@  1 travis  staff  37880 Jun 24 17:35 rustdoc_2019-06-24-173552_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34964 Jun 24 17:22 a_2019-06-24-172222-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  57364 Jun 24 17:22 a_2019-06-24-172222_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34863 Jun 24 17:22 a_2019-06-24-172213-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55585 Jun 24 17:22 a_2019-06-24-172213_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 Jun 24 17:22 a_2019-06-24-172206_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9251 Jun 24 17:22 a_2019-06-24-172200_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8936 Jun 24 17:21 a_2019-06-24-172159-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9262 Jun 24 17:21 a_2019-06-24-172159_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9305 Jun 24 17:21 a_2019-06-24-172117_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58251 Jun 24 17:21 a_2019-06-24-172101_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 Jun 24 17:21 a_2019-06-24-172100-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59516 Jun 24 17:21 a_2019-06-24-172100-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60372 Jun 24 17:21 a_2019-06-24-172100_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10877 Jun 24 17:18 a_2019-06-24-171847_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 Jun 24 17:17 a_2019-06-24-171751_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 Jun 24 17:16 a_2019-06-24-171633_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9780 Jun 24 17:15 a_2019-06-24-171534-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 Jun 24 17:15 a_2019-06-24-171534_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9482 Jun 24 17:15 a_2019-06-24-171529_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1631ba3f
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-24-171529_Traviss-Mac-1044.crash
Process:               a [37585]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [37581]
Responsible:           a [37585]
User ID:               501
Date/Time:             2019-06-24 17:15:04.028 +0000
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
0   a                              0x00063a4e abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 46
1   a                              0x00062eab std::panicking::try::do_call::h6eb939d268ee836e (.llvm.561488243965954259) + 11
2   libstd-b4e8967dfe831496.dylib  0x00217e5d __rust_maybe_catch_panic + 29
3   a                              0x00063cb5 abort_on_c_abi::main::he771bf881fc862e3 + 613
4   a                              0x0006250b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hbd0748688a4e2ac9 + 11
5   libstd-b4e8967dfe831496.dylib  0x0020656c std::sys_common::backtrace::__rust_begin_short_backtrace::hdf174f700ce58224 + 12
6   libstd-b4e8967dfe831496.dylib  0x00209824 std::panicking::try::do_call::hb8d25ca477c5169a + 20
7   libstd-b4e8967dfe831496.dylib  0x00217e5d __rust_maybe_catch_panic + 29
8   libstd-b4e8967dfe831496.dylib  0x0020a2c7 std::rt::lang_start_internal::h98c9fcd71d147232 + 631
9   a                              0x00063fec main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x7bf0a140  ebx: 0xbff9d6a8  ecx: 0x00000000  edx: 0x00000000
  edi: 0x00217e4e  esi: 0x00000000  ebp: 0xbff9d648  esp: 0xbff9d630
   ss: 0x00000023  efl: 0x00010296  eip: 0x00063a4e   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0025c2d4
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x61000 -    0x64ffb +a (0) <61F340D0-3C8B-30A5-9B85-EF619DF56AD9> /Users/USER/*/a
  0x160000 -   0x1a5fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1e9000 -   0x282ff3 +libstd-b4e8967dfe831496.dylib (0) <BDB05B30-4FAD-3299-952A-EDF0B59D66C9> /Users/USER/*/libstd-b4e8967dfe831496.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=18.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=18.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3548K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9384K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.6M      134 
TOTAL                            569.6M      134 
TOTAL, minus reserved VM space   569.5M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-24-171534-1_Traviss-Mac-1044.crash
Process:               a [38347]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [38343]
Responsible:           a [38347]
User ID:               501
Date/Time:             2019-06-24 17:15:33.574 +0000
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
0   libstd-b4e8967dfe831496.dylib  0x001bdd33 std::panicking::rust_panic_with_hook::hf2dc7d84c13c9d84 + 115
1   a                              0x000e29ff std::panicking::begin_panic::h60ecdc50edc90087 + 47 (panicking.rs:409)
2   a                              0x000e0184 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 36 (backtrace.rs:25)
3   a                              0x000df90b core::ptr::real_drop_in_place::hd5524a8477e0f1c0 + 11
4   a                              0x000e0153 backtrace::double::h35adec2a6f63ef6c + 51
5   a                              0x000e1778 backtrace::main::hc9a5bc8fc93ded64 + 5352 (backtrace.rs:119)
6   a                              0x000df63b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb55d8e362dc73a64 + 11 (rt.rs:64)
7   libstd-b4e8967dfe831496.dylib  0x001ba56c std::sys_common::backtrace::__rust_begin_short_backtrace::hdf174f700ce58224 + 12
8   libstd-b4e8967dfe831496.dylib  0x001bd824 std::panicking::try::do_call::hb8d25ca477c5169a + 20
9   libstd-b4e8967dfe831496.dylib  0x001cbe5d __rust_maybe_catch_panic + 29
10  libstd-b4e8967dfe831496.dylib  0x001be2c7 std::rt::lang_start_internal::h98c9fcd71d147232 + 631
11  a                              0x000e201c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff223f8  ebx: 0xbff22440  ecx: 0xbff222f0  edx: 0xa7702ec6
  edi: 0x002105f8  esi: 0x001bdcce  ebp: 0xbff22498  esp: 0xbff22410
   ss: 0x00000023  efl: 0x00010282  eip: 0x001bdd33   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x7be8d000
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xdc000 -    0xe3ff7 +a (0) <5C60E090-FA8F-30AB-820D-FC10C3FC38A8> /Users/USER/*/a
  0x114000 -   0x159fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x19d000 -   0x236ff3 +libstd-b4e8967dfe831496.dylib (0) <BDB05B30-4FAD-3299-952A-EDF0B59D66C9> /Users/USER/*/libstd-b4e8967dfe831496.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.3M resident=0K(0%) swapped_out_or_unallocated=83.3M(100%)
Writable regions: Total=17.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9632K       10 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3548K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9400K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.0M      136 
TOTAL                            569.0M      136 
TOTAL, minus reserved VM space   568.9M      136 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-24-171534_Traviss-Mac-1044.crash
Process:               a [38348]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [38343]
Responsible:           a [38348]
User ID:               501
Date/Time:             2019-06-24 17:15:33.653 +0000
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
0   libstd-b4e8967dfe831496.dylib  0x00124d33 std::panicking::rust_panic_with_hook::hf2dc7d84c13c9d84 + 115
1   a                              0x0000f9ff std::panicking::begin_panic::h60ecdc50edc90087 + 47 (panicking.rs:409)
2   a                              0x0000d184 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 36 (backtrace.rs:25)
3   a                              0x0000c90b core::ptr::real_drop_in_place::hd5524a8477e0f1c0 + 11
4   a                              0x0000d153 backtrace::double::h35adec2a6f63ef6c + 51
5   a                              0x0000e778 backtrace::main::hc9a5bc8fc93ded64 + 5352 (backtrace.rs:119)
6   a                              0x0000c63b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb55d8e362dc73a64 + 11 (rt.rs:64)
7   libstd-b4e8967dfe831496.dylib  0x0012156c std::sys_common::backtrace::__rust_begin_short_backtrace::hdf174f700ce58224 + 12
8   libstd-b4e8967dfe831496.dylib  0x00124824 std::panicking::try::do_call::hb8d25ca477c5169a + 20
9   libstd-b4e8967dfe831496.dylib  0x00132e5d __rust_maybe_catch_panic + 29
10  libstd-b4e8967dfe831496.dylib  0x001252c7 std::rt::lang_start_internal::h98c9fcd71d147232 + 631
11  a                              0x0000f01c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbfff53e8  ebx: 0xbfff5430  ecx: 0xbfff52e0  edx: 0xa7702ec6
  edi: 0x001775f8  esi: 0x00124cce  ebp: 0xbfff5488  esp: 0xbfff5400
   ss: 0x00000023  efl: 0x00010282  eip: 0x00124d33   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0082b000
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
    0x9000 -    0x10ff7 +a (0) <5C60E090-FA8F-30AB-820D-FC10C3FC38A8> /Users/USER/*/a
   0x7b000 -    0xc0fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x104000 -   0x19dff3 +libstd-b4e8967dfe831496.dylib (0) <BDB05B30-4FAD-3299-952A-EDF0B59D66C9> /Users/USER/*/libstd-b4e8967dfe831496.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.3M resident=0K(0%) swapped_out_or_unallocated=83.3M(100%)
Writable regions: Total=26.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=26.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.4M       10 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3548K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9400K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            578.0M      136 
TOTAL                            578.0M      136 
TOTAL, minus reserved VM space   577.9M      136 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-24-171633_Traviss-Mac-1044.crash
Process:               a [40072]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [40070]
Responsible:           a [40072]
User ID:               501
Date/Time:             2019-06-24 17:16:32.993 +0000
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
3   libstd-b4e8967dfe831496.dylib  0x001fa2db std::sys::unix::abort_internal::h16150b015f3f5168 + 11
4   libstd-b4e8967dfe831496.dylib  0x001ebd80 rust_oom + 48
5   libstd-b4e8967dfe831496.dylib  0x002119d4 alloc::alloc::handle_alloc_error::hcfc92d053cb11f4f + 20
6   a                              0x0008d4cd default_alloc_error_hook::main::h0fe124586986ad5d + 781
7   a                              0x0008c8eb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h1682164a1f617d88 + 11
8   libstd-b4e8967dfe831496.dylib  0x001e956c std::sys_common::backtrace::__rust_begin_short_backtrace::hdf174f700ce58224 + 12
9   libstd-b4e8967dfe831496.dylib  0x001ec824 std::panicking::try::do_call::hb8d25ca477c5169a + 20
10  libstd-b4e8967dfe831496.dylib  0x001fae5d __rust_maybe_catch_panic + 29
11  libstd-b4e8967dfe831496.dylib  0x001ed2c7 std::rt::lang_start_internal::h98c9fcd71d147232 + 631
12  a                              0x0008d62c main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff725cc  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff725f8  esp: 0xbff725cc
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x8c000 -    0x8dffb +a (0) <EB9D909D-1EE6-3055-8592-57433779E9B7> /Users/USER/*/a
  0x143000 -   0x188fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1cc000 -   0x265ff3 +libstd-b4e8967dfe831496.dylib (0) <BDB05B30-4FAD-3299-952A-EDF0B59D66C9> /Users/USER/*/libstd-b4e8967dfe831496.dylib
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
---
===========                     =======  ======= 
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-24-172100-2_Traviss-Mac-1044.crash
Process:               a [46524]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [46522]
Responsible:           a [46524]
User ID:               501
Date/Time:             2019-06-24 17:20:59.714 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4700 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0132fec
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0132fec:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0132000-00000000b0133000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0133000-00000000b0334000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-b4e8967dfe831496.dylib  0x001eeae0 std::sys::unix::thread::Thread::join::ha98b3ce7abd668b3 + 32
4   a                              0x0003f036 std::thread::JoinHandle$LT$T$GT$::join::hcd97b90c2f54ac33 + 70
5   a                              0x0003dcef out_of_stack::main::h51a6e88d0363a411 + 255
6   a                              0x0003d0fb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h39fce4c3b765deb0 + 11
7   libstd-b4e8967dfe831496.dylib  0x001de56c std::sys_common::backtrace::__rust_begin_short_backtrace::hdf174f700ce58224 + 12
8   libstd-b4e8967dfe831496.dylib  0x001e1824 std::panicking::try::do_call::hb8d25ca477c5169a + 20
9   libstd-b4e8967dfe831496.dylib  0x001efe5d __rust_maybe_catch_panic + 29
10  libstd-b4e8967dfe831496.dylib  0x001e22c7 std::rt::lang_start_internal::h98c9fcd71d147232 + 631
11  a                              0x0003eacc main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-b4e8967dfe831496.dylib  0x001ef2db std::sys::unix::abort_internal::h16150b015f3f5168 + 11
4   libstd-b4e8967dfe831496.dylib  0x001dff99 std::sys_common::util::abort::h172cbba7f0e37c0e + 73
5   libstd-b4e8967dfe831496.dylib  0x001ee598 std::sys::unix::stack_overflow::imp::signal_handler::hc293ff2bba79270f + 744
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-b4e8967dfe831496.dylib  0x001ee2b0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hc388ac34d0ab2c0a + 80
9   libstd-b4e8967dfe831496.dylib  0x001cd8bc _$LT$std..io..buffered..LineWriter$LT$W$GT$$u20$as$u20$std..io..Write$GT$::write::hb4e8db10506717bb + 220
10  libstd-b4e8967dfe831496.dylib  0x001d2005 std::io::Write::write_all::h75b46d67dd6f8b16 + 101
11  libstd-b4e8967dfe831496.dylib  0x001d2513 _$LT$std..io..Write..write_fmt..Adaptor$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$::write_str::h61275d623c91ee96 + 35
12  libstd-b4e8967dfe831496.dylib  0x0021473d core::fmt::write::h4525a2ca55d60faf + 749
13  libstd-b4e8967dfe831496.dylib  0x001d0416 _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::h68fb758a450ce0b2 + 182
14  libstd-b4e8967dfe831496.dylib  0x001d16db std::io::stdio::_print::h77c4fca210fcdcc0 + 155
15  a                              0x0003dbdf out_of_stack::loud_recurse::h48447a21f69c373e + 63
16  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
17  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
18  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
19  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
20  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
21  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
22  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
23  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
24  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
25  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
26  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
27  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
28  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
29  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
30  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
31  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
32  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
33  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
34  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
35  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
36  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
37  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
38  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
39  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
40  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
41  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
42  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
43  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
44  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
45  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
46  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
47  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
48  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
49  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
50  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
51  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
52  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
53  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
54  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
55  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
56  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
57  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
58  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
59  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
60  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
61  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
62  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
63  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
64  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
65  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
66  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
67  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
68  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
69  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
70  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
71  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
72  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
73  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
74  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
75  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
76  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
77  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
78  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
79  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
80  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
81  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
82  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
83  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
84  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
85  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
86  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
87  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
88  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
89  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
90  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
91  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
92  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
93  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
94  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
95  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
96  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
97  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
98  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
99  a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
100 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
101 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
102 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
103 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
104 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
105 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
106 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
107 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
108 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
109 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
110 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
111 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
112 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
113 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
114 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
115 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
116 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
117 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
118 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
119 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
120 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
121 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
122 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
123 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
124 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
125 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
126 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
127 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
128 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
129 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
130 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
131 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
132 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
133 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
134 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
135 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
136 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
137 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
138 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
139 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
140 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
141 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
142 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
143 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
144 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
145 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
146 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
147 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
148 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
149 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
150 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
151 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
152 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
153 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
154 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
155 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
156 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
157 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
158 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
159 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
160 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
161 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
162 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
163 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
164 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
165 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
166 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
167 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
168 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
169 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
170 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
171 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
172 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
173 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
174 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
175 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
176 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
177 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
178 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
179 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
180 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
181 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
182 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
183 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
184 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
185 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
186 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
187 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
188 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
189 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
190 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
191 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
192 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
193 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
194 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
195 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
196 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
197 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
198 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
199 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
200 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
201 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
202 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
203 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
204 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
205 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
206 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
207 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
208 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
209 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
210 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
211 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
212 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
213 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
214 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
215 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
216 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
217 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
218 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
219 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
220 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
221 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
222 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
223 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
224 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
225 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
226 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
227 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
228 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
229 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
230 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
231 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
232 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
233 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
234 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
235 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
236 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
237 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
238 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
239 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
240 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
241 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
242 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
243 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
244 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
245 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
246 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
247 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
248 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
249 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
250 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
251 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
252 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
253 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
254 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
255 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
256 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
257 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
258 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
259 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
260 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
261 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
262 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
263 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
264 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
265 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
266 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
267 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
268 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
269 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
270 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
271 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
272 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
273 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
274 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
275 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
276 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
277 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
278 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
279 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
280 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
281 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
282 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
283 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
284 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
285 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
286 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
287 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
288 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
289 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
290 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
291 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
292 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
293 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
294 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
295 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
296 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
297 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
298 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
299 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
300 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
301 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
302 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
303 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
304 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
305 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
306 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
307 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
308 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
309 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
310 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
311 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
312 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
313 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
314 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
315 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
316 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
317 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
318 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
319 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
320 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
321 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
322 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
323 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
324 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
325 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
326 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
327 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
328 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
329 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
330 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
331 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
332 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
333 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
334 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
335 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
336 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
337 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
338 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
339 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
340 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
341 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
342 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
343 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
344 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
345 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
346 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
347 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
348 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
349 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
350 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
351 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
352 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
353 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
354 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
355 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
356 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
357 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
358 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
359 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
360 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
361 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
362 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
363 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
364 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
365 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
366 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
367 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
368 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
369 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
370 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
371 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
372 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
373 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
374 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
375 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
376 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
377 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
378 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
379 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
380 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
381 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
382 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
383 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
384 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
385 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
386 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
387 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
388 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
389 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
390 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
391 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
392 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
393 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
394 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
395 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
396 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
397 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
398 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
399 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
400 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
401 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
402 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
403 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
404 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
405 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
406 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
407 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
408 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
409 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
410 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
411 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
412 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
413 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
414 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
415 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
416 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
417 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
418 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
419 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
420 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
421 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
422 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
423 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
424 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
425 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
426 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
427 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
428 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
429 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
430 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
431 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
432 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
433 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
434 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
435 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
436 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
437 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
438 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
439 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
440 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
441 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
442 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
443 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
444 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
445 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
446 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
447 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
448 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
449 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
450 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
451 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
452 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
453 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
454 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
455 a                              0x0003dbe4 out_of_stack::loud_recurse::h48447a21f69c373e + 68
---
===========                     =======  ======= 
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-24-172117_Traviss-Mac-1044.crash
Process:               a [46741]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [46740]
Responsible:           a [46741]
User ID:               501
Date/Time:             2019-06-24 17:21:15.236 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4700 seconds
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
3   a                              0x000ab75b panic_abort::__rust_start_panic::abort::h8aca610fbe16975e + 11
4   a                              0x000ab74b __rust_start_panic + 11
5   a                              0x000a1f9b rust_panic + 11
6   a                              0x000a1b9e std::panicking::rust_panic_with_hook::hf2dc7d84c13c9d84 + 1070
7   a                              0x000b71fa std::panicking::begin_panic::h5892a7ad06d41f7b + 42
8   a                              0x000a0caf lto_abort::main::hd3a7c21d2a5e6b65 + 2879
9   a                              0x000b735b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h601bcf97cc3111f2 + 11
10  a                              0x000ab5cc std::sys_common::backtrace::__rust_begin_short_backtrace::hdf174f700ce58224 + 12
11  a                              0x000a0f35 main + 645
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff5f5ac  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff5f5d8  esp: 0xbff5f5ac
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x9f000 -    0xc6ffb +a (0) <58A7FE80-78DB-302E-B37C-4AD6FFBA0538> /Users/USER/*/a
  0x19c000 -   0x1e1fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=82.4M resident=0K(0%) swapped_out_or_unallocated=82.4M(100%)
Writable regions: Total=17.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            1336K       43 
__LINKEDIT                        73.7M        4 
__OBJC                              36K        6 
__TEXT                            8912K       43 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            565.6M      131 
TOTAL                            565.6M      131 
TOTAL, minus reserved VM space   565.5M      131 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-24-172159-1_Traviss-Mac-1044.crash
Process:               a [47697]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47688]
Responsible:           a [47697]
User ID:               501
Date/Time:             2019-06-24 17:21:54.999 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
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
3   libstd-b4e8967dfe831496.dylib  0x001622db std::sys::unix::abort_internal::h16150b015f3f5168 + 11
4   libstd-b4e8967dfe831496.dylib  0x00152f99 std::sys_common::util::abort::h172cbba7f0e37c0e + 73
5   libstd-b4e8967dfe831496.dylib  0x00155042 rust_panic + 98
6   libstd-b4e8967dfe831496.dylib  0x00154f0e std::panicking::rust_panic_with_hook::hf2dc7d84c13c9d84 + 590
7   a                              0x0007e9cf std::panicking::begin_panic::h04ad2760ca9e117d + 47
8   a                              0x0007fb0c main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff8159c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff815c8  esp: 0xbff8159c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x7d000 -    0x80fff +a (0) <94E7FCC3-B5FA-398D-A181-A64D4CEB56D2> /Users/USER/*/a
   0xab000 -    0xf0fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x134000 -   0x1cdff3 +libstd-b4e8967dfe831496.dylib (0) <BDB05B30-4FAD-3299-952A-EDF0B59D66C9> /Users/USER/*/libstd-b4e8967dfe831496.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=18.2M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=18.2M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
__DATA                            3548K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__OBJC                              36K        6 
__TEXT                            9384K       44 
mapped file                      408.7M       21 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.5M      132 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-24-172159_Traviss-Mac-1044.crash
Process:               a [47722]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47720]
Responsible:           a [47722]
User ID:               501
Date/Time:             2019-06-24 17:21:55.807 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
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
    __TEXT                 00000000000c2000-00000000000c5000 [   12K] r-x/rwx SM=COW  Ԓ [/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-pass/segfault-no-out-of-stack/a]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000c3f62 segfault_no_out_of_stack::main::h579ee62fe996355a + 2034
1   a                              0x000c298b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h9df8bd3d742d7899 + 11
2   libstd-b4e8967dfe831496.dylib  0x001f956c std::sys_common::backtrace::__rust_begin_short_backtrace::hdf174f700ce58224 + 12
3   libstd-b4e8967dfe831496.dylib  0x001fc824 std::panicking::try::do_call::hb8d25ca477c5169a + 20
4   libstd-b4e8967dfe831496.dylib  0x0020ae5d __rust_maybe_catch_panic + 29
5   libstd-b4e8967dfe831496.dylib  0x001fd2c7 std::rt::lang_start_internal::h98c9fcd71d147232 + 631
6   a                              0x000c423c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x7b61e1e0  ecx: 0x00000000  edx: 0x00000000
  edi: 0x0020ae4e  esi: 0xbff3c6d0  ebp: 0xbff3c7b8  esp: 0xbff3c610
   ss: 0x00000023  efl: 0x00010246  eip: 0x000c3f62   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     1
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xc2000 -    0xc4ffb +a (0) <37D196BA-6781-3C4D-8412-EF3F6E8196FB> /Users/USER/*/a
  0x153000 -   0x198fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1dc000 -   0x275ff3 +libstd-b4e8967dfe831496.dylib (0) <BDB05B30-4FAD-3299-952A-EDF0B59D66C9> /Users/USER/*/libstd-b4e8967dfe831496.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=17.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                        132K        3 
__DATA                            3548K       44 
__LINKEDIT                        74.1M        5 
---
===========                     =======  ======= 
TOTAL                            568.6M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-24-172200_Traviss-Mac-1044.crash
Process:               a [47804]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47803]
Responsible:           a [47804]
User ID:               501
Date/Time:             2019-06-24 17:21:59.048 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
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
    __TEXT                 000000000004e000-0000000000051000 [   12K] r-x/rwx SM=COW  Ԓ [/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-pass/signal-exit-status/a]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000505f4 signal_exit_status::main::hc6663d816ec186eb + 436
1   a                              0x0004f49b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc8aa8e5c768c895a + 11
2   libstd-b4e8967dfe831496.dylib  0x0012a56c std::sys_common::backtrace::__rust_begin_short_backtrace::hdf174f700ce58224 + 12
3   libstd-b4e8967dfe831496.dylib  0x0012d824 std::panicking::try::do_call::hb8d25ca477c5169a + 20
4   libstd-b4e8967dfe831496.dylib  0x0013be5d __rust_maybe_catch_panic + 29
5   libstd-b4e8967dfe831496.dylib  0x0012e2c7 std::rt::lang_start_internal::h98c9fcd71d147232 + 631
6   a                              0x000506cc main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x7be34fe0
  edi: 0x7be35070  esi: 0xbffb0740  ebp: 0xbffb07d8  esp: 0xbffb06c0
   ss: 0x00000023  efl: 0x00010246  eip: 0x000505f4   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     3
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x4e000 -    0x50ff7 +a (0) <293BE013-A3F6-368B-853B-DF34A2B0CA9B> /Users/USER/*/a
   0x84000 -    0xc9fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x10d000 -   0x1a6ff3 +libstd-b4e8967dfe831496.dylib (0) <BDB05B30-4FAD-3299-952A-EDF0B59D66C9> /Users/USER/*/libstd-b4e8967dfe831496.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=17.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                        132K        3 
__DATA                            3548K       44 
__LINKEDIT                        74.1M        5 
---
===========                     =======  ======= 
TOTAL                            568.6M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-24-172206_Traviss-Mac-1044.crash
Process:               a [47901]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47898]
Responsible:           a [47901]
User ID:               501
Date/Time:             2019-06-24 17:22:04.480 +0000
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
0   a                              0x000fe8e6 simd_target_feature_mixup::test::id_avx512_512::hf3a1395d43161fbe + 102
1   a                              0x000fd69f simd_target_feature_mixup::test::main::h611f15c116e6c273 + 1647
2   a                              0x000ffd60 simd_target_feature_mixup::main::hfcef50d014c08dda + 896
3   a                              0x000fcdab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h633392e18ebb89d6 + 11
4   libstd-b4e8967dfe831496.dylib  0x001c056c std::sys_common::backtrace::__rust_begin_short_backtrace::hdf174f700ce58224 + 12
5   libstd-b4e8967dfe831496.dylib  0x001c3824 std::panicking::try::do_call::hb8d25ca477c5169a + 20
6   libstd-b4e8967dfe831496.dylib  0x001d1e5d __rust_maybe_catch_panic + 29
7   libstd-b4e8967dfe831496.dylib  0x001c42c7 std::rt::lang_start_internal::h98c9fcd71d147232 + 631
8   a                              0x000fff3c main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff02400  ebx: 0xbff02380  ecx: 0x000fe88e  edx: 0xbff02380
  edi: 0x000fd044  esi: 0x00000000  ebp: 0xbff02378  esp: 0xbff02340
   ss: 0x00000023  efl: 0x00010246  eip: 0x000fe8e6   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000fe520
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xfc000 -   0x100fcf +a (0) <F313BE1A-E0C0-36AD-AE0F-3A363CA8FF6D> /Users/USER/*/a
  0x11a000 -   0x15ffdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1a3000 -   0x23cff3 +libstd-b4e8967dfe831496.dylib (0) <BDB05B30-4FAD-3299-952A-EDF0B59D66C9> /Users/USER/*/libstd-b4e8967dfe831496.dylib
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
    task_for_pid: 2336
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=17.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3548K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9388K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            568.6M      134 
TOTAL                            568.6M      134 
TOTAL, minus reserved VM space   568.5M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-24-172213-1_Traviss-Mac-1044.crash
Process:               a [48056]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [48050]
Responsible:           a [48056]
User ID:               501
Date/Time:             2019-06-24 17:22:10.263 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0143e68
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0143e68:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0143000-00000000b0144000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0144000-00000000b0345000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-b4e8967dfe831496.dylib  0x001b8ae0 std::sys::unix::thread::Thread::join::ha98b3ce7abd668b3 + 32
4   a                              0x00043666 std::thread::JoinHandle$LT$T$GT$::join::h387ccd7a04e46b39 + 70
5   a                              0x00042885 stack_probes::main::h385e455299c91d04 + 597
6   a                              0x000416ab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h4ea758ab3c70bfd0 + 11
7   libstd-b4e8967dfe831496.dylib  0x001a856c std::sys_common::backtrace::__rust_begin_short_backtrace::hdf174f700ce58224 + 12
8   libstd-b4e8967dfe831496.dylib  0x001ab824 std::panicking::try::do_call::hb8d25ca477c5169a + 20
9   libstd-b4e8967dfe831496.dylib  0x001b9e5d __rust_maybe_catch_panic + 29
10  libstd-b4e8967dfe831496.dylib  0x001ac2c7 std::rt::lang_start_internal::h98c9fcd71d147232 + 631
11  a                              0x000432ac main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-b4e8967dfe831496.dylib  0x001b92db std::sys::unix::abort_internal::h16150b015f3f5168 + 11
4   libstd-b4e8967dfe831496.dylib  0x001a9f99 std::sys_common::util::abort::h172cbba7f0e37c0e + 73
5   libstd-b4e8967dfe831496.dylib  0x001b8598 std::sys::unix::stack_overflow::imp::signal_handler::hc293ff2bba79270f + 744
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-b4e8967dfe831496.dylib  0x001b82b0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hc388ac34d0ab2c0a + 80
9   a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
10  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
11  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
12  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
13  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
14  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
15  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
16  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
17  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
18  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
19  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
20  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
21  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
22  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
23  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
24  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
25  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
26  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
27  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
28  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
29  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
30  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
31  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
32  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
33  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
34  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
35  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
36  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
37  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
38  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
39  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
40  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
41  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
42  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
43  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
44  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
45  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
46  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
47  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
48  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
49  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
50  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
51  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
52  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
53  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
54  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
55  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
56  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
57  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
58  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
59  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
60  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
61  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
62  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
63  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
64  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
65  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
66  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
67  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
68  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
69  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
70  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
71  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
72  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
73  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
74  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
75  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
76  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
77  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
78  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
79  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
80  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
81  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
82  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
83  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
84  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
85  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
86  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
87  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
88  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
89  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
90  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
91  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
92  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
93  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
94  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
95  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
96  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
97  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
98  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
99  a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
100 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
101 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
102 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
103 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
104 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
105 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
106 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
107 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
108 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
109 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
110 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
111 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
112 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
113 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
114 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
115 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
116 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
117 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
118 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
119 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
120 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
121 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
122 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
123 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
124 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
125 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
126 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
127 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
128 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
129 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
130 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
131 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
132 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
133 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
134 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
135 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
136 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
137 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
138 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
139 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
140 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
141 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
142 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
143 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
144 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
145 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
146 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
147 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
148 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
149 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
150 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
151 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
152 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
153 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
154 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
155 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
156 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
157 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
158 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
159 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
160 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
161 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
162 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
163 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
164 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
165 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
166 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
167 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
168 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
169 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
170 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
171 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
172 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
173 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
174 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
175 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
176 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
177 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
178 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
179 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
180 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
181 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
182 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
183 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
184 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
185 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
186 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
187 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
188 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
189 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
190 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
191 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
192 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
193 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
194 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
195 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
196 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
197 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
198 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
199 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
200 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
201 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
202 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
203 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
204 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
205 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
206 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
207 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
208 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
209 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
210 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
211 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
212 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
213 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
214 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
215 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
216 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
217 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
218 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
219 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
220 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
221 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
222 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
223 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
224 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
225 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
226 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
227 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
228 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
229 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
230 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
231 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
232 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
233 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
234 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
235 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
236 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
237 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
238 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
239 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
240 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
241 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
242 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
243 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
244 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
245 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
246 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
247 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
248 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
249 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
250 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
251 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
252 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
253 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
254 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
255 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
256 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
257 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
258 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
259 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
260 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
261 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
262 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
263 a                              0x000429d0 stack_probes::recurse::h35e743424666f0f3 + 48
264 a                              0x0004174d std::sys_common::backtrace::__rust_begin_short_backtrace::hab58155ca92a13cb + 29
265 libstd-b4e8967dfe831496.dylib  0x001b9e5d __rust_maybe_catch_panic + 29
266 a                              0x000439c3 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h708364346184c3eb + 131
267 libstd-b4e8967dfe831496.dylib  0x00190e81 _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hd45bbdc831375cfb + 65
268 libstd-b4e8967dfe831496.dylib  0x001b8a18 std::sys::unix::thread::Thread::new::thread_start::h5a078f29d274ef9c + 184
269 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
270 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
271 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb0344000  ecx: 0x00096b1c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x00096b48  esp: 0x00096b1c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x001b8600
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0x41000 -    0x44ff3 +a (0) <B33C10BC-8BAC-3B09-9A3D-74A4252B496F> /Users/USER/*/a
  0x102000 -   0x147fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x18b000 -   0x224ff3 +libstd-b4e8967dfe831496.dylib (0) <BDB05B30-4FAD-3299-952A-EDF0B59D66C9> /Users/USER/*/libstd-b4e8967dfe831496.dylib
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-24-172222-1_Traviss-Mac-1044.crash
Process:               a [48218]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [48212]
Responsible:           a [48218]
User ID:               501
Date/Time:             2019-06-24 17:22:19.725 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0367ec8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0367ec8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0367000-00000000b0368000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0368000-00000000b0569000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   a                              0x000a7021 stack_probes_lto::main::haeaf2f44e7253541 + 2545
4   a                              0x000c322b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h3a74775ec8e19b9b + 11
5   a                              0x000b5f0c std::sys_common::backtrace::__rust_begin_short_backtrace::hdf174f700ce58224 + 12
6   a                              0x000a879d main + 589
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000a992b std::sys::unix::abort_internal::h16150b015f3f5168 + 11
4   a                              0x000a9919 std::sys_common::util::abort::h172cbba7f0e37c0e + 73
5   a                              0x000b5e70 std::sys::unix::stack_overflow::imp::signal_handler::hc293ff2bba79270f + 640
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000b5bf0 rust_begin_unwind + 16
9   a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
10  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
11  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
12  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
13  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
14  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
15  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
16  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
17  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
18  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
19  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
20  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
21  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
22  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
23  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
24  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
25  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
26  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
27  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
28  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
29  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
30  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
31  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
32  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
33  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
34  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
35  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
36  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
37  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
38  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
39  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
40  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
41  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
42  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
43  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
44  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
45  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
46  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
47  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
48  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
49  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
50  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
51  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
52  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
53  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
54  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
55  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
56  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
57  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
58  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
59  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
60  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
61  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
62  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
63  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
64  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
65  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
66  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
67  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
68  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
69  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
70  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
71  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
72  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
73  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
74  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
75  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
76  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
77  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
78  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
79  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
80  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
81  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
82  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
83  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
84  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
85  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
86  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
87  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
88  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
89  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
90  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
91  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
92  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
93  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
94  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
95  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
96  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
97  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
98  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
99  a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
100 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
101 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
102 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
103 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
104 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
105 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
106 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
107 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
108 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
109 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
110 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
111 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
112 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
113 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
114 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
115 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
116 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
117 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
118 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
119 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
120 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
121 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
122 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
123 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
124 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
125 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
126 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
127 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
128 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
129 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
130 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
131 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
132 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
133 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
134 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
135 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
136 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
137 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
138 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
139 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
140 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
141 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
142 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
143 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
144 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
145 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
146 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
147 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
148 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
149 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
150 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
151 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
152 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
153 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
154 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
155 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
156 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
157 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
158 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
159 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
160 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
161 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
162 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
163 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
164 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
165 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
166 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
167 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
168 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
169 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
170 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
171 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
172 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
173 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
174 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
175 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
176 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
177 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
178 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
179 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
180 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
181 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
182 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
183 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
184 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
185 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
186 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
187 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
188 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
189 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
190 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
191 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
192 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
193 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
194 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
195 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
196 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
197 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
198 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
199 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
200 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
201 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
202 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
203 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
204 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
205 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
206 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
207 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
208 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
209 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
210 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
211 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
212 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
213 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
214 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
215 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
216 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
217 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
218 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
219 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
220 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
221 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
222 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
223 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
224 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
225 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
226 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
227 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
228 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
229 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
230 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
231 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
232 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
233 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
234 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
235 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
236 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
237 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
238 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
239 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
240 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
241 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
242 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
243 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
244 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
245 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
246 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
247 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
248 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
249 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
250 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
251 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
252 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
253 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
254 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
255 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
256 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
257 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
258 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
259 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
260 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
261 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
262 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
263 a                              0x000a7528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
264 a                              0x000c33c4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h5c2d08a13c86ed60 + 116
265 a                              0x000b58d1 _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hd45bbdc831375cfb + 65
266 a                              0x000b61b4 std::sys::unix::thread::Thread::new::thread_start::h5a078f29d274ef9c + 180
267 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
268 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
269 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb0568000  ecx: 0x0012db2c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x0012db58  esp: 0x0012db2c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xa5000 -    0xd4ffb +a (0) <7E96B76D-4020-31C5-88D3-B6F6F6136C39> /Users/USER/*/a
  0x1b1000 -   0x1f6fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2336
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=82.4M resident=0K(0%) swapped_out_or_unallocated=82.4M(100%)
Writable regions: Total=19.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=19.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            1336K       43 
__LINKEDIT                        73.7M        4 
__OBJC                              36K        6 
__TEXT                            8944K       43 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            567.8M      134 
TOTAL                            567.8M      134 
TOTAL, minus reserved VM space   567.6M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-24-172222_Traviss-Mac-1044.crash
Process:               a [48215]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [48212]
Responsible:           a [48215]
User ID:               501
Date/Time:             2019-06-24 17:22:19.602 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bf735738
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbf735738:
    Stack Guard            00000000bbf35000-00000000bf735000 [ 56.0M] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bf735000-00000000bf736000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bf736000-00000000bff35000 [ 8188K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000d092b std::sys::unix::abort_internal::h16150b015f3f5168 + 11
4   a                              0x000d0919 std::sys_common::util::abort::h172cbba7f0e37c0e + 73
5   a                              0x000dce70 std::sys::unix::stack_overflow::imp::signal_handler::hc293ff2bba79270f + 640
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000dcbf0 rust_begin_unwind + 16
9   a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
10  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
11  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
12  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
13  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
14  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
15  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
16  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
17  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
18  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
19  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
20  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
21  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
22  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
23  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
24  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
25  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
26  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
27  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
28  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
29  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
30  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
31  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
32  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
33  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
34  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
35  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
36  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
37  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
38  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
39  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
40  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
41  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
42  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
43  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
44  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
45  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
46  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
47  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
48  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
49  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
50  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
51  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
52  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
53  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
54  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
55  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
56  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
57  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
58  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
59  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
60  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
61  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
62  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
63  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
64  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
65  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
66  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
67  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
68  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
69  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
70  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
71  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
72  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
73  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
74  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
75  a                              0x000ce528 stack_probes_lto::recurse::h0c7a1532fbc2220d + 40
---
===========                     =======  ======= 
TOTAL                            567.6M      131 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/rustdoc_2019-06-24-173552_Traviss-Mac-1044.crash
Process:               rustdoc [55873]
Path:                  /Users/USER/*/rustdoc
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [55866]
Responsible:           rustdoc [55873]
User ID:               501
Date/Time:             2019-06-24 17:35:44.603 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5600 seconds
System Integrity Protection: enabled
Crashed Thread:        0
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Application Specific Information:
crashed on child side of fork pre-exec
BUG IN CLIENT OF LIBPLATFORM: os_once_t is corrupt
Thread 0 Crashed:
0   libsystem_platform.dylib       0xa7829f76 _os_once_gate_corruption_abort + 37
1   libsystem_platform.dylib       0xa78263f0 _os_once + 62
2   libsystem_platform.dylib       0xa78263a5 _os_alloc_once + 51
3   libsystem_notify.dylib         0xa78207bf _notify_fork_child + 168
4   libSystem.B.dylib              0xa58bfb35 libSystem_atfork_child + 51
5   libsystem_c.dylib              0xa7606d17 fork + 50
6   libstd-b4e8967dfe831496.dylib  0x04551331 std::sys::unix::process::process_inner::_$LT$impl$u20$std..sys..unix..process..process_common..Command$GT$::spawn::hf840b6baa03a101d + 929
7   libstd-b4e8967dfe831496.dylib  0x0453f59c std::process::Command::spawn::hebe2f53d327614ba + 28
8   librustc_codegen_ssa-9d89a337c4a41ad2.dylib 0x01430426 rustc_codegen_ssa::back::link::exec_linker::hd0e82cd33d63f1fb + 102
9   librustc_codegen_llvm-llvm.dylib 0x0496ca07 rustc::util::common::time::h534db7b0eeac6d87 + 295
10  librustc_codegen_llvm-llvm.dylib 0x0491a9dd rustc_codegen_ssa::back::link::link_natively::he752e73c64388e11 + 9757
11  librustc_codegen_llvm-llvm.dylib 0x04917df1 rustc_codegen_ssa::back::link::link_binary::h5c10aa3ff95484c8 + 1137
12  librustc_codegen_llvm-llvm.dylib 0x0496cc55 rustc::util::common::time::hd9f7f1a6143b6cfa + 277
13  librustc_codegen_llvm-llvm.dylib 0x04a3cadf _$LT$rustc_codegen_llvm..LlvmCodegenBackend$u20$as$u20$rustc_codegen_utils..codegen_backend..CodegenBackend$GT$::join_codegen_and_link::h6813348449409257 + 447
14  librustc_interface-11b5d8ad1cf2ed97.dylib 0x007d99e0 rustc_interface::queries::Query$LT$T$GT$::compute::h6a932d66357e37f6 + 464
15  librustc_interface-11b5d8ad1cf2ed97.dylib 0x008a95e7 rustc_interface::queries::_$LT$impl$u20$rustc_interface..interface..Compiler$GT$::compile::h9ccecab10ca56ccf + 439
16  rustdoc                        0x0013d21c rustc_interface::interface::run_compiler_in_existing_thread_pool::hd7fd524b725049f3 + 1292
17  rustdoc                        0x0010edf5 std::thread::local::LocalKey$LT$T$GT$::with::h526add21bac3c4c8 + 261
18  rustdoc                        0x00137cbf scoped_tls::ScopedKey$LT$T$GT$::set::h280e1fb12768b976 + 399
19  rustdoc                        0x0012679a syntax::with_globals::h7fa6b46ad74e907a + 106
20  rustdoc                        0x0015fe38 std::sys_common::backtrace::__rust_begin_short_backtrace::h6e5112f21fbdb863 (.llvm.17970027902023359223) + 1912
21  rustdoc                        0x000cf4c3 std::panicking::try::do_call::h654910d6204895ea (.llvm.1614282359348160699) + 19
22  libstd-b4e8967dfe831496.dylib  0x04553e5d __rust_maybe_catch_panic + 29
23  rustdoc                        0x002953a0 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hfe20e34c3d0b3cef + 128
24  libstd-b4e8967dfe831496.dylib  0x0452ae81 _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hd45bbdc831375cfb + 65
25  libstd-b4e8967dfe831496.dylib  0x04552a18 std::sys::unix::thread::Thread::new::thread_start::h5a078f29d274ef9c + 184
26  libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
27  libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
28  libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xa782bbb0  ebx: 0xa9b2e1f8  ecx: 0x00002307  edx: 0xa9b3b184
  edi: 0x00002307  esi: 0x00000000  ebp: 0xb85b37f8  esp: 0xb85b37c8
   ss: 0x00000023  efl: 0x00010246  eip: 0xa7829f76   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000023   gs: 0x0000000f
  cr2: 0xa9b3b1c0
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x49000 -   0x469ff3 +rustdoc (0) <C6D85333-9589-353B-AFF6-7DC9A805C1F0> /Users/USER/*/rustdoc
  0x549000 -   0x57dffb +librustc_plugin-d206e70a73aafd1f.dylib (0) <769CD07E-1A2B-3E4B-989D-D50FDC876C93> /Users/USER/*/librustc_plugin-d206e70a73aafd1f.dylib
  0x598000 -   0x59effb +libfmt_macros-1f381383475865c4.dylib (0) <5156D0B5-0B6F-3ED4-93AF-7AEA0D8C4C5E> /Users/USER/*/libfmt_macros-1f381383475865c4.dylib
  0x5a8000 -   0x5a8ff7 +librustc_fs_util-de7a35c8c66b36f8.dylib (0) <6E6FA698-1F21-3562-816E-7607E49DA8E9> /Users/USER/*/librustc_fs_util-de7a35c8c66b36f8.dylib
  0x5ac000 -   0x5acff7 +libarena-f427c10566ef1f57.dylib (0) <BFCD0F00-9464-3BDF-B6CA-6E8C9D7A7FCA> /Users/USER/*/libarena-f427c10566ef1f57.dylib
  0x5bd000 -   0x602fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x646000 -   0x71fffb +librustc_driver-997551140410925a.dylib (0) <F7C9D4B6-AD0A-3A46-A494-3EE673C96160> /Users/USER/*/librustc_driver-997551140410925a.dylib
  0x79e000 -   0x92efff +librustc_interface-11b5d8ad1cf2ed97.dylib (0) <7B9EB419-76CE-37AC-A3E8-46F6F3E6F06A> /Users/USER/*/librustc_interface-11b5d8ad1cf2ed97.dylib
  0x9dd000 -   0xa84ff7 +librustc_lint-e10ca50f559264bd.dylib (0) <9F3636E8-8016-3A04-BD82-72EEE0A3A4EE> /Users/USER/*/librustc_lint-e10ca50f559264bd.dylib
  0xaef000 -   0xbe6fff +librustc_traits-7ffb80393c1b79cd.dylib (0) <E30B2B16-AEA4-39EE-A52F-4C0D124640FD> /Users/USER/*/librustc_traits-7ffb80393c1b79cd.dylib
  0xc5d000 -   0xd3dfff +librustc_resolve-4e19919c39391eda.dylib (0) <05F335AB-7495-37A6-AD68-A5B2F417EA87> /Users/USER/*/librustc_resolve-4e19919c39391eda.dylib
  0xdd6000 -   0xe5efff +librustc_privacy-199f431acca30c52.dylib (0) <818B44A1-F312-3D40-96C3-D27E9C9E5150> /Users/USER/*/librustc_privacy-199f431acca30c52.dylib
  0xe91000 -  0x114effb +librustc_typeck-a210c0a4650ad504.dylib (0) <5BDDA768-EA60-34D5-83AF-984E45A6F7B5> /Users/USER/*/librustc_typeck-a210c0a4650ad504.dylib
 0x1306000 -  0x1389ff7 +librustc_passes-1e19ca3b13cfc28d.dylib (0) <652C0C17-993F-322C-8FCE-8370FF9F60C2> /Users/USER/*/librustc_passes-1e19ca3b13cfc28d.dylib
 0x13c7000 -  0x14bdff3 +librustc_codegen_ssa-9d89a337c4a41ad2.dylib (0) <4557C245-6CAA-3E9E-B278-D22CA702FEAF> /Users/USER/*/librustc_codegen_ssa-9d89a337c4a41ad2.dylib
 0x15e9000 -  0x166bff3 +librustc_incremental-a4e7ab3d21c20445.dylib (0) <4A1827E9-83F6-3B5E-8B05-EE0472C90384> /Users/USER/*/librustc_incremental-a4e7ab3d21c20445.dylib
 0x16b3000 -  0x1716fff +librustc_borrowck-e2b0b34db501d2f4.dylib (0) <064F4890-AC8A-3E30-86D7-A609B1AE75D3> /Users/USER/*/librustc_borrowck-e2b0b34db501d2f4.dylib
 0x176d000 -  0x1c84ff7 +librustc_mir-d87fe9e5fc338784.dylib (0) <7C0D21CD-EC91-36E1-9D27-82893D07925B> /Users/USER/*/librustc_mir-d87fe9e5fc338784.dylib
 0x2091000 -  0x20daff7 +librustc_allocator-2382bb74bcdb2f55.dylib (0) <4CC5C757-70FC-3891-B09A-219B7E4E0A84> /Users/USER/*/librustc_allocator-2382bb74bcdb2f55.dylib
 0x20f7000 -  0x21e0ff3 +librustc_save_analysis-73931d0ef4d511e0.dylib (0) <4BC3D554-4D77-305D-BD78-A74D92C05A19> /Users/USER/*/librustc_save_analysis-73931d0ef4d511e0.dylib
 0x226e000 -  0x230bff3 +librustc_codegen_utils-e17d69b09cb015d0.dylib (0) <D81C9C89-8C41-3375-A38A-8BC45D544BD6> /Users/USER/*/librustc_codegen_utils-e17d69b09cb015d0.dylib
 0x234b000 -  0x252bff7 +librustc_metadata-6a72ba49bd4f26f0.dylib (0) <FDB34C10-A804-35ED-B599-A49DA1F45AA6> /Users/USER/*/librustc_metadata-6a72ba49bd4f26f0.dylib
 0x2619000 -  0x2705ff7 +libsyntax_ext-c88e503002708e25.dylib (0) <5FA94C05-4F79-39D9-A4CB-8127D0FBF25D> /Users/USER/*/libsyntax_ext-c88e503002708e25.dylib
 0x2796000 -  0x306bfd3 +librustc-a74ed4eb24361f0c.dylib (0) <215A46BA-0DE4-3BFF-A4DA-B8D006ED9DC4> /Users/USER/*/librustc-a74ed4eb24361f0c.dylib
 0x3b99000 -  0x3bcfff7 +libtest-49456401932fdbb8.dylib (0) <7F7C9522-E62D-3802-B18E-71B810D6075E> /Users/USER/*/libtest-49456401932fdbb8.dylib
 0x3c0e000 -  0x3c25ffb +libterm-61ea53d10f5d4a6f.dylib (0) <10B821FC-0F83-3385-B5C0-B7C569A37A4E> /Users/USER/*/libterm-61ea53d10f5d4a6f.dylib
 0x3c40000 -  0x3e88ff7 +libsyntax-417ee3ebceb335aa.dylib (0) <CCCB42BF-1454-3748-911B-84A49BA268D5> /Users/USER/*/libsyntax-417ee3ebceb335aa.dylib
 0x4154000 -  0x41e9ff3 +librustc_target-b362c928fc9f84db.dylib (0) <2C6704FE-A63D-3A93-8BF5-24433E7EC5FC> /Users/USER/*/librustc_target-b362c928fc9f84db.dylib
 0x427b000 -  0x42c0fff +librustc_errors-5ae716d05e3b7a8f.dylib (0) <CBF6ADFB-9607-3776-8140-D1535E566F55> /Users/USER/*/librustc_errors-5ae716d05e3b7a8f.dylib
 0x4311000 -  0x4329ff7 +libsyntax_pos-80faeaf48c7db52d.dylib (0) <4C2B9DB4-8519-34A9-A298-EA58777C43D1> /Users/USER/*/libsyntax_pos-80faeaf48c7db52d.dylib
 0x4387000 -  0x4399ff7 +librustc_data_structures-95a0702315ffe8c3.dylib (0) <AB366EF1-1F1E-37A7-A99A-14955AD329DA> /Users/USER/*/librustc_data_structures-95a0702315ffe8c3.dylib
 0x443f000 -  0x4441ff3 +libgraphviz-b74cebb3bbc56831.dylib (0) <628B1BA7-0420-3A8D-97BA-0CA04E8D4437> /Users/USER/*/libgraphviz-b74cebb3bbc56831.dylib
 0x444f000 -  0x4482ffb +librustc_cratesio_shim-2335960ca5d6daec.dylib (0) <10BFEF07-1659-376D-AF4D-44FD1930B336> /Users/USER/*/librustc_cratesio_shim-2335960ca5d6daec.dylib
 0x4494000 -  0x44acff7 +libserialize-4dc46604853234cf.dylib (0) <0943D792-7AD2-3580-8159-814BC7AE4E7D> /Users/USER/*/libserialize-4dc46604853234cf.dylib
 0x4525000 -  0x45beff3 +libstd-b4e8967dfe831496.dylib (0) <BDB05B30-4FAD-3299-952A-EDF0B59D66C9> /Users/USER/*/libstd-b4e8967dfe831496.dylib
 0x48e5000 -  0x6da2fff +librustc_codegen_llvm-llvm.dylib (0) <F0BB0831-DE01-3677-AC12-F2352B1150DF> /Users/USER/*/librustc_codegen_llvm-llvm.dylib
0x90298000 - 0x90298fff  com.apple.Accelerate (1.11 - Accelerate 1.11) <F4A138F5-290D-3413-AD17-ECD395935FF3> /System/Library/Frameworks/Accelerate.framework/Versions/A/Accelerate
0x902b0000 - 0x909f1fdf  com.apple.vImage (8.1 - ???) <591C941E-6475-347E-89DA-F541E88F949A> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vImage.framework/Versions/A/vImage
0x909f2000 - 0x90b2dff7  libBLAS.dylib (1211.30.1) <A850E0E2-3A72-3916-9907-AF1E7ECC95F0> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBLAS.dylib
0x90b2e000 - 0x90b5bffb  libBNNS.dylib (37) <C29094A0-5C89-3C5E-AB37-510C28588E2E> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBNNS.dylib
0x90b5c000 - 0x90ecffff  libLAPACK.dylib (1211.30.1) <2DDDE838-0FF1-3679-8E62-9C09923ECB7E> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLAPACK.dylib
0x90ed0000 - 0x90ee6ffb  libLinearAlgebra.dylib (1211.30.1) <8A120E75-CAF4-3CAE-BBE6-E2F5FAE44DB8> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLinearAlgebra.dylib
0x90ee7000 - 0x90f00ff7  libSparseBLAS.dylib (1211.30.1) <0C5E0EF4-E9A5-3FC4-B7A3-1FE59DB4A2AA> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libSparseBLAS.dylib
0x90f01000 - 0x9105ffc7  libvDSP.dylib (622.20.8) <C5F16300-061F-3DF0-B91E-8BD0D2173351> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvDSP.dylib
0x91060000 - 0x9113effb  libvMisc.dylib (622.20.8) <1C8D5D80-F32C-3853-8309-57C8A82B7DA5> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvMisc.dylib
0x9113f000 - 0x9113ffff  com.apple.Accelerate.vecLib (3.11 - vec
