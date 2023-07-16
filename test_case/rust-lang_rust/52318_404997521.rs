plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:09] 
[00:43:09] running 1565 tests
[00:43:12] ..................................................................................................i.
[00:43:16] ..........................F..............F..F...................i...................................
[00:43:21] ....................................................................................................
[00:43:23] ....................................................................................................
[00:43:25] ....................................................................................................
[00:43:29] ....................................................................................................
---
[00:43:59] 
[00:43:59] 
[00:43:59] 
[00:43:59] The actual stderr differed from the expected stderr.
[00:43:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/closure_promotion/closure_promotion.stderr
[00:43:59] To update references, rerun the tests and pass the `--bless` flag
[00:43:59] To only update this specific test, also pass `--test-args const-eval/closure_promotion.rs`
[00:43:59] error: 1 errors occurred comparing output.
[00:43:59] status: exit code: 101
[00:43:59] status: exit code: 101
[00:43:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/closure_promotion.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/closure_promotion/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/closure_promotion/auxiliary" "-A" "unused"
[00:43:59] ------------------------------------------
[00:43:59] 
[00:43:59] ------------------------------------------
[00:43:59] stderr:
[00:43:59] stderr:
[00:43:59] ------------------------------------------
[00:43:59] {"message":"borrowed value does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n