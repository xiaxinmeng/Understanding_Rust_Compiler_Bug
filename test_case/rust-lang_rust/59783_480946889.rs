plain
[00:03:52]       Memory: 8 GB
[00:03:52]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:52]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:52]       SMC Version (system): 2.8f0
[00:03:52]       Serial Number (system): VMYXR2S1lDYM
[00:03:52] 
[00:03:52] hw.ncpu: 4
[00:03:52] hw.byteorder: 1234
[00:03:52] hw.memsize: 8589934592
---
[02:31:37] stdout:
[02:31:37] ------------------------------------------
[02:31:37] 
[02:31:37] running 3 tests
[02:31:37] test src/test/rustdoc/test_option_check/test.rs - Foo (line 8) ... FAILED
[02:31:37] test src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6) ... ok
[02:31:37] test src/test/rustdoc/test_option_check/test.rs - Bar (line 15) ... ok
[02:31:37] failures:
[02:31:37] 
[02:31:37] 
[02:31:37] ---- src/test/rustdoc/test_option_check/test.rs - Foo (line 8) stdout ----
[02:31:37] error: linking with `cc` failed: signal: 4
[02:31:37]   |
[02:31:37]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestvbnvSr/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestvbnvSr/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestvbnvSr/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestvbnvSr/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestvbnvSr/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestvbnvSr/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestvbnvSr/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestvbnvSr/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/rustdoc/test_option_check/test/auxiliary" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-f23ea46703c918bb.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-9594c37878cc9962.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-15eb3ddee796149a.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-e069d4823e178d08.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-276abfca01d24267.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-376cf09404a6e600.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-c8ecb1e61d78c34d.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-233abb06723a85b7.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-98b24fb5d296bb0b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-43ba55b0f02ba456.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:31:37] 
[02:31:37] error: aborting due to previous error
[02:31:37] 
[02:31:37] thread 'src/test/rustdoc/test_option_check/test.rs - Foo (line 8)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[02:31:37] 
[02:31:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[02:31:37] 
[02:31:37] 
[02:31:37] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/rustdoc" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/rustdoc" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "rustdoc" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0-rust-1.35.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[02:31:37] 
[02:31:37] 
[02:31:37] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:31:37] Build completed unsuccessfully in 0:42:46
[02:31:37] Build completed unsuccessfully in 0:42:46
[02:31:37] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0dda31f8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr  8 18:19:11 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:3f0560d0
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  22 travis  staff    748 Apr  8 18:19 .
-rw-------@  1 travis  staff  44518 Apr  8 18:19 rustdoc_2019-04-08-181910_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62246 Apr  8 17:58 a_2019-04-08-175851-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 Apr  8 17:58 a_2019-04-08-175851_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60387 Apr  8 17:58 a_2019-04-08-175841-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37411 Apr  8 17:58 a_2019-04-08-175841_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Apr  8 17:58 a_2019-04-08-175832_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Apr  8 17:58 a_2019-04-08-175826_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Apr  8 17:58 a_2019-04-08-175815_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 Apr  8 17:58 a_2019-04-08-175814_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Apr  8 17:57 a_2019-04-08-175717_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63100 Apr  8 17:57 a_2019-04-08-175701_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 Apr  8 17:56 a_2019-04-08-175657-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65082 Apr  8 17:56 a_2019-04-08-175657-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64211 Apr  8 17:56 a_2019-04-08-175657_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11712 Apr  8 17:53 a_2019-04-08-175344_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Apr  8 17:52 a_2019-04-08-175228_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Apr  8 17:50 a_2019-04-08-175036_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10489 Apr  8 17:49 a_2019-04-08-174931_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Apr  8 17:49 a_2019-04-08-174929_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 Apr  8 17:49 a_2019-04-08-174919_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:008db381
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-08-174919_Traviss-Mac-1044.crash
Process:               a [44212]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44210]
Responsible:           a [44212]
User ID:               501
Date/Time:             2019-04-08 17:48:49.048 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 7400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x0000000101bd08ae abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x0000000101bcfca9 std::panicking::try::do_call::h5c81075da2dcccf9 (.llvm.12675958016211414747) + 9
2   libstd-f23ea46703c918bb.dylib  0x0000000101c0aa3f __rust_maybe_catch_panic + 31
3   a                              0x0000000101bd0b01 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x0000000101bcf0f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf20099c8e9367143 + 6
5   libstd-f23ea46703c918bb.dylib  0x0000000101bfadc8 std::panicking::try::do_call::h5758c52bb7fcd96d + 24
6   libstd-f23ea46703c918bb.dylib  0x0000000101c0aa3f __rust_maybe_catch_panic + 31
7   libstd-f23ea46703c918bb.dylib  0x0000000101bfb8ae std::rt::lang_start_internal::ha0d5ff149ddc1e03 + 542
8   a                              0x0000000101bd0e09 main + 41
9   libdyld.dylib                  0x00007fff60802115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007f991ec02be0  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffeee02ec08  rsi: 0x000000007fffffff  rbp: 0x00007ffeee02f660  rsp: 0x00007ffeee02f660
   r8: 0x0000000091ec02c3   r9: 0x0000000000000004  r10: 0x0000000102f028c0  r11: 0x00007fff60ab996c
  r12: 0x0000000101f26000  r13: 0x0000000000000000  r14: 0x00007ffeee02f780  r15: 0x00007ffeee02f6c8
  rip: 0x0000000101bd08ae  rfl: 0x0000000000010206  cr2: 0x0000000101c41a28
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x101bce000 -        0x101bd1fff +a (0) <B3EF0B91-4C5A-3DA3-899B-20741C0D8C51> /Users/USER/*/a
       0x101bd6000 -        0x101c6dff7 +libstd-f23ea46703c918bb.dylib (0) <816188D2-23CA-3FC3-83F1-B20ADF674FCA> /Users/USER/*/libstd-f23ea46703c918bb.dylib
       0x102eb0000 -        0x102efa98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5e06c000 -     0x7fff5e09ffff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5e57e000 -     0x7fff5e57fff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5e834000 -     0x7fff5e88afff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5e88b000 -     0x7fff5e8afff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5fc01000 -     0x7fff5fff23b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff600bf000 -     0x7fff600dbffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff60699000 -     0x7fff6069dff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6069e000 -     0x7fff606a8ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff606a9000 -     0x7fff606b0fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff606b1000 -     0x7fff606b9ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff606ba000 -     0x7fff6073ffff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff607c7000 -     0x7fff60800ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff60801000 -     0x7fff6081eff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6081f000 -     0x7fff6081fffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6082d000 -     0x7fff6082dff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6082e000 -     0x7fff60832ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff60833000 -     0x7fff60835ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff60836000 -     0x7fff60837ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff60838000 -     0x7fff6084ffff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff60850000 -     0x7fff60850fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff60851000 -     0x7fff608daff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff608db000 -     0x7fff608deffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff608df000 -     0x7fff608e2ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff608e3000 -     0x7fff608e4fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff608e5000 -     0x7fff608ebff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff608ec000 -     0x7fff60935ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff60936000 -     0x7fff6095bff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6095c000 -     0x7fff609a7fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff609a8000 -     0x7fff609c7fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff609c8000 -     0x7fff60a6cff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff60a6d000 -     0x7fff60a77ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff60a78000 -     0x7fff60a81ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff60a82000 -     0x7fff60a89ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff60a8a000 -     0x7fff60a95fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff60a96000 -     0x7fff60a99ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff60a9a000 -     0x7fff60a9bff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff60a9c000 -     0x7fff60aa3ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff60aa4000 -     0x7fff60ab7ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff60ab9000 -     0x7fff60abeff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff60abf000 -     0x7fff60aebff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 3346
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=73.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9260K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4636K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            276.1M      109 
TOTAL                            276.1M      109 
TOTAL, minus reserved VM space   276.0M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-08-174929_Traviss-Mac-1044.crash
Process:               a [45030]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45025]
Responsible:           a [45030]
User ID:               501
Date/Time:             2019-04-08 17:49:20.939 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 7400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-f23ea46703c918bb.dylib  0x000000010055d34e std::panicking::rust_panic_with_hook::h59ed2c15bfb73246 + 142
1   a                              0x000000010052b8e5 std::panicking::begin_panic::h55d7cb6258d03287 + 37
2   a                              0x000000010052940c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x0000000100528969 core::ptr::real_drop_in_place::h08174337b74568a1 + 9
4   a                              0x00000001005293e3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x000000010052a559 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x0000000100528936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb4bf0af51319d429 + 6 (rt.rs:64)
7   libstd-f23ea46703c918bb.dylib  0x000000010055cdc8 std::panicking::try::do_call::h5758c52bb7fcd96d + 24
8   libstd-f23ea46703c918bb.dylib  0x000000010056ca3f __rust_maybe_catch_panic + 31
9   libstd-f23ea46703c918bb.dylib  0x000000010055d8ae std::rt::lang_start_internal::ha0d5ff149ddc1e03 + 542
10  a                              0x000000010052ad99 main + 41
11  libdyld.dylib                  0x00007fff60802115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeef6d83a8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001005a4ed2  rbp: 0x00007ffeef6d84a0  rsp: 0x00007ffeef6d83d0
   r8: 0xffffffff00000100   r9: 0x00000001005d79f0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001005a3c38  r14: 0x000000010052d460  r15: 0x00007ffeef6d84b0
  rip: 0x000000010055d34e  rfl: 0x0000000000010206  cr2: 0x000000010b539000
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x100525000 -        0x10052cfff +a (0) <2193879B-D85C-3F79-B7C8-D52628682CFD> /Users/USER/*/a
       0x100538000 -        0x1005cfff7 +libstd-f23ea46703c918bb.dylib (0) <816188D2-23CA-3FC3-83F1-B20ADF674FCA> /Users/USER/*/libstd-f23ea46703c918bb.dylib
       0x10e813000 -        0x10e85d98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5e06c000 -     0x7fff5e09ffff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5e57e000 -     0x7fff5e57fff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5e834000 -     0x7fff5e88afff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5e88b000 -     0x7fff5e8afff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5fc01000 -     0x7fff5fff23b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff600bf000 -     0x7fff600dbffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff60699000 -     0x7fff6069dff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6069e000 -     0x7fff606a8ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff606a9000 -     0x7fff606b0fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff606b1000 -     0x7fff606b9ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff606ba000 -     0x7fff6073ffff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff607c7000 -     0x7fff60800ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff60801000 -     0x7fff6081eff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6081f000 -     0x7fff6081fffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6082d000 -     0x7fff6082dff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6082e000 -     0x7fff60832ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff60833000 -     0x7fff60835ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff60836000 -     0x7fff60837ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff60838000 -     0x7fff6084ffff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff60850000 -     0x7fff60850fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff60851000 -     0x7fff608daff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff608db000 -     0x7fff608deffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff608df000 -     0x7fff608e2ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff608e3000 -     0x7fff608e4fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff608e5000 -     0x7fff608ebff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff608ec000 -     0x7fff60935ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff60936000 -     0x7fff6095bff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6095c000 -     0x7fff609a7fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff609a8000 -     0x7fff609c7fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff609c8000 -     0x7fff60a6cff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff60a6d000 -     0x7fff60a77ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff60a78000 -     0x7fff60a81ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff60a82000 -     0x7fff60a89ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff60a8a000 -     0x7fff60a95fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff60a96000 -     0x7fff60a99ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff60a9a000 -     0x7fff60a9bff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff60a9c000 -     0x7fff60aa3ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff60aa4000 -     0x7fff60ab7ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff60ab9000 -     0x7fff60abeff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff60abf000 -     0x7fff60aebff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 3346
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=74.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=74.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.5M        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4636K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            277.5M      110 
TOTAL                            277.5M      110 
TOTAL, minus reserved VM space   277.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-08-174931_Traviss-Mac-1044.crash
Process:               a [45029]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        a [45025]
Responsible:           a [45029]
User ID:               501
Date/Time:             2019-04-08 17:49:20.878 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 7400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-f23ea46703c918bb.dylib  0x0000000104d7034e std::panicking::rust_panic_with_hook::h59ed2c15bfb73246 + 142
1   a                              0x0000000104d3a8e5 std::panicking::begin_panic::h55d7cb6258d03287 + 37
2   a                              0x0000000104d3840c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x0000000104d37969 core::ptr::real_drop_in_place::h08174337b74568a1 + 9
4   a                              0x0000000104d383e3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x0000000104d39559 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x0000000104d37936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb4bf0af51319d429 + 6 (rt.rs:64)
7   libstd-f23ea46703c918bb.dylib  0x0000000104d6fdc8 std::panicking::try::do_call::h5758c52bb7fcd96d + 24
8   libstd-f23ea46703c918bb.dylib  0x0000000104d7fa3f __rust_maybe_catch_panic + 31
9   libstd-f23ea46703c918bb.dylib  0x0000000104d708ae std::rt::lang_start_internal::ha0d5ff149ddc1e03 + 542
10  a                              0x0000000104d39d99 main + 41
11  libdyld.dylib                  0x00007fff60802115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeeaec93c8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000104db7ed2  rbp: 0x00007ffeeaec94c0  rsp: 0x00007ffeeaec93f0
   r8: 0xffffffff00000100   r9: 0x0000000104dea9f0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000104db6c38  r14: 0x0000000104d3c460  r15: 0x00007ffeeaec94d0
  rip: 0x0000000104d7034e  rfl: 0x0000000000010202  cr2: 0x0000000105058434
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x104d34000 -        0x104d3bfff +a (0) <2193879B-D85C-3F79-B7C8-D52628682CFD> /Users/USER/*/a
       0x104d4b000 -        0x104de2ff7 +libstd-f23ea46703c918bb.dylib (0) <816188D2-23CA-3FC3-83F1-B20ADF674FCA> /Users/USER/*/libstd-f23ea46703c918bb.dylib
       0x10e279000 -        0x10e2c398f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5e06c000 -     0x7fff5e09ffff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5e57e000 -     0x7fff5e57fff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5e834000 -     0x7fff5e88afff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5e88b000 -     0x7fff5e8afff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5fc01000 -     0x7fff5fff23b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff600bf000 -     0x7fff600dbffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff60699000 -     0x7fff6069dff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6069e000 -     0x7fff606a8ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff606a9000 -     0x7fff606b0fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff606b1000 -     0x7fff606b9ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff606ba000 -     0x7fff6073ffff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff607c7000 -     0x7fff60800ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff60801000 -     0x7fff6081eff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6081f000 -     0x7fff6081fffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6082d000 -     0x7fff6082dff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6082e000 -     0x7fff60832ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff60833000 -     0x7fff60835ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff60836000 -     0x7fff60837ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff60838000 -     0x7fff6084ffff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff60850000 -     0x7fff60850fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff60851000 -     0x7fff608daff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff608db000 -     0x7fff608deffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff608df000 -     0x7fff608e2ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff608e3000 -     0x7fff608e4fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff608e5000 -     0x7fff608ebff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff608ec000 -     0x7fff60935ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff60936000 -     0x7fff6095bff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6095c000 -     0x7fff609a7fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff609a8000 -     0x7fff609c7fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff609c8000 -     0x7fff60a6cff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff60a6d000 -     0x7fff60a77ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff60a78000 -     0x7fff60a81ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff60a82000 -     0x7fff60a89ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff60a8a000 -     0x7fff60a95fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff60a96000 -     0x7fff60a99ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff60a9a000 -     0x7fff60a9bff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff60a9c000 -     0x7fff60aa3ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff60aa4000 -     0x7fff60ab7ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff60ab9000 -     0x7fff60abeff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff60abf000 -     0x7fff60aebff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 3346
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=91.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=91.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            27.5M       10 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4636K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            294.5M      110 
TOTAL                            294.5M      110 
TOTAL, minus reserved VM space   294.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-08-175036_Traviss-Mac-1044.crash
Process:               a [46710]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [46708]
Responsible:           a [46710]
User ID:               501
Date/Time:             2019-04-08 17:50:35.452 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 7500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff60951e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff60a90150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff608ae312 abort + 127
3   libstd-f23ea46703c918bb.dylib  0x000000010f1e4f29 std::sys::unix::abort_internal::hfa6963a1e064fb02 + 9
4   libstd-f23ea46703c918bb.dylib  0x000000010f1d53e0 rust_oom + 32
5   libstd-f23ea46703c918bb.dylib  0x000000010f1f61e9 alloc::alloc::handle_alloc_error::h85e2cdc83e93f20e + 9
6   a                              0x000000010f1a840f default_alloc_error_hook::main::hbf2d06db626d002e + 767
7   a                              0x000000010f1a7686 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hdfc862f94f5529c1 + 6
8   libstd-f23ea46703c918bb.dylib  0x000000010f1d5dc8 std::panicking::try::do_call::h5758c52bb7fcd96d + 24
9   libstd-f23ea46703c918bb.dylib  0x000000010f1e5a3f __rust_maybe_catch_panic + 31
10  libstd-f23ea46703c918bb.dylib  0x000000010f1d68ae std::rt::lang_start_internal::ha0d5ff149ddc1e03 + 542
11  a                              0x000000010f1a8569 main + 41
12  libdyld.dylib                  0x00007fff60802115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff99528340  rcx: 0x00007ffee0a575d8  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee0a57610  rsp: 0x00007ffee0a575d8
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff60951e3e  rfl: 0x0000000000000206  cr2: 0x00007fff99506148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10f1a6000 -        0x10f1a8ff7 +a (0) <45E49D60-88E2-3840-ABD7-591175213979> /Users/USER/*/a
       0x10f1b1000 -        0x10f248ff7 +libstd-f23ea46703c918bb.dylib (0) <816188D2-23CA-3FC3-83F1-B20ADF674FCA> /Users/USER/*/libstd-f23ea46703c918bb.dylib
       0x112407000 -        0x11245198f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5e06c000 -     0x7fff5e09ffff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5e57e000 -     0x7fff5e57fff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5e834000 -     0x7fff5e88afff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5e88b000 -     0x7fff5e8afff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5fc01000 -     0x7fff5fff23b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff600bf000 -     0x7fff600dbffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff60699000 -     0x7fff6069dff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6069e000 -     0x7fff606a8ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff606a9000 -     0x7fff606b0fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff606b1000 -     0x7fff606b9ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff606ba000 -     0x7fff6073ffff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff607c7000 -     0x7fff60800ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff60801000 -     0x7fff6081eff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6081f000 -     0x7fff6081fffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6082d000 -     0x7fff6082dff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6082e000 -     0x7fff60832ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff60833000 -     0x7fff60835ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff60836000 -     0x7fff60837ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff60838000 -     0x7fff6084ffff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff60850000 -     0x7fff60850fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff60851000 -     0x7fff608daff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff608db000 -     0x7fff608deffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff608df000 -     0x7fff608e2ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff608e3000 -     0x7fff608e4fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff608e5000 -     0x7fff608ebff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff608ec000 -     0x7fff60935ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff60936000 -     0x7fff6095bff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6095c000 -     0x7fff609a7fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff609a8000 -     0x7fff609c7fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff609c8000 -     0x7fff60a6cff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff60a6d000 -     0x7fff60a77ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff60a78000 -     0x7fff60a81ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff60a82000 -     0x7fff60a89ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff60a8a000 -     0x7fff60a95fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff60a96000 -     0x7fff60a99ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff60a9a000 -     0x7fff60a9bff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff60a9c000 -     0x7fff60aa3ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff60aa4000 -     0x7fff60ab7ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff60ab9000 -     0x7fff60abeff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff60abf000 -     0x7fff60aebff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 3346
    thread_create: 0
VM Region Summary:
