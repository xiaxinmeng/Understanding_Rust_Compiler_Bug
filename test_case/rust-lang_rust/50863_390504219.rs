plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:42:22] 
[00:42:22] running 1446 tests
[00:42:26] .......................................................................................i............
[00:42:31] ..................................F......i..........................................................
[00:42:38] ....................................................................................................
[00:42:42] ....................................................................................................
[00:42:45] ....................................................................................................
[00:42:50] ....................................................................................................
---
[00:43:26] failures:
[00:43:26] 
[00:43:26] ---- [ui] ui/const-eval/strlen.rs stdout ----
[00:43:26] 
[00:43:26] error: test compilation failed although it shouldn't!
[00:43:26] status: exit code: 101
[00:43:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/strlen.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/strlen/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/strlen/auxiliary" "-A" "unused"
[00:43:26] ------------------------------------------
[00:43:26] 
[00:43:26] ------------------------------------------
[00:43:26] stderr:
[00:43:26] stderr:
[00:43:26] ------------------------------------------
[00:43:26] {"message":"the trait bound `[u8; 3]: std::cmp::PartialEq<&[u8; 3]>` is not satisfied","code":{"code":"E0277","explanation":"\he trait `std::cmp::PartialEq<&[u8; 3]>` is not implemented for `[u8; 3]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `[u8; 3]: std::cmp::PartialEq<&[u8; 3]>` is not satisfied\n  --> /checkout/src/test/ui/const-eval/strlen.rs:27:5\n   |\nLL |     assert_eq!(foo(), b\"foo\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't compare `[u8; 3]` with `&[u8; 3]`\n   |\n   = help: the trait `std::cmp::PartialEq<&[u8; 3]>` is not implemented for `[u8; 3]`\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:43:26] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:43:26] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:26] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:43:26] ------------------------------------------
[00:43:26] 
[00:43:26] thread '[ui] ui/const-eval/strlen.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3042:9
[00:43:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
