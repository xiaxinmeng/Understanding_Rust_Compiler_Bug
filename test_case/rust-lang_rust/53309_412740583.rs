plain
[00:02:42]       Memory: 8 GB
[00:02:42]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:42]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:42]       SMC Version (system): 2.8f0
[00:02:42]       Serial Number (system): VM/P21+6wCVj
[00:02:42] 
[00:02:42] hw.ncpu: 4
[00:02:42] hw.byteorder: 1234
[00:02:42] hw.memsize: 8589934592
---
[01:12:00] test [debuginfo-lldb] debuginfo/vec.rs ... ok
[01:12:00] 
[01:12:00] failures:
[01:12:00] 
[01:12:00] ---- [debuginfo-lldb] debuginfo/function-prologue-stepping-regular.rs stdout ----
[01:12:00] NOTE: compiletest thinks it is using LLDB version 902
[01:12:00] 
[01:12:00] error: line not found in debugger output: [...]$29 = 43
[01:12:00] status: exit code: 0
[01:12:00] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/function-prologue-stepping-regular/a" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/function-prologue-stepping-regular/function-prologue-stepping-regular.debugger.script"
[01:12:00] ------------------------------------------
[01:12:00] ------------------------------------------
[01:12:00] LLDB batch-mode script
[01:12:00] ----------------------
[01:12:00] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/function-prologue-stepping-regular/function-prologue-stepping-regular.debugger.script'.
[01:12:00] Target executable is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/function-prologue-stepping-regular/a'.
[01:12:00] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:12:00] Creating a target for '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/function-prologue-stepping-regular/a'
[01:12:00] settings set auto-confirm true
[01:12:00] version
[01:12:00] version
[01:12:00] lldb-902.0.73.1 Swift-4.1 
[01:12:00] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:12:00] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:12:00] type category enable Rust
[01:12:00] 
[01:12:00] breakpoint set --name immediate_args
[01:12:00] Breakpoint 1: where = a`function_prologue_stepping_regular::immediate_args::h8ee6385a86a1b9af + 25 at function-prologue-stepping-regular.rs:134, address = 0x0000000100001449 
[01:12:00] breakpoint set --name non_immediate_args
[01:12:00] Breakpoint 2: where = a`function_prologue_stepping_regular::non_immediate_args::h49fd5bec03f5e790 + 8 at function-prologue-stepping-regular.rs:149, address = 0x0000000100001458 
[01:12:00] breakpoint set --name binding
[01:12:00] Breakpoint 3: where = a`function_prologue_stepping_regular::binding::h8755cf3716df322f + 21 at function-prologue-stepping-regular.rs:152, address = 0x0000000100001485 
[01:12:00] breakpoint set --name assignment
[01:12:00] Breakpoint 4: where = a`function_prologue_stepping_regular::assignment::hfcc3fa6139456f10 + 21 at function-prologue-stepping-regular.rs:156, address = 0x00000001000014b5 
[01:12:00] breakpoint set --name function_call
[01:12:00] Breakpoint 5: where = a`function_prologue_stepping_regular::function_call::he49983b61de006de + 47 at function-prologue-stepping-regular.rs:160, address = 0x00000001000014ff 
[01:12:00] breakpoint set --name identifier
[01:12:00] Breakpoint 6: where = a`function_prologue_stepping_regular::identifier::h760317716245ebf2 + 21 at function-prologue-stepping-regular.rs:164, address = 0x0000000100001535 
[01:12:00] breakpoint set --name return_expr
[01:12:00] Breakpoint 7: where = a`function_prologue_stepping_regular::return_expr::hebf7418d5357fd8a + 21 at function-prologue-stepping-regular.rs:168, address = 0x0000000100001555 
[01:12:00] breakpoint set --name arithmetic_expr
[01:12:00] Breakpoint 8: where = a`function_prologue_stepping_regular::arithmetic_expr::h3a94165d39204beb + 21 at function-prologue-stepping-regular.rs:172, address = 0x0000000100001575 
[01:12:00] breakpoint set --name if_expr
[01:12:00] Breakpoint 9: where = a`function_prologue_stepping_regular::if_expr::hc37664f2a27c4c84 + 21 at function-prologue-stepping-regular.rs:176, address = 0x00000001000015b5 
[01:12:00] breakpoint set --name while_expr
[01:12:00] Breakpoint 10: where = a`function_prologue_stepping_regular::while_expr::h9165cd70b93f098b + 20 at function-prologue-stepping-regular.rs:184, address = 0x0000000100001614 
[01:12:00] breakpoint set --name loop_expr
[01:12:00] Breakpoint 11: where = a`function_prologue_stepping_regular::loop_expr::h3f5398587a67bbdf + 20 at function-prologue-stepping-regular.rs:192, address = 0x0000000100001694 
[01:12:00] run
[01:12:00] Hit breakpoint 1.1: where = a`function_prologue_stepping_regular::immediate_args::h8ee6385a86a1b9af + 25 at function-prologue-stepping-regular.rs:134, address = 0x0000000100001449, resolved, hit count = 1 
[01:12:00] Process 51562 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100001449 a`function_prologue_stepping_regular::immediate_args::h8ee6385a86a1b9af(a=1, b=true, c=2.5) at function-prologue-stepping-regular.rs:134 131 132 fn immediate_args(a: isize, b: bool, c: f64) { 133 () -> 134 } 135 136 struct BigStruct { 137 a: u64, Target 0: (a) stopped. Process 51562 launched: '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/function-prologue-stepping-regular/a' (x86_64) 
[01:12:00] print a
[01:12:00] (long) $0 = 1 
[01:12:00] print b
[01:12:00] (bool) $1 = true 
[01:12:00] print c
[01:12:00] (double) $2 = 2.5 
[01:12:00] continue
[01:12:00] Hit breakpoint 2.1: where = a`function_prologue_stepping_regular::non_immediate_args::h49fd5bec03f5e790 + 8 at function-prologue-stepping-regular.rs:149, address = 0x0000000100001458, resolved, hit count = 1 
[01:12:00] print a
[01:12:00] (function_prologue_stepping_regular::BigStruct) $3 = BigStruct { a: 3, b: 4, c: 5, d: 6, e: 7, f: 8, g: 9, h: 10 } 
[01:12:00] print b
[01:12:00] (function_prologue_stepping_regular::BigStruct) $4 = BigStruct { a: 11, b: 12, c: 13, d: 14, e: 15, f: 16, g: 17, h: 18 } 
[01:12:00] continue
[01:12:00] Hit breakpoint 3.1: where = a`function_prologue_stepping_regular::binding::h8755cf3716df322f + 21 at function-prologue-stepping-regular.rs:152, address = 0x0000000100001485, resolved, hit count = 1 
[01:12:00] print a
[01:12:00] (long) $5 = 19 
[01:12:00] print b
[01:12:00] (unsigned long) $6 = 20 
[01:12:00] print c
[01:12:00] (double) $7 = 21.5 
[01:12:00] continue
[01:12:00] Hit breakpoint 4.1: where = a`function_prologue_stepping_regular::assignment::hfcc3fa6139456f10 + 21 at function-prologue-stepping-regular.rs:156, address = 0x00000001000014b5, resolved, hit count = 1 
[01:12:00] print a
[01:12:00] (unsigned long) $8 = 22 
[01:12:00] print b
[01:12:00] (unsigned long) $9 = 23 
[01:12:00] print c
[01:12:00] (double) $10 = 24.5 
[01:12:00] continue
[01:12:00] Hit breakpoint 5.1: where = a`function_prologue_stepping_regular::function_call::he49983b61de006de + 47 at function-prologue-stepping-regular.rs:160, address = 0x00000001000014ff, resolved, hit count = 1 
[01:12:00] print x
[01:12:00] (unsigned long) $11 = 25 
[01:12:00] print y
[01:12:00] (unsigned long) $12 = 26 
[01:12:00] print z
[01:12:00] (double) $13 = 27.5 
[01:12:00] continue
[01:12:00] Hit breakpoint 6.1: where = a`function_prologue_stepping_regular::identifier::h760317716245ebf2 + 21 at function-prologue-stepping-regular.rs:164, address = 0x0000000100001535, resolved, hit count = 1 
[01:12:00] print x
[01:12:00] (unsigned long) $14 = 28 
[01:12:00] print y
[01:12:00] (unsigned long) $15 = 29 
[01:12:00] print z
[01:12:00] (double) $16 = 30.5 
[01:12:00] continue
[01:12:00] Hit breakpoint 7.1: where = a`function_prologue_stepping_regular::return_expr::hebf7418d5357fd8a + 21 at function-prologue-stepping-regular.rs:168, address = 0x0000000100001555, resolved, hit count = 1 
[01:12:00] print x
[01:12:00] (unsigned long) $17 = 31 
[01:12:00] print y
[01:12:00] (unsigned long) $18 = 32 
[01:12:00] print z
[01:12:00] (double) $19 = 33.5 
[01:12:00] continue
[01:12:00] Hit breakpoint 8.1: where = a`function_prologue_stepping_regular::arithmetic_expr::h3a94165d39204beb + 21 at function-prologue-stepping-regular.rs:172, address = 0x0000000100001575, resolved, hit count = 1 
[01:12:00] print x
[01:12:00] (unsigned long) $20 = 34 
[01:12:00] print y
[01:12:00] (unsigned long) $21 = 35 
[01:12:00] print z
[01:12:00] (double) $22 = 36.5 
[01:12:00] continue
[01:12:00] Hit breakpoint 9.1: where = a`function_prologue_stepping_regular::if_expr::hc37664f2a27c4c84 + 21 at function-prologue-stepping-regular.rs:176, address = 0x00000001000015b5, resolved, hit count = 1 
[01:12:00] print x
[01:12:00] (unsigned long) $23 = 37 
[01:12:00] print y
[01:12:00] (unsigned long) $24 = 38 
[01:12:00] print z
[01:12:00] (double) $25 = 39.5 
[01:12:00] continue
[01:12:00] Hit breakpoint 10.1: where = a`function_prologue_stepping_regular::while_expr::h9165cd70b93f098b + 20 at function-prologue-stepping-regular.rs:184, address = 0x0000000100001614, resolved, hit count = 1 
[01:12:00] print x
[01:12:00] (unsigned long) $26 = 40 
[01:12:00] print y
[01:12:00] (unsigned long) $27 = 41 
[01:12:00] print z
[01:12:00] (unsigned long) $28 = 42 
[01:12:00] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:12:00] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:12:00] Hit breakpoint 10.1: where = a`function_prologue_stepping_regular::while_expr::h9165cd70b93f098b + 20 at function-prologue-stepping-regular.rs:184, address = 0x0000000100001614, resolved, hit count = 2 
[01:12:00] print x
[01:12:00] (unsigned long) $29 = 82 
[01:12:00] print y
[01:12:00] (unsigned long) $30 = 41 
[01:12:00] print z
[01:12:00] (unsigned long) $31 = 42 
[01:12:00] continue
[01:12:00] Hit breakpoint 10.1: where = a`function_prologue_stepping_regular::while_expr::h9165cd70b93f098b + 20 at function-prologue-stepping-regular.rs:184, address = 0x0000000100001614, resolved, hit count = 3 
[01:12:00] quit
[01:12:00] 
[01:12:00] ------------------------------------------
[01:12:00] stderr:
[01:12:00] ------------------------------------------
---
[01:12:00] test result: FAILED. 93 passed; 1 failed; 15 ignored; 0 measured; 0 filtered out
[01:12:00] 
[01:12:00] 
[01:12:00] 
[01:12:00] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/debuginfo" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "debuginfo-lldb" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "7.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:00] 
[01:12:00] 
[01:12:00] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:12:00] Build completed unsuccessfully in 0:17:31
[01:12:00] Build completed unsuccessfully in 0:17:31
[01:12:00] make: *** [check] Error 1
travis_time:end:0596f700:start=1534211926484337000,finish=1534216246459356000,duration=4319975019000

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:097a461a
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0164d1ec
$ sudo tail -n 500 /var/log/syslog
tail: /var/log/syslog: No such file or directory
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:2cd44370
---
travis_fold:start:after_failure.3
travis_time:start:18f7464a
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1184
drwx------  21 travis  staff    714 Aug 14 03:04 .
-rw-------@  1 travis  staff  37510 Aug 14 03:04 a_2018-08-14-030400-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62273 Aug 14 03:04 a_2018-08-14-030400_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60415 Aug 14 03:03 a_2018-08-14-030355-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37409 Aug 14 03:03 a_2018-08-14-030355_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10170 Aug 14 03:03 a_2018-08-14-030347_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9901 Aug 14 03:03 a_2018-08-14-030342_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9870 Aug 14 03:03 a_2018-08-14-030333-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9906 Aug 14 03:03 a_2018-08-14-030333_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9965 Aug 14 03:03 a_2018-08-14-030309_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63133 Aug 14 03:03 a_2018-08-14-030300_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63943 Aug 14 03:02 a_2018-08-14-030256-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64334 Aug 14 03:02 a_2018-08-14-030256-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65147 Aug 14 03:02 a_2018-08-14-030256_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11784 Aug 14 03:01 a_2018-08-14-030104_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9925 Aug 14 03:00 a_2018-08-14-030011_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10514 Aug 14 02:59 a_2018-08-14-025908-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10332 Aug 14 02:59 a_2018-08-14-025908-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10241 Aug 14 02:59 a_2018-08-14-025908-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10514 Aug 14 02:59 a_2018-08-14-025908_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2853702f
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-14-025908-1_Traviss-Mac-1044.crash
Process:               a [34384]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [34379]
Responsible:           a [34384]
User ID:               501
Date/Time:             2018-08-14 02:58:20.900 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 3700 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-f2b94f7472d04a63.dylib  0x000000010608048b std::panicking::rust_panic_with_hook::h39c424b2431f95a3 + 139
1   a                              0x00000001060417d8 std::panicking::begin_panic::h961d1df48108f75e + 40
2   a                              0x000000010603f25c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hee8ed3175b25d327 + 28
3   a                              0x000000010603eda9 core::ptr::drop_in_place::hb47aeb4441f77ce0 + 9
4   a                              0x000000010603f233 backtrace::double::h3a79da7ae181846f + 35
5   a                              0x00000001060403ee backtrace::main::h9751c5dfa7559d82 + 4254 (backtrace.rs:113)
6   a                              0x000000010603e736 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h97a45ae91858d9cb + 6 (rt.rs:74)
7   libstd-f2b94f7472d04a63.dylib  0x000000010607ff58 std::panicking::try::do_call::h1c08d360da421690 (.llvm.9150073407061812456) + 24
8   libstd-f2b94f7472d04a63.dylib  0x00000001060913bf __rust_maybe_catch_panic + 31
9   libstd-f2b94f7472d04a63.dylib  0x000000010605108d std::rt::lang_start_internal::h60d7a932751df80f + 237
10  a                              0x0000000106040c7c main + 44
11  libdyld.dylib                  0x00007fff7ae03115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee9bc3e03  rbx: 0x0000000000000002  rcx: 0x00007fff7af549d2  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001060ef395  rbp: 0x00007ffee9bc3fa0  rsp: 0x00007ffee9bc3f20
   r8: 0x00000001060eef90   r9: 0x0000000106134f80  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001060eef90  r14: 0x0000000106043460  r15: 0x00007ffee9bc3fb0
  rip: 0x000000010608048b  rfl: 0x0000000000010206  cr2: 0x0000000106365ae7
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10603a000 -        0x106042fff +a (0) <F9C78381-D35E-38CC-AEA1-B8F2B17D3B80> /Users/USER/*/a
       0x10604c000 -        0x10612afef +libstd-f2b94f7472d04a63.dylib (0) <9F1F8107-1B14-3E00-BCEB-EDFF5172574B> /Users/USER/*/libstd-f2b94f7472d04a63.dylib
       0x10eb6a000 -        0x10ebb498f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7866d000 -     0x7fff786a0fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff78b7f000 -     0x7fff78b80ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff78e35000 -     0x7fff78e8bfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff78e8c000 -     0x7fff78eb0ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff7a202000 -     0x7fff7a5f33b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7a6c0000 -     0x7fff7a6dcffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff7ac9a000 -     0x7fff7ac9eff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff7ac9f000 -     0x7fff7aca9ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff7acaa000 -     0x7fff7acb1fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7acb2000 -     0x7fff7acbaffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff7acbb000 -     0x7fff7ad40fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff7adc8000 -     0x7fff7ae01ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7ae02000 -     0x7fff7ae1fff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff7ae20000 -     0x7fff7ae20ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff7ae2e000 -     0x7fff7ae2eff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff7ae2f000 -     0x7fff7ae33ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff7ae34000 -     0x7fff7ae36ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff7ae37000 -     0x7fff7ae38ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff7ae39000 -     0x7fff7ae50fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7ae51000 -     0x7fff7ae51fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7ae52000 -     0x7fff7aedbff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7aedc000 -     0x7fff7aedfffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7aee0000 -     0x7fff7aee3ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7aee4000 -     0x7fff7aee5fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7aee6000 -     0x7fff7aeecff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7aeed000 -     0x7fff7af36ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7af37000 -     0x7fff7af5cff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7af5d000 -     0x7fff7afa8fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7afa9000 -     0x7fff7afc8fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7afc9000 -     0x7fff7b06dff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7b06e000 -     0x7fff7b078ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7b079000 -     0x7fff7b082ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7b083000 -     0x7fff7b08aff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7b08b000 -     0x7fff7b096fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff7b097000 -     0x7fff7b09aff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7b09b000 -     0x7fff7b09cff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7b09d000 -     0x7fff7b0a4ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7b0a5000 -     0x7fff7b0b8ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7b0ba000 -     0x7fff7b0bfff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff7b0c0000 -     0x7fff7b0ecff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2610
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.8M resident=0K(0%) swapped_out_or_unallocated=198.8M(100%)
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
__TEXT                            9928K       44 
===========                     =======  ======= 
TOTAL                            280.2M      113 
TOTAL                            280.2M      113 
TOTAL, minus reserved VM space   280.1M      113 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-14-025908-2_Traviss-Mac-1044.crash
Process:               a [35764]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [35763]
Responsible:           a [35764]
User ID:               501
Date/Time:             2018-08-14 02:59:02.338 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 3700 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff7af52e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff7b091150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff7aeaf312 abort + 127
3   libstd-f2b94f7472d04a63.dylib  0x0000000104b89a09 std::sys::unix::abort_internal::h64e1784b64c80adb + 9
4   libstd-f2b94f7472d04a63.dylib  0x0000000104b94030 rust_oom + 32
5   libstd-f2b94f7472d04a63.dylib  0x0000000104bfd789 alloc::alloc::handle_alloc_error::he3610629cd582526 + 9
6   a                              0x0000000104b7bacd default_alloc_error_hook::main::hbf3cf79eecbb97ff + 797
7   a                              0x0000000104b7bcc6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::haa2604b1296d00b8 + 6
8   libstd-f2b94f7472d04a63.dylib  0x0000000104bb6f58 std::panicking::try::do_call::h1c08d360da421690 (.llvm.9150073407061812456) + 24
9   libstd-f2b94f7472d04a63.dylib  0x0000000104bc83bf __rust_maybe_catch_panic + 31
10  libstd-f2b94f7472d04a63.dylib  0x0000000104b8808d std::rt::lang_start_internal::h60d7a932751df80f + 237
11  a                              0x0000000104b7bc3c main + 44
12  libdyld.dylib                  0x00007fff7ae03115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fffb3b29340  rcx: 0x00007ffeeb0840b8  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffeeb0840f0  rsp: 0x00007ffeeb0840b8
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff7af52e3e  rfl: 0x0000000000000206  cr2: 0x00007fffb3b07148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x104b7a000 -        0x104b7cfff +a (0) <98DB1505-EBE2-3FB7-BE38-40D8558A0C4F> /Users/USER/*/a
       0x104b83000 -        0x104c61fef +libstd-f2b94f7472d04a63.dylib (0) <9F1F8107-1B14-3E00-BCEB-EDFF5172574B> /Users/USER/*/libstd-f2b94f7472d04a63.dylib
       0x10bcc9000 -        0x10bd1398f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7866d000 -     0x7fff786a0fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff78b7f000 -     0x7fff78b80ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff78e35000 -     0x7fff78e8bfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff78e8c000 -     0x7fff78eb0ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff7a202000 -     0x7fff7a5f33b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7a6c0000 -     0x7fff7a6dcffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff7ac9a000 -     0x7fff7ac9eff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff7ac9f000 -     0x7fff7aca9ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff7acaa000 -     0x7fff7acb1fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7acb2000 -     0x7fff7acbaffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff7acbb000 -     0x7fff7ad40fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff7adc8000 -     0x7fff7ae01ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7ae02000 -     0x7fff7ae1fff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff7ae20000 -     0x7fff7ae20ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff7ae2e000 -     0x7fff7ae2eff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff7ae2f000 -     0x7fff7ae33ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff7ae34000 -     0x7fff7ae36ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff7ae37000 -     0x7fff7ae38ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff7ae39000 -     0x7fff7ae50fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7ae51000 -     0x7fff7ae51fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7ae52000 -     0x7fff7aedbff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7aedc000 -     0x7fff7aedfffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7aee0000 -     0x7fff7aee3ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7aee4000 -     0x7fff7aee5fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7aee6000 -     0x7fff7aeecff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7aeed000 -     0x7fff7af36ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7af37000 -     0x7fff7af5cff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7af5d000 -     0x7fff7afa8fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7afa9000 -     0x7fff7afc8fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7afc9000 -     0x7fff7b06dff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7b06e000 -     0x7fff7b078ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7b079000 -     0x7fff7b082ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7b083000 -     0x7fff7b08aff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7b08b000 -     0x7fff7b096fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff7b097000 -     0x7fff7b09aff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7b09b000 -     0x7fff7b09cff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7b09d000 -     0x7fff7b0a4ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7b0a5000 -     0x7fff7b0b8ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7b0ba000 -     0x7fff7b0bfff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff7b0c0000 -     0x7fff7b0ecff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2610
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
__DATA                            4344K       45 
__LINKEDIT                       189.1M        5 
__TEXT                            9904K       44 
===========                     =======  ======= 
TOTAL                            280.2M      114 
TOTAL                            280.2M      114 
TOTAL, minus reserved VM space   280.1M      114 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-14-025908-3_Traviss-Mac-1044.crash
Process:               a [33689]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [33687]
Responsible:           a [33689]
User ID:               501
Date/Time:             2018-08-14 02:57:59.434 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 3700 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000000010babd74e abort_on_c_abi::panic_in_ffi::h305168285df224eb + 30
1   a                              0x000000010babcb29 std::panicking::try::do_call::h4d54029088ba88d0 (.llvm.3952037849817213076) + 9
2   libstd-f2b94f7472d04a63.dylib  0x000000010bb083bf __rust_maybe_catch_panic + 31
3   a                              0x000000010babd9a1 abort_on_c_abi::main::h82c0574e6cdd7b93 + 593
4   a                              0x000000010babbda6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h40da0b28bce0b6fe + 6
5   libstd-f2b94f7472d04a63.dylib  0x000000010baf6f58 std::panicking::try::do_call::h1c08d360da421690 (.llvm.9150073407061812456) + 24
6   libstd-f2b94f7472d04a63.dylib  0x000000010bb083bf __rust_maybe_catch_panic + 31
7   libstd-f2b94f7472d04a63.dylib  0x000000010bac808d std::rt::lang_start_internal::h60d7a932751df80f + 237
8   a                              0x000000010babdcac main + 44
9   libdyld.dylib                  0x00007fff7ae03115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x000000010c21c050  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee4142748  rsi: 0xfffffffffffffce8  rbp: 0x00007ffee4143150  rsp: 0x00007ffee4143150
   r8: 0x0000000000000000   r9: 0x000000010bbabf80  r10: 0x000000011503d8d0  r11: 0x00007fff7b0ba96c
  r12: 0x0000000000000000  r13: 0x0000000000000000  r14: 0x00007ffee4143270  r15: 0x00007ffee41431b8
  rip: 0x000000010babd74e  rfl: 0x0000000000010202  cr2: 0x000000010bb80390
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10babb000 -        0x10babeff7 +a (0) <DB032AF7-6E37-3281-BADB-54FE6381BE03> /Users/USER/*/a
       0x10bac3000 -        0x10bba1fef +libstd-f2b94f7472d04a63.dylib (0) <9F1F8107-1B14-3E00-BCEB-EDFF5172574B> /Users/USER/*/libstd-f2b94f7472d04a63.dylib
       0x114feb000 -        0x11503598f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7866d000 -     0x7fff786a0fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff78b7f000 -     0x7fff78b80ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff78e35000 -     0x7fff78e8bfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff78e8c000 -     0x7fff78eb0ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff7a202000 -     0x7fff7a5f33b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7a6c0000 -     0x7fff7a6dcffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff7ac9a000 -     0x7fff7ac9eff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff7ac9f000 -     0x7fff7aca9ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff7acaa000 -     0x7fff7acb1fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7acb2000 -     0x7fff7acbaffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff7acbb000 -     0x7fff7ad40fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff7adc8000 -     0x7fff7ae01ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7ae02000 -     0x7fff7ae1fff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff7ae20000 -     0x7fff7ae20ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff7ae2e000 -     0x7fff7ae2eff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff7ae2f000 -     0x7fff7ae33ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff7ae34000 -     0x7fff7ae36ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff7ae37000 -     0x7fff7ae38ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff7ae39000 -     0x7fff7ae50fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7ae51000 -     0x7fff7ae51fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7ae52000 -     0x7fff7aedbff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7aedc000 -     0x7fff7aedfffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7aee0000 -     0x7fff7aee3ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7aee4000 -     0x7fff7aee5fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7aee6000 -     0x7fff7aeecff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7aeed000 -     0x7fff7af36ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7af37000 -     0x7fff7af5cff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7af5d000 -     0x7fff7afa8fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7afa9000 -     0x7fff7afc8fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7afc9000 -     0x7fff7b06dff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7b06e000 -     0x7fff7b078ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7b079000 -     0x7fff7b082ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7b083000 -     0x7fff7b08aff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7b08b000 -     0x7fff7b096fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff7b097000 -     0x7fff7b09aff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7b09b000 -     0x7fff7b09cff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7b09d000 -     0x7fff7b0a4ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7b0a5000 -     0x7fff7b0b8ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7b0ba000 -     0x7fff7b0bfff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff7b0c0000 -     0x7fff7b0ecff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2610
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.7M resident=0K(0%) swapped_out_or_unallocated=198.7M(100%)
Writable regions: Total=76.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.1M       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4344K       45 
__LINKEDIT                       189.1M        5 
__TEXT                            9908K       44 
===========                     =======  ======= 
TOTAL                            281.2M      114 
TOTAL                            281.2M      114 
TOTAL, minus reserved VM space   281.1M      114 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-14-025908_Traviss-Mac-1044.crash
Process:               a [34386]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [34379]
Responsible:           a [34386]
User ID:               501
Date/Time:             2018-08-14 02:58:20.901 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 3700 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-f2b94f7472d04a63.dylib  0x000000010a40948b std::panicking::rust_panic_with_hook::h39c424b2431f95a3 + 139
1   a                              0x000000010a3c67d8 std::panicking::begin_panic::h961d1df48108f75e + 40
2   a                              0x000000010a3c425c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hee8ed3175b25d327 + 28
3   a                              0x000000010a3c3da9 core::ptr::drop_in_place::hb47aeb4441f77ce0 + 9
4   a                              0x000000010a3c4233 backtrace::double::h3a79da7ae181846f + 35
5   a                              0x000000010a3c53ee backtrace::main::h9751c5dfa7559d82 + 4254 (backtrace.rs:113)
6   a                              0x000000010a3c3736 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h97a45ae91858d9cb + 6 (rt.rs:74)
7   libstd-f2b94f7472d04a63.dylib  0x000000010a408f58 std::panicking::try::do_call::h1c08d360da421690 (.llvm.9150073407061812456) + 24
8   libstd-f2b94f7472d04a63.dylib  0x000000010a41a3bf __rust_maybe_catch_panic + 31
9   libstd-f2b94f7472d04a63.dylib  0x000000010a3da08d std::rt::lang_start_internal::h60d7a932751df80f + 237
10  a                              0x000000010a3c5c7c main + 44
11  libdyld.dylib                  0x00007fff7ae03115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee583ee03  rbx: 0x0000000000000002  rcx: 0x00007fff7af549d2  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010a478395  rbp: 0x00007ffee583ef90  rsp: 0x00007ffee583ef10
   r8: 0x000000010a477f90   r9: 0x000000010a4bdf80  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010a477f90  r14: 0x000000010a3c8460  r15: 0x00007ffee583efa0
  rip: 0x000000010a40948b  rfl: 0x0000000000010206  cr2: 0x000000010a3f2d60
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10a3bf000 -        0x10a3c7fff +a (0) <F9C78381-D35E-38CC-AEA1-B8F2B17D3B80> /Users/USER/*/a
       0x10a3d5000 -        0x10a4b3fef +libstd-f2b94f7472d04a63.dylib (0) <9F1F8107-1B14-3E00-BCEB-EDFF5172574B> /Users/USER/*/libstd-f2b94f7472d04a63.dylib
       0x10d2d5000 -        0x10d31f98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7866d000 -     0x7fff786a0fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff78b7f000 -     0x7fff78b80ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff78e35000 -     0x7fff78e8bfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff78e8c000 -     0x7fff78eb0ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff7a202000 -     0x7fff7a5f33b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7a6c0000 -     0x7fff7a6dcffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff7ac9a000 -     0x7fff7ac9eff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff7ac9f000 -     0x7fff7aca9ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff7acaa000 -     0x7fff7acb1fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff7acb2000 -     0x7fff7acbaffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff7acbb000 -     0x7fff7ad40fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff7adc8000 -     0x7fff7ae01ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff7ae02000 -     0x7fff7ae1fff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff7ae20000 -     0x7fff7ae20ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff7ae2e000 -     0x7fff7ae2eff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff7ae2f000 -     0x7fff7ae33ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff7ae34000 -     0x7fff7ae36ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff7ae37000 -     0x7fff7ae38ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff7ae39000 -     0x7fff7ae50fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7ae51000 -     0x7fff7ae51fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7ae52000 -     0x7fff7aedbff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7aedc000 -     0x7fff7aedfffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7aee0000 -     0x7fff7aee3ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7aee4000 -     0x7fff7aee5fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7aee6000 -     0x7fff7aeecff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7aeed000 -     0x7fff7af36ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7af37000 -     0x7fff7af5cff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7af5d000 -     0x7fff7afa8fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7afa9000 -     0x7fff7afc8fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7afc9000 -     0x7fff7b06dff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7b06e000 -     0x7fff7b078ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7b079000 -     0x7fff7b082ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7b083000 -     0x7fff7b08aff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7b08b000 -     0x7fff7b096fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff7b097000 -     0x7fff7b09aff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7b09b000 -     0x7fff7b09cff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7b09d000 -     0x7fff7b0a4ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7b0a5000 -     0x7fff7b0b8ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7b0ba000 -     0x7fff7b0bfff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff7b0c0000 -     0x7fff7b0ecff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2610
    thread_create: 0
VM Region Summary:
VM Region Summary:
