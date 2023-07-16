plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:45:48] 
[00:45:48] running 1568 tests
[00:45:51] ..................................................................................................i.
[00:45:55] ............................................F...................i...................................
[00:46:00] ....................................................................................................
[00:46:03] ....................................................................................................
[00:46:05] ....................................................................................................
[00:46:08] ....................................................................................................
---
[00:46:36] .................................................................................i..................
[00:46:39] ....................................................................
[00:46:39] failures:
[00:46:39] 
[00:46:39] ---- [ui] ui/const-eval/match-test-ptr-null.rs stdout ----
[00:46:39] 
[00:46:39] 
[00:46:39] 1 error[E0018]: raw pointers cannot be cast to integers in constants
[00:46:39] -   --> $DIR/match-test-ptr-null.rs:6:15
[00:46:39] +   --> $DIR/match-test-ptr-null.rs:16:15
[00:46:39] 3    |
[00:46:39] 4 LL |         match &1 as *const i32 as usize { //~ ERROR raw pointers cannot be cast to integers
[00:46:39] 
[00:46:39] 6 
[00:46:39] 7 error[E0019]: constant contains unimplemented expression type
[00:46:39] 7 error[E0019]: constant contains unimplemented expression type
[00:46:39] -   --> $DIR/match-test-ptr-null.rs:7:13
[00:46:39] +   --> $DIR/match-test-ptr-null.rs:17:13
[00:46:39] 9    |
[00:46:39] 10 LL |             0 => 42, //~ ERROR constant contains unimplemented expression type
[00:46:39] 
[00:46:39] 12 
[00:46:39] 13 error[E0080]: could not evaluate repeat length
[00:46:39] 13 error[E0080]: could not evaluate repeat length
[00:46:39] -   --> $DIR/match-test-ptr-null.rs:5:26
[00:46:39] +   --> $DIR/match-test-ptr-null.rs:15:26
[00:46:39] 15    |
[00:46:39] 16 LL |       let _: [u8; 0] = [4; { //~ ERROR could not evaluate repeat length
[00:46:39] 
[00:46:39] 
[00:46:39] The actual stderr differed from the expected stderr.
[00:46:39] The actual stderr differed from the expected stderr.
[00:46:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/match-test-ptr-null/match-test-ptr-null.stderr
[00:46:39] To update references, rerun the tests and pass the `--bless` flag
[00:46:39] To only update this specific test, also pass `--test-args const-eval/match-test-ptr-null.rs`
[00:46:39] error: 1 errors occurred comparing output.
[00:46:39] status: exit code: 101
[00:46:39] status: exit code: 101
[00:46:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/match-test-ptr-null.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/match-test-ptr-null/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/match-test-ptr-null/auxiliary" "-A" "unused"
[00:46:39] ------------------------------------------
[00:46:39] 
[00:46:39] ------------------------------------------
[00:46:39] stderr:
[00:46:39] stderr:
[00:46:39] ------------------------------------------
[00:46:39] {"message":"raw pointers cannot be cast to integers in constants","code":{"code":"E0018","explanation":"\n\nThe value of static and constant integers must be known at compile time. You\ncan't cast a pointer to an integer because the address of a pointer can\nvary.\n\nFor example, if you write:\n\n