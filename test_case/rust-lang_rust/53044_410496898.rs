plain
[00:44:29] ....................................................................................................
[00:44:32] ....................................................................................................
[00:44:34] ....................................................................................................
[00:44:37] ....................................................................................................
[00:44:40] iiiiiiiii...........................................................................................
[00:44:46] ....................................................................................................
[00:44:49] .....i..............................................................................................
[00:44:52] ..........i.........................................................................................
[00:44:55] ....................................................................................................
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:52:55] 
[00:52:55] running 96 tests
[00:54:44] ...............................................................F....test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[00:56:03] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:03] failures:
[00:56:03] 
[00:56:03] ---- [run-pass] run-pass-fulldeps/proc-macro/derive-b.rs stdout ----
[00:56:03] 
[00:56:03] 
[00:56:03] error: compilation failed!
[00:56:03] status: exit code: 1
[00:56:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/derive-b.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-b/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-b/auxiliary"
[00:56:03] ------------------------------------------
[00:56:03] 
[00:56:03] ------------------------------------------
[00:56:03] stderr:
[00:56:03] stderr:
[00:56:03] ------------------------------------------
[00:56:03] error: expected one of `(` or `=`, found `arbitrary`
[00:56:03]    |
[00:56:03]    |
[00:56:03] 1  | // Copyright 2016 The Rust Project Developers. See the COPYRIGHT
[00:56:03]    | - expected one of `(` or `=` here
[00:56:03] ...
[00:56:03] 19 | #[cfg_attr(all(), B arbitrary tokens)]
[00:56:03]    |                     ^^^^^^^^^ unexpected token
[00:56:03] error: aborting due to previous error
[00:56:03] 
[00:56:03] 
[00:56:03] ------------------------------------------
---
[00:56:03] test result: FAILED. 95 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:56:03] 
[00:56:03] 
[00:56:03] 
[00:56:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checko
