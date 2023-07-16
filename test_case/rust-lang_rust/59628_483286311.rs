plain
travis_time:end:01c88a3b:start=1555335979079856581,finish=1555335979880535696,duration=800679115
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:06:47] .................................................................................................... 2500/5543
[01:06:51] .................................................................................................... 2600/5543
[01:06:55] .................................................................................................... 2700/5543
[01:06:59] .................................................................................................... 2800/5543
[01:07:03] ...................................................................................F....FFFF.FF..... 2900/5543
[01:07:11] .................................................................................................... 3100/5543
[01:07:14] .................................................................................................... 3200/5543
[01:07:18] .................................................................................................... 3300/5543
[01:07:22] .......................................i............................................................ 3400/5543
---
[01:08:43] diff of stderr:
[01:08:43] 
[01:08:43] 2   --> $DIR/option-as_deref.rs:4:29
[01:08:43] 3    |
[01:08:43] 4 LL |     let _result = &Some(42).as_deref();
[01:08:43] -    |                             ^^^^^^^^ help: did you mean: `as_ref`
[01:08:43] 6    |
[01:08:43] 7    = note: the method `as_deref` exists but the following trait bounds were not satisfied:
[01:08:43] 7    = note: the method `as_deref` exists but the following trait bounds were not satisfied:
[01:08:43] 8            `{integer} : std::ops::Deref`
[01:08:43] 
[01:08:43] The actual stderr differed from the expected stderr.
[01:08:43] The actual stderr differed from the expected stderr.
[01:08:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/option-as_deref/option-as_deref.stderr
[01:08:43] To update references, rerun the tests and pass the `--bless` flag
[01:08:43] To only update this specific test, also pass `--test-args issues/issue-50264-inner-deref-trait/option-as_deref.rs`
[01:08:43] error: 1 errors occurred comparing output.
[01:08:43] status: exit code: 1
[01:08:43] status: exit code: 1
[01:08:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/option-as_deref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/option-as_deref/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/option-as_deref/auxiliary" "-A" "unused"
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] stderr:
[01:08:43] stderr:
[01:08:43] ------------------------------------------
[01:08:43] {"message":"no method named `as_deref` found for type `std::option::Option<{integer}>` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n