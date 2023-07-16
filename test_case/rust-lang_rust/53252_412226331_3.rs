\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs","byte_start":611,"byte_end":620,"line_start":17,"line_end":17,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    x.push(y); //~ ERROR explicit lifetime required","highlight_start":5,"highlight_end":14}],"label":"lifetime `'a` required","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add explicit lifetime `'a` to the type of `y`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs","byte_start":554,"byte_end":556,"line_start":13,"line_end":13,"column_start":42,"column_end":44,"is_primary":true,"text":[{"text":"fn baz<'a, 'b, T>(x: &mut Vec<&'a T>, y: &T)","highlight_start":42,"highlight_end":44}],"label":null,"suggested_replacement":"&'a T","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0621]: explicit lifetime required in the type of `y`\n  --> /checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs:17:5\n   |\nLL | fn baz<'a, 'b, T>(x: &mut Vec<&'a T>, y: &T)\n   |                                          -- help: add explicit lifetime `'a` to the type of `y`: `&'a T`\n...\nLL |     x.push(y); //~ ERROR explicit lifetime required\n   |     ^^^^^^^^^ lifetime `'a` required\n\n"}
[00:43:17] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:17] {"message":"For more information about this error, try `rustc --explain E0621`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0621`.\n"}
[00:43:17] ------------------------------------------
[00:43:17] 
[00:43:17] thread '[ui (nll)] ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:43:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:43:17] 
[00:43:17] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:43:17] 
[00:43:17] 
[00:43:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--mode" "ui" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-x86_64/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "7.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:43:17] 
[00:43:17] 
[00:43:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target x86_64-unknown-linux-musl
[00:43:17] Build completed unsuccessfully in 0:39:53
---
travis_time:end:0f962a85:start=1533941520424258840,finish=1533941520432262177,duration=8003337
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0387e8da
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13ed5c62
travis_time:start:13ed5c62
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13eb22ed
$ dmesg | grep -i kill
