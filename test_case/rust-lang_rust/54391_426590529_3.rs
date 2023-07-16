\n"},"level":"error","spans":[{"file_name":"/Users/travis/build/rust-lang/rust/src/test/ui/rust-2018/issue-54006.rs","byte_start":523,"byte_end":528,"line_start":16,"line_end":16,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"use alloc::vec;","highlight_start":5,"highlight_end":10}],"label":"Did you mean `core::alloc`?","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0432]: unresolved import `alloc`\n  --> /Users/travis/build/rust-lang/rust/src/test/ui/rust-2018/issue-54006.rs:16:5\n   |\nLL | use alloc::vec;\n   |     ^^^^^ Did you mean `core::alloc`?\n\n"}
[01:01:27] {"message":"cannot determine resolution for the macro `vec`","code":null,"level":"error","spans":[{"file_name":"/Users/travis/build/rust-lang/rust/src/test/ui/rust-2018/issue-54006.rs","byte_start":605,"byte_end":608,"line_start":20,"line_end":20,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"    let mut xs = vec![];","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"import resolution is stuck, try simplifying macro imports","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot determine resolution for the macro `vec`\n  --> /Users/travis/build/rust-lang/rust/src/test/ui/rust-2018/issue-54006.rs:20:18\n   |\nLL |     let mut xs = vec![];\n   |                  ^^^\n   |\n   = note: import resolution is stuck, try simplifying macro imports\n\n"}
[01:01:27] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:01:27] {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
[01:01:27] ------------------------------------------
[01:01:27] 
[01:01:27] thread '[ui] ui/rust-2018/issue-54006.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[01:01:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:27] 
[01:01:27] 
[01:01:27] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[01:01:27] make: *** [check] Error 1
[01:01:27]     [ui] ui/rust-2018/issue-54006.rs
[01:01:27] 
[01:01:27] test result: FAILED. 4518 passed; 1 failed; 26 ignored; 0 measured; 0 filtered out
[01:01:27] 
[01:01:27] 
[01:01:27] 
[01:01:27] 
[01:01:27] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/ui" "--build-base" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:27] 
[01:01:27] 
[01:01:27] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:01:27] Build completed unsuccessfully in 0:06:28
---
travis_fold:start:after_failure.2
travis_time:start:0cc47d5b
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:134f1195
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:134f1195:start=1538563405324143000,finish=1538563405345197000,duration=21054000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b2a678f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03ae871c
travis_time:start:03ae871c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0835dcc2
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
