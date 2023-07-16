plain
travis_time:end:05563470:start=1554868913098569244,finish=1554868914132548417,duration=1033979173
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:06:31] .................................................................................................... 4400/5530
[01:06:35] .................................................................................................... 4500/5530
[01:06:38] .................................................................................................... 4600/5530
[01:06:42] .................................................................................................... 4700/5530
[01:06:48] ...F................................................................................................ 4800/5530
[01:06:54] .................................................................................................... 5000/5530
[01:06:59] .................................................................................................... 5100/5530
[01:07:02] .................................................................................................... 5200/5530
[01:07:06] .................................................................................................... 5300/5530
[01:07:06] .................................................................................................... 5300/5530
[01:07:09] .................................................................................................... 5400/5530
[01:07:11] ....................................................................i............................... 5500/5530
[01:07:12] ..............................
[01:07:12] failures:
[01:07:12] 
[01:07:12] ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
[01:07:12] 
[01:07:12] error: test compilation failed although it shouldn't!
[01:07:12] status: exit code: 1
[01:07:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/auxiliary" "-A" "unused"
[01:07:12] ------------------------------------------
[01:07:12] 
[01:07:12] ------------------------------------------
[01:07:12] stderr:
[01:07:12] stderr:
[01:07:12] ------------------------------------------
[01:07:12] {"message":"can't compare `&str` with `(&str,)`","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n