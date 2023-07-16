plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:44:23] 
[00:44:23] running 1562 tests
[00:44:26] ..................................................................................................i.
[00:44:30] ..........................F...F.................................i...................................
[00:44:35] ....................................................................................................
[00:44:37] ....................................................................................................
[00:44:39] ....................................................................................................
[00:44:43] ....................................................................................................
[00:44:43] ....................................................................................................
[00:44:46] ....................................................................................................
[00:44:49] ....................................................................................................
[00:44:52] ....................................................................................................
[00:44:56] .......................................i............................................................
[00:44:59] ............................i.......................................................................
[00:45:03] ....................................................................................................
[00:45:07] ..............................F..................F...........................................F......
[00:45:11] ................................................F...........................i.......................
[00:45:13] failures:
[00:45:13] 
[00:45:13] ---- [ui] ui/const-eval/closure_promotion.rs stdout ----
[00:45:13] normalized stderr:
[00:45:13] normalized stderr:
[00:45:13] error[E0597]: borrowed value does not live long enough
[00:45:13]    |
[00:45:13]    |
[00:45:13] LL |     let x: &'static _ = &|| { let z = 3; z }; //~ ERROR does not live long enough
[00:45:13]    |                          ^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:45:13] LL | }
[00:45:13]    | - temporary value only lives until here
[00:45:13]    |
[00:45:13]    = note: borrowed value must be valid for the static lifetime...
[00:45:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:45:13] error: aborting due to previous error
[00:45:13] 
[00:45:13] For more information about this error, try `rustc --explain E0597`.
[00:45:13] 
[00:45:13] 
[00:45:13] 
[00:45:13] 
[00:45:13] The actual stderr differed from the expected stderr.
[00:45:13] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/closure_promotion/closure_promotion.stderr
[00:45:13] To update references, rerun the tests and pass the `--bless` flag
[00:45:13] To only update this specific test, also pass `--test-args const-eval/closure_promotion.rs`
[00:45:13] error: 1 errors occurred comparing output.
[00:45:13] status: exit code: 101
[00:45:13] status: exit code: 101
[00:45:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/closure_promotion.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/closure_promotion/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/closure_promotion/auxiliary" "-A" "unused"
[00:45:13] ------------------------------------------
[00:45:13] 
[00:45:13] ------------------------------------------
[00:45:13] stderr:
[00:45:13] stderr:
[00:45:13] ------------------------------------------
[00:45:13] {"message":"borrowed value does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n