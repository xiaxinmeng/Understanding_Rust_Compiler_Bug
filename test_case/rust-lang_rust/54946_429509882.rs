plain
[00:48:15] failures:
[00:48:15] 
[00:48:15] ---- [ui] ui/iterators/array-of-ranges.rs stdout ----
[00:48:15] 
[00:48:15] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:48:15] status: exit code: 101
[00:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/array-of-ranges.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array-of-ranges/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array-of-ranges/auxiliary" "-A" "unused"
[00:48:15] ------------------------------------------
[00:48:15] 
[00:48:15] ------------------------------------------
[00:48:15] stderr:
---
[00:48:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:48:15] 
[00:48:15] note: rustc 1.31.0-nightly (73008786a 2018-10-13) running on x86_64-unknown-linux-gnu
[00:48:15] 
[00:48:15] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C linker=cc
[00:48:15] 
[00:48:15] ------------------------------------------
[00:48:15] 
[00:48:15] thread '[ui] ui/iterators/array-of-ranges.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
---
[00:48:15] 
[00:48:15] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[00:48:15] 
[00:48:15] 
[00:48:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:15] 
[00:48:15] 
[00:48:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:48:15] Build completed unsuccessfully in 0:44:34
---
travis_time:end:2c05f0b0:start=1539404487309690055,finish=1539404487316532091,duration=6842036
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19b0e31f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:112f6efc
travis_time:start:112f6efc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:19767630
$ dmesg | grep -i kill
