plain
[00:51:22] ....................................................................................................
[00:51:26] ....................................................................................................
[00:51:33] ....................................................................................................
[00:51:39] ....................................................................................................
[00:51:44] ........i....................................................F...........................i..........
[00:51:55] ....................................................................................................
[00:52:02] ....................................................................................................
[00:52:08] ..........................i.........................................................................
[00:52:09] ...........
[00:52:09] ...........
[00:52:09] failures:
[00:52:09] 
[00:52:09] ---- [ui] ui/nll/issue-51244.rs stdout ----
[00:52:09] diff of stderr:
[00:52:09] 
[00:52:09] - error[E0594]: cannot assign to data in a `&` reference
[00:52:09] + error[E0594]: cannot assign to `*my_ref` which is behind a `&` reference
[00:52:09] 2   --> $DIR/issue-51244.rs:15:5
[00:52:09] 3    |
[00:52:09] - LL |     let ref my_ref @ _ = 0;
[00:52:09] -    |         -------------- help: consider changing this to be a mutable reference: `&mut ef my_ref @ _`
[00:52:09] - LL |     *my_ref = 0; //~ ERROR cannot assign to immutable borrowed content `*my_ref` [E0594]
[00:52:09] -    |     ^^^^^^^^^^^
[00:52:09] + LL |     *my_ref = 0; //~ ERROR  cannot assign to data in a `&` reference [E0594]
[00:52:09] 8 
[00:52:09] 9 error: aborting due to previous error
[00:52:09] 10 
[00:52:09] 
[00:52:09] 
[00:52:09] 
[00:52:09] The actual stderr differed from the expected stderr.
[00:52:09] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51244/issue-51244.stderr
[00:52:09] To update references, rerun the tests and pass the `--bless` flag
[00:52:09] To only update this specific test, also pass `--test-args nll/issue-51244.rs`
[00:52:09] error: 1 errors occurred comparing output.
[00:52:09] status: exit code: 101
[00:52:09] status: exit code: 101
[00:52:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-51244.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51244/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51244/auxiliary" "-A" "unused"
[00:52:09] ------------------------------------------
[00:52:09] 
[00:52:09] ------------------------------------------
[00:52:09] stderr:
[00:52:09] stderr:
[00:52:09] ------------------------------------------
[00:52:09] {"message":"cannot assign to `*my_ref` which is behind a `&` reference","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-51244.rs","byte_start":529,"byte_end":540,"line_start":15,"line_end":15,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    *my_ref = 0; //~ ERROR  cannot assign to data in a `&` reference [E0594]","highlight_start":5,"highlight_end":16}],"label":"cannot assign","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to `*my_ref` which is behind a `&` reference\n  --> /checkout/src/test/ui/nll/issue-51244.rs:15:5\n   |\nLL |     *my_ref = 0; //~ ERROR  cannot assign to data in a `&` reference [E0594]\n   |     ^^^^^^^^^^^ cannot assign\n\n"}
[00:52:09] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:52:09] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:52:09] ------------------------------------------
[00:52:09] 
[00:52:09] thread '[ui] ui/nll/issue-51244.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:52:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:52:09] 
[00:52:09] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:52:09] 
[00:52:09] 
[00:52:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:09] 
[00:52:09] 
[00:52:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:09] Build completed unsuccessfully in 0:02:15
[00:52:09] Build completed unsuccessfully in 0:02:15
[00:52:09] Makefile:58: recipe for target 'check' failed
[00:52:09] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:023b8be9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0c9af5e4:start=1529674736997423275,finish=1529674737004438225,duration=7014950
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1be88a77
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05260bc4
$ dmesg | grep -i kill
