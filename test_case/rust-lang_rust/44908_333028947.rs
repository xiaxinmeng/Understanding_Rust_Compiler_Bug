
[00:40:45] ---- [run-pass] run-pass/issue-43132.rs stdout ----
[00:40:45] 	
[00:40:45] error: compilation failed!
[00:40:45] status: exit code: 101
[00:40:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issue-43132.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-43132.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-43132.stage2-x86_64-unknown-linux-gnu.run-pass.libaux"
[00:40:45] stdout:
[00:40:45] ------------------------------------------
[00:40:45] 
[00:40:45] ------------------------------------------
[00:40:45] stderr:
[00:40:45] ------------------------------------------
[00:40:45] error: internal compiler error: /checkout/src/librustc_trans/context.rs:694: failed to get layout for `*mut MaybeDone<(Forward<(std::boxed::Box<Future<Error=u32>>,)>, [closure@/checkout/src/test/run-pass/issue-43132.rs:20:15: 20:21])>`: the type `MaybeDone<(Forward<(std::boxed::Box<Future<Error=u32>>,)>, [closure@/checkout/src/test/run-pass/issue-43132.rs:20:15: 20:21])>` has an unknown layout
[00:40:45] 
[00:40:45] note: the compiler unexpectedly panicked. this is a bug.
[00:40:45] 
[00:40:45] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:40:45] 
[00:40:45] note: rustc 1.21.0-dev (d158897d9 2017-09-28) running on x86_64-unknown-linux-gnu
[00:40:45] 
[00:40:45] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:490:8
[00:40:45] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:40:45] 
[00:40:45] 
[00:40:45] ------------------------------------------
[00:40:45] 
[00:40:45] thread '[run-pass] run-pass/issue-43132.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2435:8
[00:40:45] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:40:45] 
[00:40:45] 
[00:40:45] failures:
[00:40:45]     [run-pass] run-pass/issue-43132.rs
[00:40:45] 
[00:40:45] test result: FAILED. 2729 passed; 1 failed; 8 ignored; 0 measured; 0 filtered out
