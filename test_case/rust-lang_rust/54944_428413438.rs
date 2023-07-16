plain
[00:48:46] 
[00:48:46] ---- [ui] ui/lint/must_use-unit.rs stdout ----
[00:48:46] diff of stderr:
[00:48:46] 
[00:48:46] - error: unused return value of `foo` which must be used
[00:48:46] + error: unused return value of `foo` that must be used
[00:48:46] 3    |
[00:48:46] 3    |
[00:48:46] 4 LL |     foo(); //~ unused return value of `foo`
[00:48:46] 
[00:48:46] 10 LL | #![deny(unused_must_use)]
[00:48:46] 12 
[00:48:46] 12 
[00:48:46] - error: unused return value of `bar` which must be used
[00:48:46] + error: unused return value of `bar` that must be used
[00:48:46] 15    |
[00:48:46] 15    |
[00:48:46] 16 LL |     bar(); //~ unused return value of `bar`
[00:48:46] 
[00:48:46] The actual stderr differed from the expected stderr.
[00:48:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit/must_use-unit.stderr
[00:48:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit/must_use-unit.stderr
[00:48:46] To update references, rerun the tests and pass the `--bless` flag
[00:48:46] To only update this specific test, also pass `--test-args lint/must_use-unit.rs`
[00:48:46] error: 1 errors occurred comparing output.
[00:48:46] status: exit code: 1
[00:48:46] status: exit code: 1
[00:48:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_use-unit.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit/auxiliary" "-A" "unused"
[00:48:46] ------------------------------------------
[00:48:46] 
[00:48:46] ------------------------------------------
[00:48:46] stderr:
[00:48:46] stderr:
[00:48:46] ------------------------------------------
[00:48:46] {"message":"unused return value of `foo` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":145,"byte_end":151,"line_start":14,"line_end":14,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    foo(); //~ unused return value of `foo`","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":33,"byte_end":48,"line_start":3,"line_end":3,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![deny(unused_must_use)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused return value of `foo` that must be used\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:14:5\n   |\nLL |     foo(); //~ unused return value of `foo`\n   |     ^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:3:9\n   |\nLL | #![deny(unused_must_use)]\n   |         ^^^^^^^^^^^^^^^\n\n"}
[00:48:46] {"message":"unused return value of `bar` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":190,"byte_end":196,"line_start":16,"line_end":16,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    bar(); //~ unused return value of `bar`","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unused return value of `bar` that must be used\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:16:5\n   |\nLL |     bar(); //~ unused return value of `bar`\n   |     ^^^^^^\n\n"}
[00:48:46] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:48:46] ------------------------------------------
[00:48:46] 
[00:48:46] thread '[ui] ui/lint/must_use-unit.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:48:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:48:46] 
[00:48:46] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[00:48:46] 
[00:48:46] 
[00:48:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:46] 
[00:48:46] 
[00:48:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:46] Build completed unsuccessfully in 0:03:31
[00:48:46] Build completed unsuccessfully in 0:03:31
[00:48:46] make: *** [check] Error 1
[00:48:46] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1cce3363
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2b062634:start=1539137280271758836,finish=1539137280279800976,duration=8042140
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:129957a2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:292704d5
travis_time:start:292704d5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00bff821
$ dmesg | grep -i kill
