plain
[00:58:39] .............................i...................................................................... 600/5371
[00:58:42] .................................................................................................... 700/5371
[00:58:47] .................................................................................................... 800/5371
[00:58:52] ............................................................................i...............i....... 900/5371
[00:58:55] .............................................................................................F...... 1000/5371
[00:58:59] ...iiiii............................................................................................ 1100/5371
[00:59:04] .................................................................................................... 1300/5371
[00:59:07] .................................................................................................... 1400/5371
[00:59:09] .................................................................................................... 1500/5371
[00:59:12] ..............................................................................................i..... 1600/5371
---
[01:01:27] error: /checkout/src/test/ui/did_you_mean/issue-42764.rs:11: unexpected help message: '11:43: 11:44: try using a variant of the expected type'
[01:01:27] 
[01:01:27] error: 1 unexpected errors found, 0 expected errors not found
[01:01:27] status: exit code: 1
[01:01:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-42764.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-42764/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-42764/auxiliary" "-A" "unused"
[01:01:27]     Error {
[01:01:27]         line_num: 11,
[01:01:27]         kind: Some(
[01:01:27]             Help
---
[01:01:27] 
[01:01:27] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:01:27] 
[01:01:27] 
[01:01:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-travis_time:end:0d6555ce:start=1549462016272574206,finish=1549465704630746780,duration=3688358172574
travis_time:start:30853a3a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 15:08:24 UTC 2019
Wed, 06 Feb 2019 15:08:25 GMT
