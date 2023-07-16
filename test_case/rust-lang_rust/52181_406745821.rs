plain
[00:02:45]       Memory: 8 GB
[00:02:45]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:45]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:45]       SMC Version (system): 2.8f0
[00:02:45]       Serial Number (system): VMGe2THCWTBS
[00:02:45] 
[00:02:45] hw.ncpu: 4
[00:02:45] hw.byteorder: 1234
[00:02:45] hw.memsize: 8589934592
---
[01:37:10] 
[01:37:10] 12 3 | no
[01:37:10] 13   | ^^ not found in this scope
[01:37:10] 14 
[01:37:10] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 26)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:37:10] + thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:37:10] 17 
[01:37:10] 17 
[01:37:10] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 20) stdout ----
[01:37:10] 
[01:37:10] - thread '$DIR/failed-doctest-output.rs - SomeStruct (line 20)' panicked at 'test executable failed:
[01:37:10] + thread 'rustc' panicked at 'test executable failed:
[01:37:10] 20 
[01:37:10] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:37:10] 
[01:37:10] 
[01:37:10] The actual stdout differed from the expected stdout.
[01:37:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:37:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:37:10] Actual stdout saved to /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:37:10] To update references, rerun the tests and pass the `--bless` flag
[01:37:10] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:37:10] error: 1 errors occurred comparing output.
[01:37:10] status: exit code: 101
[01:37:10] status: exit code: 101
[01:37:10] command: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc" "/Users/travis/build/rust-lang/rust/src/test/rustdoc-ui/failed-doctest-output.rs" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-o" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--test" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:37:10] ------------------------------------------
[01:37:10] 
[01:37:10] running 2 tests
[01:37:10] running 2 tests
[01:37:10] test src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) ... FAILED
[01:37:10] test src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20) ... FAILED
[01:37:10] failures:
[01:37:10] 
[01:37:10] 
[01:37:10] ---- src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) stdout ----
[01:37:10] error[E0425]: cannot find value `no` in this scope
[01:37:10]  --> src/test/rustdoc-ui/failed-doctest-output.rs:27:1
[01:37:10] 3 | no
[01:37:10]   | ^^ not found in this scope
[01:37:10] 
[01:37:10] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:37:10] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:37:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:37:10] 
[01:37:10] ---- src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20) stdout ----
[01:37:10] thread 'rustc' panicked at 'test executable failed:
[01:37:10] 
[01:37:10] thread 'main' panicked at 'oh no', src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:37:10] 
[01:37:10] ', librustdoc/test.rs:367:17
[01:37:10] 
[01:37:10] 
[01:37:10] 
[01:37:10] failures:
[01:37:10]     src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26)
[01:37:10]     src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20)
[01:37:10] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:37:10] 
[01:37:10] 
[01:37:10] ------------------------------------------
---
[01:37:10] test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:37:10] 
[01:37:10] 
[01:37:10] 
[01:37:10] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/rustdoc-ui" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/rustdoc-ui" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "7.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:37:10] 
[01:37:10] 
[01:37:10] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:37:10] Build completed unsuccessfully in 0:44:24
[01:37:10] Build completed unsuccessfully in 0:44:24
[01:37:10] make: *** [check] Error 1
travis_time:end:0b5cb0b2:start=1532121213857999000,finish=1532127043749774000,duration=5829891775000

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00b466b1
---
travis_fold:start:after_failure.2
travis_time:start:01e4086c
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1296
drwx------  28 travis  staff    952 Jul 20 22:48 .
-rw-------@  1 travis  staff  13746 Jul 20 22:48 overflow_2018-07-20-224834_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Jul 20 22:48 foo_2018-07-20-224815_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1417 Jul 20 22:47 m4_2018-07-20-224751_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Jul 20 22:47 bar_2018-07-20-224744_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10690 Jul 20 22:47 b_2018-07-20-224743_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Jul 20 22:47 bar_2018-07-20-224743_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37510 Jul 20 22:16 a_2018-07-20-221617-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62273 Jul 20 22:16 a_2018-07-20-221617_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37295 Jul 20 22:16 a_2018-07-20-221611-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60415 Jul 20 22:16 a_2018-07-20-221611_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10171 Jul 20 22:16 a_2018-07-20-221602_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9902 Jul 20 22:15 a_2018-07-20-221557_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9907 Jul 20 22:15 a_2018-07-20-221549_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9871 Jul 20 22:15 a_2018-07-20-221548_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9965 Jul 20 22:15 a_2018-07-20-221524_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10382 Jul 20 22:15 a_2018-07-20-221520-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10630 Jul 20 22:15 a_2018-07-20-221520_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63133 Jul 20 22:15 a_2018-07-20-221514_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64389 Jul 20 22:15 a_2018-07-20-221510-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65148 Jul 20 22:15 a_2018-07-20-221510-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63943 Jul 20 22:15 a_2018-07-20-221510_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11785 Jul 20 22:13 a_2018-07-20-221314_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10333 Jul 20 22:11 a_2018-07-20-221124_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10243 Jul 20 22:11 a_2018-07-20-221116-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10513 Jul 20 22:11 a_2018-07-20-221116-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10515 Jul 20 22:11 a_2018-07-20-221116_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25 19:20 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0775ba31
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-07-20-221116-1_Traviss-Mac-1044.crash
Process:               a [32928]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [32927]
Responsible:           a [32928]
User ID:               501
Date/Time:             2018-07-20 22:10:25.213 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Report Version:        12
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 3500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00000001017e177e abort_on_c_abi::panic_in_ffi::h305168285df224eb + 30
1   a                              0x00000001017e0b79 std::panicking::try::do_call::h93ddb7cee707c6a7 (.llvm.16562876749859169606) + 9
2   libstd-6510406d9674bb80.dylib  0x000000010183014f __rust_maybe_catch_panic + 31
3   a                              0x00000001017e19d1 abort_on_c_abi::main::h82c0574e6cdd7b93 + 593
4   a                              0x00000001017dfed6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h8b0c1266fabf66f2 + 6
5   libstd-6510406d9674bb80.dylib  0x0000000101823508 std::panicking::try::do_call::hb67a0b74fa1ac76f (.llvm.11167475731577206974) + 24
6   libstd-6510406d9674bb80.dylib  0x000000010183014f __rust_maybe_catch_panic + 31
7   libstd-6510406d9674bb80.dylib  0x000000010180435d std::rt::lang_start_internal::h9a51de15dc126962 + 237
8   a                              0x00000001017e1cdc main + 44
9   libdyld.dylib                  0x00007fff5e18f115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000101e1c050  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffeee41e818  rsi: 0xfffffffffffffce8  rbp: 0x00007ffeee41f220  rsp: 0x00007ffeee41f220
   r8: 0x0000000000000000   r9: 0x00000001018f36b0  r10: 0x000000010d98c8d0  r11: 0x00007fff5e44696c
  r12: 0x0000000000000000  r13: 0x0000000000000000  r14: 0x00007ffeee41f340  r15: 0x00007ffeee41f288
  rip: 0x00000001017e177e  rfl: 0x0000000000010202  cr2: 0x00000001018a7dd8
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1017df000 -        0x1017e2ff7 +a (0) <570268C4-DD80-320B-8782-26C437061D35> /Users/USER/*/a
       0x1017e9000 -        0x1018e8fcf +libstd-6510406d9674bb80.dylib (0) <9F640CEE-2835-3913-9F78-A0BE0FE709C7> /Users/USER/*/libstd-6510406d9674bb80.dylib
       0x10d93a000 -        0x10d98498f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5b9f9000 -     0x7fff5ba2cfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5bf0b000 -     0x7fff5bf0cff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5c1c1000 -     0x7fff5c217fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5c218000 -     0x7fff5c23cff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5d58e000 -     0x7fff5d97f3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5da4c000 -     0x7fff5da68ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5e026000 -     0x7fff5e02aff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5e02b000 -     0x7fff5e035ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5e036000 -     0x7fff5e03dfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5e03e000 -     0x7fff5e046ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5e047000 -     0x7fff5e0ccfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5e154000 -     0x7fff5e18dff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5e18e000 -     0x7fff5e1abff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5e1ac000 -     0x7fff5e1acffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5e1ba000 -     0x7fff5e1baff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5e1bb000 -     0x7fff5e1bfffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5e1c0000 -     0x7fff5e1c2ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5e1c3000 -     0x7fff5e1c4ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5e1c5000 -     0x7fff5e1dcfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5e1dd000 -     0x7fff5e1ddfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5e1de000 -     0x7fff5e267ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5e268000 -     0x7fff5e26bffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5e26c000 -     0x7fff5e26fffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5e270000 -     0x7fff5e271fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5e272000 -     0x7fff5e278ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5e279000 -     0x7fff5e2c2ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5e2c3000 -     0x7fff5e2e8ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5e2e9000 -     0x7fff5e334fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5e335000 -     0x7fff5e354fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5e355000 -     0x7fff5e3f9ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5e3fa000 -     0x7fff5e404ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5e405000 -     0x7fff5e40eff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5e40f000 -     0x7fff5e416ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5e417000 -     0x7fff5e422fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5e423000 -     0x7fff5e426ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5e427000 -     0x7fff5e428ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5e429000 -     0x7fff5e430ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5e431000 -     0x7fff5e444ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5e446000 -     0x7fff5e44bff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5e44c000 -     0x7fff5e478ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2391
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.9M resident=0K(0%) swapped_out_or_unallocated=198.9M(100%)
Writable regions: Total=75.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9268K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4344K       45 
__LINKEDIT                       189.1M        5 
__TEXT                             9.8M       44 
===========                     =======  ======= 
TOTAL                            280.4M      114 
TOTAL                            280.4M      114 
TOTAL, minus reserved VM space   280.2M      114 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-07-20-221116-2_Traviss-Mac-1044.crash
Process:               a [33596]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        a [33589]
Responsible:           a [33596]
User ID:               501
Date/Time:             2018-07-20 22:10:43.879 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Report Version:        12
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 3500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-6510406d9674bb80.dylib  0x000000010c11da3b std::panicking::rust_panic_with_hook::h6b739c227cfc90e3 + 139
1   a                              0x000000010c0d5708 std::panicking::begin_panic::h2c3439866cead107 + 40
2   a                              0x000000010c0d31bc _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hee8ed3175b25d327 + 28
3   a                              0x000000010c0d2e69 core::ptr::drop_in_place::hccefe1efda33bf5c + 9
4   a                              0x000000010c0d3193 backtrace::double::h3a79da7ae181846f + 35
5   a                              0x000000010c0d434e backtrace::main::h9751c5dfa7559d82 + 4254 (backtrace.rs:113)
6   a                              0x000000010c0d26a6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hab96ed8a34843ae1 + 6 (rt.rs:74)
7   libstd-6510406d9674bb80.dylib  0x000000010c11d508 std::panicking::try::do_call::hb67a0b74fa1ac76f (.llvm.11167475731577206974) + 24
8   libstd-6510406d9674bb80.dylib  0x000000010c12a14f __rust_maybe_catch_panic + 31
9   libstd-6510406d9674bb80.dylib  0x000000010c0fe35d std::rt::lang_start_internal::h9a51de15dc126962 + 237
10  a                              0x000000010c0d4bdc main + 44
11  libdyld.dylib                  0x00007fff5e18f115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee3b2ff03  rbx: 0x0000000000000002  rcx: 0x000000000000002b  rdx: 0x0000000000000000
  rdi: 0x0000000000000000  rsi: 0x0000000000000000  rbp: 0x00007ffee3b30080  rsp: 0x00007ffee3b30000
   r8: 0x000000010c1a4fb0   r9: 0x000000010c1ed6b0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010c1a4fb0  r14: 0x000000010c0d7460  r15: 0x00007ffee3b30090
  rip: 0x000000010c11da3b  rfl: 0x0000000000010202  cr2: 0x000000010c429be3
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10c0ce000 -        0x10c0d6ff7 +a (0) <80A4A7A9-33B6-3182-8CE4-DD5345A32ECF> /Users/USER/*/a
       0x10c0e3000 -        0x10c1e2fcf +libstd-6510406d9674bb80.dylib (0) <9F640CEE-2835-3913-9F78-A0BE0FE709C7> /Users/USER/*/libstd-6510406d9674bb80.dylib
       0x11b812000 -        0x11b85c98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5b9f9000 -     0x7fff5ba2cfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5bf0b000 -     0x7fff5bf0cff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5c1c1000 -     0x7fff5c217fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5c218000 -     0x7fff5c23cff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5d58e000 -     0x7fff5d97f3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5da4c000 -     0x7fff5da68ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5e026000 -     0x7fff5e02aff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5e02b000 -     0x7fff5e035ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5e036000 -     0x7fff5e03dfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5e03e000 -     0x7fff5e046ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5e047000 -     0x7fff5e0ccfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5e154000 -     0x7fff5e18dff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5e18e000 -     0x7fff5e1abff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5e1ac000 -     0x7fff5e1acffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5e1ba000 -     0x7fff5e1baff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5e1bb000 -     0x7fff5e1bfffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5e1c0000 -     0x7fff5e1c2ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5e1c3000 -     0x7fff5e1c4ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5e1c5000 -     0x7fff5e1dcfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5e1dd000 -     0x7fff5e1ddfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5e1de000 -     0x7fff5e267ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5e268000 -     0x7fff5e26bffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5e26c000 -     0x7fff5e26fffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5e270000 -     0x7fff5e271fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5e272000 -     0x7fff5e278ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5e279000 -     0x7fff5e2c2ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5e2c3000 -     0x7fff5e2e8ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5e2e9000 -     0x7fff5e334fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5e335000 -     0x7fff5e354fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5e355000 -     0x7fff5e3f9ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5e3fa000 -     0x7fff5e404ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5e405000 -     0x7fff5e40eff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5e40f000 -     0x7fff5e416ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5e417000 -     0x7fff5e422fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5e423000 -     0x7fff5e426ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5e427000 -     0x7fff5e428ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5e429000 -     0x7fff5e430ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5e431000 -     0x7fff5e444ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5e446000 -     0x7fff5e44bff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5e44c000 -     0x7fff5e478ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2391
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.9M resident=0K(0%) swapped_out_or_unallocated=198.9M(100%)
Writable regions: Total=76.9M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.9M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9268K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4344K       45 
__LINKEDIT                       189.1M        5 
__TEXT                             9.8M       44 
===========                     =======  ======= 
TOTAL                            280.4M      114 
TOTAL                            280.4M      114 
TOTAL, minus reserved VM space   280.3M      114 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-07-20-221116_Traviss-Mac-1044.crash
Process:               a [33597]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [33589]
Responsible:           a [33597]
User ID:               501
Date/Time:             2018-07-20 22:10:43.931 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Report Version:        12
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 3500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-6510406d9674bb80.dylib  0x0000000103aaaa3b std::panicking::rust_panic_with_hook::h6b739c227cfc90e3 + 139
1   a                              0x0000000103a5e708 std::panicking::begin_panic::h2c3439866cead107 + 40
2   a                              0x0000000103a5c1bc _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hee8ed3175b25d327 + 28
3   a                              0x0000000103a5be69 core::ptr::drop_in_place::hccefe1efda33bf5c + 9
4   a                              0x0000000103a5c193 backtrace::double::h3a79da7ae181846f + 35
5   a                              0x0000000103a5d34e backtrace::main::h9751c5dfa7559d82 + 4254 (backtrace.rs:113)
6   a                              0x0000000103a5b6a6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hab96ed8a34843ae1 + 6 (rt.rs:74)
7   libstd-6510406d9674bb80.dylib  0x0000000103aaa508 std::panicking::try::do_call::hb67a0b74fa1ac76f (.llvm.11167475731577206974) + 24
8   libstd-6510406d9674bb80.dylib  0x0000000103ab714f __rust_maybe_catch_panic + 31
9   libstd-6510406d9674bb80.dylib  0x0000000103a8b35d std::rt::lang_start_internal::h9a51de15dc126962 + 237
10  a                              0x0000000103a5dbdc main + 44
11  libdyld.dylib                  0x00007fff5e18f115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeec1a6f03  rbx: 0x0000000000000002  rcx: 0x000000000000002b  rdx: 0x0000000000000000
  rdi: 0x0000000000000000  rsi: 0x0000000000000000  rbp: 0x00007ffeec1a7060  rsp: 0x00007ffeec1a6fe0
   r8: 0x0000000103b31fb0   r9: 0x0000000103b7a6b0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000103b31fb0  r14: 0x0000000103a60460  r15: 0x00007ffeec1a7070
  rip: 0x0000000103aaaa3b  rfl: 0x0000000000010206  cr2: 0x0000000103b188c0
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x103a57000 -        0x103a5fff7 +a (0) <80A4A7A9-33B6-3182-8CE4-DD5345A32ECF> /Users/USER/*/a
       0x103a70000 -        0x103b6ffcf +libstd-6510406d9674bb80.dylib (0) <9F640CEE-2835-3913-9F78-A0BE0FE709C7> /Users/USER/*/libstd-6510406d9674bb80.dylib
       0x10bb4d000 -        0x10bb9798f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5b9f9000 -     0x7fff5ba2cfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5bf0b000 -     0x7fff5bf0cff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5c1c1000 -     0x7fff5c217fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5c218000 -     0x7fff5c23cff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5d58e000 -     0x7fff5d97f3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5da4c000 -     0x7fff5da68ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5e026000 -     0x7fff5e02aff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5e02b000 -     0x7fff5e035ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5e036000 -     0x7fff5e03dfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5e03e000 -     0x7fff5e046ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5e047000 -     0x7fff5e0ccfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5e154000 -     0x7fff5e18dff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5e18e000 -     0x7fff5e1abff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5e1ac000 -     0x7fff5e1acffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5e1ba000 -     0x7fff5e1baff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5e1bb000 -     0x7fff5e1bfffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5e1c0000 -     0x7fff5e1c2ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5e1c3000 -     0x7fff5e1c4ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5e1c5000 -     0x7fff5e1dcfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5e1dd000 -     0x7fff5e1ddfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5e1de000 -     0x7fff5e267ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5e268000 -     0x7fff5e26bffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5e26c000 -     0x7fff5e26fffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5e270000 -     0x7fff5e271fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5e272000 -     0x7fff5e278ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5e279000 -     0x7fff5e2c2ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5e2c3000 -     0x7fff5e2e8ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5e2e9000 -     0x7fff5e334fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5e335000 -     0x7fff5e354fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5e355000 -     0x7fff5e3f9ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5e3fa000 -     0x7fff5e404ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5e405000 -     0x7fff5e40eff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5e40f000 -     0x7fff5e416ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5e417000 -     0x7fff5e422fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5e423000 -     0x7fff5e426ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5e427000 -     0x7fff5e428ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5e429000 -     0x7fff5e430ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5e431000 -     0x7fff5e444ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5e446000 -     0x7fff5e44bff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5e44c000 -     0x7fff5e478ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2391
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.9M resident=0K(0%) swapped_out_or_unallocated=198.9M(100%)
Writable regions: Total=76.9M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.9M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9268K       10 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4344K       45 
__LINKEDIT                       189.1M        5 
__TEXT                             9.8M       44 
===========                     =======  ======= 
TOTAL                            280.4M      113 
TOTAL                            280.4M      113 
TOTAL, minus reserved VM space   280.3M      113 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-07-20-221124_Traviss-Mac-1044.crash
Process:               a [35004]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [35002]
Responsible:           a [35004]
User ID:               501
Date/Time:             2018-07-20 22:11:24.657 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Report Version:        12
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 3600 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff5e2dee3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff5e41d150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff5e23b312 abort + 127
3   libstd-6510406d9674bb80.dylib  0x00000001009c1a89 std::sys::unix::abort_internal::h8ebb89e0ea83e887 + 9
4   libstd-6510406d9674bb80.dylib  0x000000010099e610 rust_oom + 32
5   libstd-6510406d9674bb80.dylib  0x0000000100a117a9 alloc::alloc::handle_alloc_error::h8063cd65b49c5321 + 9
6   a                              0x000000010098fa6d default_alloc_error_hook::main::hbf3cf79eecbb97ff + 797
7   a                              0x000000010098fc66 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd2becf04f82d6cb3 + 6
8   libstd-6510406d9674bb80.dylib  0x00000001009cf508 std::panicking::try::do_call::hb67a0b74fa1ac76f (.llvm.11167475731577206974) + 24
9   libstd-6510406d9674bb80.dylib  0x00000001009dc14f __rust_maybe_catch_panic + 31
10  libstd-6510406d9674bb80.dylib  0x00000001009b035d std::rt::lang_start_internal::h9a51de15dc126962 + 237
11  a                              0x000000010098fbdc main + 44
12  libdyld.dylib                  0x00007fff5e18f115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff96eb5340  rcx: 0x00007ffeef270198  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffeef2701d0  rsp: 0x00007ffeef270198
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff5e2dee3e  rfl: 0x0000000000000206  cr2: 0x00007fff96e93148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10098e000 -        0x100990fff +a (0) <DB1D828A-157D-32AB-9F88-AD3C71C26126> /Users/USER/*/a
       0x100995000 -        0x100a94fcf +libstd-6510406d9674bb80.dylib (0) <9F640CEE-2835-3913-9F78-A0BE0FE709C7> /Users/USER/*/libstd-6510406d9674bb80.dylib
       0x10dfba000 -        0x10e00498f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5b9f9000 -     0x7fff5ba2cfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5bf0b000 -     0x7fff5bf0cff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5c1c1000 -     0x7fff5c217fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5c218000 -     0x7fff5c23cff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5d58e000 -     0x7fff5d97f3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5da4c000 -     0x7fff5da68ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5e026000 -     0x7fff5e02aff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5e02b000 -     0x7fff5e035ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5e036000 -     0x7fff5e03dfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5e03e000 -     0x7fff5e046ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5e047000 -     0x7fff5e0ccfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5e154000 -     0x7fff5e18dff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5e18e000 -     0x7fff5e1abff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5e1ac000 -     0x7fff5e1acffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5e1ba000 -     0x7fff5e1baff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5e1bb000 -     0x7fff5e1bfffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5e1c0000 -     0x7fff5e1c2ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5e1c3000 -     0x7fff5e1c4ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5e1c5000 -     0x7fff5e1dcfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5e1dd000 -     0x7fff5e1ddfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5e1de000 -     0x7fff5e267ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5e268000 -     0x7fff5e26bffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5e26c000 -     0x7fff5e26fffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5e270000 -     0x7fff5e271fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5e272000 -     0x7fff5e278ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5e279000 -     0x7fff5e2c2ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5e2c3000 -     0x7fff5e2e8ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5e2e9000 -     0x7fff5e334fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5e335000 -     0x7fff5e354fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5e355000 -     0x7fff5e3f9ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5e3fa000 -     0x7fff5e404ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5e405000 -     0x7fff5e40eff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5e40f000 -     0x7fff5e416ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5e417000 -     0x7fff5e422fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5e423000 -     0x7fff5e426ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5e427000 -     0x7fff5e428ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5e429000 -     0x7fff5e430ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5e431000 -     0x7fff5e444ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5e446000 -     0x7fff5e44bff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5e44c000 -     0x7fff5e478ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
