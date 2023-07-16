
[01:23:25] 

[01:23:25] failures:

[01:23:25] 

[01:23:25] ---- [debuginfo-lldb] debuginfo/macro-stepping.rs stdout ----

[01:23:25] 	NOTE: compiletest thinks it is using LLDB version 902

[01:23:25] 

[01:23:25] error: line not found in debugger output: [...]#loc5[...]

[01:23:25] status: exit code: 0

[01:23:25] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/macro-stepping.stage2-i686-apple-darwin" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/macro-stepping.debugger.script"

[01:23:25] stdout:

[01:23:25] ------------------------------------------

[01:23:25] LLDB batch-mode script

[01:23:25] ----------------------

[01:23:25] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/macro-stepping.debugger.script'.

[01:23:25] Target executable is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/macro-stepping.stage2-i686-apple-darwin'.

[01:23:25] Current working directory is '/Users/travis/build/rust-lang/rust'

[01:23:25] Creating a target for '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/macro-stepping.stage2-i686-apple-darwin'

[01:23:25] settings set auto-confirm true

[01:23:25] 

[01:23:25] version

[01:23:25] lldb-902.0.73.1 Swift-4.1 

[01:23:25] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py

[01:23:25] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust

[01:23:25] type category enable Rust

[01:23:25] 

[01:23:25] breakpoint set --file 'macro-stepping.rs' --line 111

[01:23:25] Breakpoint 1: where = macro-stepping.stage2-i686-apple-darwin`macro_stepping::main::hb15c3f8d880d6dba at macro-stepping.rs:111, address = 0x00003030 

[01:23:25] breakpoint set --file 'macro-stepping.rs' --line 126

[01:23:25] Breakpoint 2: where = macro-stepping.stage2-i686-apple-darwin`macro_stepping::main::hb15c3f8d880d6dba + 402 at macro-stepping.rs:126, address = 0x000031c2 

[01:23:25] set set stop-line-count-before 0

[01:23:25] 

[01:23:25] set set stop-line-count-after 1

[01:23:25] 

[01:23:25] run

[01:23:25] Hit breakpoint 1.1: where = macro-stepping.stage2-i686-apple-darwin`macro_stepping::main::hb15c3f8d880d6dba at macro-stepping.rs:111, address = 0x00003030, resolved, hit count = 1 

[01:23:25] Process 44098 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00003030 macro-stepping.stage2-i686-apple-darwin`macro_stepping::main::hb15c3f8d880d6dba at macro-stepping.rs:111 -> 111 zzz(); // #break 112 Target 0: (macro-stepping.stage2-i686-apple-darwin) stopped. Process 44098 launched: '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/macro-stepping.stage2-i686-apple-darwin' (i386) 

[01:23:25] next

[01:23:25] frame select

[01:23:25] frame #0: 0x00003060 macro-stepping.stage2-i686-apple-darwin`macro_stepping::main::hb15c3f8d880d6dba at macro-stepping.rs:113 -> 113 foo!(); // #loc1 114 

[01:23:25] next

[01:23:25] frame select

[01:23:25] frame #0: 0x00003078 macro-stepping.stage2-i686-apple-darwin`macro_stepping::main::hb15c3f8d880d6dba at macro-stepping.rs:115 -> 115 foo2!(); // #loc2 116 

[01:23:25] next

[01:23:25] frame select

[01:23:25] frame #0: 0x000030b0 macro-stepping.stage2-i686-apple-darwin`macro_stepping::main::hb15c3f8d880d6dba at macro-stepping.rs:117 -> 117 let x = vec![42]; // #loc3 118 

[01:23:25] next

[01:23:25] frame select

[01:23:25] frame #0: 0x000030f1 macro-stepping.stage2-i686-apple-darwin`macro_stepping::main::hb15c3f8d880d6dba at macro-stepping.rs:117 -> 117 let x = vec![42]; // #loc3 118 

[01:23:25] next

[01:23:25] frame select

[01:23:25] frame #0: 0x000030f4 macro-stepping.stage2-i686-apple-darwin`macro_stepping::main::hb15c3f8d880d6dba at macro-stepping.rs:119 -> 119 new_scope!(); // #loc4 120 

[01:23:25] continue

[01:23:25] Hit breakpoint 2.1: where = macro-stepping.stage2-i686-apple-darwin`macro_stepping::main::hb15c3f8d880d6dba + 402 at macro-stepping.rs:126, address = 0x000031c2, resolved, hit count = 1 

[01:23:25] step

[01:23:25] frame select

[01:23:25] frame #0: 0x00003200 macro-stepping.stage2-i686-apple-darwin`macro_stepping::included::h0d608a1ca9b366e4 at macro-stepping.inc:12 -> 12 foo!(); // #inc-loc1 13 

[01:23:25] next

[01:23:25] frame select

[01:23:25] frame #0: 0x0000321b macro-stepping.stage2-i686-apple-darwin`macro_stepping::included::h0d608a1ca9b366e4 at macro-stepping.inc:14 -> 14 foo2!(); // #inc-loc2 15 

[01:23:25] next

[01:23:25] frame select

[01:23:25] frame #0: 0x00003253 macro-stepping.stage2-i686-apple-darwin`macro_stepping::included::h0d608a1ca9b366e4 at macro-stepping.inc:16 -> 16 zzz(); // #inc-loc3 17 } 

[01:23:25] quit

[01:23:25] None

[01:23:25] 

[01:23:25] ------------------------------------------

[01:23:25] stderr:

[01:23:25] ------------------------------------------

[01:23:25] warning: (i386) /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/macro-stepping.stage2-i686-apple-darwin.lldb.aux/libmacro_stepping.dylib empty dSYM file detected, dSYM was created with an executable with no debug info.

[01:23:25] 

[01:23:25] ------------------------------------------

[01:23:25] 

[01:23:25] thread '[debuginfo-lldb] debuginfo/macro-stepping.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2900:9

[01:23:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.

[01:23:25] 

[01:23:25] 

[01:23:25] failures:

[01:23:25]     [debuginfo-lldb] debuginfo/macro-stepping.rs

[01:23:25] 

[01:23:25] test result: [31mFAILED(B[m. 89 passed; 1 failed; 19 ignored; 0 measured; 0 filtered out

[01:23:25] 

[01:23:25] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:478:22

[01:23:25] 

[01:23:25] 

[01:23:25] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/debuginfo" "--build-base" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo" "--stage-id" "stage2-i686-apple-darwin" "--mode" "debuginfo-lldb" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "6.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"

[01:23:25] expected success, got: exit code: 101

[01:23:25] 

[01:23:25] 

[01:23:25] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test

[01:23:25] Build completed unsuccessfully in 0:23:32

[01:23:25] make: *** [check] Error 1

[01:23:25] make: INTERNAL: Exiting with 1 jobserver tokens available; should be 3!
