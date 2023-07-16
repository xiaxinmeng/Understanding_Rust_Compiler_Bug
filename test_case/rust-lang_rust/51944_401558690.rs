plain
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:48:13] 
[00:48:13] running 3033 tests
[00:48:27] ....................................................................................................
[00:48:41] ..................F............................i....................................................
[00:49:11] ....................................................................................................
[00:49:24] ....................................................................................................
[00:49:44] ....................................................................................................
[00:49:57] ....................................................................................................
---
[00:57:13] 
[00:57:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:57:13] 
[00:57:13] 
[00:57:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docc

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14032df7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
