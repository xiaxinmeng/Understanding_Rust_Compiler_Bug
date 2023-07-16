plain
[00:03:23]       Memory: 8 GB
[00:03:23]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:23]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:23]       SMC Version (system): 2.8f0
[00:03:23]       Serial Number (system): VM+deZWVsTtz
[00:03:23] 
[00:03:23] hw.ncpu: 4
[00:03:23] hw.byteorder: 1234
[00:03:23] hw.memsize: 8589934592
---
[01:15:54] normalized stderr:
[01:15:54] warning: unused variable: `file`
[01:15:54]   --> $DIR/issue-45731.rs:25:13
[01:15:54]    |
[01:15:54] LL |         let file = fs::OpenOptions::new().read(false).write(true).truncate(true).create(false)
[01:15:54]    |             ^^^^ help: consider using `_file` instead
[01:15:54]    = note: #[warn(unused_variables)] on by default
[01:15:54] 
[01:15:54] 
[01:15:54] 
[01:15:54] 
[01:15:54] 
[01:15:54] The actual stderr differed from the expected stderr.
[01:15:54] Actual stderr saved to /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass/issues/issue-45731/issue-45731.stderr
[01:15:54] To update references, rerun the tests and pass the `--bless` flag
[01:15:54] To only update this specific test, also pass `--test-args issues/issue-45731.rs`
[01:15:54] error: 1 errors occurred comparing output.
[01:15:54] status: exit code: 0
[01:15:54] status: exit code: 0
[01:15:54] command: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/travis/build/rust-lang/rust/src/test/run-pass/issues/issue-45731.rs" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass/issues/issue-45731/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--test" "-g" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass/issues/issue-45731/auxiliary"
[01:15:54] ------------------------------------------
[01:15:54] 
[01:15:54] ------------------------------------------
[01:15:54] stderr:
[01:15:54] stderr:
[01:15:54] ------------------------------------------
[01:15:54] {"message":"unused variable: `file`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/Users/travis/build/rust-lang/rust/src/test/run-pass/issues/issue-45731.rs","byte_start":956,"byte_end":960,"line_start":25,"line_end":25,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"        let file = fs::OpenOptions::new().read(false).write(true).truncate(true).create(false)","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_file` instead","code":null,"level":"help","spans":[{"file_name":"/Users/travis/build/rust-lang/rust/src/test/run-pass/issues/issue-45731.rs","byte_start":956,"byte_end":960,"line_start":25,"line_end":25,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"        let file = fs::OpenOptions::new().read(false).write(true).truncate(true).create(false)","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":"_file","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `file`\n  --> /Users/travis/build/rust-lang/rust/src/test/run-pass/issues/issue-45731.rs:25:13\n   |\nLL |         let file = fs::OpenOptions::new().read(false).write(true).truncate(true).create(false)\n   |             ^^^^ help: consider using `_file` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:15:54] ------------------------------------------
[01:15:54] 
[01:15:54] thread '[run-pass] run-pass/issues/issue-45731.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[01:15:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:15:54] 
[01:15:54] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:15:54] 
[01:15:54] 
[01:15:54] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/run-pass" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-pass" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "run-pass" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:54] 
[01:15:54] 
[01:15:54] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:15:54] Build completed unsuccessfully in 0:20:47
[01:15:54] Build completed unsuccessfully in 0:20:47
[01:15:54] make: *** [check] Error 1
travis_time:end:01414fe5:start=1538062174323953000,finish=1538066728759469000,duration=4554435516000

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1423dbe2
---
travis_fold:start:after_failure.2
travis_time:start:0a298f49
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1184
drwx------  21 travis  staff    714 Sep 27 16:43 .
-rw-------@  1 travis  staff  62273 Sep 27 16:43 a_2018-09-27-164331-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37510 Sep 27 16:43 a_2018-09-27-164331_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37410 Sep 27 16:43 a_2018-09-27-164325-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60415 Sep 27 16:43 a_2018-09-27-164325_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10171 Sep 27 16:43 a_2018-09-27-164318_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9902 Sep 27 16:43 a_2018-09-27-164311_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9907 Sep 27 16:43 a_2018-09-27-164302-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9871 Sep 27 16:43 a_2018-09-27-164302_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9965 Sep 27 16:42 a_2018-09-27-164231_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63100 Sep 27 16:42 a_2018-09-27-164220_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64304 Sep 27 16:42 a_2018-09-27-164217-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65148 Sep 27 16:42 a_2018-09-27-164217-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63943 Sep 27 16:42 a_2018-09-27-164217_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11787 Sep 27 16:39 a_2018-09-27-163941_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9926 Sep 27 16:38 a_2018-09-27-163839_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10333 Sep 27 16:37 a_2018-09-27-163743-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10513 Sep 27 16:37 a_2018-09-27-163743-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10243 Sep 27 16:37 a_2018-09-27-163743-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10515 Sep 27 16:37 a_2018-09-27-163743_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:120ab397
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-09-27-163743-1_Traviss-Mac-1044.crash
Process:               a [41437]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41435]
Responsible:           a [41437]
User ID:               501
Date/Time:             2018-09-27 16:37:30.713 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4200 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff52661e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff527a0150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff525be312 abort + 127
3   libstd-7a48a99fa0910c06.dylib  0x000000010cdfdd59 std::sys::unix::abort_internal::hed1a563349acc5db + 9
4   libstd-7a48a99fa0910c06.dylib  0x000000010cdf31a0 rust_oom + 32
5   libstd-7a48a99fa0910c06.dylib  0x000000010ce51059 alloc::alloc::handle_alloc_error::hff2b40b7be48e824 + 9
6   a                              0x000000010cdcfe2d default_alloc_error_hook::main::hbf3cf79eecbb97ff + 797
7   a                              0x000000010cdcf616 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h74713e24acc67ad4 + 6
8   libstd-7a48a99fa0910c06.dylib  0x000000010ce0e0a8 std::panicking::try::do_call::he1ad92af1b4befcd (.llvm.17655180579473033632) + 24
9   libstd-7a48a99fa0910c06.dylib  0x000000010ce1bc3f __rust_maybe_catch_panic + 31
10  libstd-7a48a99fa0910c06.dylib  0x000000010cddceed std::rt::lang_start_internal::h714ca9bb91facf70 + 237
11  a                              0x000000010cdcff9c main + 44
12  libdyld.dylib                  0x00007fff52512115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff8b238340  rcx: 0x00007ffee2e30008  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee2e30040  rsp: 0x00007ffee2e30008
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff52661e3e  rfl: 0x0000000000000206  cr2: 0x00007fff8b216148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10cdce000 -        0x10cdd0ff7 +a (0) <2AA7CCC4-E733-335C-871A-38585BDC75A8> /Users/USER/*/a
       0x10cdd6000 -        0x10ceb4fe7 +libstd-7a48a99fa0910c06.dylib (0) <CF314E58-FAC8-3186-8042-DB7A281218E1> /Users/USER/*/libstd-7a48a99fa0910c06.dylib
       0x11bd5b000 -        0x11bda598f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff4fd7c000 -     0x7fff4fdaffff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5028e000 -     0x7fff5028fff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff50544000 -     0x7fff5059afff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5059b000 -     0x7fff505bfff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff51911000 -     0x7fff51d023b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff51dcf000 -     0x7fff51debffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff523a9000 -     0x7fff523adff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff523ae000 -     0x7fff523b8ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff523b9000 -     0x7fff523c0fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff523c1000 -     0x7fff523c9ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff523ca000 -     0x7fff5244ffff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff524d7000 -     0x7fff52510ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff52511000 -     0x7fff5252eff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5252f000 -     0x7fff5252fffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5253d000 -     0x7fff5253dff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5253e000 -     0x7fff52542ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff52543000 -     0x7fff52545ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff52546000 -     0x7fff52547ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff52548000 -     0x7fff5255ffff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff52560000 -     0x7fff52560fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff52561000 -     0x7fff525eaff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff525eb000 -     0x7fff525eeffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff525ef000 -     0x7fff525f2ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff525f3000 -     0x7fff525f4fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff525f5000 -     0x7fff525fbff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff525fc000 -     0x7fff52645ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff52646000 -     0x7fff5266bff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5266c000 -     0x7fff526b7fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff526b8000 -     0x7fff526d7fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff526d8000 -     0x7fff5277cff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5277d000 -     0x7fff52787ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff52788000 -     0x7fff52791ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff52792000 -     0x7fff52799ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5279a000 -     0x7fff527a5fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff527a6000 -     0x7fff527a9ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff527aa000 -     0x7fff527abff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff527ac000 -     0x7fff527b3ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff527b4000 -     0x7fff527c7ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff527c9000 -     0x7fff527ceff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff527cf000 -     0x7fff527fbff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2300
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.7M resident=0K(0%) swapped_out_or_unallocated=198.7M(100%)
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
__DATA                            4340K       45 
__LINKEDIT                       189.1M        5 
__TEXT                            9904K       44 
===========                     =======  ======= 
TOTAL                            280.2M      114 
TOTAL                            280.2M      114 
TOTAL, minus reserved VM space   280.1M      114 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-09-27-163743-2_Traviss-Mac-1044.crash
Process:               a [39846]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        a [39842]
Responsible:           a [39846]
User ID:               501
Date/Time:             2018-09-27 16:36:38.778 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4100 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-7a48a99fa0910c06.dylib  0x000000010a0be5eb std::panicking::rust_panic_with_hook::h82a10f8b059597fd + 139
1   a                              0x000000010a0754e8 std::panicking::begin_panic::h5a4cd09a609b1909 + 40
2   a                              0x000000010a07633c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::h9a15dfda4629574d + 28
3   a                              0x000000010a075f19 core::ptr::drop_in_place::hc2bd66f444e7f166 + 9
4   a                              0x000000010a076313 backtrace::double::h3a79da7ae181846f + 35
5   a                              0x000000010a0774ce backtrace::main::h9751c5dfa7559d82 + 4254 (backtrace.rs:113)
6   a                              0x000000010a075806 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc857d66566dde18f + 6 (rt.rs:74)
7   libstd-7a48a99fa0910c06.dylib  0x000000010a0be0a8 std::panicking::try::do_call::he1ad92af1b4befcd (.llvm.17655180579473033632) + 24
8   libstd-7a48a99fa0910c06.dylib  0x000000010a0cbc3f __rust_maybe_catch_panic + 31
9   libstd-7a48a99fa0910c06.dylib  0x000000010a08ceed std::rt::lang_start_internal::h714ca9bb91facf70 + 237
10  a                              0x000000010a077d5c main + 44
11  libdyld.dylib                  0x00007fff52512115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010a129715  rbp: 0x00007ffee5b8cef0  rsp: 0x00007ffee5b8ce70
   r8: 0x000000010a129518   r9: 0x000000010a16ef10  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010a129518  r14: 0x000000010a07a1d0  r15: 0x00007ffee5b8cf00
  rip: 0x000000010a0be5eb  rfl: 0x0000000000010202  cr2: 0x000000010a39ea3f
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10a071000 -        0x10a079fff +a (0) <D1AC8472-62DA-369D-A0E8-30C15E56AC60> /Users/USER/*/a
       0x10a086000 -        0x10a164fe7 +libstd-7a48a99fa0910c06.dylib (0) <CF314E58-FAC8-3186-8042-DB7A281218E1> /Users/USER/*/libstd-7a48a99fa0910c06.dylib
       0x111c5f000 -        0x111ca998f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff4fd7c000 -     0x7fff4fdaffff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5028e000 -     0x7fff5028fff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff50544000 -     0x7fff5059afff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5059b000 -     0x7fff505bfff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff51911000 -     0x7fff51d023b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff51dcf000 -     0x7fff51debffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff523a9000 -     0x7fff523adff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff523ae000 -     0x7fff523b8ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff523b9000 -     0x7fff523c0fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff523c1000 -     0x7fff523c9ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff523ca000 -     0x7fff5244ffff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff524d7000 -     0x7fff52510ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff52511000 -     0x7fff5252eff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5252f000 -     0x7fff5252fffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5253d000 -     0x7fff5253dff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5253e000 -     0x7fff52542ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff52543000 -     0x7fff52545ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff52546000 -     0x7fff52547ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff52548000 -     0x7fff5255ffff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff52560000 -     0x7fff52560fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff52561000 -     0x7fff525eaff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff525eb000 -     0x7fff525eeffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff525ef000 -     0x7fff525f2ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff525f3000 -     0x7fff525f4fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff525f5000 -     0x7fff525fbff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff525fc000 -     0x7fff52645ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff52646000 -     0x7fff5266bff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5266c000 -     0x7fff526b7fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff526b8000 -     0x7fff526d7fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff526d8000 -     0x7fff5277cff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5277d000 -     0x7fff52787ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff52788000 -     0x7fff52791ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff52792000 -     0x7fff52799ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5279a000 -     0x7fff527a5fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff527a6000 -     0x7fff527a9ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff527aa000 -     0x7fff527abff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff527ac000 -     0x7fff527b3ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff527b4000 -     0x7fff527c7ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff527c9000 -     0x7fff527ceff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff527cf000 -     0x7fff527fbff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2300
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.8M resident=0K(0%) swapped_out_or_unallocated=198.8M(100%)
Writable regions: Total=76.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9268K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       3988K        5 
VM_ALLOCATE                       3988K        5 
VM_ALLOCATE (reserved)             240K        3         reserved VM address space (unallocated)
__DATA                            4340K       45 
__LINKEDIT                       189.1M        5 
__TEXT                            9928K       44 
===========                     =======  ======= 
TOTAL                            280.2M      115 
TOTAL                            280.2M      115 
TOTAL, minus reserved VM space   280.0M      115 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-09-27-163743-3_Traviss-Mac-1044.crash
Process:               a [39063]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [39062]
Responsible:           a [39063]
User ID:               501
Date/Time:             2018-09-27 16:36:11.423 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4100 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000000010d97185e abort_on_c_abi::panic_in_ffi::h305168285df224eb + 30
1   a                              0x000000010d970c59 std::panicking::try::do_call::hf452941a1e868dde (.llvm.11841950188121132191) + 9
2   libstd-7a48a99fa0910c06.dylib  0x000000010d9bdc3f __rust_maybe_catch_panic + 31
3   a                              0x000000010d971ab1 abort_on_c_abi::main::h82c0574e6cdd7b93 + 593
4   a                              0x000000010d96ffd6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hec6f84baafc8adc0 + 6
5   libstd-7a48a99fa0910c06.dylib  0x000000010d9b00a8 std::panicking::try::do_call::he1ad92af1b4befcd (.llvm.17655180579473033632) + 24
6   libstd-7a48a99fa0910c06.dylib  0x000000010d9bdc3f __rust_maybe_catch_panic + 31
7   libstd-7a48a99fa0910c06.dylib  0x000000010d97eeed std::rt::lang_start_internal::h714ca9bb91facf70 + 237
8   a                              0x000000010d971dbc main + 44
9   libdyld.dylib                  0x00007fff52512115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x000000010e01c050  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee228e698  rsi: 0xfffffffffffffce8  rbp: 0x00007ffee228f0a0  rsp: 0x00007ffee228f0a0
   r8: 0xffffffff00000000   r9: 0x000000010da60f10  r10: 0x0000000112c508d0  r11: 0x00007fff527c996c
  r12: 0x0000000000000000  r13: 0x0000000000000000  r14: 0x00007ffee228f1c0  r15: 0x00007ffee228f108
  rip: 0x000000010d97185e  rfl: 0x0000000000010206  cr2: 0x000000010da3529c
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10d96f000 -        0x10d972ff7 +a (0) <AB1FCA1F-8016-35A9-9A43-681E0D3A9D65> /Users/USER/*/a
       0x10d978000 -        0x10da56fe7 +libstd-7a48a99fa0910c06.dylib (0) <CF314E58-FAC8-3186-8042-DB7A281218E1> /Users/USER/*/libstd-7a48a99fa0910c06.dylib
       0x112bfe000 -        0x112c4898f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff4fd7c000 -     0x7fff4fdaffff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5028e000 -     0x7fff5028fff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff50544000 -     0x7fff5059afff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5059b000 -     0x7fff505bfff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff51911000 -     0x7fff51d023b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff51dcf000 -     0x7fff51debffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff523a9000 -     0x7fff523adff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff523ae000 -     0x7fff523b8ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff523b9000 -     0x7fff523c0fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff523c1000 -     0x7fff523c9ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff523ca000 -     0x7fff5244ffff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff524d7000 -     0x7fff52510ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff52511000 -     0x7fff5252eff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5252f000 -     0x7fff5252fffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5253d000 -     0x7fff5253dff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5253e000 -     0x7fff52542ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff52543000 -     0x7fff52545ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff52546000 -     0x7fff52547ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff52548000 -     0x7fff5255ffff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff52560000 -     0x7fff52560fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff52561000 -     0x7fff525eaff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff525eb000 -     0x7fff525eeffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff525ef000 -     0x7fff525f2ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff525f3000 -     0x7fff525f4fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff525f5000 -     0x7fff525fbff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff525fc000 -     0x7fff52645ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff52646000 -     0x7fff5266bff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5266c000 -     0x7fff526b7fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff526b8000 -     0x7fff526d7fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff526d8000 -     0x7fff5277cff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5277d000 -     0x7fff52787ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff52788000 -     0x7fff52791ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff52792000 -     0x7fff52799ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5279a000 -     0x7fff527a5fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff527a6000 -     0x7fff527a9ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff527aa000 -     0x7fff527abff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff527ac000 -     0x7fff527b3ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff527b4000 -     0x7fff527c7ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff527c9000 -     0x7fff527ceff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff527cf000 -     0x7fff527fbff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2300
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.7M resident=0K(0%) swapped_out_or_unallocated=198.7M(100%)
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
__DATA                            4340K       45 
__LINKEDIT                       189.1M        5 
__TEXT                            9908K       44 
===========                     =======  ======= 
TOTAL                            280.2M      114 
TOTAL                            280.2M      114 
TOTAL, minus reserved VM space   280.1M      114 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-09-27-163743_Traviss-Mac-1044.crash
Process:               a [39848]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [39842]
Responsible:           a [39848]
User ID:               501
Date/Time:             2018-09-27 16:36:38.836 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4100 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-7a48a99fa0910c06.dylib  0x0000000106d905eb std::panicking::rust_panic_with_hook::h82a10f8b059597fd + 139
1   a                              0x0000000106d484e8 std::panicking::begin_panic::h5a4cd09a609b1909 + 40
2   a                              0x0000000106d4933c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::h9a15dfda4629574d + 28
3   a                              0x0000000106d48f19 core::ptr::drop_in_place::hc2bd66f444e7f166 + 9
4   a                              0x0000000106d49313 backtrace::double::h3a79da7ae181846f + 35
5   a                              0x0000000106d4a4ce backtrace::main::h9751c5dfa7559d82 + 4254 (backtrace.rs:113)
6   a                              0x0000000106d48806 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc857d66566dde18f + 6 (rt.rs:74)
7   libstd-7a48a99fa0910c06.dylib  0x0000000106d900a8 std::panicking::try::do_call::he1ad92af1b4befcd (.llvm.17655180579473033632) + 24
8   libstd-7a48a99fa0910c06.dylib  0x0000000106d9dc3f __rust_maybe_catch_panic + 31
9   libstd-7a48a99fa0910c06.dylib  0x0000000106d5eeed std::rt::lang_start_internal::h714ca9bb91facf70 + 237
10  a                              0x0000000106d4ad5c main + 44
11  libdyld.dylib                  0x00007fff52512115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000106dfb715  rbp: 0x00007ffee8eb9ee0  rsp: 0x00007ffee8eb9e60
   r8: 0x0000000106dfb518   r9: 0x0000000106e40f10  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000106dfb518  r14: 0x0000000106d4d1d0  r15: 0x00007ffee8eb9ef0
  rip: 0x0000000106d905eb  rfl: 0x0000000000010206  cr2: 0x0000000106dd5c90
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x106d44000 -        0x106d4cfff +a (0) <D1AC8472-62DA-369D-A0E8-30C15E56AC60> /Users/USER/*/a
       0x106d58000 -        0x106e36fe7 +libstd-7a48a99fa0910c06.dylib (0) <CF314E58-FAC8-3186-8042-DB7A281218E1> /Users/USER/*/libstd-7a48a99fa0910c06.dylib
       0x10a320000 -        0x10a36a98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff4fd7c000 -     0x7fff4fdaffff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5028e000 -     0x7fff5028fff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff50544000 -     0x7fff5059afff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5059b000 -     0x7fff505bfff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff51911000 -     0x7fff51d023b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff51dcf000 -     0x7fff51debffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff523a9000 -     0x7fff523adff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff523ae000 -     0x7fff523b8ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff523b9000 -     0x7fff523c0fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff523c1000 -     0x7fff523c9ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff523ca000 -     0x7fff5244ffff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff524d7000 -     0x7fff52510ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff52511000 -     0x7fff5252eff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5252f000 -     0x7fff5252fffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5253d000 -     0x7fff5253dff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5253e000 -     0x7fff52542ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff52543000 -     0x7fff52545ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff52546000 -     0x7fff52547ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff52548000 -     0x7fff5255ffff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff52560000 -     0x7fff52560fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff52561000 -     0x7fff525eaff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff525eb000 -     0x7fff525eeffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff525ef000 -     0x7fff525f2ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff525f3000 -     0x7fff525f4fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff525f5000 -     0x7fff525fbff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff525fc000 -     0x7fff52645ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff52646000 -     0x7fff5266bff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5266c000 -     0x7fff526b7fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff526b8000 -     0x7fff526d7fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff526d8000 -     0x7fff5277cff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5277d000 -     0x7fff52787ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff52788000 -     0x7fff52791ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff52792000 -     0x7fff52799ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5279a000 -     0x7fff527a5fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff527a6000 -     0x7fff527a9ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff527aa000 -     0x7fff527abff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff527ac000 -     0x7fff527b3ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff527b4000 -     0x7fff527c7ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff527c9000 -     0x7fff527ceff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff527cf000 -     0x7fff527fbff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2300
    thread_create: 0
VM Region Summary:
VM Region Summary:
