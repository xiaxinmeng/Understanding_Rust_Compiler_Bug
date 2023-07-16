plain
travis_time:end:042adcf4:start=1549889491878821295,finish=1549889636913451617,duration=145034630322
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:03:23] .................................................................................................... 2200/5380
[01:03:27] .................................................................................................... 2300/5380
[01:03:31] .................................................................................................... 2400/5380
[01:03:36] .................................................................................................... 2500/5380
[01:03:40] ..........................................F......................................................... 2600/5380
[01:03:49] .................................................................................................... 2800/5380
[01:03:53] .................................................................................................... 2900/5380
[01:03:57] .................................................................................................... 3000/5380
[01:04:00] .................................................................................................... 3100/5380
---
[01:05:36] 
[01:05:36] ---- [ui] ui/issues/issue-35677.rs stdout ----
[01:05:36] diff of stderr:
[01:05:36] 
[01:05:36] - error[E0599]: no method named `drain` found for type `&mut std::collections::HashMap<K, V>` in the current scope
[01:05:36] -   --> $DIR/issue-35677.rs:3:10
[01:05:36] + error[E0599]: no method named `is_subset` found for type `&std::collections::HashSet<T>` in the current scope
[01:05:36] 3    |
[01:05:36] 3    |
[01:05:36] - LL |     this.drain()
[01:05:36] -    |          ^^^^^
[01:05:36] + LL |     this.is_subset(other)
[01:05:36] 6    |
[01:05:36] 6    |
[01:05:36] -    = note: the method `drain` exists but the following trait bounds were not satisfied:
[01:05:36] -            `K : std::cmp::Eq`
[01:05:36] -            `K : std::hash::Hash`
[01:05:36] +    = note: the method `is_subset` exists but the following trait bounds were not satisfied:
[01:05:36] +            `T : std::cmp::Eq`
[01:05:36] +            `T : std::hash::Hash`
[01:05:36] 11 error: aborting due to previous error
[01:05:36] 12 
[01:05:36] 
[01:05:36] 
[01:05:36] 
[01:05:36] The actual stderr differed from the expected stderr.
[01:05:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35677/issue-35677.stderr
[01:05:36] To update references, rerun the tests and pass the `--bless` flag
[01:05:36] To only update this specific test, also pass `--test-args issues/issue-35677.rs`
[01:05:36] error: 1 errors occurred comparing output.
[01:05:36] status: exit code: 1
[01:05:36] status: exit code: 1
[01:05:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35677.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35677/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35677/auxiliary" "-A" "unused"
[01:05:36] ------------------------------------------
[01:05:36] 
[01:05:36] ------------------------------------------
[01:05:36] stderr:
[01:05:36] stderr:
[01:05:36] ------------------------------------------
[01:05:36] {"message":"no method named `is_subset` found for type `&std::collections::HashSet<T>` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n