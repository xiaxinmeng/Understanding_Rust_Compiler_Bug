plain
[00:02:52]       Memory: 8 GB
[00:02:52]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:02:52]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:52]       SMC Version (system): 2.8f0
[00:02:52]       Serial Number (system): VMk+UwpsdFRX
[00:02:52] 
[00:02:52] hw.ncpu: 4
[00:02:52] hw.byteorder: 1234
[00:02:52] hw.memsize: 8589934592
---
[01:42:30] stdout:
[01:42:30] ------------------------------------------
[01:42:30] 
[01:42:30] running 3 tests
[01:42:30] test src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6) ... FAILED
[01:42:30] test src/test/rustdoc/test_option_check/test.rs - Bar (line 15) ... ok
[01:42:30] test src/test/rustdoc/test_option_check/test.rs - Foo (line 8) ... ok
[01:42:30] failures:
[01:42:30] 
[01:42:30] 
[01:42:30] ---- src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6) stdout ----
[01:42:30] error: linking with `cc` failed: signal: 4
[01:42:30]   |
[01:42:30]   = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestQlk6i0/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestQlk6i0/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestQlk6i0/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestQlk6i0/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestQlk6i0/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestQlk6i0/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestQlk6i0/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestQlk6i0/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/rustdoc/test_option_check/test/auxiliary" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libstd-a818a4b7490f1bef.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libpanic_unwind-782be5cba0a53fa8.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libbacktrace_sys-1405c679deb4b710.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_demangle-022938c840899ee8.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libunwind-1ecb62ae80798d9b.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liblibc-e1bf62a52accec8f.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liballoc-5cc329465c73523e.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_core-635c35a1095754d1.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcore-a9d318fb39582bdb.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-78e8331284e7e75f.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[01:42:30] 
[01:42:30] error: aborting due to previous error
[01:42:30] 
[01:42:30] 
[01:42:30] thread 'src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:30] 
[01:42:30] 
[01:42:30] failures:
[01:42:30] failures:
[01:42:30]     src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6)
[01:42:30] test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:42:30] 
[01:42:30] 
[01:42:30] ------------------------------------------
---
[01:42:30] 
[01:42:30] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:42:30] 
[01:42:30] 
[01:42:30] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/rustdoc" "--build-base" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/rustdoc" "--stage-id" "stage2-i686-apple-darwin" "--mode" "rustdoc" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0-rust-1.35.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:42:30] 
[01:42:30] 
[01:42:30] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:42:30] Build completed unsuccessfully in 0:28:41
[01:42:30] Build completed unsuccessfully in 0:28:41
[01:42:30] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04adcc0e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 25 05:37:49 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:29bae04d
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1200
drwx------  22 travis  staff    748 Mar 25 05:37 .
-rw-------@  1 travis  staff  38006 Mar 25 05:37 rustdoc_2019-03-25-053747_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34787 Mar 25 05:24 a_2019-03-25-052434_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  57366 Mar 25 05:24 a_2019-03-25-052431_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55583 Mar 25 05:24 a_2019-03-25-052423-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34718 Mar 25 05:24 a_2019-03-25-052423_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 Mar 25 05:24 a_2019-03-25-052418_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9166 Mar 25 05:24 a_2019-03-25-052413_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9171 Mar 25 05:24 a_2019-03-25-052406_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8934 Mar 25 05:24 a_2019-03-25-052405_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9304 Mar 25 05:23 a_2019-03-25-052332_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58236 Mar 25 05:23 a_2019-03-25-052322_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59503 Mar 25 05:23 a_2019-03-25-052317-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 Mar 25 05:23 a_2019-03-25-052317-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60372 Mar 25 05:23 a_2019-03-25-052317_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10880 Mar 25 05:21 a_2019-03-25-052114_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 Mar 25 05:20 a_2019-03-25-052016_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 Mar 25 05:19 a_2019-03-25-051900_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 Mar 25 05:18 a_2019-03-25-051808-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 Mar 25 05:18 a_2019-03-25-051808_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9484 Mar 25 05:18 a_2019-03-25-051800_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0b56189b
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-051800_Traviss-Mac-1044.crash
Process:               a [37358]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [37357]
Responsible:           a [37358]
User ID:               501
Date/Time:             2019-03-25 05:17:33.296 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00045afe abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 46
1   a                              0x00044f4b std::panicking::try::do_call::h70a3c61b79532463 (.llvm.13473107800807215613) + 11
2   libstd-a818a4b7490f1bef.dylib  0x001a1a3d __rust_maybe_catch_panic + 29
3   a                              0x00045d65 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 613
4   a                              0x000445ab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb3f0cb28b66a3895 + 11
5   libstd-a818a4b7490f1bef.dylib  0x001907ec std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
6   libstd-a818a4b7490f1bef.dylib  0x00192ee4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
7   libstd-a818a4b7490f1bef.dylib  0x001a1a3d __rust_maybe_catch_panic + 29
8   libstd-a818a4b7490f1bef.dylib  0x00193987 std::rt::lang_start_internal::h94f504c75da6b468 + 631
9   a                              0x0004609c main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x7ae807e0  ebx: 0xbffbb808  ecx: 0x00000000  edx: 0x00000000
  edi: 0x001a1a2e  esi: 0x00000000  ebp: 0xbffbb7a8  esp: 0xbffbb790
   ss: 0x00000023  efl: 0x00010296  eip: 0x00045afe   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x001db110
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x43000 -    0x46ffb +a (0) <5BE4A94A-593D-304D-BC69-7D5031626C31> /Users/USER/*/a
   0xe9000 -   0x12efdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x172000 -   0x201fff +libstd-a818a4b7490f1bef.dylib (0) <C138E5A7-FD17-3C5E-BC0C-0ADCF0A4302E> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2373
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
__DATA                            3576K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-051808-1_Traviss-Mac-1044.crash
Process:               a [38176]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [38169]
Responsible:           a [38176]
User ID:               501
Date/Time:             2019-03-25 05:18:03.023 +0000
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
0   libstd-a818a4b7490f1bef.dylib  0x001fe3f3 std::panicking::rust_panic_with_hook::hc0870bbc354eb56b + 115
1   a                              0x000e4bff std::panicking::begin_panic::h0d9f547d0675ac23 + 47 (panicking.rs:408)
2   a                              0x000e26e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 36 (backtrace.rs:24)
3   a                              0x000e20fb core::ptr::real_drop_in_place::hcb0474b887bab5fa + 11
4   a                              0x000e26b3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x000e39c8 backtrace::main::hcde7a1a1c3c85e77 + 4568 (backtrace.rs:103)
6   a                              0x000e1bbb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h1ff5f8c969ea1487 + 11 (rt.rs:64)
7   libstd-a818a4b7490f1bef.dylib  0x001fb7ec std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
8   libstd-a818a4b7490f1bef.dylib  0x001fdee4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
9   libstd-a818a4b7490f1bef.dylib  0x0020ca3d __rust_maybe_catch_panic + 29
10  libstd-a818a4b7490f1bef.dylib  0x001fe987 std::rt::lang_start_internal::h94f504c75da6b468 + 631
11  a                              0x000e421c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff1f578  ebx: 0xbff1f5c0  ecx: 0xbff1f470  edx: 0xa7702ec6
  edi: 0x00246458  esi: 0x001fe38e  ebp: 0xbff1f618  esp: 0xbff1f590
   ss: 0x00000023  efl: 0x00010286  eip: 0x001fe3f3   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x004cf214
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xdf000 -    0xe5ff7 +a (0) <9A3AE5C8-604A-364B-8E23-83852089DD81> /Users/USER/*/a
  0x154000 -   0x199fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1dd000 -   0x26cfff +libstd-a818a4b7490f1bef.dylib (0) <C138E5A7-FD17-3C5E-BC0C-0ADCF0A4302E> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2373
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=82.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=82.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.4M       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3576K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9356K       44 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            577.9M      136 
TOTAL                            577.9M      136 
TOTAL, minus reserved VM space   577.8M      136 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-051808_Traviss-Mac-1044.crash
Process:               a [38177]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [38169]
Responsible:           a [38177]
User ID:               501
Date/Time:             2019-03-25 05:18:03.146 +0000
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
0   libstd-a818a4b7490f1bef.dylib  0x001943f3 std::panicking::rust_panic_with_hook::hc0870bbc354eb56b + 115
1   a                              0x00076bff std::panicking::begin_panic::h0d9f547d0675ac23 + 47 (panicking.rs:408)
2   a                              0x000746e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 36 (backtrace.rs:24)
3   a                              0x000740fb core::ptr::real_drop_in_place::hcb0474b887bab5fa + 11
4   a                              0x000746b3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x000759c8 backtrace::main::hcde7a1a1c3c85e77 + 4568 (backtrace.rs:103)
6   a                              0x00073bbb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h1ff5f8c969ea1487 + 11 (rt.rs:64)
7   libstd-a818a4b7490f1bef.dylib  0x001917ec std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
8   libstd-a818a4b7490f1bef.dylib  0x00193ee4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
9   libstd-a818a4b7490f1bef.dylib  0x001a2a3d __rust_maybe_catch_panic + 29
10  libstd-a818a4b7490f1bef.dylib  0x00194987 std::rt::lang_start_internal::h94f504c75da6b468 + 631
11  a                              0x0007621c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff8d568  ebx: 0xbff8d5b0  ecx: 0xbff8d460  edx: 0xa7702ec6
  edi: 0x001dc458  esi: 0x0019438e  ebp: 0xbff8d608  esp: 0xbff8d580
   ss: 0x00000023  efl: 0x00010286  eip: 0x001943f3   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x7ca0f000
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x71000 -    0x77ff7 +a (0) <9A3AE5C8-604A-364B-8E23-83852089DD81> /Users/USER/*/a
   0xea000 -   0x12ffdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x173000 -   0x202fff +libstd-a818a4b7490f1bef.dylib (0) <C138E5A7-FD17-3C5E-BC0C-0ADCF0A4302E> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2373
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
__DATA                            3576K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-051900_Traviss-Mac-1044.crash
Process:               a [39842]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [39840]
Responsible:           a [39842]
User ID:               501
Date/Time:             2019-03-25 05:18:59.848 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5100 seconds
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
3   libstd-a818a4b7490f1bef.dylib  0x00137ebb std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   libstd-a818a4b7490f1bef.dylib  0x00129570 rust_oom + 48
5   libstd-a818a4b7490f1bef.dylib  0x0014a6e4 alloc::alloc::handle_alloc_error::h21cceb3ebb916356 + 20
6   a                              0x000246bd default_alloc_error_hook::main::hbf2d06db626d002e + 781
7   a                              0x0002483b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::he2b2d8ee9e61762f + 11
8   libstd-a818a4b7490f1bef.dylib  0x001277ec std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
9   libstd-a818a4b7490f1bef.dylib  0x00129ee4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
10  libstd-a818a4b7490f1bef.dylib  0x00138a3d __rust_maybe_catch_panic + 29
11  libstd-a818a4b7490f1bef.dylib  0x0012a987 std::rt::lang_start_internal::h94f504c75da6b468 + 631
12  a                              0x0002481c main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffdb72c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffdb758  esp: 0xbffdb72c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x23000 -    0x24ffb +a (0) <B19C67EA-9A1A-3B6D-8715-7EC3909B32E1> /Users/USER/*/a
   0x80000 -    0xc5fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x109000 -   0x198fff +libstd-a818a4b7490f1bef.dylib (0) <C138E5A7-FD17-3C5E-BC0C-0ADCF0A4302E> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-052317_Traviss-Mac-1044.crash
Process:               a [46188]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [46186]
Responsible:           a [46188]
User ID:               501
Date/Time:             2019-03-25 05:23:16.321 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0326eb8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0326eb8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0326000-00000000b0327000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0327000-00000000b0528000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-a818a4b7490f1bef.dylib  0x002287f0 std::sys::unix::thread::Thread::join::h64f7b24ccc77abf0 + 32
4   a                              0x00087026 std::thread::JoinHandle$LT$T$GT$::join::h931137fe47d1360b + 70
5   a                              0x000866fd out_of_stack::main::hfb05bc1bb33cf0c4 + 2605
6   a                              0x000851ab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc5ab2f91b133232a + 11
7   libstd-a818a4b7490f1bef.dylib  0x002187ec std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
8   libstd-a818a4b7490f1bef.dylib  0x0021aee4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
9   libstd-a818a4b7490f1bef.dylib  0x00229a3d __rust_maybe_catch_panic + 29
10  libstd-a818a4b7490f1bef.dylib  0x0021b987 std::rt::lang_start_internal::h94f504c75da6b468 + 631
11  a                              0x00086abc main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-a818a4b7490f1bef.dylib  0x00228ebb std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   libstd-a818a4b7490f1bef.dylib  0x002197c9 std::sys_common::util::abort::hbe26e41b27dc2e02 + 73
5   libstd-a818a4b7490f1bef.dylib  0x00228288 std::sys::unix::stack_overflow::imp::signal_handler::h8ff5a4b7bca12f82 + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-a818a4b7490f1bef.dylib  0x00227ed0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hd1a54ee3a938991c + 80
9   a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
10  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
11  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
12  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
13  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
14  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
15  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
16  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
17  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
18  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
19  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
20  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
21  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
22  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
23  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
24  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
25  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
26  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
27  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
28  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
29  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
30  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
31  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
32  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
33  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
34  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
35  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
36  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
37  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
38  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
39  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
40  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
41  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
42  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
43  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
44  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
45  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
46  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
47  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
48  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
49  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
50  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
51  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
52  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
53  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
54  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
55  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
56  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
57  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
58  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
59  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
60  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
61  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
62  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
63  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
64  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
65  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
66  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
67  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
68  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
69  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
70  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
71  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
72  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
73  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
74  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
75  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
76  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
77  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
78  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
79  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
80  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
81  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
82  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
83  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
84  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
85  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
86  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
87  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
88  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
89  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
90  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
91  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
92  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
93  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
94  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
95  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
96  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
97  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
98  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
99  a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
100 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
101 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
102 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
103 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
104 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
105 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
106 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
107 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
108 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
109 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
110 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
111 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
112 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
113 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
114 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
115 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
116 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
117 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
118 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
119 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
120 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
121 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
122 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
123 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
124 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
125 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
126 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
127 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
128 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
129 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
130 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
131 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
132 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
133 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
134 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
135 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
136 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
137 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
138 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
139 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
140 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
141 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
142 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
143 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
144 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
145 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
146 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
147 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
148 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
149 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
150 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
151 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
152 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
153 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
154 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
155 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
156 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
157 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
158 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
159 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
160 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
161 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
162 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
163 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
164 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
165 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
166 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
167 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
168 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
169 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
170 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
171 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
172 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
173 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
174 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
175 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
176 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
177 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
178 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
179 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
180 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
181 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
182 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
183 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
184 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
185 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
186 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
187 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
188 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
189 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
190 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
191 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
192 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
193 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
194 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
195 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
196 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
197 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
198 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
199 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
200 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
201 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
202 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
203 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
204 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
205 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
206 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
207 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
208 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
209 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
210 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
211 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
212 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
213 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
214 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
215 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
216 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
217 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
218 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
219 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
220 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
221 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
222 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
223 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
224 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
225 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
226 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
227 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
228 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
229 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
230 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
231 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
232 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
233 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
234 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
235 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
236 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
237 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
238 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
239 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
240 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
241 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
242 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
243 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
244 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
245 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
246 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
247 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
248 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
249 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
250 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
251 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
252 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
253 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
254 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
255 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
256 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
257 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
258 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
259 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
260 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
261 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
262 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
263 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
264 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
265 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
266 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
267 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
268 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
269 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
270 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
271 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
272 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
273 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
274 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
275 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
276 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
277 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
278 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
279 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
280 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
281 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
282 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
283 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
284 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
285 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
286 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
287 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
288 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
289 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
290 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
291 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
292 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
293 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
294 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
295 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
296 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
297 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
298 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
299 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
300 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
301 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
302 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
303 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
304 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
305 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
306 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
307 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
308 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
309 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
310 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
311 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
312 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
313 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
314 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
315 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
316 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
317 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
318 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
319 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
320 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
321 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
322 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
323 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
324 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
325 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
326 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
327 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
328 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
329 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
330 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
331 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
332 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
333 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
334 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
335 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
336 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
337 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
338 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
339 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
340 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
341 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
342 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
343 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
344 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
345 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
346 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
347 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
348 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
349 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
350 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
351 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
352 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
353 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
354 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
355 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
356 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
357 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
358 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
359 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
360 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
361 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
362 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
363 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
364 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
365 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
366 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
367 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
368 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
369 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
370 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
371 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
372 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
373 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
374 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
375 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
376 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
377 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
378 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
379 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
380 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
381 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
382 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
383 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
384 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
385 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
386 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
387 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
388 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
389 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
390 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
391 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
392 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
393 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
394 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
395 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
396 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
397 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
398 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
399 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
400 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
401 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
402 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
403 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
404 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
405 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
406 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
407 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
408 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
409 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
410 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
411 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
412 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
413 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
414 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
415 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
416 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
417 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
418 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
419 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
420 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
421 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
422 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
423 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
424 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
425 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
426 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
427 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
428 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
429 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
430 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
431 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
432 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
433 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
434 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
435 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
436 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
437 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
438 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
439 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
440 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
441 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
442 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
443 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
444 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
445 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
446 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
447 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
448 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
449 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
450 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
451 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
452 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
453 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
454 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
455 a                              0x00085c68 out_of_stack::silent_recurse::h0ed5da602c91a854 + 40
---
===========                     =======  ======= 
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-052332_Traviss-Mac-1044.crash
Process:               a [46417]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [46415]
Responsible:           a [46417]
User ID:               501
Date/Time:             2019-03-25 05:23:30.160 +0000
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
3   a                              0x000c058b panic_abort::__rust_start_panic::abort::ha3c705416e7f7175 + 11
4   a                              0x000c057b __rust_start_panic + 11
5   a                              0x000b4aab rust_panic + 11
6   a                              0x000b4695 std::panicking::rust_panic_with_hook::hc0870bbc354eb56b + 997
7   a                              0x000c632a std::panicking::begin_panic::hb6db9f05650430f8 + 42
8   a                              0x000b348d lto_abort::main::h9419a0043b6e0505 + 2909
9   a                              0x000c648b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf1a7859257dd6080 + 11
10  a                              0x000c03fc std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
11  a                              0x000b3868 main + 984
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff4c6fc  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff4c728  esp: 0xbff4c6fc
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xb2000 -    0xd5ffb +a (0) <94B3DDF3-6A9D-3491-8843-61589E5DFA25> /Users/USER/*/a
  0x19d000 -   0x1e2fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2625
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-052405_Traviss-Mac-1044.crash
Process:               a [47383]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [47378]
Responsible:           a [47383]
User ID:               501
Date/Time:             2019-03-25 05:24:05.110 +0000
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
3   libstd-a818a4b7490f1bef.dylib  0x001e9ebb std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   libstd-a818a4b7490f1bef.dylib  0x001da7c9 std::sys_common::util::abort::hbe26e41b27dc2e02 + 73
5   libstd-a818a4b7490f1bef.dylib  0x001dc702 rust_panic + 98
6   libstd-a818a4b7490f1bef.dylib  0x001dc5ce std::panicking::rust_panic_with_hook::hc0870bbc354eb56b + 590
7   a                              0x0007b9cf std::panicking::begin_panic::h492b4fbd30659c42 + 47
8   a                              0x0007cefc main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff846fc  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff84728  esp: 0xbff846fc
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x7a000 -    0x7dfff +a (0) <774A3BEA-CDA7-38C3-A58B-9BBF7AAC31B2> /Users/USER/*/a
  0x132000 -   0x177fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1bb000 -   0x24afff +libstd-a818a4b7490f1bef.dylib (0) <C138E5A7-FD17-3C5E-BC0C-0ADCF0A4302E> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2625
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
__DATA                            3576K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-052406_Traviss-Mac-1044.crash
Process:               a [47417]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47414]
Responsible:           a [47417]
User ID:               501
Date/Time:             2019-03-25 05:24:06.374 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
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
    __TEXT                 0000000000036000-0000000000039000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00037f62 segfault_no_out_of_stack::main::haec5f9680e9a04a3 + 2034
1   a                              0x0003698b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd9acd0ba19943a0d + 11
2   libstd-a818a4b7490f1bef.dylib  0x001817ec std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
3   libstd-a818a4b7490f1bef.dylib  0x00183ee4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
4   libstd-a818a4b7490f1bef.dylib  0x00192a3d __rust_maybe_catch_panic + 29
5   libstd-a818a4b7490f1bef.dylib  0x00184987 std::rt::lang_start_internal::h94f504c75da6b468 + 631
6   a                              0x0003823c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x7be1f7e0  ecx: 0x00000000  edx: 0x00000000
  edi: 0x00192a2e  esi: 0xbffc8830  ebp: 0xbffc8918  esp: 0xbffc8770
   ss: 0x00000023  efl: 0x00010246  eip: 0x00037f62   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     2
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x36000 -    0x38ff3 +a (0) <EE7511F6-8428-3271-AB3E-CBF278F5F263> /Users/USER/*/a
   0xda000 -   0x11ffdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x163000 -   0x1f2fff +libstd-a818a4b7490f1bef.dylib (0) <C138E5A7-FD17-3C5E-BC0C-0ADCF0A4302E> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2625
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
__DATA                            3576K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.6M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-052413_Traviss-Mac-1044.crash
Process:               a [47593]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47592]
Responsible:           a [47593]
User ID:               501
Date/Time:             2019-03-25 05:24:13.644 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
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
    __TEXT                 00000000000f0000-00000000000f3000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000f25d4 signal_exit_status::main::h013a3960fc5c363d + 436
1   a                              0x000f147b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h33c45f87a006e61b + 11
2   libstd-a818a4b7490f1bef.dylib  0x002847ec std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
3   libstd-a818a4b7490f1bef.dylib  0x00286ee4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
4   libstd-a818a4b7490f1bef.dylib  0x00295a3d __rust_maybe_catch_panic + 29
5   libstd-a818a4b7490f1bef.dylib  0x00287987 std::rt::lang_start_internal::h94f504c75da6b468 + 631
6   a                              0x000f26ac main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x7a65e040
  edi: 0x7a65e0d0  esi: 0xbff0e8a0  ebp: 0xbff0e938  esp: 0xbff0e820
   ss: 0x00000023  efl: 0x00010246  eip: 0x000f25d4   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     0
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xf0000 -    0xf2ff7 +a (0) <39EBFBB9-05B5-3BF2-96F2-42ECEA0AC4F2> /Users/USER/*/a
  0x1dd000 -   0x222fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x266000 -   0x2f5fff +libstd-a818a4b7490f1bef.dylib (0) <C138E5A7-FD17-3C5E-BC0C-0ADCF0A4302E> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2625
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
__DATA                            3576K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.6M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-052418_Traviss-Mac-1044.crash
Process:               a [47699]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47690]
Responsible:           a [47699]
User ID:               501
Date/Time:             2019-03-25 05:24:18.179 +0000
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
0   a                              0x000bdec6 simd_target_feature_mixup::test::id_avx512_512::h967bf37482962390 + 102
1   a                              0x000bcc7f simd_target_feature_mixup::test::main::h379367934b9623dc + 1647
2   a                              0x000bf1f0 simd_target_feature_mixup::main::h4f60990077aff357 + 896
3   a                              0x000bc38b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h08b0578e0d501144 + 11
4   libstd-a818a4b7490f1bef.dylib  0x001c87ec std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
5   libstd-a818a4b7490f1bef.dylib  0x001caee4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
6   libstd-a818a4b7490f1bef.dylib  0x001d9a3d __rust_maybe_catch_panic + 29
7   libstd-a818a4b7490f1bef.dylib  0x001cb987 std::rt::lang_start_internal::h94f504c75da6b468 + 631
8   a                              0x000bf3cc main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff43540  ebx: 0xbff434c0  ecx: 0x000bde6e  edx: 0xbff434c0
  edi: 0x000bc624  esi: 0x00000000  ebp: 0xbff434b8  esp: 0xbff43480
   ss: 0x00000023  efl: 0x00010246  eip: 0x000bdec6   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000bdb00
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xbb000 -    0xbfff7 +a (0) <D4981F96-0FE2-337E-9B08-1F337D657681> /Users/USER/*/a
  0x121000 -   0x166fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1aa000 -   0x239fff +libstd-a818a4b7490f1bef.dylib (0) <C138E5A7-FD17-3C5E-BC0C-0ADCF0A4302E> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2625
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
__DATA                            3576K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-052423-1_Traviss-Mac-1044.crash
Process:               a [47832]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [47830]
Responsible:           a [47832]
User ID:               501
Date/Time:             2019-03-25 05:24:23.273 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf5e948
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf5e948:
    Stack Guard            00000000bbf5d000-00000000bbf5e000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf5e000-00000000bbf5f000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf5f000-00000000bff5d000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-a818a4b7490f1bef.dylib  0x00202ebb std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   libstd-a818a4b7490f1bef.dylib  0x001f37c9 std::sys_common::util::abort::hbe26e41b27dc2e02 + 73
5   libstd-a818a4b7490f1bef.dylib  0x00202288 std::sys::unix::stack_overflow::imp::signal_handler::h8ff5a4b7bca12f82 + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-a818a4b7490f1bef.dylib  0x00201ed0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hd1a54ee3a938991c + 80
9   a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
265 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
266 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
267 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
268 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
269 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
270 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
271 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
272 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
273 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
274 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
275 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
276 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
277 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
278 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
279 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
280 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
281 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
282 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
283 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
284 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
285 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
286 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
287 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
288 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
289 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
290 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
291 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
292 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
293 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
294 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
295 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
296 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
297 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
298 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
299 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
300 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
301 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
302 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
303 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
304 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
305 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
306 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
307 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
308 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
309 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
310 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
311 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
312 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
313 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
314 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
315 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
316 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
317 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
318 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
319 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
320 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
321 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
322 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
323 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
324 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
325 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
326 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
327 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
328 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
329 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
330 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
331 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
332 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
333 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
334 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
335 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
336 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
337 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
338 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
339 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
340 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
341 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
342 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
343 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
344 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
345 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
346 a                              0x000a6950 stack_probes::recurse::h24283d9484398da0 + 48
---
===========                     =======  ======= 
TOTAL                            568.6M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-052423_Traviss-Mac-1044.crash
Process:               a [47835]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47830]
Responsible:           a [47835]
User ID:               501
Date/Time:             2019-03-25 05:24:23.303 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0249ea8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0249ea8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0249000-00000000b024a000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b024a000-00000000b044b000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-a818a4b7490f1bef.dylib  0x001897f0 std::sys::unix::thread::Thread::join::h64f7b24ccc77abf0 + 32
4   a                              0x000355e6 std::thread::JoinHandle$LT$T$GT$::join::hf9830934bede0ff9 + 70
5   a                              0x00034805 stack_probes::main::hc5f49a55fd7e038b + 597
6   a                              0x0003361b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hec3c4a6375b3cdb3 + 11
7   libstd-a818a4b7490f1bef.dylib  0x001797ec std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
8   libstd-a818a4b7490f1bef.dylib  0x0017bee4 std::panicking::try::do_call::h48eb2606c6f77d4b + 20
9   libstd-a818a4b7490f1bef.dylib  0x0018aa3d __rust_maybe_catch_panic + 29
10  libstd-a818a4b7490f1bef.dylib  0x0017c987 std::rt::lang_start_internal::h94f504c75da6b468 + 631
11  a                              0x0003522c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-a818a4b7490f1bef.dylib  0x00189ebb std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   libstd-a818a4b7490f1bef.dylib  0x0017a7c9 std::sys_common::util::abort::hbe26e41b27dc2e02 + 73
5   libstd-a818a4b7490f1bef.dylib  0x00189288 std::sys::unix::stack_overflow::imp::signal_handler::h8ff5a4b7bca12f82 + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-a818a4b7490f1bef.dylib  0x00188ed0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hd1a54ee3a938991c + 80
9   a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x00034950 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x0003367d std::sys_common::backtrace::__rust_begin_short_backtrace::h50a627a5caed610d (.llvm.12402068326511782026) + 29
265 libstd-a818a4b7490f1bef.dylib  0x0018aa3d __rust_maybe_catch_panic + 29
266 a                              0x00035b13 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hac4ea714c2ac4977 + 131
267 libstd-a818a4b7490f1bef.dylib  0x0018972b std::sys::unix::thread::Thread::new::thread_start::h66952d8be93428dd + 187
268 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
269 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
270 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb044a000  ecx: 0x00088b0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x00088b38  esp: 0x00088b0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00189310
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0x32000 -    0x36ff7 +a (0) <F0C0579C-B13E-3B1C-BB0C-48AB6075E0F9> /Users/USER/*/a
   0xd2000 -   0x117fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x15b000 -   0x1eafff +libstd-a818a4b7490f1bef.dylib (0) <C138E5A7-FD17-3C5E-BC0C-0ADCF0A4302E> /Users/USER/*/libstd-a818a4b7490f1bef.dylib
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
    task_for_pid: 2625
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
__DATA                            3576K       44 
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-052431_Traviss-Mac-1044.crash
Process:               a [47962]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47958]
Responsible:           a [47962]
User ID:               501
Date/Time:             2019-03-25 05:24:30.143 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf54988
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf54988:
    Stack Guard            00000000bbf53000-00000000bbf54000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf54000-00000000bbf55000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf55000-00000000bff53000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000b212b std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   a                              0x000b2119 std::sys_common::util::abort::hbe26e41b27dc2e02 + 73
5   a                              0x000c0bfc std::sys::unix::stack_overflow::imp::signal_handler::h8ff5a4b7bca12f82 + 860
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000c08a0 rust_begin_unwind + 16
9   a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
10  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
11  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
12  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
13  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
14  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
15  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
16  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
17  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
18  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
19  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
20  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
21  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
22  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
23  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
24  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
25  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
26  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
27  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
28  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
29  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
30  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
31  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
32  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
33  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
34  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
35  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
36  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
37  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
38  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
39  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
40  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
41  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
42  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
43  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
44  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
45  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
46  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
47  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
48  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
49  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
50  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
51  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
52  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
53  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
54  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
55  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
56  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
57  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
58  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
59  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
60  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
61  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
62  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
63  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
64  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
65  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
66  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
67  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
68  a                              0x000afb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
---
===========                     =======  ======= 
TOTAL                            565.6M      130 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-03-25-052434_Traviss-Mac-1044.crash
Process:               a [47966]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47958]
Responsible:           a [47966]
User ID:               501
Date/Time:             2019-03-25 05:24:30.145 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0389f08
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0389f08:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0389000-00000000b038a000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b038a000-00000000b058b000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa77000b6 __bsdthread_create + 10
1   libsystem_pthread.dylib        0xa7832824 _pthread_create + 235
2   libsystem_pthread.dylib        0xa782f228 pthread_create + 28
3   a                              0x000cc561 stack_probes_lto::main::h7f9d4bbc5a99dd1a + 2177
4   a                              0x000e52cb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h8fa609264f6694d3 + 11
5   a                              0x000ddc9c std::sys_common::backtrace::__rust_begin_short_backtrace::hcbf5f546a2daec04 + 12
6   a                              0x000cdd9d main + 589
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000cf12b std::sys::unix::abort_internal::h0d557f9865d64bf7 + 11
4   a                              0x000cf119 std::sys_common::util::abort::hbe26e41b27dc2e02 + 73
5   a                              0x000ddbfc std::sys::unix::stack_overflow::imp::signal_handler::h8ff5a4b7bca12f82 + 860
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000dd8a0 rust_begin_unwind + 16
9   a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
10  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
11  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
12  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
13  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
14  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
15  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
16  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
17  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
18  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
19  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
20  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
21  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
22  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
23  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
24  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
25  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
26  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
27  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
28  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
29  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
30  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
31  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
32  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
33  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
34  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
35  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
36  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
37  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
38  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
39  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
40  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
41  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
42  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
43  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
44  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
45  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
46  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
47  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
48  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
49  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
50  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
51  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
52  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
53  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
54  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
55  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
56  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
57  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
58  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
59  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
60  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
61  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
62  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
63  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
64  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
65  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
66  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
67  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
68  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
69  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
70  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
71  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
72  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
73  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
74  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
75  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
76  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
77  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
78  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
79  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
80  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
81  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
82  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
83  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
84  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
85  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
86  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
87  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
88  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
89  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
90  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
91  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
92  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
93  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
94  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
95  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
96  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
97  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
98  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
99  a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
100 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
101 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
102 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
103 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
104 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
105 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
106 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
107 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
108 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
109 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
110 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
111 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
112 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
113 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
114 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
115 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
116 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
117 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
118 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
119 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
120 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
121 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
122 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
123 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
124 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
125 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
126 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
127 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
128 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
129 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
130 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
131 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
132 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
133 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
134 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
135 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
136 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
137 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
138 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
139 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
140 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
141 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
142 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
143 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
144 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
145 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
146 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
147 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
148 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
149 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
150 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
151 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
152 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
153 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
154 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
155 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
156 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
157 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
158 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
159 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
160 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
161 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
162 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
163 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
164 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
165 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
166 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
167 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
168 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
169 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
170 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
171 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
172 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
173 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
174 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
175 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
176 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
177 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
178 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
179 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
180 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
181 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
182 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
183 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
184 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
185 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
186 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
187 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
188 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
189 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
190 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
191 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
192 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
193 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
194 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
195 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
196 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
197 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
198 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
199 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
200 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
201 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
202 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
203 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
204 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
205 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
206 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
207 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
208 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
209 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
210 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
211 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
212 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
213 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
214 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
215 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
216 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
217 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
218 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
219 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
220 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
221 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
222 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
223 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
224 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
225 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
226 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
227 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
228 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
229 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
230 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
231 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
232 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
233 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
234 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
235 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
236 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
237 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
238 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
239 a                              0x000ccb28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
