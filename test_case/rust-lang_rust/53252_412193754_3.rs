\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs","byte_start":611,"byte_end":620,"line_start":17,"line_end":17,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    x.push(y); //~ ERROR explicit lifetime required","highlight_start":5,"highlight_end":14}],"label":"lifetime `'a` required","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add explicit lifetime `'a` to the type of `y`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs","byte_start":554,"byte_end":556,"line_start":13,"line_end":13,"column_start":42,"column_end":44,"is_primary":true,"text":[{"text":"fn baz<'a, 'b, T>(x: &mut Vec<&'a T>, y: &T)","highlight_start":42,"highlight_end":44}],"label":null,"suggested_replacement":"&'a T","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0621]: explicit lifetime required in the type of `y`\n  --> /checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs:17:5\n   |\nLL | fn baz<'a, 'b, T>(x: &mut Vec<&'a T>, y: &T)\n   |                                          -- help: add explicit lifetime `'a` to the type of `y`: `&'a T`\n...\nLL |     x.push(y); //~ ERROR explicit lifetime required\n   |     ^^^^^^^^^ lifetime `'a` required\n\n"}
[00:58:47] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:58:47] {"message":"For more information about this error, try `rustc --explain E0621`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0621`.\n"}
[00:58:47] ------------------------------------------
[00:58:47] 
[00:58:47] thread '[ui (nll)] ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:58:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:58:47] 
[00:58:47] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:58:47] 
[00:58:47] 
[00:58:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:58:47] 
[00:58:47] 
[00:58:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:47] Build completed unsuccessfully in 0:03:08
[00:58:47] Build completed unsuccessfully in 0:03:08
[00:58:47] Makefile:58: recipe for target 'check' failed
[00:58:47] make: *** [check] Error 1
Checking DNS for travis-ci/travis-ci#9696! --------------------------
[00:58:47] travis_fold:start:resolv-conf
cat /etc/resolv.conf --------------------------------------------
[00:58:47] search c.travis-ci-prod-4.internal google.internal
