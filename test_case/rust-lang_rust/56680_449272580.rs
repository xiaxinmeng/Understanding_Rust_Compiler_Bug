plain
[00:02:18]       Memory: 8 GB
[00:02:18]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:18]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:18]       SMC Version (system): 2.8f0
[00:02:18]       Serial Number (system): VMRygrVZYW0x
[00:02:18] 
[00:02:18] hw.ncpu: 4
[00:02:18] hw.byteorder: 1234
[00:02:18] hw.memsize: 8589934592
---
[01:24:25] stdout:
[01:24:25] ------------------------------------------
[01:24:25] 
[01:24:25] running 3 tests
[01:24:25] test src/test/rustdoc/test_option_check/test.rs - Foo (line 18) ... FAILED
[01:24:25] test src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) ... FAILED
[01:24:25] test src/test/rustdoc/test_option_check/test.rs - Bar (line 25) ... ok
[01:24:25] failures:
[01:24:25] 
[01:24:25] 
[01:24:25] ---- src/test/rustdoc/test_option_check/test.rs - Foo (line 18) stdout ----
[01:24:25] error: linking with `cc` failed: signal: 4
[01:24:25]   |
[01:24:25]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestpMy6ow/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestpMy6ow/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestpMy6ow/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestpMy6ow/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestpMy6ow/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestpMy6ow/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestpMy6ow/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestpMy6ow/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/rustdoc/test_option_check/test/auxiliary" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-9704616a50a0e937.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-a0b6769950f7e157.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-8796db185c6ba513.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-a66934cb1b5a42c7.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-06b245903ea78666.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-10dc856d5a0a5273.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-8c58198746621e91.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-4a71e986e661e449.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-209499a34281d3a0.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[01:24:25] 
[01:24:25] thread 'src/test/rustdoc/test_option_check/test.rs - Foo (line 18)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:24:25] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:24:25] 
[01:24:25] 
[01:24:25] ---- src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) stdout ----
[01:24:25] error: linking with `cc` failed: signal: 4
[01:24:25]   |
[01:24:25]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestRfMcu1/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestRfMcu1/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestRfMcu1/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestRfMcu1/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestRfMcu1/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestRfMcu1/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestRfMcu1/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestRfMcu1/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/rustdoc/test_option_check/test/auxiliary" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-9704616a50a0e937.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-a0b6769950f7e157.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-8796db185c6ba513.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-a66934cb1b5a42c7.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-06b245903ea78666.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-10dc856d5a0a5273.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-8c58198746621e91.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-4a71e986e661e449.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-209499a34281d3a0.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[01:24:25] 
[01:24:25] 
[01:24:25] thread 'src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:24:25] 
[01:24:25] failures:
[01:24:25] failures:
[01:24:25]     src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16)
[01:24:25]     src/test/rustdoc/test_option_check/test.rs - Foo (line 18)
[01:24:25] 
[01:24:25] test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:24:25] 
[01:24:25] 
[01:24:25] make: *** [check] Error 1
[01:24:25] ------------------------------------------
[01:24:25] stderr:
[01:24:25] ------------------------------------------
[01:24:25] 
---
[01:24:25] test result: FAILED. 282 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
[01:24:25] 
[01:24:25] 
[01:24:25] 
[01:24:25] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/rustdoc" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/rustdoc" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "rustdoc" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:24:25] 
[01:24:25] 
[01:24:25] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:24:25] Build completed unsuccessfully in 0:28:37
---
travis_fold:start:after_failure.2
travis_time:start:07c15388
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1184
drwx------  21 travis  staff    714 Dec 21 06:08 .
-rw-------@  1 travis  staff  62244 Dec 21 06:08 a_2018-12-21-060858_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37481 Dec 21 06:08 a_2018-12-21-060857_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60389 Dec 21 06:08 a_2018-12-21-060850-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37238 Dec 21 06:08 a_2018-12-21-060850_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Dec 21 06:08 a_2018-12-21-060845_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Dec 21 06:08 a_2018-12-21-060841_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Dec 21 06:08 a_2018-12-21-060834_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9795 Dec 21 06:08 a_2018-12-21-060833_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Dec 21 06:07 a_2018-12-21-060758_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63126 Dec 21 06:07 a_2018-12-21-060748_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 Dec 21 06:07 a_2018-12-21-060746_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64248 Dec 21 06:07 a_2018-12-21-060745-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65091 Dec 21 06:07 a_2018-12-21-060745_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11583 Dec 21 06:05 a_2018-12-21-060549_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Dec 21 06:04 a_2018-12-21-060459_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Dec 21 06:03 a_2018-12-21-060352_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10489 Dec 21 06:02 a_2018-12-21-060258-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Dec 21 06:02 a_2018-12-21-060258_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10213 Dec 21 06:02 a_2018-12-21-060256_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0dd92142
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-21-060256_Traviss-Mac-1044.crash
Process:               a [41109]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41108]
Responsible:           a [41109]
User ID:               501
Date/Time:             2018-12-21 06:02:26.829 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 3900 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x0000000108ab775e abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x0000000108ab6b59 std::panicking::try::do_call::hcd71044095ad0489 (.llvm.7622872935148447779) + 9
2   libstd-9704616a50a0e937.dylib  0x0000000108afb3ff __rust_maybe_catch_panic + 31
3   a                              0x0000000108ab79b1 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x0000000108ab5f06 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h9b0735fb04836369 + 6
5   libstd-9704616a50a0e937.dylib  0x0000000108adfbf8 std::panicking::try::do_call::h15f3aa7ff9620fa7 + 24
6   libstd-9704616a50a0e937.dylib  0x0000000108afb3ff __rust_maybe_catch_panic + 31
7   libstd-9704616a50a0e937.dylib  0x0000000108ae064b std::rt::lang_start_internal::h948ce4de2cd5b179 + 379
8   a                              0x0000000108ab7cb9 main + 41
9   libdyld.dylib                  0x00007fff6fa37115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007fa61d500010  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee7148448  rsi: 0x00000000ffffffc3  rbp: 0x00007ffee7148ea0  rsp: 0x00007ffee7148ea0
   r8: 0x0000000061d50006   r9: 0x0000000000000004  r10: 0x000000010fa538d0  r11: 0x00007fff6fcee96c
  r12: 0x0000000000000000  r13: 0x0000000000000000  r14: 0x00007ffee7148fc0  r15: 0x00007ffee7148f08
  rip: 0x0000000108ab775e  rfl: 0x0000000000010206  cr2: 0x0000000108b3b62c
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x108ab5000 -        0x108ab8fff +a (0) <A194790B-0D5A-381C-80A2-DF595EFAC725> /Users/USER/*/a
       0x108abd000 -        0x108b4fff7 +libstd-9704616a50a0e937.dylib (0) <BD0470BE-04AB-360E-BDF3-D0FF14EC2E1A> /Users/USER/*/libstd-9704616a50a0e937.dylib
       0x10fa01000 -        0x10fa4b98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6d2a1000 -     0x7fff6d2d4fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6d7b3000 -     0x7fff6d7b4ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6da69000 -     0x7fff6dabffff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6dac0000 -     0x7fff6dae4ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6ee36000 -     0x7fff6f2273b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6f2f4000 -     0x7fff6f310ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6f8ce000 -     0x7fff6f8d2ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6f8d3000 -     0x7fff6f8ddff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6f8de000 -     0x7fff6f8e5fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6f8e6000 -     0x7fff6f8eeffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6f8ef000 -     0x7fff6f974fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6f9fc000 -     0x7fff6fa35ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6fa36000 -     0x7fff6fa53ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6fa54000 -     0x7fff6fa54ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6fa62000 -     0x7fff6fa62ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6fa63000 -     0x7fff6fa67ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6fa68000 -     0x7fff6fa6aff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6fa6b000 -     0x7fff6fa6cff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6fa6d000 -     0x7fff6fa84fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6fa85000 -     0x7fff6fa85fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6fa86000 -     0x7fff6fb0fff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6fb10000 -     0x7fff6fb13ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6fb14000 -     0x7fff6fb17ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6fb18000 -     0x7fff6fb19fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6fb1a000 -     0x7fff6fb20ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6fb21000 -     0x7fff6fb6aff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6fb6b000 -     0x7fff6fb90ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6fb91000 -     0x7fff6fbdcfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6fbdd000 -     0x7fff6fbfcfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6fbfd000 -     0x7fff6fca1ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6fca2000 -     0x7fff6fcacffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6fcad000 -     0x7fff6fcb6ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6fcb7000 -     0x7fff6fcbeff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6fcbf000 -     0x7fff6fccafff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6fccb000 -     0x7fff6fcceff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6fccf000 -     0x7fff6fcd0ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6fcd1000 -     0x7fff6fcd8ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6fcd9000 -     0x7fff6fcecff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6fcee000 -     0x7fff6fcf3ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6fcf4000 -     0x7fff6fd20ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 1811
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.3M resident=0K(0%) swapped_out_or_unallocated=198.3M(100%)
Writable regions: Total=74.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=74.4M(100%)
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
__DATA                            4528K       44 
__LINKEDIT                       188.9M        5 
__TEXT                            9604K       44 
===========                     =======  ======= 
TOTAL                            276.9M      108 
TOTAL                            276.9M      108 
TOTAL, minus reserved VM space   276.8M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-21-060258-1_Traviss-Mac-1044.crash
Process:               a [41901]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        a [41893]
Responsible:           a [41901]
User ID:               501
Date/Time:             2018-12-21 06:02:57.576 +0000
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
0   libstd-9704616a50a0e937.dylib  0x0000000108abb38c std::panicking::rust_panic_with_hook::h62ca8c89ccbdc3f7 + 668
1   a                              0x0000000108a8d7f5 std::panicking::begin_panic::h0183cf39c4f26474 + 37
2   a                              0x0000000108a8b2bc _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hbddd8cc943232966 + 28
3   a                              0x0000000108a8a9e9 core::ptr::real_drop_in_place::h0bccc5556cf0dbf4 + 9
4   a                              0x0000000108a8b293 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x0000000108a8c42e backtrace::main::hcde7a1a1c3c85e77 + 4238 (backtrace.rs:113)
6   a                              0x0000000108a8a7b6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h84a8225cc4fa24d8 + 6 (rt.rs:74)
7   libstd-9704616a50a0e937.dylib  0x0000000108ababf8 std::panicking::try::do_call::h15f3aa7ff9620fa7 + 24
8   libstd-9704616a50a0e937.dylib  0x0000000108ad63ff __rust_maybe_catch_panic + 31
9   libstd-9704616a50a0e937.dylib  0x0000000108abb64b std::rt::lang_start_internal::h948ce4de2cd5b179 + 379
10  a                              0x0000000108a8ccb9 main + 41
11  libdyld.dylib                  0x00007fff6fa37115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee7177c08  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000108b00522  rbp: 0x00007ffee7177d00  rsp: 0x00007ffee7177c30
   r8: 0x0000000108afed40   r9: 0x0000000108b32610  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000108afed40  r14: 0x0000000108a8f460  r15: 0x00007ffee7177d10
  rip: 0x0000000108abb38c  rfl: 0x0000000000010202  cr2: 0x0000000108d83e3d
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x108a86000 -        0x108a8eff7 +a (0) <89339646-EEFA-3774-BD84-0D2EFCC446BB> /Users/USER/*/a
       0x108a98000 -        0x108b2aff7 +libstd-9704616a50a0e937.dylib (0) <BD0470BE-04AB-360E-BDF3-D0FF14EC2E1A> /Users/USER/*/libstd-9704616a50a0e937.dylib
       0x115bb1000 -        0x115bfb98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6d2a1000 -     0x7fff6d2d4fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6d7b3000 -     0x7fff6d7b4ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6da69000 -     0x7fff6dabffff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6dac0000 -     0x7fff6dae4ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6ee36000 -     0x7fff6f2273b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6f2f4000 -     0x7fff6f310ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6f8ce000 -     0x7fff6f8d2ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6f8d3000 -     0x7fff6f8ddff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6f8de000 -     0x7fff6f8e5fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6f8e6000 -     0x7fff6f8eeffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6f8ef000 -     0x7fff6f974fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6f9fc000 -     0x7fff6fa35ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6fa36000 -     0x7fff6fa53ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6fa54000 -     0x7fff6fa54ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6fa62000 -     0x7fff6fa62ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6fa63000 -     0x7fff6fa67ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6fa68000 -     0x7fff6fa6aff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6fa6b000 -     0x7fff6fa6cff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6fa6d000 -     0x7fff6fa84fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6fa85000 -     0x7fff6fa85fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6fa86000 -     0x7fff6fb0fff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6fb10000 -     0x7fff6fb13ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6fb14000 -     0x7fff6fb17ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6fb18000 -     0x7fff6fb19fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6fb1a000 -     0x7fff6fb20ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6fb21000 -     0x7fff6fb6aff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6fb6b000 -     0x7fff6fb90ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6fb91000 -     0x7fff6fbdcfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6fbdd000 -     0x7fff6fbfcfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6fbfd000 -     0x7fff6fca1ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6fca2000 -     0x7fff6fcacffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6fcad000 -     0x7fff6fcb6ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6fcb7000 -     0x7fff6fcbeff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6fcbf000 -     0x7fff6fccafff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6fccb000 -     0x7fff6fcceff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6fccf000 -     0x7fff6fcd0ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6fcd1000 -     0x7fff6fcd8ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6fcd9000 -     0x7fff6fcecff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6fcee000 -     0x7fff6fcf3ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6fcf4000 -     0x7fff6fd20ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 1811
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=82.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=82.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.5M        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4528K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            285.4M      109 
TOTAL                            285.4M      109 
TOTAL, minus reserved VM space   285.3M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-21-060258_Traviss-Mac-1044.crash
Process:               a [41902]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41893]
Responsible:           a [41902]
User ID:               501
Date/Time:             2018-12-21 06:02:57.604 +0000
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
0   libstd-9704616a50a0e937.dylib  0x0000000105a3938c std::panicking::rust_panic_with_hook::h62ca8c89ccbdc3f7 + 668
1   a                              0x0000000105a047f5 std::panicking::begin_panic::h0183cf39c4f26474 + 37
2   a                              0x0000000105a022bc _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hbddd8cc943232966 + 28
3   a                              0x0000000105a019e9 core::ptr::real_drop_in_place::h0bccc5556cf0dbf4 + 9
4   a                              0x0000000105a02293 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x0000000105a0342e backtrace::main::hcde7a1a1c3c85e77 + 4238 (backtrace.rs:113)
6   a                              0x0000000105a017b6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h84a8225cc4fa24d8 + 6 (rt.rs:74)
7   libstd-9704616a50a0e937.dylib  0x0000000105a38bf8 std::panicking::try::do_call::h15f3aa7ff9620fa7 + 24
8   libstd-9704616a50a0e937.dylib  0x0000000105a543ff __rust_maybe_catch_panic + 31
9   libstd-9704616a50a0e937.dylib  0x0000000105a3964b std::rt::lang_start_internal::h948ce4de2cd5b179 + 379
10  a                              0x0000000105a03cb9 main + 41
11  libdyld.dylib                  0x00007fff6fa37115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeea200be8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000105a7e522  rbp: 0x00007ffeea200ce0  rsp: 0x00007ffeea200c10
   r8: 0x0000000105a7cd40   r9: 0x0000000105ab0610  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000105a7cd40  r14: 0x0000000105a06460  r15: 0x00007ffeea200cf0
  rip: 0x0000000105a3938c  rfl: 0x0000000000010206  cr2: 0x0000000105a72dc0
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1059fd000 -        0x105a05ff7 +a (0) <89339646-EEFA-3774-BD84-0D2EFCC446BB> /Users/USER/*/a
       0x105a16000 -        0x105aa8ff7 +libstd-9704616a50a0e937.dylib (0) <BD0470BE-04AB-360E-BDF3-D0FF14EC2E1A> /Users/USER/*/libstd-9704616a50a0e937.dylib
       0x11460b000 -        0x11465598f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6d2a1000 -     0x7fff6d2d4fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6d7b3000 -     0x7fff6d7b4ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6da69000 -     0x7fff6dabffff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6dac0000 -     0x7fff6dae4ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6ee36000 -     0x7fff6f2273b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6f2f4000 -     0x7fff6f310ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6f8ce000 -     0x7fff6f8d2ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6f8d3000 -     0x7fff6f8ddff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6f8de000 -     0x7fff6f8e5fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6f8e6000 -     0x7fff6f8eeffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6f8ef000 -     0x7fff6f974fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6f9fc000 -     0x7fff6fa35ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6fa36000 -     0x7fff6fa53ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6fa54000 -     0x7fff6fa54ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6fa62000 -     0x7fff6fa62ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6fa63000 -     0x7fff6fa67ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6fa68000 -     0x7fff6fa6aff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6fa6b000 -     0x7fff6fa6cff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6fa6d000 -     0x7fff6fa84fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6fa85000 -     0x7fff6fa85fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6fa86000 -     0x7fff6fb0fff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6fb10000 -     0x7fff6fb13ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6fb14000 -     0x7fff6fb17ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6fb18000 -     0x7fff6fb19fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6fb1a000 -     0x7fff6fb20ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6fb21000 -     0x7fff6fb6aff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6fb6b000 -     0x7fff6fb90ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6fb91000 -     0x7fff6fbdcfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6fbdd000 -     0x7fff6fbfcfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6fbfd000 -     0x7fff6fca1ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6fca2000 -     0x7fff6fcacffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6fcad000 -     0x7fff6fcb6ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6fcb7000 -     0x7fff6fcbeff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6fcbf000 -     0x7fff6fccafff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6fccb000 -     0x7fff6fcceff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6fccf000 -     0x7fff6fcd0ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6fcd1000 -     0x7fff6fcd8ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6fcd9000 -     0x7fff6fcecff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6fcee000 -     0x7fff6fcf3ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6fcf4000 -     0x7fff6fd20ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 1811
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=73.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9684K       10 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4528K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            276.4M      109 
TOTAL                            276.4M      109 
TOTAL, minus reserved VM space   276.3M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-21-060352_Traviss-Mac-1044.crash
Process:               a [43499]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [43497]
Responsible:           a [43499]
User ID:               501
Date/Time:             2018-12-21 06:03:51.493 +0000
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
0   libsystem_kernel.dylib         0x00007fff6fb86e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff6fcc5150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff6fae3312 abort + 127
3   libstd-9704616a50a0e937.dylib  0x00000001037e9669 std::sys::unix::abort_internal::haf7423cf66b1806e + 9
4   libstd-9704616a50a0e937.dylib  0x00000001037d8f50 rust_oom + 32
5   libstd-9704616a50a0e937.dylib  0x00000001037fb3d9 alloc::alloc::handle_alloc_error::h8a946e1e8920f188 + 9
6   a                              0x00000001037ac28d default_alloc_error_hook::main::hbf2d06db626d002e + 781
7   a                              0x00000001037ab426 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h06f7457321a1f120 + 6
8   libstd-9704616a50a0e937.dylib  0x00000001037d9bf8 std::panicking::try::do_call::h15f3aa7ff9620fa7 + 24
9   libstd-9704616a50a0e937.dylib  0x00000001037f53ff __rust_maybe_catch_panic + 31
10  libstd-9704616a50a0e937.dylib  0x00000001037da64b std::rt::lang_start_internal::h948ce4de2cd5b179 + 379
11  a                              0x00000001037ac3f9 main + 41
12  libdyld.dylib                  0x00007fff6fa37115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fffa875d340  rcx: 0x00007ffeec453e18  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffeec453e50  rsp: 0x00007ffeec453e18
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff6fb86e3e  rfl: 0x0000000000000206  cr2: 0x00007fffa873b148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x1037aa000 -        0x1037acff7 +a (0) <5ECD23E9-9F6D-3A64-8363-3DA3E179E655> /Users/USER/*/a
       0x1037b7000 -        0x103849ff7 +libstd-9704616a50a0e937.dylib (0) <BD0470BE-04AB-360E-BDF3-D0FF14EC2E1A> /Users/USER/*/libstd-9704616a50a0e937.dylib
       0x104d59000 -        0x104da398f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6d2a1000 -     0x7fff6d2d4fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6d7b3000 -     0x7fff6d7b4ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6da69000 -     0x7fff6dabffff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6dac0000 -     0x7fff6dae4ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6ee36000 -     0x7fff6f2273b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6f2f4000 -     0x7fff6f310ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6f8ce000 -     0x7fff6f8d2ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6f8d3000 -     0x7fff6f8ddff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6f8de000 -     0x7fff6f8e5fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6f8e6000 -     0x7fff6f8eeffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6f8ef000 -     0x7fff6f974fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6f9fc000 -     0x7fff6fa35ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6fa36000 -     0x7fff6fa53ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6fa54000 -     0x7fff6fa54ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6fa62000 -     0x7fff6fa62ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6fa63000 -     0x7fff6fa67ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6fa68000 -     0x7fff6fa6aff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6fa6b000 -     0x7fff6fa6cff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6fa6d000 -     0x7fff6fa84fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6fa85000 -     0x7fff6fa85fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6fa86000 -     0x7fff6fb0fff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6fb10000 -     0x7fff6fb13ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6fb14000 -     0x7fff6fb17ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6fb18000 -     0x7fff6fb19fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6fb1a000 -     0x7fff6fb20ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6fb21000 -     0x7fff6fb6aff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6fb6b000 -     0x7fff6fb90ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6fb91000 -     0x7fff6fbdcfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6fbdd000 -     0x7fff6fbfcfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6fbfd000 -     0x7fff6fca1ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6fca2000 -     0x7fff6fcacffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6fcad000 -     0x7fff6fcb6ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6fcb7000 -     0x7fff6fcbeff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6fcbf000 -     0x7fff6fccafff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6fccb000 -     0x7fff6fcceff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6fccf000 -     0x7fff6fcd0ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6fcd1000 -     0x7fff6fcd8ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6fcd9000 -     0x7fff6fcecff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6fcee000 -     0x7fff6fcf3ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6fcf4000 -     0x7fff6fd20ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 1811
    thread_create: 0
VM Region Summary:
VM Region Summary:
---
travis_time:end:0dd92142:start=1545373393504352000,finish=1545373402994165000,duration=9489813000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02f0cfe2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01d89d2c
travis_time:start:01d89d2c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08ad0ea0
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
