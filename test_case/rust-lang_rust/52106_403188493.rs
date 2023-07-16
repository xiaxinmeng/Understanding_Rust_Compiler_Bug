plain
[00:41:53] .................................i..................................................................
[00:41:57] ......................i.............................................................................
[00:42:02] ....................................................................................................
[00:42:08] ....................................................................................................
aborting due to previous error\n\n"}
[00:42:17] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:42:17] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:42:17] ------------------------------------------
[00:42:17] 
[00:42:17] thread '[ui (nll)] ui/suggestions/issue-52059.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:42:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:42:17] test result: FAILED. 1547 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[00:42:17] 
[00:42:17] 
[00:42:17] 
[00:42:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rus

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18010de0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
