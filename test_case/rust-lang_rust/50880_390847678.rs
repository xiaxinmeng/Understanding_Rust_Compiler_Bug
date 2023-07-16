plain
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:49:00] 
[00:49:00] running 3007 tests
[00:49:15] ......F.............................................................................................
[00:49:45] ....................................................................................................
[00:50:00] ....................................................................................................
[00:50:13] ....................................................................................................
[00:50:33] ....................................................................................................
---
[00:53:35] ....................................................................................................
[00:53:48] ....................................................................................................
[00:54:05] ...........................................i........................................................
[00:54:25] ..........................................i.........................................................
[00:54:52] ........................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:54:55] .......F....
[00:55:08] .....................................................F..............................................
[00:56:11] ...........................i.ii.....................................................................
[00:56:39] ...................................iiiiiii..........................................................
[00:57:00] ....................................................................................................
[00:57:15] ....................................................................................................
---
[00:57:33] ---- [run-pass] run-pass/allocator-alloc-one.rs stdout ----
[00:57:33] 
[00:57:33] error: compilation failed!
[00:57:33] status: exit code: 101
[00:57:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator-alloc-one.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator-alloc-one/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator-alloc-one/auxiliary"
[00:57:33] ------------------------------------------
[00:57:33] 
[00:57:33] ------------------------------------------
[00:57:33] stderr:
[00:57:33] stderr:
[00:57:33] ------------------------------------------
[00:57:33] error[E0061]: this function takes 1 parameter but 0 parameters were supplied
[00:57:33]   --> /checkout/src/test/run-pass/allocator-alloc-one.rs:17:64
[00:57:33]    |
[00:57:33] 17 |         let ptr = Global.alloc_one::<i32>().unwrap_or_else(|_| oom());
[00:57:33] 
[00:57:33] error: aborting due to previous error
[00:57:33] 
[00:57:33] For more information about this error, try `rustc --explain E0061`.
[00:57:33] For more information about this error, try `rustc --explain E0061`.
[00:57:33] 
[00:5716687.rs:76:33
[00:57:33]    |
[00:57:33] 76 |             .unwrap_or_else(|_| oom());
[00:57:33] 
[00:57:33] error: aborting due to 2 previous errors
[00:57:33] 
[00:57:33] For more information about this error, try `rustc --explain E0061`.
---
[00:57:33] ---- [run-pass] run-pass/regions-mock-codegen.rs stdout ----
[00:57:33] 
[00:57:33] error: compilation failed!
[00:57:33] status: exit code: 101
[00:57:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/regions-mock-codegen.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-mock-codegen/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-mock-codegen/auxiliary"
[00:57:33] ------------------------------------------
[00:57:33] 
[00:57:33] ------------------------------------------
[00:57:33] stderr:
[00:57:33] stderr:
[00:57:33] ------------------------------------------
[00:57:33] error[E0061]: this function takes 1 parameter but 0 parameters were supplied
[00:57:33]   --> /checkout/src/test/run-pass/regions-mock-codegen.rs:36:33
[00:57:33]    |
[00:57:33] 36 |             .unwrap_or_else(|_| oom());
[00:57:33] 
[00:57:33] error: aborting due to previous error
[00:57:33] 
[00:57:33] For more information about this error, try `rustc --explain E0061`.
---
[00:57:33] test result: FAILED. 2985 passed; 3 failed; 19 ignored; 0 measured; 0 filtered out
[00:57:33] 
[00:57:33] 
[00:57:33] 
[00:57:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -
