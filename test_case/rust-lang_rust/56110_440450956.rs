plain
travis_time:end:2ec65b8c:start=1542749730926092894,finish=1542749731997702145,duration=1071609251
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:46:45] .................................................................................................... 2200/5042
[00:46:48] .................................................................................................... 2300/5042
[00:46:52] .................................................................................................... 2400/5042
[00:46:55] .................................................................................................... 2500/5042
[00:46:59] .......................................................................F............................ 2600/5042
[00:47:06] .................................................................................................... 2800/5042
[00:47:09] .................................................................................................... 2900/5042
[00:47:13] .................................................................................................... 3000/5042
[00:47:16] ...............................................i.................................................... 3100/5042
---
[00:47:57] .................................................................................................... 4500/5042
[00:48:00] .................................................................................................... 4600/5042
[00:48:03] .........i.......................................................................................... 4700/5042
[00:48:07] .................................................................................................... 4800/5042
[00:48:09] ...........................................................................F........................ 4900/5042
[00:48:13] failures:
[00:48:13] 
[00:48:13] ---- [ui] ui/issues/issue-44402.rs stdout ----
[00:48:13] 
[00:48:13] 
[00:48:13] error: test compilation failed although it shouldn't!
[00:48:13] status: exit code: 1
[00:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-44402.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44402/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44402/auxiliary" "-A" "unused"
[00:48:13] ------------------------------------------
[00:48:13] 
[00:48:13] ------------------------------------------
[00:48:13] stderr:
[00:48:13] stderr:
[00:48:13] ------------------------------------------
[00:48:13] {"message":"non-exhaustive patterns: `Some(_)` not covered","code":{"code":"E0004","explanation":"\nThis error indicates that the compiler cannot guarantee a matching patte6:11\n   |\nLL |     match x { None => () }\n   |           ^ pattern `Some(_)` not covered\n\n"}
[00:48:13] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:13] {"message":"For more information about this error, try `rustc --explain E0004`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0004`.\n"}
[00:48:13] ------------------------------------------
[00:48:13] 
[00:48:13] thread '[ui] ui/issues/issue-44402.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:48:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:13] 
[00:48:13] ---- [ui] ui/unreachable/unreachable-loop-patterns.rs stdout ----
[00:48:13] 
[00:48:13] error: ui test compiled successfully!
[00:48:13] status: exit code: 0
[00:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unreachable/unreachable-loop-patterns.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unreachable/unreachable-loop-patterns/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unreachable/unreachable-loop-patterns/auxiliary" "-A" "unused"
[00:48:13] ------------------------------------------
[00:48:13] 
[00:48:13] 
[00:48:13] ---------------------------/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6uxilxudn-osfrn5-2q1xp1m83q6vz
130760 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
130756 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
123684 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
111088 ./src/llvm/test/CodeGen
