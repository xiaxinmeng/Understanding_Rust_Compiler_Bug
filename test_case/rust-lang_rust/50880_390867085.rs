plain
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:44:19] 
[00:44:19] running 3007 tests
[00:44:32] .......F............................................................................................
[00:45:00] ....................................................................................................
[00:45:13] ....................................................................................................
[00:45:26] ....................................................................................................
[00:45:44] ....................................................................................................
---
[00:52:14] ---- [run-pass] run-pass/allocator-alloc-one.rs stdout ----
[00:52:14] 
[00:52:14] error: compilation failed!
[00:52:14] status: exit code: 101
[00:52:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator-alloc-one.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator-alloc-one/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator-alloc-one/auxiliary"
[00:52:14] ------------------------------------------
[00:52:14] 
[00:52:14] ------------------------------------------
[00:52:14] stderr:
[00:52:14] stderr:
[00:52:14] ------------------------------------------
[00:52:14] error[E0433]: failed to resolve. Use of undeclared type or module `Layout`
[00:52:14]    |
[00:52:14]    |
[00:52:14] 17 |         let ptr = Global.alloc_one::<i32>().unwrap_or_else(|_| oom(Layout::new::<i32>()));
[00:52:14]    |                                                                    ^^^^^^ Use of undeclared type or module `Layout`
[00:52:14] error: aborting due to previous error
[00:52:14] 
[00:52:14] For more information about this error, try `rustc --explain E0433`.
[00:52:14] 
