plain
[01:08:27] normalized stderr:
[01:08:27] warning: unused import: `std::env`
[01:08:27]   --> $DIR/env-null-vars.rs:19:5
[01:08:27]    |
[01:08:27] LL | use std::env;
[01:08:27]    |
[01:08:27] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:08:27]    = note: #[warn(unused_imports)] on by default
[01:08:27] 
[01:08:27] 
[01:08:27] 
[01:08:27] 
[01:08:27] 
[01:08:27] The actual stderr differed from the expected stderr.
[01:08:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/env-null-vars/env-null-vars.stderr
[01:08:27] To update references, rerun the tests and pass the `--bless` flag
[01:08:27] To only update this specific test, also pass `--test-args env-null-vars.rs`
[01:08:27] error: 1 errors occurred comparing output.
[01:08:27] status: exit code: 0
[01:08:27] status: exit code: 0
[01:08:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/env-null-vars.rs" "--target=asmjs-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/env-null-vars/a.js" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/env-null-vars/auxiliary"
[01:08:27] ------------------------------------------
[01:08:27] 
[01:08:27] ------------------------------------------
[01:08:27] stderr:
[01:08:27] stderr:
[01:08:27] ------------------------------------------
[01:08:27] {"message":"unused import: `std::env`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/env-null-vars.rs","byte_start":591,"byte_end":599,"line_start":19,"line_end":19,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"use std::env;","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `std::env`\n  --> /checkout/src/test/run-pass/env-null-vars.rs:19:5\n   |\nLL | use std::env;\n   |     ^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:08:27] ------------------------------------------
[01:08:27] 
[01:08:27] thread '[run-pass] run-pass/env-null-vars.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[01:08:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:08:27] test result: FAILED. 499 passed; 1 failed; 49 ignored; 0 measured; 0 filtered out
[01:08:27] 
[01:08:27] 
[01:08:27] 
[01:08:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "run-pass" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/4.1.1_64bit/bin/node" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:27] 
[01:08:27] 
[01:08:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/run-pass src/test/run-fail src/libstd src/liballoc src/libcore
[01:08:27] Build completed unsuccessfully in 1:02:49
---
travis_time:end:2837e623:start=1537448334689643781,finish=1537448334696010040,duration=6366259
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02553735
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1317b730
travis_time:start:1317b730
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a6dfb4b
$ dmesg | grep -i kill
