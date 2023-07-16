plain
travis_time:end:011bdb8c:start=1552624215207636617,finish=1552624217584035906,duration=2376399289
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:15:14] .................................................................................................... 1800/2957
[01:15:25] .................................................................................................... 1900/2957
[01:15:40] .......i......................................................................i..................... 2000/2957
[01:16:06] .................................................................................................... 2100/2957
[01:16:27] ................................................................F....test [run-pass] run-pass/mpsc_stress.rs has been running for over 60 seconds
[01:16:41] ............................................................................ii...................... 2300/2957
[01:17:00] ..............................................ii.................................................... 2400/2957
[01:17:15] .................................................................................................... 2500/2957
[01:17:40] .................................................................................................... 2600/2957
---
[01:18:31] failures:
[01:18:31] 
[01:18:31] ---- [run-pass] run-pass/raw-fat-ptr.rs stdout ----
[01:18:31] 
[01:18:31] error: test compilation failed although it shouldn't!
[01:18:31] status: exit code: 1
[01:18:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/raw-fat-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/raw-fat-ptr/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/raw-fat-ptr/auxiliary"
[01:18:31] ------------------------------------------
[01:18:31] 
[01:18:31] ------------------------------------------
[01:18:31] stderr:
[01:18:31] stderr:
[01:18:31] ------------------------------------------
[01:18:31] {"message":"`T` doesn't implement `std::fmt::Debug`","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n