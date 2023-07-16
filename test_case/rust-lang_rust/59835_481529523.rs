plain
travis_time:end:0419fb10:start=1554866831093664378,finish=1554866832102191880,duration=1008527502
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:05:08] .................................................................................................... 4800/5530
[01:05:11] .................................................................................................... 4900/5530
[01:05:14] .................................................................................................... 5000/5530
[01:05:18] .................................................................................................... 5100/5530
[01:05:22] ..........................................................................F......................... 5200/5530
[01:05:28] .................................................................................................... 5400/5530
[01:05:31] ....................................................................i............................... 5500/5530
[01:05:32] ..............................
[01:05:32] failures:
[01:05:32] failures:
[01:05:32] 
[01:05:32] ---- [ui] ui/try-block/try-block-bad-type.rs stdout ----
[01:05:32] diff of stderr:
[01:05:32] 
[01:05:32] 6    |
[01:05:32] 7    = help: the following implementations were found:
[01:05:32] 8              <i32 as std::convert::From<bool>>
[01:05:32] -              <i32 as std::convert::From<core::num::NonZeroI32>>
[01:05:32] 10              <i32 as std::convert::From<i16>>
[01:05:32] 11              <i32 as std::convert::From<i8>>
[01:05:32] +              <i32 as std::convert::From<std::num::NonZeroI32>>
[01:05:32] 13    = note: required by `std::convert::From::from`
[01:05:32] 14 
[01:05:32] 
[01:05:32] 
[01:05:32] 
[01:05:32] The actual stderr differed from the expected stderr.
[01:05:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
[01:05:32] To update references, rerun the tests and pass the `--bless` flag
[01:05:32] To only update this specific test, also pass `--test-args try-block/try-block-bad-type.rs`
[01:05:32] error: 1 errors occurred comparing output.
[01:05:32] status: exit code: 1
[01:05:32] status: exit code: 1
[01:05:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-bad-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/auxiliary" "-A" "unused"
[01:05:32] ------------------------------------------
[01:05:32] 
[01:05:32] ------------------------------------------
[01:05:32] stderr:
[01:05:32] stderr:
[01:05:32] ------------------------------------------
[01:05:32] {"message":"the trait bound `i32: std::convert::From<&str>` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n