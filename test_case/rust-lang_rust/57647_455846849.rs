plain
[01:34:05] stdout:
[01:34:05] ------------------------------------------
[01:34:05] 
[01:34:05] running 3 tests
[01:34:05] test src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6) ... FAILED
[01:34:05] test src/test/rustdoc/test_option_check/test.rs - Bar (line 15) ... ok
[01:34:05] test src/test/rustdoc/test_option_check/test.rs - Foo (line 8) ... ok
[01:34:05] failures:
[01:34:05] 
[01:34:05] 
[01:34:05] ---- src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6) stdout ----
[01:34:05] error: linking with `cc` failed: signal: 4
[01:34:05]   |
[01:34:05]   = note: "cc" "-m32" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestYGdmv6/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestYGdmv6/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestYGdmv6/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestYGdmv6/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestYGdmv6/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestYGdmv6/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestYGdmv6/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestYGdmv6/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/rustdoc/test_option_check/test/auxiliary" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libstd-59cbda0315ed40bd.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libpanic_unwind-ee41a585a8595c74.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libbacktrace_sys-107e44e68f173509.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libunwind-197af4d8035098c0.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_demangle-99f381b1c7fa1606.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liblibc-a8fb57ce325829c3.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/liballoc-846fb2a9f5d1aaed.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/librustc_std_workspace_core-d5966ba5cb1db2a4.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcore-2828b2d17cf57c90.rlib" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib/libcompiler_builtins-6402986580e93bde.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[01:34:05] 
[01:34:05] 
[01:34:05] thread 'src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:34:05] 
[01:34:05] 
[01:34:05] failures:
[01:34:05] failures:
[01:34:05]     src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6)
[01:34:05] test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:34:05] 
[01:34:05] 
[01:34:05] ------------------------------------------
---
[01:34:05] 
[01:34:05] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:34:05] 
[01:34:05] 
[01:34:05] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/rustdoc" "--build-base" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/rustdoc" "--stage-id" "stage2-i686-apple-darwin" "--mode" "rustdoc" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:34:05] 
[01:34:05] 
[01:34:05] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:34:05] Build completed unsuccessfully in 0:30:13
[01:34:05] Build completed unsuccessfully in 0:30:13
[01:34:05] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a11632c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 20 08:25:36 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:00c36be8
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1200
drwx------  22 travis  staff    748 Jan 20 08:25 .
-rw-------@  1 travis  staff  37740 Jan 20 08:25 rustdoc_2019-01-20-082535_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34790 Jan 20 08:10 a_2019-01-20-081052-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  57364 Jan 20 08:10 a_2019-01-20-081052_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34698 Jan 20 08:10 a_2019-01-20-081044-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55583 Jan 20 08:10 a_2019-01-20-081044_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 Jan 20 08:10 a_2019-01-20-081039_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9166 Jan 20 08:10 a_2019-01-20-081034_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9171 Jan 20 08:10 a_2019-01-20-081028_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8937 Jan 20 08:10 a_2019-01-20-081026_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9305 Jan 20 08:09 a_2019-01-20-080950_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58266 Jan 20 08:09 a_2019-01-20-080941_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 Jan 20 08:09 a_2019-01-20-080934-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59540 Jan 20 08:09 a_2019-01-20-080934-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60381 Jan 20 08:09 a_2019-01-20-080934_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10759 Jan 20 08:07 a_2019-01-20-080727_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 Jan 20 08:06 a_2019-01-20-080628_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 Jan 20 08:05 a_2019-01-20-080508_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9780 Jan 20 08:04 a_2019-01-20-080409-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 Jan 20 08:04 a_2019-01-20-080409_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9484 Jan 20 08:04 a_2019-01-20-080406_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:05abe9ba
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-20-080406_Traviss-Mac-1044.crash
Process:               a [36785]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [36783]
Responsible:           a [36785]
User ID:               501
Date/Time:             2019-01-20 08:03:40.547 +0000
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
0   a                              0x000c6aae abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 46
1   a                              0x000c5efb std::panicking::try::do_call::he6b26b8de6839310 (.llvm.15680363606803471465) + 11
2   libstd-59cbda0315ed40bd.dylib  0x00262d1d __rust_maybe_catch_panic + 29
3   a                              0x000c6d15 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 613
4   a                              0x000c54bb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::he9e40a337d00faee + 11
5   libstd-59cbda0315ed40bd.dylib  0x00251d1c std::sys_common::backtrace::__rust_begin_short_backtrace::ha2aa1e5f8c9cbf9f + 12
6   libstd-59cbda0315ed40bd.dylib  0x002541a4 std::panicking::try::do_call::h7b5be8fee5c2e08d + 20
7   libstd-59cbda0315ed40bd.dylib  0x00262d1d __rust_maybe_catch_panic + 29
8   libstd-59cbda0315ed40bd.dylib  0x00254c57 std::rt::lang_start_internal::ha4e45e7ccf10cd29 + 631
9   a                              0x000c704c main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x7871ff20  ebx: 0xbff3b288  ecx: 0x00000000  edx: 0x00000000
  edi: 0x00262d0e  esi: 0x00000000  ebp: 0xbff3b228  esp: 0xbff3b210
   ss: 0x00000023  efl: 0x00010292  eip: 0x000c6aae   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00299ce0
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xc4000 -    0xc7ff3 +a (0) <48149893-EBBE-3440-A0D1-E6DAD5C5E861> /Users/USER/*/a
  0x1ab000 -   0x1f0fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x234000 -   0x2bfff7 +libstd-59cbda0315ed40bd.dylib (0) <AFDDB41D-C12A-38EB-8A0D-C8508D6B0AF7> /Users/USER/*/libstd-59cbda0315ed40bd.dylib
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
    task_for_pid: 2085
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
__DATA                            3532K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9328K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            569.5M      134 
TOTAL                            569.5M      134 
TOTAL, minus reserved VM space   569.4M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-20-080409-1_Traviss-Mac-1044.crash
Process:               a [37563]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [37559]
Responsible:           a [37563]
User ID:               501
Date/Time:             2019-01-20 08:04:09.238 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-59cbda0315ed40bd.dylib  0x001ae887 std::panicking::rust_panic_with_hook::h672c6c818d1d7b0c + 583
1   a                              0x000d5b6f std::panicking::begin_panic::h1b97475efb91579c + 47 (panicking.rs:412)
2   a                              0x000d35f4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hbddd8cc943232966 + 36 (backtrace.rs:24)
3   a                              0x000d2d0b core::ptr::real_drop_in_place::h280f13d74e1439c0 + 11
4   a                              0x000d35c3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x000d48f8 backtrace::main::hcde7a1a1c3c85e77 + 4600 (backtrace.rs:103)
6   a                              0x000d2a7b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hbba8b2da7b0b8913 + 11 (rt.rs:64)
7   libstd-59cbda0315ed40bd.dylib  0x001abd1c std::sys_common::backtrace::__rust_begin_short_backtrace::ha2aa1e5f8c9cbf9f + 12
8   libstd-59cbda0315ed40bd.dylib  0x001ae1a4 std::panicking::try::do_call::h7b5be8fee5c2e08d + 20
9   libstd-59cbda0315ed40bd.dylib  0x001bcd1d __rust_maybe_catch_panic + 29
10  libstd-59cbda0315ed40bd.dylib  0x001aec57 std::rt::lang_start_internal::ha4e45e7ccf10cd29 + 631
11  a                              0x000d517c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff2fff8  ebx: 0xbff30040  ecx: 0xbff2fee0  edx: 0xa7702ec6
  edi: 0x001f3fac  esi: 0x001ae64e  ebp: 0xbff30098  esp: 0xbff30010
   ss: 0x00000023  efl: 0x00010282  eip: 0x001ae887   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00470389
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xcf000 -    0xd6ff7 +a (0) <194D5428-C514-3186-B02E-B504A67E6788> /Users/USER/*/a
  0x105000 -   0x14afdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x18e000 -   0x219ff7 +libstd-59cbda0315ed40bd.dylib (0) <AFDDB41D-C12A-38EB-8A0D-C8508D6B0AF7> /Users/USER/*/libstd-59cbda0315ed40bd.dylib
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
    task_for_pid: 2085
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
__DATA                            3532K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9344K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            586.9M      138 
TOTAL                            586.9M      138 
TOTAL, minus reserved VM space   586.8M      138 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-20-080409_Traviss-Mac-1044.crash
Process:               a [37564]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [37559]
Responsible:           a [37564]
User ID:               501
Date/Time:             2019-01-20 08:04:09.260 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-59cbda0315ed40bd.dylib  0x0014c887 std::panicking::rust_panic_with_hook::h672c6c818d1d7b0c + 583
1   a                              0x0006cb6f std::panicking::begin_panic::h1b97475efb91579c + 47 (panicking.rs:412)
2   a                              0x0006a5f4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hbddd8cc943232966 + 36 (backtrace.rs:24)
3   a                              0x00069d0b core::ptr::real_drop_in_place::h280f13d74e1439c0 + 11
4   a                              0x0006a5c3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x0006b8f8 backtrace::main::hcde7a1a1c3c85e77 + 4600 (backtrace.rs:103)
6   a                              0x00069a7b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hbba8b2da7b0b8913 + 11 (rt.rs:64)
7   libstd-59cbda0315ed40bd.dylib  0x00149d1c std::sys_common::backtrace::__rust_begin_short_backtrace::ha2aa1e5f8c9cbf9f + 12
8   libstd-59cbda0315ed40bd.dylib  0x0014c1a4 std::panicking::try::do_call::h7b5be8fee5c2e08d + 20
9   libstd-59cbda0315ed40bd.dylib  0x0015ad1d __rust_maybe_catch_panic + 29
10  libstd-59cbda0315ed40bd.dylib  0x0014cc57 std::rt::lang_start_internal::ha4e45e7ccf10cd29 + 631
11  a                              0x0006c17c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff98fe8  ebx: 0xbff99030  ecx: 0xbff98ed0  edx: 0xa7702ec6
  edi: 0x00191fac  esi: 0x0014c64e  ebp: 0xbff99088  esp: 0xbff99000
   ss: 0x00000023  efl: 0x00010282  eip: 0x0014c887   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b2700c
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x66000 -    0x6dff7 +a (0) <194D5428-C514-3186-B02E-B504A67E6788> /Users/USER/*/a
   0xa3000 -    0xe8fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x12c000 -   0x1b7ff7 +libstd-59cbda0315ed40bd.dylib (0) <AFDDB41D-C12A-38EB-8A0D-C8508D6B0AF7> /Users/USER/*/libstd-59cbda0315ed40bd.dylib
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
    task_for_pid: 2085
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
__DATA                            3532K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9344K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            577.9M      136 
TOTAL                            577.9M      136 
TOTAL, minus reserved VM space   577.8M      136 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-20-080508_Traviss-Mac-1044.crash
Process:               a [39256]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [39254]
Responsible:           a [39256]
User ID:               501
Date/Time:             2019-01-20 08:05:07.932 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4500 seconds
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
3   libstd-59cbda0315ed40bd.dylib  0x0020b18b std::sys::unix::abort_internal::h1303f06e8ad0ed3f + 11
4   libstd-59cbda0315ed40bd.dylib  0x001fc630 rust_oom + 48
5   libstd-59cbda0315ed40bd.dylib  0x0021e684 alloc::alloc::handle_alloc_error::h8e193745f8d77a4e + 20
6   a                              0x00074006 default_alloc_error_hook::main::hbf2d06db626d002e + 790
7   a                              0x000737cb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h9e90cf769b1f1ba0 + 11
8   libstd-59cbda0315ed40bd.dylib  0x001fad1c std::sys_common::backtrace::__rust_begin_short_backtrace::ha2aa1e5f8c9cbf9f + 12
9   libstd-59cbda0315ed40bd.dylib  0x001fd1a4 std::panicking::try::do_call::h7b5be8fee5c2e08d + 20
10  libstd-59cbda0315ed40bd.dylib  0x0020bd1d __rust_maybe_catch_panic + 29
11  libstd-59cbda0315ed40bd.dylib  0x001fdc57 std::rt::lang_start_internal::ha4e45e7ccf10cd29 + 631
12  a                              0x0007416c main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff8c1ac  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff8c1d8  esp: 0xbff8c1ac
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x73000 -    0x74ff3 +a (0) <61AA4188-B0A1-3B7C-A454-CAC4D5D2D3B0> /Users/USER/*/a
  0x154000 -   0x199fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1dd000 -   0x268ff7 +libstd-59cbda0315ed40bd.dylib (0) <AFDDB41D-C12A-38EB-8A0D-C8508D6B0AF7> /Users/USER/*/libstd-59cbda0315ed40bd.dylib
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
TOTAL                            568.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-20-080934-2_Traviss-Mac-1044.crash
Process:               a [45558]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [45552]
Responsible:           a [45558]
User ID:               501
Date/Time:             2019-01-20 08:09:34.035 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b01e8fec
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb01e8fec:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b01e8000-00000000b01e9000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b01e9000-00000000b03ea000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-59cbda0315ed40bd.dylib  0x00224cb0 std::sys::unix::thread::Thread::join::h6baf73e947f979fb + 32
4   a                              0x00075406 _$LT$std..thread..JoinHandle$LT$T$GT$$GT$::join::h486b41e03cb24b04 + 70
5   a                              0x00073c59 out_of_stack::main::hfb05bc1bb33cf0c4 + 233
6   a                              0x0007304b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd554d95ddf159ddd + 11
7   libstd-59cbda0315ed40bd.dylib  0x00214d1c std::sys_common::backtrace::__rust_begin_short_backtrace::ha2aa1e5f8c9cbf9f + 12
8   libstd-59cbda0315ed40bd.dylib  0x002171a4 std::panicking::try::do_call::h7b5be8fee5c2e08d + 20
9   libstd-59cbda0315ed40bd.dylib  0x00225d1d __rust_maybe_catch_panic + 29
10  libstd-59cbda0315ed40bd.dylib  0x00217c57 std::rt::lang_start_internal::ha4e45e7ccf10cd29 + 631
11  a                              0x0007497c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-59cbda0315ed40bd.dylib  0x0022518b std::sys::unix::abort_internal::h1303f06e8ad0ed3f + 11
4   libstd-59cbda0315ed40bd.dylib  0x00215882 std::sys_common::util::abort::h3daeed059ec162ec + 82
5   libstd-59cbda0315ed40bd.dylib  0x0022471b std::sys::unix::stack_overflow::imp::signal_handler::h2f4d723442aba6c4 + 955
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-59cbda0315ed40bd.dylib  0x00224360 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h71ec79f9178ccc1f + 80
9   libstd-59cbda0315ed40bd.dylib  0x00205c97 _$LT$std..io..stdio..StdoutLock$LT$$u27$a$GT$$u20$as$u20$std..io..Write$GT$::write::hd1d74cc3d2799446 + 263
10  libstd-59cbda0315ed40bd.dylib  0x002074c7 std::io::Write::write_all::h43c7a1f6a6cb6eb9 + 71
11  libstd-59cbda0315ed40bd.dylib  0x00207963 _$LT$std..io..Write..write_fmt..Adaptor$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..fmt..Write$GT$::write_str::h6cee5b6fcd378856 + 35
12  libstd-59cbda0315ed40bd.dylib  0x002460b4 core::fmt::write::hd10c080826c6c104 + 740
13  libstd-59cbda0315ed40bd.dylib  0x00205a66 _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::ha00d961333e46ff0 + 182
14  libstd-59cbda0315ed40bd.dylib  0x00206d1c std::io::stdio::_print::h89fb91e409f7997f + 396
15  a                              0x00073b5f out_of_stack::loud_recurse::hcd528ebf130a94fa + 63
16  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
17  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
18  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
19  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
20  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
21  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
22  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
23  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
24  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
25  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
26  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
27  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
28  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
29  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
30  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
31  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
32  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
33  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
34  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
35  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
36  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
37  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
38  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
39  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
40  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
41  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
42  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
43  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
44  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
45  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
46  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
47  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
48  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
49  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
50  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
51  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
52  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
53  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
54  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
55  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
56  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
57  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
58  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
59  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
60  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
61  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
62  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
63  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
64  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
65  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
66  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
67  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
68  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
69  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
70  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
71  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
72  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
73  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
74  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
75  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
76  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
77  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
78  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
79  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
80  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
81  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
82  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
83  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
84  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
85  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
86  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
87  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
88  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
89  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
90  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
91  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
92  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
93  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
94  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
95  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
96  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
97  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
98  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
99  a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
100 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
101 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
102 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
103 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
104 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
105 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
106 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
107 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
108 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
109 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
110 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
111 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
112 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
113 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
114 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
115 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
116 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
117 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
118 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
119 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
120 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
121 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
122 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
123 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
124 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
125 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
126 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
127 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
128 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
129 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
130 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
131 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
132 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
133 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
134 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
135 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
136 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
137 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
138 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
139 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
140 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
141 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
142 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
143 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
144 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
145 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
146 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
147 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
148 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
149 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
150 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
151 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
152 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
153 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
154 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
155 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
156 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
157 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
158 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
159 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
160 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
161 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
162 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
163 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
164 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
165 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
166 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
167 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
168 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
169 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
170 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
171 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
172 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
173 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
174 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
175 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
176 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
177 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
178 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
179 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
180 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
181 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
182 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
183 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
184 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
185 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
186 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
187 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
188 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
189 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
190 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
191 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
192 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
193 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
194 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
195 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
196 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
197 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
198 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
199 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
200 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
201 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
202 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
203 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
204 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
205 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
206 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
207 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
208 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
209 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
210 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
211 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
212 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
213 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
214 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
215 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
216 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
217 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
218 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
219 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
220 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
221 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
222 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
223 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
224 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
225 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
226 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
227 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
228 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
229 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
230 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
231 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
232 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
233 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
234 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
235 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
236 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
237 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
238 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
239 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
240 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
241 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
242 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
243 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
244 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
245 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
246 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
247 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
248 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
249 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
250 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
251 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
252 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
253 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
254 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
255 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
256 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
257 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
258 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
259 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
260 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
261 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
262 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
263 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
264 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
265 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
266 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
267 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
268 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
269 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
270 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
271 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
272 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
273 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
274 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
275 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
276 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
277 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
278 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
279 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
280 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
281 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
282 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
283 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
284 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
285 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
286 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
287 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
288 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
289 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
290 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
291 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
292 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
293 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
294 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
295 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
296 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
297 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
298 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
299 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
300 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
301 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
302 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
303 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
304 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
305 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
306 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
307 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
308 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
309 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
310 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
311 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
312 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
313 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
314 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
315 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
316 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
317 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
318 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
319 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
320 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
321 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
322 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
323 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
324 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
325 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
326 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
327 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
328 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
329 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
330 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
331 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
332 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
333 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
334 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
335 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
336 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
337 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
338 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
339 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
340 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
341 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
342 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
343 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
344 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
345 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
346 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
347 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
348 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
349 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
350 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
351 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
352 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
353 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
354 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
355 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
356 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
357 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
358 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
359 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
360 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
361 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
362 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
363 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
364 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
365 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
366 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
367 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
368 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
369 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
370 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
371 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
372 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
373 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
374 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
375 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
376 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
377 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
378 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
379 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
380 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
381 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
382 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
383 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
384 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
385 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
386 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
387 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
388 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
389 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
390 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
391 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
392 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
393 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
394 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
395 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
396 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
397 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
398 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
399 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
400 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
401 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
402 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
403 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
404 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
405 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
406 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
407 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
408 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
409 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
410 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
411 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
412 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
413 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
414 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
415 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
416 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
417 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
418 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
419 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
420 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
421 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
422 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
423 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
424 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
425 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
426 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
427 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
428 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
429 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
430 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
431 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
432 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
433 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
434 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
435 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
436 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
437 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
438 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
439 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
440 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
441 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
442 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
443 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
444 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
445 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
446 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
447 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
448 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
449 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
450 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
451 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
452 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
453 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
454 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
455 a                              0x00073b64 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
---
===========                     =======  ======= 
TOTAL                            568.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-20-080950_Traviss-Mac-1044.crash
Process:               a [45768]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [45767]
Responsible:           a [45768]
User ID:               501
Date/Time:             2019-01-20 08:09:49.355 +0000
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
3   a                              0x0003f42b panic_abort::__rust_start_panic::abort::h734703ebb87378cb + 11
4   a                              0x0003f41b __rust_start_panic + 11
5   a                              0x00033cfb rust_panic + 11
6   a                              0x00033889 std::panicking::rust_panic_with_hook::h672c6c818d1d7b0c + 1321
7   a                              0x0004524a std::panicking::begin_panic::hdc4401a54ad97bbb + 42
8   a                              0x0003269f lto_abort::main::h9419a0043b6e0505 + 2991
9   a                              0x000453bb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h8feecf4d0ae7c16a + 11
10  a                              0x0003f29c std::sys_common::backtrace::__rust_begin_short_backtrace::ha2aa1e5f8c9cbf9f + 12
11  a                              0x00032a84 main + 996
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffce16c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffce198  esp: 0xbffce16c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x31000 -    0x54ffb +a (0) <F7A1E390-F437-3FEC-8ACE-6BE97F5049DF> /Users/USER/*/a
   0x92000 -    0xd7fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2329
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-20-081026_Traviss-Mac-1044.crash
Process:               a [46750]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [46745]
Responsible:           a [46750]
User ID:               501
Date/Time:             2019-01-20 08:10:25.854 +0000
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
3   libstd-59cbda0315ed40bd.dylib  0x0018218b std::sys::unix::abort_internal::h1303f06e8ad0ed3f + 11
4   libstd-59cbda0315ed40bd.dylib  0x00172882 std::sys_common::util::abort::h3daeed059ec162ec + 82
5   libstd-59cbda0315ed40bd.dylib  0x001749db rust_panic + 107
6   libstd-59cbda0315ed40bd.dylib  0x0017489b std::panicking::rust_panic_with_hook::h672c6c818d1d7b0c + 603
7   a                              0x000b4d1f std::panicking::begin_panic::he9a89394182850fc + 47
8   a                              0x000b5e6c main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff4c17c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff4c1a8  esp: 0xbff4c17c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xb3000 -    0xb6ff7 +a (0) <A50923FC-987C-32F3-9D32-CB31B372AC81> /Users/USER/*/a
   0xcb000 -   0x110fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x154000 -   0x1dfff7 +libstd-59cbda0315ed40bd.dylib (0) <AFDDB41D-C12A-38EB-8A0D-C8508D6B0AF7> /Users/USER/*/libstd-59cbda0315ed40bd.dylib
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
    task_for_pid: 2329
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
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
__TEXT                            9328K       44 
mapped file                      408.7M       21 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.4M      132 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-20-081028_Traviss-Mac-1044.crash
Process:               a [46775]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [46772]
Responsible:           a [46775]
User ID:               501
Date/Time:             2019-01-20 08:10:26.599 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4900 seconds
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
    __TEXT                 0000000000018000-000000000001b000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00019efe segfault_no_out_of_stack::main::haec5f9680e9a04a3 + 2110
1   a                              0x0001883b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h485b55b3f31a5625 + 11
2   libstd-59cbda0315ed40bd.dylib  0x00167d1c std::sys_common::backtrace::__rust_begin_short_backtrace::ha2aa1e5f8c9cbf9f + 12
3   libstd-59cbda0315ed40bd.dylib  0x0016a1a4 std::panicking::try::do_call::h7b5be8fee5c2e08d + 20
4   libstd-59cbda0315ed40bd.dylib  0x00178d1d __rust_maybe_catch_panic + 29
5   libstd-59cbda0315ed40bd.dylib  0x0016ac57 std::rt::lang_start_internal::ha4e45e7ccf10cd29 + 631
6   a                              0x0001a1cc main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x7be2aa50  ecx: 0x00000000  edx: 0x00000000
  edi: 0x00178d0e  esi: 0xbffe72d0  ebp: 0xbffe7398  esp: 0xbffe71f0
   ss: 0x00000023  efl: 0x00010246  eip: 0x00019efe   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     0
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x18000 -    0x1aff3 +a (0) <0DE44F87-AE42-3CA8-B4FA-9EA8FBB35926> /Users/USER/*/a
   0xc1000 -   0x106fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x14a000 -   0x1d5ff7 +libstd-59cbda0315ed40bd.dylib (0) <AFDDB41D-C12A-38EB-8A0D-C8508D6B0AF7> /Users/USER/*/libstd-59cbda0315ed40bd.dylib
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
    task_for_pid: 2329
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
VM_ALLOCATE                        132K        3 
__DATA                            3532K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            569.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-20-081034_Traviss-Mac-1044.crash
Process:               a [46958]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [46956]
Responsible:           a [46958]
User ID:               501
Date/Time:             2019-01-20 08:10:34.218 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4900 seconds
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
    __TEXT                 00000000000a7000-00000000000aa000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000a9584 signal_exit_status::main::h013a3960fc5c363d + 436
1   a                              0x000a838b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha99faa484de490ec + 11
2   libstd-59cbda0315ed40bd.dylib  0x001b9d1c std::sys_common::backtrace::__rust_begin_short_backtrace::ha2aa1e5f8c9cbf9f + 12
3   libstd-59cbda0315ed40bd.dylib  0x001bc1a4 std::panicking::try::do_call::h7b5be8fee5c2e08d + 20
4   libstd-59cbda0315ed40bd.dylib  0x001cad1d __rust_maybe_catch_panic + 29
5   libstd-59cbda0315ed40bd.dylib  0x001bcc57 std::rt::lang_start_internal::ha4e45e7ccf10cd29 + 631
6   a                              0x000a965c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x7be545f0
  edi: 0x7be54680  esi: 0xbff58320  ebp: 0xbff583b8  esp: 0xbff582a0
   ss: 0x00000023  efl: 0x00010246  eip: 0x000a9584   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     2
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xa7000 -    0xa9ff7 +a (0) <F2187FF8-FC9B-3110-9A5E-5CD6E68CEA09> /Users/USER/*/a
  0x113000 -   0x158fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x19c000 -   0x227ff7 +libstd-59cbda0315ed40bd.dylib (0) <AFDDB41D-C12A-38EB-8A0D-C8508D6B0AF7> /Users/USER/*/libstd-59cbda0315ed40bd.dylib
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
    task_for_pid: 2329
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
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-20-081039_Traviss-Mac-1044.crash
Process:               a [47058]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47049]
Responsible:           a [47058]
User ID:               501
Date/Time:             2019-01-20 08:10:38.971 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4900 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000a37e2 simd_target_feature_mixup::test::id_avx512_512::h052c674ab7f4f4bc + 114
1   a                              0x000a2538 simd_target_feature_mixup::test::main::h379367934b9623dc + 1848
2   a                              0x000a4a69 simd_target_feature_mixup::main::h4f60990077aff357 + 937
3   a                              0x000a1b7b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h99b2cb8aa31a4fa7 + 11
4   libstd-59cbda0315ed40bd.dylib  0x0021cd1c std::sys_common::backtrace::__rust_begin_short_backtrace::ha2aa1e5f8c9cbf9f + 12
5   libstd-59cbda0315ed40bd.dylib  0x0021f1a4 std::panicking::try::do_call::h7b5be8fee5c2e08d + 20
6   libstd-59cbda0315ed40bd.dylib  0x0022dd1d __rust_maybe_catch_panic + 29
7   libstd-59cbda0315ed40bd.dylib  0x0021fc57 std::rt::lang_start_internal::ha4e45e7ccf10cd29 + 631
8   a                              0x000a4c5c main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff5e000  ebx: 0xbff5df00  ecx: 0x000a377e  edx: 0xbff5df00
  edi: 0x000a1e14  esi: 0x00000000  ebp: 0xbff5def8  esp: 0xbff5dec0
   ss: 0x00000023  efl: 0x00010246  eip: 0x000a37e2   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000a33c0
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xa1000 -    0xa5fcf +a (0) <3F33E0D1-FFC0-3E0E-A993-42ACEF6759CF> /Users/USER/*/a
  0x176000 -   0x1bbfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1ff000 -   0x28aff7 +libstd-59cbda0315ed40bd.dylib (0) <AFDDB41D-C12A-38EB-8A0D-C8508D6B0AF7> /Users/USER/*/libstd-59cbda0315ed40bd.dylib
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
    task_for_pid: 2329
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
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3532K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9332K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            568.5M      133 
TOTAL                            568.5M      133 
TOTAL, minus reserved VM space   568.4M      133 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-20-081044-1_Traviss-Mac-1044.crash
Process:               a [47199]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47191]
Responsible:           a [47199]
User ID:               501
Date/Time:             2019-01-20 08:10:44.568 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4900 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b03adea8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb03adea8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b03ad000-00000000b03ae000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b03ae000-00000000b05af000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-59cbda0315ed40bd.dylib  0x001bfcb0 std::sys::unix::thread::Thread::join::h6baf73e947f979fb + 32
4   a                              0x00099796 _$LT$std..thread..JoinHandle$LT$T$GT$$GT$::join::h96788d3433db416c + 70
5   a                              0x00098765 stack_probes::main::hc5f49a55fd7e038b + 597
6   a                              0x0009752b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h21a8e39d03106798 + 11
7   libstd-59cbda0315ed40bd.dylib  0x001afd1c std::sys_common::backtrace::__rust_begin_short_backtrace::ha2aa1e5f8c9cbf9f + 12
8   libstd-59cbda0315ed40bd.dylib  0x001b21a4 std::panicking::try::do_call::h7b5be8fee5c2e08d + 20
9   libstd-59cbda0315ed40bd.dylib  0x001c0d1d __rust_maybe_catch_panic + 29
10  libstd-59cbda0315ed40bd.dylib  0x001b2c57 std::rt::lang_start_internal::ha4e45e7ccf10cd29 + 631
11  a                              0x0009918c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-59cbda0315ed40bd.dylib  0x001c018b std::sys::unix::abort_internal::h1303f06e8ad0ed3f + 11
4   libstd-59cbda0315ed40bd.dylib  0x001b0882 std::sys_common::util::abort::h3daeed059ec162ec + 82
5   libstd-59cbda0315ed40bd.dylib  0x001bf71b std::sys::unix::stack_overflow::imp::signal_handler::h2f4d723442aba6c4 + 955
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-59cbda0315ed40bd.dylib  0x001bf360 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h71ec79f9178ccc1f + 80
9   a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x000988b0 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x000974bd std::sys_common::backtrace::__rust_begin_short_backtrace::h4711a900bc1c58c6 + 29
265 libstd-59cbda0315ed40bd.dylib  0x001c0d1d __rust_maybe_catch_panic + 29
266 a                              0x00099a73 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h12b478605d31474a + 131
267 libstd-59cbda0315ed40bd.dylib  0x001bfbeb std::sys::unix::thread::Thread::new::thread_start::h964751a69b368174 + 187
268 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
269 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
270 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb05ae000  ecx: 0x000ecb0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x000ecb38  esp: 0x000ecb0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x96000 -    0x9aff7 +a (0) <811E4362-6636-35ED-94EE-6AD61D662FCA> /Users/USER/*/a
  0x109000 -   0x14efdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x192000 -   0x21dff7 +libstd-59cbda0315ed40bd.dylib (0) <AFDDB41D-C12A-38EB-8A0D-C8508D6B0AF7> /Users/USER/*/libstd-59cbda0315ed40bd.dylib
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
---
===========                     =======  ======= 
TOTAL                            569.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-20-081052-1_Traviss-Mac-1044.crash
Process:               a [47323]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47315]
Responsible:           a [47323]
User ID:               501
Date/Time:             2019-01-20 08:10:51.310 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4900 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b02eaf08
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb02eaf08:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b02ea000-00000000b02eb000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b02eb000-00000000b04ec000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   a                              0x000e99a9 stack_probes_lto::main::h7f9d4bbc5a99dd1a + 2553
4   a                              0x0010230b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h8a1229fb7caf2e45 + 11
5   a                              0x000fac1c std::sys_common::backtrace::__rust_begin_short_backtrace::ha2aa1e5f8c9cbf9f + 12
6   a                              0x000eb03d main + 589
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000ec4ab std::sys::unix::abort_internal::h1303f06e8ad0ed3f + 11
4   a                              0x000ec492 std::sys_common::util::abort::h3daeed059ec162ec + 82
5   a                              0x000fab74 std::sys::unix::stack_overflow::imp::signal_handler::h2f4d723442aba6c4 + 868
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000fa810 rust_begin_unwind + 16
9   a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
10  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
11  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
12  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
13  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
14  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
15  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
16  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
17  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
18  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
19  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
20  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
21  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
22  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
23  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
24  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
25  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
26  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
27  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
28  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
29  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
30  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
31  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
32  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
33  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
34  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
35  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
36  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
37  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
38  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
39  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
40  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
41  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
42  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
43  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
44  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
45  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
46  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
47  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
48  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
49  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
50  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
51  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
52  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
53  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
54  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
55  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
56  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
57  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
58  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
59  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
60  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
61  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
62  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
63  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
64  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
65  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
66  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
67  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
68  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
69  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
70  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
71  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
72  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
73  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
74  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
75  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
76  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
77  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
78  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
79  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
80  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
81  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
82  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
83  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
84  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
85  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
86  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
87  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
88  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
89  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
90  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
91  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
92  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
93  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
94  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
95  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
96  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
97  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
98  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
99  a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
100 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
101 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
102 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
103 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
104 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
105 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
106 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
107 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
108 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
109 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
110 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
111 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
112 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
113 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
114 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
115 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
116 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
117 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
118 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
119 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
120 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
121 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
122 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
123 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
124 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
125 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
126 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
127 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
128 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
129 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
130 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
131 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
132 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
133 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
134 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
135 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
136 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
137 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
138 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
139 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
140 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
141 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
142 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
143 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
144 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
145 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
146 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
147 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
148 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
149 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
150 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
151 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
152 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
153 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
154 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
155 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
156 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
157 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
158 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
159 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
160 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
161 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
162 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
163 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
164 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
165 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
166 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
167 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
168 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
169 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
170 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
171 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
172 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
173 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
174 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
175 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
176 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
177 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
178 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
179 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
180 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
181 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
182 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
183 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
184 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
185 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
186 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
187 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
188 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
189 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
190 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
191 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
192 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
193 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
194 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
195 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
196 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
197 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
198 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
199 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
200 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
201 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
202 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
203 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
204 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
205 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
206 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
207 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
208 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
209 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
210 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
211 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
212 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
213 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
214 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
215 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
216 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
217 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
218 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
219 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
220 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
221 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
222 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
223 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
224 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
225 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
226 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
227 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
228 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
229 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
230 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
231 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
232 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
233 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
234 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
235 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
236 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
237 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
238 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
239 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
240 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
241 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
242 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
243 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
244 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
245 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
246 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
247 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
248 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
249 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
250 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
251 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
252 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
253 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
254 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
255 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
256 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
257 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
258 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
259 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
260 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
261 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
262 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
263 a                              0x000e9e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
264 a                              0x001024a4 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h8f92ca76caaed9f2 + 116
265 a                              0x000faecb std::sys::unix::thread::Thread::new::thread_start::h964751a69b368174 + 187
266 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
267 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
268 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb04eb000  ecx: 0x0016cb0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x0016cb38  esp: 0x0016cb0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xe8000 -   0x113ffb +a (0) <16145C5F-D1D3-3920-9EFE-94FA9F2B98E6> /Users/USER/*/a
  0x21f000 -   0x264fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2329
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
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-20-081052_Traviss-Mac-1044.crash
Process:               a [47319]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [47315]
Responsible:           a [47319]
User ID:               501
Date/Time:             2019-01-20 08:10:51.121 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4900 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf603e8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf603e8:
    Stack Guard            00000000bbf5f000-00000000bbf60000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf60000-00000000bbf61000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf61000-00000000bff5f000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000a64ab std::sys::unix::abort_internal::h1303f06e8ad0ed3f + 11
4   a                              0x000a6492 std::sys_common::util::abort::h3daeed059ec162ec + 82
5   a                              0x000b4b74 std::sys::unix::stack_overflow::imp::signal_handler::h2f4d723442aba6c4 + 868
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000b4810 rust_begin_unwind + 16
9   a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
10  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
11  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
12  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
13  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
14  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
15  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
16  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
17  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
18  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
19  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
20  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
21  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
22  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
23  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
24  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
25  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
26  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
27  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
28  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
29  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
30  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
31  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
32  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
33  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
34  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
35  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
36  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
37  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
38  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
39  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
40  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
41  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
42  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
43  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
44  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
45  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
46  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
47  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
48  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
49  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
50  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
51  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
52  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
53  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
54  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
55  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
56  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
57  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
58  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
59  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
60  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
61  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
62  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
63  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
64  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
65  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
66  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
67  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
68  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
69  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
70  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
71  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
72  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
73  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
74  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
75  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
76  a                              0x000a3e28 stack_probes_lto::recurse::h907252696a8f0ddd + 40
