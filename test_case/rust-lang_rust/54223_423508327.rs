plain
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> asmjs-unknown-emscripten)
[00:59:14] 
[00:59:14] running 549 tests
[01:00:59] i........i..............i...............iii...................................................i....i
[01:02:08] .i....ii........i.........ii............................iii...F.i.............................i....i
[01:04:44] ...................i...i.........................i...........i.................................i..i.
[01:06:08] .....ii.....i.....iii...............ii..........................i...................................
[01:06:42] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:06:42] ..........ii.......i.........i.i..........ii.....
[01:06:42] ..........ii.......i.........i.i..........ii.....
[01:06:42] failures:
[01:06:42] 
[01:06:42] ---- [run-pass] run-pass/env-null-vars.rs stdout ----
[01:06:42] normalized stderr:
[01:06:42] warning: unused import: `std::env`
[01:06:42]   --> $DIR/env-null-vars.rs:21:5
[01:06:42]    |
[01:06:42] LL | use std::env;
[01:06:42]    |
[01:06:42]    = note: #[warn(unused_imports)] on by default
[01:06:42] 
[01:06:42] 
[01:06:42] 
[01:06:42] 
[01:06:42] 
[01:06:42] The actual stderr differed from the expected stderr.
[01:06:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/env-null-vars/env-null-vars.stderr
[01:06:42] To update references, rerun the tests and pass the `--bless` flag
[01:06:42] To only update this specific test, also pass `--test-args env-null-vars.rs`
[01:06:42] error: 1 errors occurred comparing output.
[01:06:42] status: exit code: 0
[01:06:42] status: exit code: 0
[01:06:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/env-null-vars.rs" "--target=asmjs-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/env-null-vars/a.js" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/env-null-vars/auxiliary"
[01:06:42] ------------------------------------------
[01:06:42] 
[01:06:42] ------------------------------------------
[01:06:42] stderr:
[01:06:42] stderr:
[01:06:42] ------------------------------------------
[01:06:42] {"message":"unused import: `std::env`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/env-null-vars.rs","byte_start":613,"byte_end":621,"line_start":21,"line_end":21,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"use std::env;","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `std::env`\n  --> /checkout/src/test/run-pass/env-null-vars.rs:21:5\n   |\nLL | use std::env;\n   |     ^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:06:42] ------------------------------------------
[01:06:42] 
[01:06:42] thread '[run-pass] run-pass/env-null-vars.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[01:06:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:06:42] test result: FAILED. 499 passed; 1 failed; 49 ignored; 0 measured; 0 filtered out
[01:06:42] 
[01:06:42] 
[01:06:42] 
[01:06:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "run-pass" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/4.1.1_64bit/bin/node" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:42] 
[01:06:42] 
[01:06:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/run-pass src/test/run-fail src/libstd src/liballoc src/libcore
[01:06:42] Build completed unsuccessfully in 1:01:39
---
travis_time:end:0e33bc0c:start=1537531351080786670,finish=1537531351086032171,duration=5245501
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09c9d788
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:071ed91f
travis_time:start:071ed91f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:118e85a8
$ dmesg | grep -i kill
