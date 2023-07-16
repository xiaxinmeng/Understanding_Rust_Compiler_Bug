plain
[01:10:05] test [ui] ui/imports/extern-prelude-extern-crate-restricted-shadowing.rs ... ok
[01:10:05] test [ui] ui/imports/extern-prelude-extern-crate-pass.rs ... ok
[01:10:05] test [ui] ui/imports/extern-prelude-extern-crate-shadowing.rs ... ok
[01:10:05] test [ui] ui/imports/glob-shadowing.rs ... ok
[01:10:05] test [ui] ui/imports/gensymed.rs ... ok
[01:10:05] test [ui] ui/imports/glob-conflict-cross-crate.rs ... ok
[01:10:05] test [ui] ui/imports/import-loop-2.rs ... ok
[01:10:05] test [ui] ui/imports/import-glob-0.rs ... ok
[01:10:05] test [ui] ui/imports/import-glob-circular.rs ... ok
---
[01:12:17] failures:
[01:12:17] 
[01:12:17] ---- [ui] ui/issues/issue-23458.rs stdout ----
[01:12:17] 
[01:12:17] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:12:17] status: exit code: 101
[01:12:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23458.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23458/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23458/auxiliary" "-A" "unused"
[01:12:17] ------------------------------------------
[01:12:17] 
[01:12:17] ------------------------------------------
[01:12:17] stderr:
[01:12:17] stderr:
[01:12:17] ------------------------------------------
[01:12:17] LLVM ERROR: Invalid $ operand number in inline asm string: 'int $3'
[01:12:17] ------------------------------------------
[01:12:17] 
[01:12:17] thread '[ui] ui/issues/issue-23458.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:12:17] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:12:17] 
[01:12:17] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:12:17] 
[01:12:17] 
[01:12:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.35.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:17] 
[01:12:17] 
[01:12:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:12:17] Build completed unsuccessfully in 1:03:25
---
travis_time:end:06c0f332:start=1553422789228117956,finish=1553422789237062399,duration=8944443
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01d3cf33
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1464fa59
travis_time:start:1464fa59
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c68031a
$ dmesg | grep -i kill
