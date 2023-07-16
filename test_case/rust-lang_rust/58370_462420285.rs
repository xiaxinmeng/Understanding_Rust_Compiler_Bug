plain
travis_time:end:09a1dfec:start=1549902707222763773,finish=1549902805627259766,duration=98404495993
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:00:34] .................................................................................................... 2200/5380
[01:00:38] .................................................................................................... 2300/5380
[01:00:42] .................................................................................................... 2400/5380
[01:00:46] .................................................................................................... 2500/5380
[01:00:49] ...........................................F........................................................ 2600/5380
[01:00:57] .................................................................................................... 2800/5380
[01:01:01] .................................................................................................... 2900/5380
[01:01:05] .................................................................................................... 3000/5380
[01:01:08] .................................................................................................... 3100/5380
---
[01:02:31] 
[01:02:31] ---- [ui] ui/issues/issue-35677.rs stdout ----
[01:02:31] diff of stderr:
[01:02:31] 
[01:02:31] 1 error[E0599]: no method named `is_subset` found for type `&std::collections::HashSet<T>` in the current scope
[01:02:31] +   --> $DIR/issue-35677.rs:4:10
[01:02:31] 3    |
[01:02:31] 3    |
[01:02:31] 4 LL |     this.is_subset(other)
[01:02:31] 
[01:02:31] 
[01:02:31] The actual stderr differed from the expected stderr.
[01:02:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35677/issue-35677.stderr
[01:02:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35677/issue-35677.stderr
[01:02:31] To update references, rerun the tests and pass the `--bless` flag
[01:02:31] To only update this specific test, also pass `--test-args issues/issue-35677.rs`
[01:02:31] error: 1 errors occurred comparing output.
[01:02:31] status: exit code: 1
[01:02:31] status: exit code: 1
[01:02:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35677.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35677/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35677/auxiliary" "-A" "unused"
[01:02:31] ------------------------------------------
[01:02:31] 
[01:02:31] ------------------------------------------
[01:02:31] stderr:
[01:02:31] stderr:
[01:02:31] ------------------------------------------
[01:02:31] {"message":"no method named `is_subset` found for type `&std::collections::HashSet<T>` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n