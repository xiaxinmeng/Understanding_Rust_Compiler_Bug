plain
[00:49:44] .................................................................................................... 300/4931
[00:49:47] .................................................................................................... 400/4931
[00:49:50] .................................................................................................... 500/4931
[00:49:54] ....................i............................................................................... 600/4931
[00:49:58] .....................................................................................F.............. 700/4931
[00:50:06] .........................................................................iiiii...................... 900/4931
[00:50:10] .................................................................................................... 1000/4931
[00:50:12] .................................................................................................... 1100/4931
[00:50:14] .................................................................................................... 1200/4931
---
[00:52:04] ......i............................................................................................. 4600/4931
[00:52:08] .................................................................................................... 4700/4931
[00:52:10] .................................................................................................... 4800/4931
[00:52:13] ......................................................................i............................. 4900/4931
sts/const-eval/mod-static-with-const-fn/mod-static-with-const-fn.stderr
[00:52:14] To update references, rerun the tests and pass the `--bless` flag
[00:52:14] To only update this specific test, also pass `--test-args consts/const-eval/mod-static-with-const-fn.rs`
[00:52:14] error: 1 errors occurred comparing output.
[00:52:14] status: exit code: 1
[00:52:14] status: exit code: 1
[00:52:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/mod-static-with-const-fn.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/mod-static-with-const-fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/mod-static-with-const-fn/auxiliary" "-A" "unused"
[00:52:14] ------------------------------------------
[00:52:14] 
[00:52:14] ------------------------------------------
[00:52:14] stderr:
[00:52:14] stderr:
[00:52:14] ------------------------------------------
[00:52:14] {"message":"statements in statics are unstable (see issue #48821)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n