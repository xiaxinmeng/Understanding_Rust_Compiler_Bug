plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:47:50] 
[00:47:50] running 1567 tests
[00:47:53] ..................................................................................................i.
[00:47:57] ..........................F.....................................i...................................
[00:48:02] ....................................................................................................
[00:48:05] ....................................................................................................
[00:48:07] ....................................................................................................
[00:48:10] ....................................................................................................
---
[00:48:30] ....................................................................................................
[00:48:33] ....................................................................................................
[00:48:37] ................................................................................i...................
[00:48:40] The actual stderr differed from the expected stderr.
[00:48:40] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/closure_promotion/closure_promotion.stderr
[00:48:40] To update references, rerun the tests and pass the `--bless` flag
[00:48:40] To only update this specific test, also pass `--test-args const-eval/closure_promotion.rs`
[00:48:40] error: 1 errors occurred comparing output.
[00:48:40] status: exit code: 101
[00:48:40] status: exit code: 101
[00:48:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/closure_promotion.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/closure_promotion/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/closure_promotion/auxiliary" "-A" "unused"
[00:48:40] ------------------------------------------
[00:48:40] 
[00:48:40] ------------------------------------------
[00:48:40] stderr:
[00:48:40] stderr:
[00:48:40] ------------------------------------------
[00:48:40] {"message":"borrowed value does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n