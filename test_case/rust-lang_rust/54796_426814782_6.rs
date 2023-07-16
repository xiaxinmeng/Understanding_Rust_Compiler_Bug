\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/range/issue-54505-no-std.rs","byte_start":1129,"byte_end":1134,"line_start":46,"line_end":46,"column_start":16,"column_end":21,"is_primary":true,"text":[{"text":"    take_range(..=42);","highlight_start":16,"highlight_end":21}],"label":"expected reference, found struct `core::ops::RangeToInclusive`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `&_`\n   found type `core::ops::RangeToInclusive<{integer}>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider borrowing here","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/range/issue-54505-no-std.rs","byte_start":1129,"byte_end":1134,"line_start":46,"line_end":46,"column_start":16,"column_end":21,"is_primary":true,"text":[{"text":"    take_range(..=42);","highlight_start":16,"highlight_end":21}],"label":null,"suggested_replacement":"&(..=42)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:46:16\n   |\nLL |     take_range(..=42);\n   |                ^^^^^\n   |                |\n   |                expected reference, found struct `core::ops::RangeToInclusive`\n   |                help: consider borrowing here: `&(..=42)`\n   |\n   = note: expected type `&_`\n              found type `core::ops::RangeToInclusive<{integer}>`\n\n"}
[00:51:26] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[00:51:26] {"message":"aborting due to 7 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 7 previous errors\n\n"}
[00:51:26] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[00:51:26] ------------------------------------------
[00:51:26] 
[00:51:26] thread '[ui] ui/range/issue-54505-no-std.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:51:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:26] test result: FAILED. 4504 passed; 1 failed; 46 ignored; 0 measured; 0 filtered out
[00:51:26] 
[00:51:26] 
[00:51:26] 
[00:51:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:26] 
[00:51:26] 
[00:51:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:51:26] Build completed unsuccessfully in 0:48:02
---
travis_time:end:1d354a10:start=1538603132554819199,finish=1538603132563213925,duration=8394726
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b3882db
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:109d76e8
travis_time:start:109d76e8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a1e678b
$ dmesg | grep -i kill
