plain
travis_time:end:0e65d0aa:start=1541421750992236331,finish=1541421752119611812,duration=1127375481
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:25] .................................................................................................... 2000/4990
[00:48:29] .................................................................................................... 2100/4990
[00:48:33] .................................................................................................... 2200/4990
[00:48:38] .................................................................................................... 2300/4990
[00:48:41] .................F.................................................................................. 2400/4990
[00:48:49] .................................................................................................... 2600/4990
[00:48:53] .................................................................................................... 2700/4990
[00:48:56] .................................................................................................... 2800/4990
[00:48:59] .................................................................................................... 2900/4990
---
[00:50:00] 
[00:50:00] ---- [ui] ui/issues/issue-31076.rs stdout ----
[00:50:00] diff of stderr:
[00:50:00] 
[00:50:00] 1 error[E0369]: binary operation `+` cannot be applied to type `{integer}`
[00:50:00] -   --> $DIR/typeck-issue-31076-correct-trait-impl.rs:13:13
[00:50:00] +   --> $DIR/issue-31076.rs:13:13
[00:50:00] 3    |
[00:50:00] 4 LL |     let x = 5 + 6;
[00:50:00] 
[00:50:00] 
[00:50:00] The actual stderr differed from the expected stderr.
[00:50:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076/issue-31076.stderr
[00:50:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076/issue-31076.stderr
[00:50:00] To update references, rerun the tests and pass the `--bless` flag
[00:50:00] To only update this specific test, also pass `--test-args issues/issue-31076.rs`
[00:50:00] error: 1 errors occurred comparing output.
[00:50:00] status: exit code: 1
[00:50:00] status: exit code: 1
[00:50:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31076.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076/auxiliary" "-A" "unused"
[00:50:00] ------------------------------------------
[00:50:00] 
[00:50:00] ------------------------------------------
[00:50:00] stderr:
[00:50:00] stderr:
[00:50:00] ------------------------------------------
[00:50:00] {"message":"binary operation `+` cannot be applied to type `{integer}`","code":{"code":"E0369","explanation":"\nA binary operation was attempted on a type which doesn't support it.\nErroneous code example:\n\n