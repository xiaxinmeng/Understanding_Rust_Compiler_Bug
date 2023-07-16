plain
[00:42:15] ................................................................................................i...
[00:42:21] ............................................................i.......................................
[00:42:25] ....................................................................................................
[00:42:29] ....................................................................................................
[00:42:32] ...............................................................................................F....
[00:42:41] ....................................................................................................
[00:42:45] ....................................................................................................
[00:42:51] ....................................................................................................
[00:42:56] ....................................................................................................
---
[00:43:27] 71 
[00:43:27] 72 error: could not find defining uses
[00:43:27] 73   --> $DIR/existential_type.rs:28:1
[00:43:27] 
[00:43:27] 97 LL | | }
[00:43:27] 99 
[00:43:27] - error: aborting due to 9 previous errors
[00:43:27] + error: aborting due to 8 previous errors
[00:43:27] 101 
[00:43:27] 101 
[00:43:27] - Some errors occurred: E0277, E0308, E0310.
[00:43:27] + Some errors occurred: E0277, E0308.
[00:43:27] 103 For more information about an error, try `rustc --explain E0277`.
[00:43:27] 104 
[00:43:27] 
[00:43:27] 
[00:43:27] The actual stderr differed from the expected stderr.
[00:43:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_type.nll/existential_type.nll.stderr
[00:43:27] To update references, rerun the tests and pass the `--bless` flag
[00:43:27] To only update this specific test, also pass `--test-args existential_type.rs`
[00:43:27] error: 1 errors occurred comparing output.
[00:43:27] status: exit code: 101
[00:43:27] status: exit code: 101
[00:43:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential_type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_type.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_type.nll/auxisrc/test/ui/existential_type.rs:70:1\n   |\nLL | / fn my_iter<T>(t: T) -> MyIter<T> {\nLL | |     std::iter::once(t)\nLL | | }\n   | |_^\n\n"}
[00:43:27] {"message":"aborting due to 8 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 8 previous errors\n\n"}
[00:43:27] {"message":"Some errors occurred: E0277, E0308.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0277, E0308.\n"}
[00:43:27] {"message":"For more information about an error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0277`.\n"}
[00:43:27] ------------------------------------------
[00:43:27] 
[00:43:27] thread '[ui (nll)] ui/existential_type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:43:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:43:27] 
[00:43:27] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:43:27] 
[00:43:27] 
[00:43:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/che
